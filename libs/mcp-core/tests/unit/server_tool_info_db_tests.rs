#[cfg(test)]
mod tests {
    use mcp_core::{
        database::db_manager::DBManager,
        models::types::{InputSchema, InputSchemaProperty, ServerToolInfo},
        types::ServerDefinition,
    };
    use serial_test::serial;
    use std::collections::HashMap;
    use tempfile::tempdir;

    // Helper function to set up a temporary database for testing
    fn setup_temp_db() -> (DBManager, tempfile::TempDir) {
        // Create a new temporary directory
        let temp_dir = tempdir().expect("Failed to create temp directory");

        let db = DBManager::with_path(temp_dir.path().join("mcp-dockmaster.db"))
            .expect("Failed to create database");
        db.apply_migrations().expect("Failed to apply migrations");
        (db, temp_dir)
    }

    #[test]
    #[serial]
    fn test_save_and_get_server_tool() {
        let (db, _temp) = setup_temp_db();

        // Create a test server first
        let server_id = "test_server";
        let server = ServerDefinition {
            name: "Test Server".to_string(),
            description: "A test server".to_string(),
            enabled: true,
            tools_type: "node".to_string(),
            entry_point: None,
            configuration: None,
            distribution: None,
        };
        db.save_server(server_id, &server).unwrap();

        // Create a test server tool
        let mut properties = HashMap::new();
        properties.insert(
            "param1".to_string(),
            InputSchemaProperty {
                description: "Test parameter".to_string(),
                r#type: "string".to_string(),
            },
        );

        let input_schema = InputSchema {
            properties,
            required: vec!["param1".to_string()],
            r#type: "object".to_string(),
        };

        let tool = ServerToolInfo {
            id: "test_tool".to_string(),
            name: "Test Tool".to_string(),
            description: "A test tool".to_string(),
            input_schema: Some(input_schema),
            server_id: server_id.to_string(),
            proxy_id: Some("proxy1".to_string()),
            is_active: true,
        };

        // Save the tool
        db.save_server_tool(&tool).unwrap();

        // Get the tool back
        let retrieved_tool = db.get_server_tool("test_tool", server_id).unwrap();

        assert_eq!(retrieved_tool.id, tool.id);
        assert_eq!(retrieved_tool.name, tool.name);
        assert_eq!(retrieved_tool.description, tool.description);
        assert_eq!(retrieved_tool.server_id, tool.server_id);
        assert_eq!(retrieved_tool.proxy_id, tool.proxy_id);
        assert_eq!(retrieved_tool.is_active, tool.is_active);

        // Check that the input_schema was correctly serialized and deserialized
        let retrieved_schema = retrieved_tool.input_schema.unwrap();
        assert_eq!(retrieved_schema.r#type, "object");
        assert_eq!(retrieved_schema.required, vec!["param1"]);
        assert!(retrieved_schema.properties.contains_key("param1"));
        assert_eq!(
            retrieved_schema.properties["param1"].description,
            "Test parameter"
        );
    }

    #[test]
    #[serial]
    fn test_get_server_tools() {
        let (db, _temp) = setup_temp_db();

        // Create a test server
        let server_id = "test_server";
        let server = ServerDefinition {
            name: "Test Server".to_string(),
            description: "A test server".to_string(),
            enabled: true,
            tools_type: "node".to_string(),
            entry_point: None,
            configuration: None,
            distribution: None,
        };
        db.save_server(server_id, &server).unwrap();

        // Create test tools
        let tool1 = ServerToolInfo {
            id: "test_tool_1".to_string(),
            name: "Test Tool 1".to_string(),
            description: "A test tool".to_string(),
            input_schema: None,
            server_id: server_id.to_string(),
            proxy_id: None,
            is_active: true,
        };

        let tool2 = ServerToolInfo {
            id: "test_tool_2".to_string(),
            name: "Test Tool 2".to_string(),
            description: "Another test tool".to_string(),
            input_schema: None,
            server_id: server_id.to_string(),
            proxy_id: None,
            is_active: true,
        };

        // Save the tools
        db.save_server_tool(&tool1).unwrap();
        db.save_server_tool(&tool2).unwrap();

        // Get all tools for the server
        let tools = db.get_server_tools(server_id).unwrap();

        assert_eq!(tools.len(), 2);
        assert!(tools.iter().any(|t| t.id == "test_tool_1"));
        assert!(tools.iter().any(|t| t.id == "test_tool_2"));
    }

    #[test]
    #[serial]
    fn test_delete_server_tool() {
        let (db, _temp) = setup_temp_db();

        // Create a test server
        let server_id = "test_server";
        let server = ServerDefinition {
            name: "Test Server".to_string(),
            description: "A test server".to_string(),
            enabled: true,
            tools_type: "node".to_string(),
            entry_point: None,
            configuration: None,
            distribution: None,
        };
        db.save_server(server_id, &server).unwrap();

        // Create a test tool
        let tool = ServerToolInfo {
            id: "test_tool".to_string(),
            name: "Test Tool".to_string(),
            description: "A test tool".to_string(),
            input_schema: None,
            server_id: server_id.to_string(),
            proxy_id: None,
            is_active: true,
        };

        // Save the tool
        db.save_server_tool(&tool).unwrap();

        // Delete the tool
        db.delete_server_tool("test_tool", server_id).unwrap();

        // Try to get the deleted tool
        let result = db.get_server_tool("test_tool", server_id);
        assert!(result.is_err());
    }

    #[test]
    #[serial]
    fn test_update_server_tool() {
        let (db, _temp) = setup_temp_db();

        // Create a test server
        let server_id = "test_server";
        let server = ServerDefinition {
            name: "Test Server".to_string(),
            description: "A test server".to_string(),
            enabled: true,
            tools_type: "node".to_string(),
            entry_point: None,
            configuration: None,
            distribution: None,
        };
        db.save_server(server_id, &server).unwrap();

        // Create a test tool
        let tool = ServerToolInfo {
            id: "test_tool".to_string(),
            name: "Test Tool".to_string(),
            description: "A test tool".to_string(),
            input_schema: None,
            server_id: server_id.to_string(),
            proxy_id: None,
            is_active: true,
        };

        // Save the tool
        db.save_server_tool(&tool).unwrap();

        // Update the tool
        let updated_tool = ServerToolInfo {
            id: "test_tool".to_string(),
            name: "Updated Tool".to_string(),
            description: "An updated test tool".to_string(),
            input_schema: None,
            server_id: server_id.to_string(),
            proxy_id: Some("new_proxy".to_string()),
            is_active: true,
        };

        db.save_server_tool(&updated_tool).unwrap();

        // Get the updated tool
        let retrieved_tool = db.get_server_tool("test_tool", server_id).unwrap();

        assert_eq!(retrieved_tool.name, "Updated Tool");
        assert_eq!(retrieved_tool.description, "An updated test tool");
        assert_eq!(retrieved_tool.proxy_id, Some("new_proxy".to_string()));
        assert_eq!(retrieved_tool.is_active, true);
    }

    #[test]
    #[serial]
    fn test_is_active_field() {
        let (db, _temp) = setup_temp_db();

        // Create a test server
        let server_id = "test_server";
        let server = ServerDefinition {
            name: "Test Server".to_string(),
            description: "A test server".to_string(),
            enabled: true,
            tools_type: "node".to_string(),
            entry_point: None,
            configuration: None,
            distribution: None,
        };
        db.save_server(server_id, &server).unwrap();

        // Create a test tool with is_active set to false
        let tool = ServerToolInfo {
            id: "test_tool".to_string(),
            name: "Test Tool".to_string(),
            description: "A test tool".to_string(),
            input_schema: None,
            server_id: server_id.to_string(),
            proxy_id: None,
            is_active: false,
        };

        // Save the tool
        db.save_server_tool(&tool).unwrap();

        // Get the tool back
        let retrieved_tool = db.get_server_tool("test_tool", server_id).unwrap();

        // Verify is_active is false
        assert_eq!(retrieved_tool.is_active, false);

        // Update the tool to set is_active to true
        let updated_tool = ServerToolInfo {
            id: "test_tool".to_string(),
            name: "Test Tool".to_string(),
            description: "A test tool".to_string(),
            input_schema: None,
            server_id: server_id.to_string(),
            proxy_id: None,
            is_active: true,
        };

        db.save_server_tool(&updated_tool).unwrap();

        // Get the updated tool
        let retrieved_updated_tool = db.get_server_tool("test_tool", server_id).unwrap();

        // Verify is_active is now true
        assert_eq!(retrieved_updated_tool.is_active, true);
    }


    #[test]
    #[serial]
    fn test_server_cascade_delete() {
        let (db, _temp) = setup_temp_db();

        // Create a test server
        let server_id = "test_server";
        let server = ServerDefinition {
            name: "Test Server".to_string(),
            description: "A test server".to_string(),
            enabled: true,
            tools_type: "node".to_string(),
            entry_point: None,
            configuration: None,
            distribution: None,
        };
        db.save_server(server_id, &server).unwrap();

        // Create a test tool
        let tool = ServerToolInfo {
            id: "test_tool".to_string(),
            name: "Test Tool".to_string(),
            description: "A test tool".to_string(),
            input_schema: None,
            server_id: server_id.to_string(),
            proxy_id: None,
            is_active: true,
        };

        // Save the tool
        db.save_server_tool(&tool).unwrap();

        // Delete the server
        db.delete_server(server_id).unwrap();

        // Try to get the tool - should fail because of cascade delete
        let result = db.get_server_tool("test_tool", server_id);
        assert!(result.is_err());
    }
}
