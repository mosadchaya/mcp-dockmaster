// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use env_logger::Env;
use mcp_core::mcp_installers::{is_claude_installed, is_cursor_installed};

#[tokio::main]
async fn main() {
    // Initialize the logger with default settings
    env_logger::Builder::from_env(Env::default().default_filter_or("info"))
        .format_timestamp_secs()
        .init();

    // Log a message to confirm logger is working
    log::info!("Starting MCP Dockmaster application");

    let _ = is_cursor_installed();
    let _ = is_claude_installed();

    mcp_dockmaster_lib::run().await;
}
