pub mod app_context;
pub mod database;
pub mod dm_process;
pub mod http_server;
pub mod mcp_proxy;
pub mod models;

// Re-export commonly used types and functions
pub use app_context::AppContext;
pub use database::DBManager;
pub use dm_process::DMProcess;
pub use error::{MCPError, MCPResult};
pub use models::*;

// Initialize logging
pub fn init_logging() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info"))
        .format_timestamp_secs()
        .init();

    log::info!("MCP Core library initialized");
}
