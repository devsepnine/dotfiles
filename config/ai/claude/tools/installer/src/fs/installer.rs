use std::path::Path;
use std::process::Command;
use anyhow::Result;
use serde_json::Value;

use crate::component::{Component, ComponentType, HookConfig};
use crate::mcp::{McpServer, McpScope};
use crate::plugin::Plugin;

pub fn install_component(component: &Component, _source_dir: &Path, dest_dir: &Path) -> Result<()> {
    match &component.component_type {
        ComponentType::Hooks => {
            // Copy hook binary
            copy_file(component)?;
            // Register hook in settings.json using hook_config
            if let Some(config) = &component.hook_config {
                register_hook_in_settings(dest_dir, component, config)?;
            }
        }
        ComponentType::ConfigFile if component.name == "settings.json" => {
            // Merge settings.json instead of overwriting
            merge_settings_json(&component.source_path, &component.dest_path)?;
        }
        _ => {
            copy_file(component)?;
        }
    }
    Ok(())
}

fn copy_file(component: &Component) -> Result<()> {
    // Create parent directory if needed
    if let Some(parent) = component.dest_path.parent() {
        std::fs::create_dir_all(parent)?;
    }

    // Copy file
    std::fs::copy(&component.source_path, &component.dest_path)?;

    // Set executable permission for shell scripts
    #[cfg(unix)]
    if component.component_type == ComponentType::Statusline
        || component.source_path.extension().map_or(false, |e| e == "sh")
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = std::fs::metadata(&component.dest_path)?.permissions();
        perms.set_mode(0o755);
        std::fs::set_permissions(&component.dest_path, perms)?;
    }

    Ok(())
}

pub fn install_mcp_server(server: &McpServer, scope: McpScope, project_path: Option<&str>, env_values: &[(String, String)]) -> Result<String> {
    // Build command: claude mcp add [options] <name> [commandOrUrl] [args...]
    let mut command = Command::new("claude");
    command.arg("mcp").arg("add");

    // Add scope flag
    command.arg("--scope").arg(scope.display());

    // Add server name
    command.arg(&server.def.name);

    // Add environment variables with -e flags AFTER server name
    for (key, value) in env_values {
        command.arg("-e").arg(format!("{}={}", key, value));
    }

    // Add transport-specific arguments
    if server.is_http() {
        // HTTP: claude mcp add --scope user name -e KEY=val -t http url
        command.arg("-t").arg("http");
        if let Some(url) = &server.def.url {
            command.arg(url);
        }
    } else {
        // stdio: claude mcp add --scope user name -e KEY=val -- command args
        command.arg("--");
        if let Some(cmd_str) = &server.def.command {
            // Split command into parts and add them
            for part in cmd_str.split_whitespace() {
                command.arg(part);
            }
        }
    }

    // Set working directory for local scope
    if let Some(path) = project_path {
        command.current_dir(path);
    }

    // Execute: claude mcp add ... (capture output to avoid TUI corruption)
    let output = command.output()?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!("Failed to install MCP server {}: {}", server.def.name, stderr.trim());
    }

    Ok(String::new())
}

pub fn remove_mcp_server(server: &McpServer) -> Result<()> {
    let cmd = server.remove_command();
    let parts: Vec<&str> = cmd.split_whitespace().collect();

    if parts.len() < 2 {
        anyhow::bail!("Invalid remove command for MCP server: {}", server.def.name);
    }

    // Capture output to avoid TUI corruption
    let output = Command::new(&parts[0])
        .args(&parts[1..])
        .output()?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!("Failed to remove MCP server {}: {}", server.def.name, stderr.trim());
    }
    Ok(())
}

pub fn install_plugin(plugin: &Plugin) -> Result<String> {
    // First, ensure the marketplace is added
    ensure_marketplace_added(plugin)?;

    // Now install the plugin with marketplace format
    let cmd = plugin.install_command();
    let parts: Vec<&str> = cmd.split_whitespace().collect();

    if parts.len() < 2 {
        anyhow::bail!("Invalid install command for plugin: {}", plugin.def.name);
    }

    // Execute: claude plugin install plugin@marketplace
    let output = Command::new(&parts[0])
        .args(&parts[1..])
        .output()?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!("Failed to install plugin {}: {}", plugin.def.name, stderr.trim());
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    Ok(stdout.trim().to_string())
}

fn ensure_marketplace_added(plugin: &Plugin) -> Result<()> {
    // Check if marketplace is already added
    let list_output = Command::new("claude")
        .args(["plugin", "marketplace", "list"])
        .output()?;

    if list_output.status.success() {
        let stdout = String::from_utf8_lossy(&list_output.stdout);
        // Check if marketplace name appears in the list
        if stdout.contains(&plugin.def.marketplace) {
            return Ok(()); // Already added
        }
    }

    // Add the marketplace
    let cmd = plugin.marketplace_add_command();
    let parts: Vec<&str> = cmd.split_whitespace().collect();

    if parts.len() < 2 {
        anyhow::bail!("Invalid marketplace add command for: {}", plugin.def.marketplace);
    }

    let output = Command::new(&parts[0])
        .args(&parts[1..])
        .output()?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!("Failed to add marketplace {}: {}", plugin.def.marketplace, stderr.trim());
    }

    Ok(())
}

pub fn remove_plugin(plugin: &Plugin) -> Result<()> {
    let cmd = plugin.remove_command();
    let parts: Vec<&str> = cmd.split_whitespace().collect();

    if parts.len() < 2 {
        anyhow::bail!("Invalid remove command for plugin: {}", plugin.def.name);
    }

    // Capture output to avoid TUI corruption
    let output = Command::new(&parts[0])
        .args(&parts[1..])
        .output()?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!("Failed to remove plugin {}: {}", plugin.def.name, stderr.trim());
    }
    Ok(())
}

pub fn set_output_style(dest_dir: &Path, style_name: &str) -> Result<()> {
    let settings_path = dest_dir.join("settings.json");

    let mut settings: Value = if settings_path.exists() {
        let content = std::fs::read_to_string(&settings_path)?;
        serde_json::from_str(&content)?
    } else {
        serde_json::json!({})
    };

    settings["outputStyle"] = serde_json::json!(style_name);

    if let Some(parent) = settings_path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    let output = serde_json::to_string_pretty(&settings)?;
    std::fs::write(&settings_path, output)?;

    Ok(())
}

pub fn set_statusline(dest_dir: &Path, script_name: &str) -> Result<()> {
    let settings_path = dest_dir.join("settings.json");

    let mut settings: Value = if settings_path.exists() {
        let content = std::fs::read_to_string(&settings_path)?;
        serde_json::from_str(&content)?
    } else {
        serde_json::json!({})
    };

    // Set statusLine configuration object
    let statusline_path = format!("~/.claude/statusline/{}", script_name);
    settings["statusLine"] = serde_json::json!({
        "type": "command",
        "command": statusline_path,
        "padding": 0
    });

    if let Some(parent) = settings_path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    let output = serde_json::to_string_pretty(&settings)?;
    std::fs::write(&settings_path, output)?;

    Ok(())
}

fn merge_settings_json(source: &Path, dest: &Path) -> Result<()> {
    let source_content = std::fs::read_to_string(source)?;
    let source_json: Value = serde_json::from_str(&source_content)?;

    let merged = if dest.exists() {
        let dest_content = std::fs::read_to_string(dest)?;
        let mut dest_json: Value = serde_json::from_str(&dest_content)?;

        // Deep merge source into dest
        merge_json_values(&mut dest_json, &source_json);
        dest_json
    } else {
        source_json
    };

    // Create parent directory if needed
    if let Some(parent) = dest.parent() {
        std::fs::create_dir_all(parent)?;
    }

    let output = serde_json::to_string_pretty(&merged)?;
    std::fs::write(dest, output)?;

    Ok(())
}

fn merge_json_values(dest: &mut Value, source: &Value) {
    match (dest, source) {
        (Value::Object(dest_map), Value::Object(source_map)) => {
            for (key, source_value) in source_map {
                match dest_map.get_mut(key) {
                    Some(dest_value) => {
                        // Special handling for hooks array - append instead of replace
                        if key == "hooks" {
                            merge_hooks(dest_value, source_value);
                        } else {
                            merge_json_values(dest_value, source_value);
                        }
                    }
                    None => {
                        dest_map.insert(key.clone(), source_value.clone());
                    }
                }
            }
        }
        (dest, source) => {
            *dest = source.clone();
        }
    }
}

fn merge_hooks(dest: &mut Value, source: &Value) {
    if let (Value::Object(dest_hooks), Value::Object(source_hooks)) = (dest, source) {
        for (hook_type, source_hook_array) in source_hooks {
            match dest_hooks.get_mut(hook_type) {
                Some(Value::Array(dest_array)) => {
                    if let Value::Array(source_array) = source_hook_array {
                        // Append source hooks that don't already exist
                        for source_item in source_array {
                            if !dest_array.contains(source_item) {
                                dest_array.push(source_item.clone());
                            }
                        }
                    }
                }
                None => {
                    dest_hooks.insert(hook_type.clone(), source_hook_array.clone());
                }
                _ => {}
            }
        }
    }
}

fn register_hook_in_settings(dest_dir: &Path, component: &Component, config: &HookConfig) -> Result<()> {
    let settings_path = dest_dir.join("settings.json");

    // Determine hook command path
    let hook_command = format!("~/.claude/hooks/{}", component.dest_path.file_name()
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_else(|| config.name.clone()));

    // Read or create settings
    let mut settings: Value = if settings_path.exists() {
        let content = std::fs::read_to_string(&settings_path)?;
        serde_json::from_str(&content)?
    } else {
        serde_json::json!({})
    };

    // Ensure hooks object exists
    if settings.get("hooks").is_none() {
        settings["hooks"] = serde_json::json!({});
    }

    // Use event from hook.yaml (e.g., "UserPromptSubmit")
    let event_name = &config.event;
    let hooks = settings.get_mut("hooks").unwrap();
    if hooks.get(event_name).is_none() {
        hooks[event_name] = serde_json::json!([]);
    }

    let event_hooks = hooks.get_mut(event_name).unwrap();
    if let Value::Array(arr) = event_hooks {
        // Check if hook already exists
        let hook_exists = arr.iter().any(|item| {
            item.get("hooks")
                .and_then(|h| h.as_array())
                .map(|hooks_arr| {
                    hooks_arr.iter().any(|hook| {
                        hook.get("command")
                            .and_then(|c| c.as_str())
                            .map(|cmd| cmd.contains(&config.name))
                            .unwrap_or(false)
                    })
                })
                .unwrap_or(false)
        });

        if !hook_exists {
            // Build hook entry based on hook.yaml
            let mut hook_entry = serde_json::json!({
                "type": config.hook_type,
                "command": hook_command
            });

            // Add timeout if specified
            if let Some(timeout) = config.timeout {
                hook_entry["timeout"] = serde_json::json!(timeout);
            }

            let new_hook = serde_json::json!({
                "hooks": [hook_entry]
            });
            arr.push(new_hook);
        }
    }

    // Write settings
    if let Some(parent) = settings_path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    let output = serde_json::to_string_pretty(&settings)?;
    std::fs::write(&settings_path, output)?;

    Ok(())
}
