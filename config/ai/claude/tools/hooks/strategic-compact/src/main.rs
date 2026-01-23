use std::env;
use std::fs;
use std::io;
use std::path::PathBuf;

const DEFAULT_THRESHOLD: u32 = 50;
const INTERVAL: u32 = 25;

fn get_counter_file() -> PathBuf {
    let pid = std::process::id();
    PathBuf::from(format!("/tmp/claude-tool-count-{}", pid))
}

fn read_count(path: &PathBuf) -> u32 {
    fs::read_to_string(path)
        .ok()
        .and_then(|s| s.trim().parse().ok())
        .unwrap_or(0)
}

fn write_count(path: &PathBuf, count: u32) -> io::Result<()> {
    fs::write(path, count.to_string())
}

fn main() {
    let threshold = env::var("COMPACT_THRESHOLD")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(DEFAULT_THRESHOLD);

    let counter_file = get_counter_file();
    let count = read_count(&counter_file) + 1;

    if let Err(e) = write_count(&counter_file, count) {
        eprintln!("[StrategicCompact] Warning: Failed to update counter: {}", e);
        return;
    }

    // Suggest compact at threshold
    if count == threshold {
        eprintln!(
            "[StrategicCompact] {} tool calls reached - consider /compact if transitioning phases",
            threshold
        );
    }

    // Suggest at regular intervals after threshold
    if count > threshold && count % INTERVAL == 0 {
        eprintln!(
            "[StrategicCompact] {} tool calls - good checkpoint for /compact if context is stale",
            count
        );
    }
}
