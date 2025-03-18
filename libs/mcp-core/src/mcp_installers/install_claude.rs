use crate::utils::process::kill_process_by_name;

use super::install_paths::get_claude_config_path;
use super::{install_errors::ClaudeError, install_paths};
use serde_json::{json, Value};
use std::{fs, path::Path};

pub fn is_claude_installed() -> Result<(), ClaudeError> {
    let config_path = get_claude_config_path()?;

    // Check if the file exists
    if !Path::new(&config_path).exists() {
        println!("❌ MCP Dockmaster not installed in CLAUDE");
        println!("Configuration file not found");
        return Err(ClaudeError::ConfigNotFound(config_path));
    }

    // Read and parse the JSON file
    let content = fs::read_to_string(&config_path)?;
    let parsed: Value =
        serde_json::from_str(&content).map_err(|e| ClaudeError::InvalidJson(e.to_string()))?;

    // Check for mcpServers
    if let Some(servers) = parsed.get("mcpServers") {
        // Check for mcp-dockmaster key
        if servers.get("mcp-dockmaster").is_some() {
            println!("✅ MCP Dockmaster installed in CLAUDE");
            Ok(())
        } else {
            println!("❌ MCP Dockmaster not installed in CLAUDE");
            println!("mcp-dockmaster missing from mcpServers");
            Err(ClaudeError::NoDockmaster)
        }
    } else {
        println!("❌ MCP Dockmaster not installed in CLAUDE");
        println!("No mcpServers found in the configuration");
        Err(ClaudeError::NoMcpServers)
    }
}

pub fn install_claude(binary_path: &str) -> Result<(), ClaudeError> {
    if is_claude_installed().is_ok() {
        return Ok(());
    }

    kill_process_by_name("Claude");

    let config_path = get_claude_config_path()?;

    // Create parent directory if it doesn't exist
    if let Some(parent) = Path::new(&config_path).parent() {
        fs::create_dir_all(parent)?;
    }

    // Create backup if file exists
    if Path::new(&config_path).exists() {
        println!("ℹ️ Creating backup of Claude config...");
        match install_paths::backup_file(&config_path) {
            Ok(backup_path) => println!("✅ Backup created at: {}", backup_path),
            Err(e) => println!("⚠️ Failed to create backup: {}", e),
        }
    }

    // Try to read existing config or create new one
    let mut config: Value = if Path::new(&config_path).exists() {
        let mut content = fs::read_to_string(&config_path)?;
        if content.is_empty() {
            content = "{}".to_string();
        }
        match serde_json::from_str(&content) {
            Ok(json) => json,
            Err(e) => {
                // If JSON is invalid, we can't fix it
                return Err(ClaudeError::InvalidJson(e.to_string()));
            }
        }
    } else {
        // Create new empty JSON object
        serde_json::json!({})
    };
    // Ensure mcpServers exists
    if config.get("mcpServers").is_none() {
        config["mcpServers"] = json!({});
    }

    // Add mcp-dockmaster configuration with the correct format
    config["mcpServers"]["mcp-dockmaster"] = json!({
        "args": [],
        "command": binary_path
    });

    // Write the updated configuration
    fs::write(&config_path, serde_json::to_string_pretty(&config)?)?;
    println!("✅ MCP Dockmaster installed in CLAUDE");
    println!("Please restart Claude to apply the changes.");
    println!("config_path: {}", config_path);
    println!("content: {}", config);
    is_claude_installed()
}

pub fn get_claude_config(binary_path: &str) -> Result<String, ClaudeError> {
    let config_path = get_claude_config_path()?;
    let config = json!({
        "mcpServers": {
            "mcp-dockmaster": {
                "args": [],
                "command": binary_path
            }
        }
    });
    
    // Format the JSON with proper indentation
    let pretty_json = serde_json::to_string_pretty(&config)?;
    
    Ok(format!(
        "
Open {config_path}

Add configuration:
```json
{pretty_json}
```
    "
    ))
}
