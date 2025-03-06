use std::collections::HashMap;

use crate::{
    models::models::Tool,
    DBManager,
};

/// ToolRegistry: database logic only
/// 
/// This module is responsible for reading/writing Tool objects in the database.
/// It has no knowledge about processes or server tools in memory.
pub struct ToolRegistry {
    db: DBManager,
}

impl ToolRegistry {
    pub fn new() -> Result<Self, String> {
        let db = DBManager::new()?;
        Ok(Self {
            db,
        })
    }
    
    /// Create a new ToolRegistry with a custom DBManager
    pub fn with_db_manager(db: DBManager) -> Result<Self, String> {
        Ok(Self { db })
    }

    /// Get a tool by ID
    pub fn get_tool(&self, tool_id: &str) -> Result<Tool, String> {
        self.db.get_tool(tool_id)
    }

    /// Get all tools
    pub fn get_all_tools(&self) -> Result<HashMap<String, Tool>, String> {
        self.db.get_all_tools()
    }

    /// Save or update a tool
    pub fn save_tool(&self, tool_id: &str, tool: &Tool) -> Result<(), String> {
        self.db.save_tool(tool_id, tool)
    }

    /// Delete a tool
    pub fn delete_tool(&self, tool_id: &str) -> Result<(), String> {
        self.db.delete_tool(tool_id)
    }
}

// #[cfg(test)]
// mod tests {
//     use crate::models::models::{ToolConfiguration, ToolType};

//     use super::*;

//     #[tokio::test]
//     async fn test_tool_registration() {
//         let mut registry = ToolRegistry::default();
//         let request = ToolRegistrationRequest {
//             tool_name: "test_tool".to_string(),
//             description: "Test tool".to_string(),
//             tool_type: ToolType::Node,
//             authentication: None,
//             configuration: Some(ToolConfiguration {
//                 command: "node".to_string(),
//                 args: Some(vec!["test.js".to_string()]),
//                 env: None,
//             }),
//             distribution: None,
//         };

//         let result = registry.register_tool(request).await;
//         assert!(result.is_ok());
//     }

//     // Add more tests...
// }
