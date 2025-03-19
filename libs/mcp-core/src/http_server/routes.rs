use axum::{
    routing::{get, post},
    Extension, Router,
};
use log::{error, info};
use std::net::SocketAddr;
use tower_http::cors::CorsLayer;

use crate::core::mcp_core::MCPCore;
use crate::http_server::handlers::{handle_mcp_request, health_check, sse_handler, json_rpc_handler};

pub async fn start_http_server(mcp_core: MCPCore, port: u16) -> Result<(), String> {
    let app = Router::new()
        .route("/", get(health_check))
        .route("/health", get(health_check))
        .route("/mcp/sse", get(sse_handler).post(json_rpc_handler))
        .route("/mcp", post(handle_mcp_request))
        .layer(Extension(mcp_core.clone()));

    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    info!("MCP HTTP server starting on {}", addr);

    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .map_err(|e| format!("Failed to bind to address: {}", e))?;

    // Start the server in a separate task
    tokio::spawn(async move {
        // With axum 0.8.1, we need to create a service
        let service = app.into_make_service();
        if let Err(e) = axum::serve(listener, service).await {
            error!("MCP HTTP server error: {}", e);
        } else {
            info!("MCP HTTP server terminated normally");
        }
    });

    Ok(())
}
