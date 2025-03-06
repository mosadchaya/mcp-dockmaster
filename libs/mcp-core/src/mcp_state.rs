use std::sync::Arc;

use crate::domain::traits::{ToolRepository, ProcessManager};
use crate::application::services::ToolService;

/// Application state that holds references to services and repositories
#[derive(Clone)]
pub struct MCPState {
    pub tool_service: Arc<ToolService>,
    pub tool_repository: Arc<dyn ToolRepository>,
    pub process_manager: Arc<dyn ProcessManager>,
}

impl MCPState {
    /// Create a new MCPState with the given dependencies
    pub fn new(
        tool_repository: Arc<dyn ToolRepository>,
        process_manager: Arc<dyn ProcessManager>,
    ) -> Self {
        // Create the tool service
        let tool_service = Arc::new(ToolService::new(
            tool_repository.clone(),
            process_manager.clone(),
        ));
        
        Self {
            tool_service,
            tool_repository,
            process_manager,
        }
    }
}
