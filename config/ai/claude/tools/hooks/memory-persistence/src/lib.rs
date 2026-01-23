use std::path::PathBuf;
use std::fs;
use std::io;

/// Get the sessions directory path (~/.claude/sessions)
pub fn get_sessions_dir() -> io::Result<PathBuf> {
    let home = dirs::home_dir()
        .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "Home directory not found"))?;

    let sessions_dir = home.join(".claude").join("sessions");
    fs::create_dir_all(&sessions_dir)?;

    Ok(sessions_dir)
}

/// Get the learned skills directory path (~/.claude/skills/learned)
pub fn get_learned_dir() -> io::Result<PathBuf> {
    let home = dirs::home_dir()
        .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "Home directory not found"))?;

    let learned_dir = home.join(".claude").join("skills").join("learned");
    fs::create_dir_all(&learned_dir)?;

    Ok(learned_dir)
}

/// Format current timestamp as HH:MM
pub fn format_time() -> String {
    chrono::Local::now().format("%H:%M").to_string()
}

/// Format current date as YYYY-MM-DD
pub fn format_date() -> String {
    chrono::Local::now().format("%Y-%m-%d").to_string()
}

/// Format current datetime as YYYY-MM-DD HH:MM:SS
pub fn format_datetime() -> String {
    chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string()
}

/// Print to stderr with hook prefix
pub fn eprintln_hook(message: &str) {
    eprintln!("{}", message);
}
