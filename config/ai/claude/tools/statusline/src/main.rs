use serde::Deserialize;
use std::io::{self, Read, Write};
use std::path::Path;

#[derive(Deserialize)]
struct StatusInput {
    model: Option<Model>,
    workspace: Option<Workspace>,
}

#[derive(Deserialize)]
struct Model {
    display_name: Option<String>,
}

#[derive(Deserialize)]
struct Workspace {
    current_dir: Option<String>,
}

fn get_dir_name(path: &str) -> &str {
    Path::new(path)
        .file_name()
        .and_then(|s| s.to_str())
        .unwrap_or(path)
}

fn main() {
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        return;
    }

    let status: StatusInput = match serde_json::from_str(&input) {
        Ok(v) => v,
        Err(_) => return,
    };

    let model_name = status
        .model
        .and_then(|m| m.display_name)
        .unwrap_or_else(|| "Unknown".to_string());

    let dir_name = status
        .workspace
        .and_then(|w| w.current_dir)
        .map(|d| get_dir_name(&d).to_string())
        .unwrap_or_else(|| "~".to_string());

    print!("[{}] - /{}", model_name, dir_name);
    let _ = io::stdout().flush();
}
