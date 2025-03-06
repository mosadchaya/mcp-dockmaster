use std::sync::Arc;
use tokio::sync::RwLock;

use crate::registry::ToolRegistry;

#[derive(Clone)]
pub struct MCPState {
    pub tool_registry: Arc<RwLock<ToolRegistry>>,
}

impl MCPState {
    pub fn new() -> Self {
        Self {
            tool_registry: Arc::new(RwLock::new(
                ToolRegistry::new()
                    .expect("Failed to create ToolRegistry during MCPState initialization"),
            )),
        }
    }
}
