use axum::{
    routing::{get, post},
    Extension, Router,
};
use log::{error, info};
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::mcp_state::MCPState;
use super::handlers::{handle_mcp_request, health_check};

pub async fn start_http_server(state: Arc<RwLock<MCPState>>) {
    let app = Router::new()
        .route("/mcp-proxy", post(handle_mcp_request))
        .route("/health", get(health_check))
        .layer(Extension(state));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    info!("MCP HTTP server starting on {}", addr);

    tokio::spawn(async move {
        match axum::serve(tokio::net::TcpListener::bind(&addr).await.unwrap(), app).await {
            Ok(_) => info!("MCP HTTP server terminated normally"),
            Err(e) => error!("MCP HTTP server error: {}", e),
        }
    });
}
