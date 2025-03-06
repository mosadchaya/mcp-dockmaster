use log::{error, info};
use std::net::SocketAddr;
use std::sync::Arc;

use crate::api::routes::routes;
use crate::application::AppContext;

pub async fn start_http_server(app_context: Arc<AppContext>) -> Result<(), String> {
    let app = routes(app_context);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    info!("MCP HTTP server starting on {}", addr);

    tokio::spawn(async move {
        match axum::serve(tokio::net::TcpListener::bind(&addr).await.unwrap(), app).await {
            Ok(_) => info!("MCP HTTP server terminated normally"),
            Err(e) => error!("MCP HTTP server error: {}", e),
        }
    });
    
    Ok(())
}
