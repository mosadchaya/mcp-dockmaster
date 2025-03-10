use super::mcp_core::MCPCore;
use crate::mcp_installers::{
    get_claude_config, get_cursor_config, install_claude, install_cursor, is_claude_installed,
    is_cursor_installed,
};

pub trait McpCoreInstallersExt {
    fn is_claude_installed(&self) -> Result<bool, String>;
    fn install_claude(&self) -> Result<(), String>;
    fn is_cursor_installed(&self) -> Result<bool, String>;
    fn install_cursor(&self) -> Result<(), String>;
    fn get_claude_config(&self) -> Result<String, String>;
    fn get_cursor_config(&self) -> Result<String, String>;
}

// TODO REPLACE WITH PATH TO BINARY
fn get_proxy_server_path() -> String {
    format!(
        "{}/mcp-dockmaster/dist/apps/mcp-proxy-server/mcp-proxy-server-aarch64-apple-darwin",
        std::env::var("HOME").unwrap_or_default()
    )
}

impl McpCoreInstallersExt for MCPCore {
    /// Check if the database exists and has data
    fn is_claude_installed(&self) -> Result<bool, String> {
        match is_claude_installed() {
            Ok(_) => Ok(true),
            Err(err) => Err(err.to_string()),
        }
    }
    fn install_claude(&self) -> Result<(), String> {
        match install_claude(get_proxy_server_path().as_str()) {
            Ok(_) => Ok(()),
            Err(err) => Err(err.to_string()),
        }
    }
    fn is_cursor_installed(&self) -> Result<bool, String> {
        match is_cursor_installed() {
            Ok(_) => Ok(true),
            Err(err) => Err(err.to_string()),
        }
    }
    fn install_cursor(&self) -> Result<(), String> {
        match install_cursor(get_proxy_server_path().as_str()) {
            Ok(_) => Ok(()),
            Err(err) => Err(err.to_string()),
        }
    }
    fn get_claude_config(&self) -> Result<String, String> {
        match get_claude_config(get_proxy_server_path().as_str()) {
            Ok(config) => Ok(config),
            Err(err) => Err(err.to_string()),
        }
    }
    fn get_cursor_config(&self) -> Result<String, String> {
        match get_cursor_config(get_proxy_server_path().as_str()) {
            Ok(config) => Ok(config),
            Err(err) => Err(err.to_string()),
        }
    }
}
