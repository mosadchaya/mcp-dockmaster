use std::collections::HashMap;

use crate::{
    database::db_manager::DBManager,
    models::{types::ServerDefinition, types::ServerToolInfo},
};

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

    /// Save a server tool
    pub fn save_server_tool(&self, tool_info: &ServerToolInfo) -> Result<(), String> {
        // Convert ServerToolInfo to DBServerTool before saving
        // The From<&ServerToolInfo> for DBServerTool trait should be implemented in types.rs
        // and DBServerTool should be importable by types.rs for that.
        // Assuming db_manager.save_server_tool now expects &DBServerTool
        let db_tool = tool_info.into(); // Uses From<&ServerToolInfo> for DBServerTool
        self.db_manager.save_server_tool(&db_tool)
    }

    /// Get a server tool by ID and server_id
    pub fn get_server_tool(&self, tool_id: &str, server_id: &str) -> Result<ServerToolInfo, String> {
        // db_manager.get_server_tool will return DBServerTool
        let db_tool = self.db_manager.get_server_tool(tool_id, server_id)?;
        // Convert DBServerTool to ServerToolInfo
        // The From<DBServerTool> for ServerToolInfo trait should be implemented in types.rs
        Ok(db_tool.into()) // Uses From<DBServerTool> for ServerToolInfo
    }

    /// Get all server tools for a server
    pub fn get_server_tools(&self, server_id: &str) -> Result<Vec<ServerToolInfo>, String> {
        // db_manager.get_server_tools will return Vec<DBServerTool>
        let db_tools = self.db_manager.get_server_tools(server_id)?;
        // Convert Vec<DBServerTool> to Vec<ServerToolInfo>
        let server_tool_infos = db_tools.into_iter().map(ServerToolInfo::from).collect();
        Ok(server_tool_infos)
    }

    /// Delete a server tool
    pub fn delete_server_tool(&self, tool_id: &str, server_id: &str) -> Result<(), String> {
        self.db_manager.delete_server_tool(tool_id, server_id)
    }

    /// Get a setting by key
    pub fn get_setting(&self, key: &str) -> Result<String, String> {
        self.db_manager.get_setting(key)
    }

    /// Save or update a setting
    pub fn save_setting(&self, key: &str, value: &str) -> Result<(), String> {
        self.db_manager.save_setting(key, value)
    }
}
