pub mod database;
pub mod error;
pub mod http_server;
pub mod mcp_proxy;
pub mod models;
pub mod process;
pub mod registry;

// Re-export commonly used types and functions
pub use database::DatabaseManager;
pub use error::{MCPError, MCPResult};
pub use models::*;
pub use process::ProcessManager;
pub use registry::{MCPState, ToolRegistry};

// Initialize logging
pub fn init_logging() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info"))
        .format_timestamp_secs()
        .init();

    log::info!("MCP Core library initialized");
}
