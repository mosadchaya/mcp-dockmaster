use std::sync::Arc;
use tokio::sync::RwLock;

use crate::registry::ToolRegistry;

#[derive(Clone, Default)]
pub struct MCPState {
    pub tool_registry: Arc<RwLock<ToolRegistry>>,
}
