use std::collections::HashMap;

use crate::{database::db_manager::DBManager, models::types::ServerDefinition};

/// ServerRegistry: database logic only
///
/// This module is responsible for reading/writing Server objects in the database.
/// It has no knowledge about processes or server tools in memory.
pub struct ServerRegistry {
    db_manager: DBManager,
}

impl ServerRegistry {
    pub fn new() -> Result<Self, String> {
        let db = DBManager::new()?;
        Ok(Self { db_manager: db })
    }

    /// Create a new ServerRegistry with a custom DBManager
    pub fn with_db_manager(db_manager: DBManager) -> Self {
        Self { db_manager }
    }

    /// Get a server by ID
    pub fn get_server(&self, tool_id: &str) -> Result<ServerDefinition, String> {
        self.db_manager.get_server(tool_id)
    }

    /// Get all servers
    pub fn get_all_servers(&self) -> Result<HashMap<String, ServerDefinition>, String> {
        self.db_manager.get_all_servers()
    }

    /// Save or update a server
    pub fn save_server(&self, tool_id: &str, tool: &ServerDefinition) -> Result<(), String> {
        self.db_manager.save_server(tool_id, tool)
    }

    /// Delete a server
    pub fn delete_server(&self, tool_id: &str) -> Result<(), String> {
        self.db_manager.delete_server(tool_id)
    }
}
