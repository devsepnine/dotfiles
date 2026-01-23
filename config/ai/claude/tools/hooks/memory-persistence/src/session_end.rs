use memory_persistence_hooks::{get_sessions_dir, format_date, format_time, eprintln_hook};
use std::fs;
use std::io::Read;

fn main() {
    if let Err(e) = run() {
        eprintln!("Error in session-end: {}", e);
        std::process::exit(1);
    }
}

fn run() -> std::io::Result<()> {
    let sessions_dir = get_sessions_dir()?;
    let today = format_date();
    let session_file = sessions_dir.join(format!("{}-session.tmp", today));

    if session_file.exists() {
        // Update Last Updated timestamp
        let mut content = String::new();
        {
            let mut file = fs::File::open(&session_file)?;
            file.read_to_string(&mut content)?;
        }

        // Replace the Last Updated line
        let current_time = format_time();
        let updated_content = if let Some(pos) = content.find("**Last Updated:**") {
            let line_start = content[..pos].rfind('\n').map(|i| i + 1).unwrap_or(0);
            let line_end = content[pos..].find('\n').map(|i| pos + i).unwrap_or(content.len());

            format!(
                "{}**Last Updated:** {}{}",
                &content[..line_start],
                current_time,
                &content[line_end..]
            )
        } else {
            content
        };

        fs::write(&session_file, updated_content)?;
        eprintln_hook(&format!("[SessionEnd] Updated session file: {}", session_file.display()));
    } else {
        // Create new session file with template
        let current_time = format_time();
        let template = format!(
r#"# Session: {}
**Date:** {}
**Started:** {}
**Last Updated:** {}

---

## Current State

[Session context goes here]

### Completed
- [ ]

### In Progress
- [ ]

### Notes for Next Session
-

### Context to Load
```
[relevant files]
```
"#,
            today, today, current_time, current_time
        );

        fs::write(&session_file, template)?;
        eprintln_hook(&format!("[SessionEnd] Created session file: {}", session_file.display()));
    }

    Ok(())
}
