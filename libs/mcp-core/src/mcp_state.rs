use crate::ToolRegistry;

use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Clone, Default)]
pub struct MCPState {
    pub tool_registry: Arc<RwLock<ToolRegistry>>,
}
