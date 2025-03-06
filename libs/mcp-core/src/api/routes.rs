use axum::{
    routing::{get, post},
    Extension, Router,
};
use std::sync::Arc;

use crate::api::handlers::{health_handler, tools_handler};
use crate::application::AppContext;

pub fn routes(app_context: Arc<AppContext>) -> Router {
    Router::new()
        .route("/mcp-proxy", post(tools_handler::handle_mcp_request))
        .route("/health", get(health_handler::health_check))
        .layer(Extension(app_context))
}
