use axum::{
    routing::{get, post},
    Extension, Router,
};
use log::{error, info};
use std::net::SocketAddr;
use std::sync::Arc;

use crate::app_context::AppContext;
use super::handlers::{handle_mcp_request, health_check};

pub async fn start_http_server(ctx: Arc<AppContext>) {
    let app = Router::new()
        .route("/mcp-proxy", post(handle_mcp_request))
        .route("/health", get(health_check))
        .layer(Extension(ctx));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    info!("MCP HTTP server starting on {}", addr);

    tokio::spawn(async move {
        match axum::serve(tokio::net::TcpListener::bind(&addr).await.unwrap(), app).await {
            Ok(_) => info!("MCP HTTP server terminated normally"),
            Err(e) => error!("MCP HTTP server error: {}", e),
        }
    });
}
