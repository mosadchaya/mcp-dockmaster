use serde_json::Value;
use std::env;
use std::fs;
use std::path::Path;

fn get_claude_config_path() -> Result<String, Box<dyn std::error::Error>> {
    match env::consts::OS {
        "windows" => {
            let appdata = env::var("APPDATA")?;
            Ok(format!("{}\\Claude\\claude_desktop_config.json", appdata))
        }
        "macos" => {
            let home = env::var("HOME")?;
            Ok(format!(
                "{}/Library/Application Support/Claude/claude_desktop_config.json",
                home
            ))
        }
        os => Err(format!("Unsupported operating system: {}", os).into()),
    }
}

pub fn is_claude_installed() -> Result<bool, Box<dyn std::error::Error>> {
    let config_path = get_claude_config_path()?;

    // Check if the file exists
    if !Path::new(&config_path).exists() {
        println!("❌ MCP Dockmaster not installed in CLAUDE");
        println!("Configuration file not found");
        return Ok(false);
    }

    // Read and parse the JSON file
    let content = fs::read_to_string(config_path)?;
    let parsed: Value = serde_json::from_str(&content)?;

    // Check for mcpServers
    if let Some(servers) = parsed.get("mcpServers") {
        // Check for mcp-dockmaster key
        if servers.get("mcp-dockmaster").is_some() {
            println!("✅ MCP Dockmaster installed in CLAUDE");
            return Ok(true);
        } else {
            println!("❌ MCP Dockmaster not installed in CLAUDE");
            println!("mcp-dockmaster missing from mcpServers");
        }
    } else {
        println!("❌ MCP Dockmaster not installed in CLAUDE");
        println!("No mcpServers found in the configuration");
    }

    Ok(false)
}
