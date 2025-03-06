use axum::{
    routing::{get, post},
    Extension, Router,
};
use std::sync::Arc;

use crate::api::handlers::{handle_mcp_request, health_check};
use crate::application::AppContext;

pub fn routes(app_context: Arc<AppContext>) -> Router {
    Router::new()
        .route("/mcp-proxy", post(handle_mcp_request))
        .route("/health", get(health_check))
        .layer(Extension(app_context))
}
