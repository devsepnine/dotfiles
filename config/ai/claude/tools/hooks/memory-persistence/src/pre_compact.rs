use memory_persistence_hooks::{get_sessions_dir, format_datetime, format_time, eprintln_hook};
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::time::SystemTime;

fn main() {
    if let Err(e) = run() {
        eprintln!("Error in pre-compact: {}", e);
        std::process::exit(1);
    }
}

fn run() -> std::io::Result<()> {
    let sessions_dir = get_sessions_dir()?;
    let compaction_log = sessions_dir.join("compaction-log.txt");

    // Log compaction event with timestamp
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&compaction_log)?;

    writeln!(file, "[{}] Context compaction triggered", format_datetime())?;

    // If there's an active session file, note the compaction
    // Find the most recently modified .tmp file
    let mut active_session = None;
    let mut latest_time = SystemTime::UNIX_EPOCH;

    if let Ok(entries) = fs::read_dir(&sessions_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) == Some("tmp") {
                if let Ok(metadata) = entry.metadata() {
                    if let Ok(modified) = metadata.modified() {
                        if modified > latest_time {
                            latest_time = modified;
                            active_session = Some(path);
                        }
                    }
                }
            }
        }
    }

    if let Some(session_path) = active_session {
        let mut file = OpenOptions::new()
            .append(true)
            .open(&session_path)?;

        writeln!(file)?;
        writeln!(file, "---")?;
        writeln!(
            file,
            "**[Compaction occurred at {}]** - Context was summarized",
            format_time()
        )?;
    }

    eprintln_hook("[PreCompact] State saved before compaction");

    Ok(())
}
