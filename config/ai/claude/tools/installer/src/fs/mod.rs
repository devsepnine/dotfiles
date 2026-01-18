pub mod scanner;
pub mod diff;
pub mod installer;

use std::process::{Command, Stdio};

/// Create a Command to run claude CLI.
/// On Windows, uses cmd.exe /c to properly execute claude.cmd
/// stdin is set to null to prevent blocking on interactive prompts.
#[cfg(windows)]
pub fn create_claude_command() -> Command {
    let mut cmd = Command::new("cmd");
    cmd.args(["/c", "claude"]);
    cmd.stdin(Stdio::null());
    cmd
}

#[cfg(not(windows))]
pub fn create_claude_command() -> Command {
    let mut cmd = Command::new("claude");
    cmd.stdin(Stdio::null());
    cmd
}
