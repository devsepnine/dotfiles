use memory_persistence_hooks::{get_sessions_dir, get_learned_dir, eprintln_hook};
use std::fs;
use std::time::{SystemTime, Duration, UNIX_EPOCH};

fn main() {
    if let Err(e) = run() {
        eprintln!("Error in session-start: {}", e);
        std::process::exit(1);
    }
}

fn run() -> std::io::Result<()> {
    let sessions_dir = get_sessions_dir()?;
    let learned_dir = get_learned_dir()?;

    // Check for recent session files (last 7 days)
    let seven_days_ago = SystemTime::now()
        .checked_sub(Duration::from_secs(7 * 24 * 60 * 60))
        .unwrap_or(UNIX_EPOCH);

    let mut recent_sessions = Vec::new();

    if let Ok(entries) = fs::read_dir(&sessions_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) == Some("tmp") {
                if let Ok(metadata) = entry.metadata() {
                    if let Ok(modified) = metadata.modified() {
                        if modified > seven_days_ago {
                            recent_sessions.push((modified, path));
                        }
                    }
                }
            }
        }
    }

    if !recent_sessions.is_empty() {
        // Sort by modification time (newest first)
        recent_sessions.sort_by(|a, b| b.0.cmp(&a.0));

        eprintln_hook(&format!("[SessionStart] Found {} recent session(s)", recent_sessions.len()));
        if let Some((_, latest_path)) = recent_sessions.first() {
            eprintln_hook(&format!("[SessionStart] Latest: {}", latest_path.display()));
        }
    }

    // Check for learned skills
    let learned_count = if let Ok(entries) = fs::read_dir(&learned_dir) {
        entries
            .flatten()
            .filter(|e| {
                e.path()
                    .extension()
                    .and_then(|s| s.to_str()) == Some("md")
            })
            .count()
    } else {
        0
    };

    if learned_count > 0 {
        eprintln_hook(&format!(
            "[SessionStart] {} learned skill(s) available in {}",
            learned_count,
            learned_dir.display()
        ));
    }

    Ok(())
}
