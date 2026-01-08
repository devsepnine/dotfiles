use chrono::Local;
use regex::Regex;
use serde::Deserialize;
use std::collections::HashSet;
use std::fs::{self, OpenOptions};
use std::io::{self, Read, Write};
use std::path::PathBuf;
use walkdir::WalkDir;

#[derive(Deserialize)]
struct HookInput {
    prompt: Option<String>,
}

#[derive(Deserialize)]
struct Frontmatter {
    keywords: Vec<String>,
}

struct AgentInfo {
    filename: String,
    keywords: Vec<String>,
    content: String,
}

fn get_agents_dir() -> PathBuf {
    dirs::home_dir()
        .unwrap_or_default()
        .join(".claude")
        .join("agents")
}

fn get_log_file() -> PathBuf {
    dirs::home_dir()
        .unwrap_or_default()
        .join(".claude")
        .join("hooks")
        .join("inject-guide.log")
}

fn log(message: &str) {
    let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S");
    let log_line = format!("[{}] {}\n", timestamp, message);

    if let Ok(mut file) = OpenOptions::new()
        .create(true)
        .append(true)
        .open(get_log_file())
    {
        let _ = file.write_all(log_line.as_bytes());
    }
}

fn parse_frontmatter(content: &str) -> Option<(Frontmatter, String)> {
    let content = content.trim_start();

    if !content.starts_with("---") {
        return None;
    }

    let after_first = &content[3..];
    let end_pos = after_first.find("\n---")?;

    let yaml_str = &after_first[..end_pos];
    let rest = &after_first[end_pos + 4..];

    let frontmatter: Frontmatter = serde_yaml::from_str(yaml_str).ok()?;

    Some((frontmatter, rest.trim_start().to_string()))
}

fn load_all_agents() -> Vec<AgentInfo> {
    let agents_dir = get_agents_dir();
    let mut agents = Vec::new();

    for entry in WalkDir::new(&agents_dir)
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let path = entry.path();

        if !path.is_file() {
            continue;
        }

        if path.extension().and_then(|s| s.to_str()) != Some("md") {
            continue;
        }

        // agents/ 기준 상대 경로 사용 (하위폴더 포함)
        let relative_path = match path.strip_prefix(&agents_dir) {
            Ok(p) => p.to_string_lossy().to_string(),
            Err(_) => continue,
        };

        let file_content = match fs::read_to_string(path) {
            Ok(c) => c,
            Err(_) => continue,
        };

        if let Some((frontmatter, content)) = parse_frontmatter(&file_content) {
            agents.push(AgentInfo {
                filename: relative_path,
                keywords: frontmatter.keywords,
                content,
            });
        }
    }

    agents
}

fn find_matching_agents<'a>(prompt: &str, agents: &'a [AgentInfo]) -> Vec<&'a AgentInfo> {
    let prompt_lower = prompt.to_lowercase();
    let mut matched: Vec<&AgentInfo> = Vec::new();
    let mut seen: HashSet<&str> = HashSet::new();

    for agent in agents {
        for keyword in &agent.keywords {
            let pattern = format!(r"(?i){}", regex::escape(keyword));
            if let Ok(re) = Regex::new(&pattern) {
                if re.is_match(&prompt_lower) && !seen.contains(agent.filename.as_str()) {
                    matched.push(agent);
                    seen.insert(&agent.filename);
                    break;
                }
            }
        }
    }

    matched
}

fn main() {
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        return;
    }

    let hook_input: HookInput = match serde_json::from_str(&input) {
        Ok(v) => v,
        Err(_) => {
            log("ERROR: JSON parsing failed.");
            return;
        }
    };

    let prompt = match hook_input.prompt {
        Some(p) if !p.is_empty() => p,
        _ => return,
    };

    let agents = load_all_agents();
    let matched_agents = find_matching_agents(&prompt, &agents);

    if matched_agents.is_empty() {
        let truncated: String = prompt.chars().take(50).collect();
        log(&format!("NO MATCH: '{}...'", truncated));
        return;
    }

    let filenames: Vec<&str> = matched_agents.iter().map(|a| a.filename.as_str()).collect();
    let truncated: String = prompt.chars().take(50).collect();
    log(&format!("MATCHED: {:?} <- '{}...'", filenames, truncated));

    let mut output = String::from("\n<injected-agent>\n");
    output.push_str("You MUST follow these agent instructions:");

    for agent in &matched_agents {
        output.push_str(&format!("\n\n## {}\n\n", agent.filename));
        output.push_str(&agent.content);
    }

    output.push_str("\n</injected-agent>\n");

    print!("{}", output);
}
