pub mod api;
pub mod application;
pub mod database;
pub mod dm_process;
pub mod domain;
pub mod http_server;
pub mod infrastructure;
pub mod mcp_proxy;
pub mod mcp_state;
pub mod models; // Keep for backward compatibility
pub mod registry; // Keep for backward compatibility

// Re-export commonly used types and functions
pub use database::DBManager;
pub use dm_process::DMProcess;
pub use domain::errors::DomainError;
pub use models::*;
pub use models::error::{MCPError, MCPResult};

// Initialize logging
pub fn init_logging() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info"))
        .format_timestamp_secs()
        .init();

    log::info!("MCP Core library initialized");
}

// Initialize the application
pub async fn init_application() -> std::sync::Arc<application::AppContext> {
    // Create the application context
    let app_context = application::AppContext::initialize()
        .await
        .expect("Failed to initialize application context");
    
    let app_context = std::sync::Arc::new(app_context);
    
    // Start the HTTP server
    api::start_http_server(app_context.clone()).await
        .expect("Failed to start HTTP server");
    
    app_context
}
