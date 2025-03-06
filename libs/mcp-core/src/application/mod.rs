use std::sync::Arc;

use crate::domain::traits::{ProcessManager, ToolRepository};
use crate::application::services::ToolService;
use crate::database::DBManager;
use crate::infrastructure::persistence::SqliteToolRepository;
use crate::infrastructure::process_management::TokioProcessManager;

pub mod dto;
pub mod services;

/// Application context that holds all services and repositories
pub struct AppContext {
    pub tool_service: Arc<ToolService>,
    pub tool_repository: Arc<dyn ToolRepository>,
    pub process_manager: Arc<dyn ProcessManager>,
}

impl AppContext {
    /// Create a new application context with all dependencies initialized
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
    
    /// Initialize the application context with default implementations
    pub async fn initialize() -> Result<Self, String> {
        // Create the database manager
        let db_manager = DBManager::new()
            .map_err(|e| format!("Failed to create database manager: {}", e))?;
        
        // Create the process manager
        let process_manager = Arc::new(TokioProcessManager::new());
        
        // Create the tool repository
        let tool_repository = Arc::new(SqliteToolRepository::new(
            db_manager,
            // We need to create a registry here, but this will be refactored in a future PR
            Arc::new(tokio::sync::RwLock::new(crate::registry::ToolRegistry::new().map_err(|e| format!("Failed to create tool registry: {}", e))?)),
            process_manager.clone(),
        ));
        
        // Create the application context
        Ok(Self::new(tool_repository, process_manager))
    }
}
