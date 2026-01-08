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

struct GuideInfo {
    filename: String,
    keywords: Vec<String>,
    content: String,
}

fn get_guides_dir() -> PathBuf {
    dirs::home_dir()
        .unwrap_or_default()
        .join(".claude")
        .join("guides")
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

fn load_all_guides() -> Vec<GuideInfo> {
    let guides_dir = get_guides_dir();
    let mut guides = Vec::new();

    for entry in WalkDir::new(&guides_dir)
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

        // guides/ 기준 상대 경로 사용 (하위폴더 포함)
        let relative_path = match path.strip_prefix(&guides_dir) {
            Ok(p) => p.to_string_lossy().to_string(),
            Err(_) => continue,
        };

        let file_content = match fs::read_to_string(path) {
            Ok(c) => c,
            Err(_) => continue,
        };

        if let Some((frontmatter, content)) = parse_frontmatter(&file_content) {
            guides.push(GuideInfo {
                filename: relative_path,
                keywords: frontmatter.keywords,
                content,
            });
        }
    }

    guides
}

fn find_matching_guides<'a>(prompt: &str, guides: &'a [GuideInfo]) -> Vec<&'a GuideInfo> {
    let prompt_lower = prompt.to_lowercase();
    let mut matched: Vec<&GuideInfo> = Vec::new();
    let mut seen: HashSet<&str> = HashSet::new();

    for guide in guides {
        for keyword in &guide.keywords {
            let pattern = format!(r"(?i){}", regex::escape(keyword));
            if let Ok(re) = Regex::new(&pattern) {
                if re.is_match(&prompt_lower) && !seen.contains(guide.filename.as_str()) {
                    matched.push(guide);
                    seen.insert(&guide.filename);
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
            log("ERROR: JSON 파싱 실패");
            return;
        }
    };

    let prompt = match hook_input.prompt {
        Some(p) if !p.is_empty() => p,
        _ => return,
    };

    let guides = load_all_guides();
    let matched_guides = find_matching_guides(&prompt, &guides);

    if matched_guides.is_empty() {
        let truncated: String = prompt.chars().take(50).collect();
        log(&format!("NO MATCH: '{}...'", truncated));
        return;
    }

    let filenames: Vec<&str> = matched_guides.iter().map(|g| g.filename.as_str()).collect();
    let truncated: String = prompt.chars().take(50).collect();
    log(&format!("MATCHED: {:?} <- '{}...'", filenames, truncated));

    let mut output = String::from("\n<injected-guide>\n");
    output.push_str("다음 가이드를 반드시 따라야 합니다:");

    for guide in &matched_guides {
        output.push_str(&format!("\n\n## {}\n\n", guide.filename));
        output.push_str(&guide.content);
    }

    output.push_str("\n</injected-guide>\n");

    print!("{}", output);
}
