use std::path::Path;
use anyhow::Result;
use serde_json::Value;

use crate::app::TargetCli;
use crate::component::{Component, ComponentType, HookConfig};
use crate::mcp::{McpServer, McpScope};
use crate::plugin::Plugin;
use super::{create_claude_command, create_cli_command};

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

pub fn install_mcp_server(server: &McpServer, scope: McpScope, project_path: Option<&str>, env_values: &[(String, String)], target_cli: TargetCli) -> Result<String> {
    let mut command = create_cli_command(target_cli);
    command.arg("mcp").arg("add");

    match target_cli {
        TargetCli::Claude => {
            // Claude: claude mcp add --scope user name -e KEY=val -t http url
            // or: claude mcp add --scope user name -e KEY=val -- command args
            command.arg("--scope").arg(scope.display());
            command.arg(&server.def.name);

            // Add environment variables with -e flags
            for (key, value) in env_values {
                command.arg("-e").arg(format!("{}={}", key, value));
            }

            if server.is_http() {
                command.arg("-t").arg("http");
                if let Some(url) = &server.def.url {
                    command.arg(url);
                }
            } else {
                command.arg("--");
                if let Some(cmd_str) = &server.def.command {
                    for part in cmd_str.split_whitespace() {
                        command.arg(part);
                    }
                }
            }

            // Set working directory for local scope
            if let Some(path) = project_path {
                command.current_dir(path);
            }
        }
        TargetCli::Codex => {
            // Codex: codex mcp add --env KEY=VALUE name --url URL
            // or: codex mcp add --env KEY=VALUE name -- command args

            // Add environment variables with --env flags BEFORE name
            for (key, value) in env_values {
                command.arg("--env").arg(format!("{}={}", key, value));
            }

            // Add server name
            command.arg(&server.def.name);

            if server.is_http() {
                if let Some(url) = &server.def.url {
                    command.arg("--url").arg(url);
                }
            } else {
                command.arg("--");
                if let Some(cmd_str) = &server.def.command {
                    for part in cmd_str.split_whitespace() {
                        command.arg(part);
                    }
                }
            }
        }
    }

    // Execute command (capture output to avoid TUI corruption)
    let output = command.output()?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!("Failed to install MCP server {}: {}", server.def.name, stderr.trim());
    }

    Ok(String::new())
}

pub fn remove_mcp_server(server: &McpServer, target_cli: TargetCli) -> Result<()> {
    // Build command: <cli> mcp remove <name>
    let mut command = create_cli_command(target_cli);
    command.args(["mcp", "remove", &server.def.name]);

    // Capture output to avoid TUI corruption
    let output = command.output()?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!("Failed to remove MCP server {}: {}", server.def.name, stderr.trim());
    }
    Ok(())
}

pub fn install_plugin(plugin: &Plugin) -> Result<String> {
    // First, ensure the marketplace is added
    ensure_marketplace_added(plugin)?;

    // Build command: claude plugin install plugin@marketplace
    let plugin_ref = format!("{}@{}", plugin.def.name, plugin.def.marketplace);
    let mut command = create_claude_command();
    command.args(["plugin", "install", &plugin_ref]);

    let output = command.output()?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!("Failed to install plugin {}: {}", plugin.def.name, stderr.trim());
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    Ok(stdout.trim().to_string())
}

fn ensure_marketplace_added(plugin: &Plugin) -> Result<()> {
    // Check if marketplace is already added
    let mut list_cmd = create_claude_command();
    list_cmd.args(["plugin", "marketplace", "list"]);
    let list_output = list_cmd.output()?;

    if list_output.status.success() {
        let stdout = String::from_utf8_lossy(&list_output.stdout);
        // Check if marketplace name appears in the list
        if stdout.contains(&plugin.def.marketplace) {
            return Ok(()); // Already added
        }
    }

    // Add the marketplace: claude plugin marketplace add <source-url>
    let mut command = create_claude_command();
    command.args(["plugin", "marketplace", "add", &plugin.def.source]);

    let output = command.output()?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!("Failed to add marketplace {}: {}", plugin.def.marketplace, stderr.trim());
    }

    Ok(())
}

pub fn remove_plugin(plugin: &Plugin) -> Result<()> {
    // Build command: claude plugin uninstall <name>
    let mut command = create_claude_command();
    command.args(["plugin", "uninstall", &plugin.def.name]);

    // Capture output to avoid TUI corruption
    let output = command.output()?;

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
    // Windows doesn't support ~ expansion, use absolute path
    let statusline_path = if cfg!(windows) {
        dest_dir.join("statusline").join(script_name)
            .to_string_lossy()
            .to_string()
    } else {
        format!("~/.claude/statusline/{}", script_name)
    };
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
    let binary_name = component.dest_path.file_name()
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_else(|| {
            // Select OS-specific binary name
            if cfg!(windows) {
                format!("{}.exe", config.name)
            } else if cfg!(target_os = "macos") {
                format!("{}_macos", config.name)
            } else {
                format!("{}_linux", config.name)
            }
        });
    // Windows doesn't support ~ expansion, use absolute path
    let hook_command = if cfg!(windows) {
        dest_dir.join("hooks").join(&binary_name)
            .to_string_lossy()
            .to_string()
    } else {
        format!("~/.claude/hooks/{}", binary_name)
    };

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
