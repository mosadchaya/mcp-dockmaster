pub mod database;
pub mod mcp_proxy;
pub mod http_server;

// Re-export commonly used types and functions
pub use mcp_proxy::{MCPState, ToolRegistry, MCPError};
pub use database::DatabaseManager;

// Initialize logging
pub fn init_logging() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info"))
        .format_timestamp_secs()
        .init();
    
    log::info!("MCP Core library initialized");
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
} 