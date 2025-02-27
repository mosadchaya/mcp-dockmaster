// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod mcp_proxy;
mod http_server;

use std::sync::Arc;
use tokio::sync::RwLock;
use mcp_proxy::MCPState;

// Initialize the MCP state and start the HTTP server
async fn init_mcp_services() -> MCPState {
    // Create the MCP state
    let mcp_state = MCPState::default();
    
    // Clone the Arc<RwLock<ToolRegistry>> for the HTTP server
    let http_state = Arc::new(RwLock::new(mcp_state.clone()));
    
    // Start the HTTP server
    println!("Starting MCP HTTP server...");
    http_server::start_http_server(http_state).await;
    println!("MCP HTTP server started");
    
    mcp_state
}

#[tokio::main]
async fn main() {
    // Start the MCP services and get the shared state
    let mcp_state = init_mcp_services().await;
    
    // Start the Tauri application
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            mcp_proxy::register_tool,
            mcp_proxy::list_tools,
            mcp_proxy::list_all_server_tools,
            mcp_proxy::discover_tools,
            mcp_proxy::execute_proxy_tool,
            mcp_proxy::update_tool_status,
            mcp_proxy::uninstall_tool,
            mcp_proxy::execute_tool,
            mcp_proxy::get_all_server_data,
            mcp_proxy::get_claude_config,
            mcp_proxy::mcp_hello_world,
        ])
        .manage(mcp_state)
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
