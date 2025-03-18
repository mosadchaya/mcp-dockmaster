use log::info;
use std::path::PathBuf;
use std::sync::Arc;

use mcp_core::core::mcp_core::MCPCore;
use mcp_core::core::mcp_core_database_ext::McpCoreDatabaseExt;
use mcp_core::mcp_server::MCPClientManager;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging with a specific env_logger to avoid conflicts
    env_logger::builder()
        .filter_level(log::LevelFilter::Info)
        .init();

    info!("Initializing MCP server example");

    // Create MCPCore instance
    let db_path = PathBuf::from("./mcp.db");
    let proxy_server_binary_path = PathBuf::from("./proxy_server");
    let mcp_core = MCPCore::new(db_path, proxy_server_binary_path);

    // Initialize the database if needed
    info!("Applying database migrations...");
    if let Err(e) = mcp_core.apply_database_migrations().await {
        eprintln!("Failed to apply database migrations: {}", e);
        return Err(e.into());
    }

    // Create the client manager
    info!("Creating client manager...");
    let client_manager = MCPClientManager::new(Arc::new(mcp_core));

    // Start the MCP server
    info!("Starting MCP server...");
    client_manager.start_server().await?;

    Ok(())
}
