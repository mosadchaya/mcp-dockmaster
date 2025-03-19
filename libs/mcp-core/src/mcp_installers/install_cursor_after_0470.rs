use std::{collections::HashMap, fs::File, path::PathBuf};

use home::home_dir;
use log::{error, info};
use serde::{Deserialize, Serialize};

use super::install_errors::CursorError;

#[derive(Serialize, Deserialize, Clone, Debug)]
struct CursorMcpGlobalConfig {
    #[serde(rename = "mcpServers")]
    pub mcp_servers: Option<HashMap<String, McpServer>>,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
enum McpServer {
    Command(CommandMcpServer),
    Sse(SseMcpServer),
}

#[derive(Deserialize, Serialize, Clone, Debug)]
struct CommandMcpServer {
    pub command: String,
    pub args: Vec<String>,
    pub env: Option<HashMap<String, String>>,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
struct SseMcpServer {
    pub url: String,
    pub env: Option<HashMap<String, String>>,
}

fn get_cursor_mcp_global_config_path() -> Result<PathBuf, CursorError> {
    let cursor_mcp_global_config_path = home_dir().unwrap().join(".cursor/mcp.json");
    Ok(cursor_mcp_global_config_path)
}

pub fn is_installed(app_name: &str) -> Result<bool, CursorError> {
    let cursor_mcp_global_config_path = get_cursor_mcp_global_config_path()?;
    let cursor_mcp_global_config_as_str = std::fs::read_to_string(&cursor_mcp_global_config_path)
        .map_err(|e| {
        error!(
            "cannot install: Failed to read Cursor MCP global config: {}",
            e.to_string()
        );
        CursorError::ConfigNotFound(cursor_mcp_global_config_path.to_string_lossy().to_string())
    })?;
    println!(
        "cursor_mcp_global_config_file: {}",
        cursor_mcp_global_config_as_str
    );
    let cursor_mcp_global_config: CursorMcpGlobalConfig =
        if cursor_mcp_global_config_as_str.is_empty() {
            CursorMcpGlobalConfig { mcp_servers: None }
        } else {
            serde_json::from_str(&cursor_mcp_global_config_as_str).map_err(|e| {
                error!(
                    "cannot install: Failed to parse Cursor MCP global config: {}",
                    e.to_string()
                );
                CursorError::InvalidJson(e.to_string())
            })?
        };

    let mcp_servers = cursor_mcp_global_config.mcp_servers.unwrap_or_default();
    Ok(mcp_servers.contains_key(app_name))
}

pub fn install_cursor(app_name: &str, binary_path: &str) -> Result<(), CursorError> {
    let cursor_mcp_global_config_path = get_cursor_mcp_global_config_path()?;
    let cursor_mcp_global_config_file = if !cursor_mcp_global_config_path.exists() {
        File::create(&cursor_mcp_global_config_path).map_err(|e| {
            error!(
                "cannot install: Failed to create Cursor MCP global config: {}",
                e.to_string()
            );
            CursorError::ConfigNotFound(cursor_mcp_global_config_path.to_string_lossy().to_string())
        })?;
        String::new()
    } else {
        std::fs::read_to_string(&cursor_mcp_global_config_path).map_err(|e| {
            error!(
                "cannot install: Failed to read Cursor MCP global config: {}",
                e.to_string()
            );
            CursorError::ConfigNotFound(cursor_mcp_global_config_path.to_string_lossy().to_string())
        })?
    };

    let mut cursor_mcp_global_config: CursorMcpGlobalConfig =
        serde_json::from_str::<Option<CursorMcpGlobalConfig>>(&cursor_mcp_global_config_file)
            .map_err(|e| {
                error!(
                    "cannot install: Failed to parse Cursor MCP global config: {}",
                    e.to_string()
                );
                CursorError::InvalidJson(e.to_string())
            })?
            .unwrap_or(CursorMcpGlobalConfig { mcp_servers: None });

    let mut servers = cursor_mcp_global_config.mcp_servers.unwrap_or_default();
    servers.insert(
        app_name.to_string(),
        McpServer::Command(CommandMcpServer {
            command: binary_path.to_string(),
            args: vec![],
            env: None,
        }),
    );

    cursor_mcp_global_config.mcp_servers = Some(servers);

    info!(
        "updated cursor_mcp_global_config: {:?}",
        cursor_mcp_global_config
    );
    // std::fs::write(
    //     &cursor_mcp_global_config_path,
    //     serde_json::to_string_pretty(&cursor_mcp_global_config.clone()).map_err(|e| {
    //         error!("cannot install: Failed to serialize Cursor MCP global config");
    //         CursorError::InvalidJson(e.to_string())
    //     })?,
    // )
    // .map_err(|e| {
    //     error!("cannot install: Failed to write Cursor MCP global config");
    //     CursorError::ConfigNotFound(cursor_mcp_global_config_path.to_string_lossy().to_string())
    // })?;

    Ok(())
}

pub fn get_cursor_config(app_name: &str, binary_path: &str) -> Result<String, CursorError> {
    Ok(format!(
        r#"
{{
  "mcpServers": {{
    ...
    "{app_name}": {{
      "command": "{binary_path}",
      "args": [],
      "env": {{}}
    }}
    ...
  }}
}}
"#
    ))
}
