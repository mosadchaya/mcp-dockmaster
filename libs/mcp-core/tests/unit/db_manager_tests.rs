#[cfg(test)]
mod tests {
    use mcp_core::{
        database::db_manager::DBManager, models::types::{ServerDefinition, ServerType},
        utils::default_storage_path,
    };
    use serial_test::serial;
    use std::fs;
    use tempfile::tempdir;

    // Helper function to clean up any existing default database
    fn cleanup_default_db() {
        if let Ok(storage_path) = default_storage_path() {
            let db_path = storage_path.join("mcp_dockmaster.db");
            let _ = fs::remove_file(db_path);
        }
    }

    // Helper function to set up a temporary database for testing
    fn setup_temp_db() -> (DBManager, tempfile::TempDir) {
        // Clean up any existing default database first
        cleanup_default_db();

        // Create a new temporary directory
        let temp_dir = tempdir().expect("Failed to create temp directory");

        let db = DBManager::with_path(temp_dir.path().join("mcp-dockmaster.db"))
            .expect("Failed to create database");
        db.apply_migrations().expect("Failed to apply migrations");
        (db, temp_dir)
    }

    #[test]
    #[serial]
    fn test_save_and_get_server() {
        let (db, _temp) = setup_temp_db();

        // Create a test tool
        let tool_id = "test_tool";
        let tool = ServerDefinition {
            name: "Test Tool".to_string(),
            description: "A test tool".to_string(),
            enabled: true,
            tools_type: "node".to_string(),
            entry_point: None,
            configuration: None,
            distribution: None,
            server_type: ServerType::Package,
            working_directory: None,
            executable_path: None,
        };

        // Save the tool
        db.save_server(tool_id, &tool).unwrap();

        // Get the tool back
        let retrieved_tool = db.get_server(tool_id).unwrap();

        assert_eq!(retrieved_tool.name, tool.name);
        assert_eq!(retrieved_tool.description, tool.description);
        assert_eq!(retrieved_tool.enabled, tool.enabled);
        assert_eq!(retrieved_tool.tools_type, tool.tools_type);
    }

    #[test]
    #[serial]
    fn test_get_all_servers() {
        let (db, _temp) = setup_temp_db();

        // Create test tools
        let tool1 = ServerDefinition {
            name: "Test Tool 1".to_string(),
            description: "A test tool".to_string(),
            enabled: true,
            tools_type: "node".to_string(),
            entry_point: None,
            configuration: None,
            distribution: None,
            server_type: ServerType::Package,
            working_directory: None,
            executable_path: None,
        };

        let tool2 = ServerDefinition {
            name: "Test Tool 2".to_string(),
            description: "Another test tool".to_string(),
            enabled: false,
            tools_type: "python".to_string(),
            entry_point: None,
            configuration: None,
            distribution: None,
            server_type: ServerType::Package,
            working_directory: None,
            executable_path: None,
        };

        // Save the tools
        db.save_server("test_tool_1", &tool1).unwrap();
        db.save_server("test_tool_2", &tool2).unwrap();

        // Get all tools
        let tools = db.get_all_servers().unwrap();

        assert_eq!(tools.len(), 2);
        assert!(tools.contains_key("test_tool_1"));
        assert!(tools.contains_key("test_tool_2"));
    }

    #[test]
    #[serial]
    fn test_delete_server() {
        let (db, _temp) = setup_temp_db();

        // Create a test tool
        let tool_id = "test_tool";
        let tool = ServerDefinition {
            name: "Test Tool".to_string(),
            description: "A test tool".to_string(),
            enabled: true,
            tools_type: "node".to_string(),
            entry_point: None,
            configuration: None,
            distribution: None,
            server_type: ServerType::Package,
            working_directory: None,
            executable_path: None,
        };

        // Save the tool
        db.save_server(tool_id, &tool).unwrap();

        // Delete the tool
        db.delete_server(tool_id).unwrap();

        // Try to get the deleted tool
        let result = db.get_server(tool_id);
        assert!(result.is_err());
    }

    #[test]
    #[serial]
    fn test_update_server() {
        let (db, _temp) = setup_temp_db();

        // Create a test tool
        let tool_id = "test_tool";
        let tool = ServerDefinition {
            name: "Test Tool".to_string(),
            description: "A test tool".to_string(),
            enabled: true,
            tools_type: "node".to_string(),
            entry_point: None,
            configuration: None,
            distribution: None,
            server_type: ServerType::Package,
            working_directory: None,
            executable_path: None,
        };

        // Save the tool
        db.save_server(tool_id, &tool).unwrap();

        // Update the tool
        let mut updated_tool = tool.clone();
        updated_tool.description = "Updated description".to_string();
        updated_tool.enabled = false;

        db.save_server(tool_id, &updated_tool).unwrap();

        // Get the updated tool
        let retrieved_tool = db.get_server(tool_id).unwrap();

        assert_eq!(retrieved_tool.description, "Updated description");
        assert!(!retrieved_tool.enabled);
    }

    #[test]
    #[serial]
    fn test_clear_database() {
        let (mut db, _temp) = setup_temp_db();

        // Create and save a sample tool
        let tool = ServerDefinition {
            name: "test_tool".to_string(),
            description: "A test tool".to_string(),
            enabled: true,
            tools_type: "test".to_string(),
            entry_point: None,
            configuration: None,
            distribution: None,
            server_type: ServerType::Package,
            working_directory: None,
            executable_path: None,
        };

        db.save_server("test_tool", &tool)
            .expect("Failed to save tool");

        // Clear the database
        db.clear_database().expect("Failed to clear database");

        // Verify the database is empty
        let tools = db.get_all_servers().expect("Failed to get all tools");
        assert!(tools.is_empty());
    }

    #[test]
    #[serial]
    fn test_multiple_migrations() {
        let (db, _temp) = setup_temp_db();

        // Migrations were already applied once in setup_temp_db
        // Apply migrations again to verify idempotency
        db.apply_migrations()
            .expect("Failed to apply migrations second time");

        // Second time to check that it's idempotent
        db.apply_migrations()
            .expect("Failed to apply migrations second time");

        // Verify we can still perform normal operations
        let tool = ServerDefinition {
            name: "test_tool".to_string(),
            description: "A test tool".to_string(),
            enabled: true,
            tools_type: "test".to_string(),
            entry_point: None,
            configuration: None,
            distribution: None,
            server_type: ServerType::Package,
            working_directory: None,
            executable_path: None,
        };

        // Save and retrieve to verify DB is still working
        db.save_server("test_tool", &tool)
            .expect("Failed to save tool after multiple migrations");
        let loaded_tool = db
            .get_server("test_tool")
            .expect("Failed to get tool after multiple migrations");
        assert_eq!(loaded_tool.name, "test_tool");

        db.apply_migrations()
            .expect("Failed to apply migrations second time");

        db.apply_migrations()
            .expect("Failed to apply migrations second time");
    }
}
