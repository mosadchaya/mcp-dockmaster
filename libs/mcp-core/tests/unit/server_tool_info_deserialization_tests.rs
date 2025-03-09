#[cfg(test)]
mod tests {
    use mcp_core::models::types::ServerToolInfo;
    use serde_json::json;

    #[test]
    fn test_parse_helius_tools_subset() {
        let tools_json = json!({
            "tools": [
                {
                    "name": "helius_get_balance",
                    "description": "Get the balance of a Solana address",
                    "inputSchema": {
                        "properties": {
                            "publicKey": {"type": "string"},
                            "commitment": {
                                "type": "string",
                                "enum": ["confirmed", "finalized", "processed"]
                            }
                        },
                        "required": ["publicKey"],
                        "type": "object"
                    }
                },
                {
                    "name": "helius_get_block_height",
                    "description": "Get the block height of the Solana blockchain",
                    "inputSchema": {
                        "properties": {
                            "commitment": {
                                "type": "string",
                                "enum": ["confirmed", "finalized", "processed"]
                            }
                        },
                        "required": [],
                        "type": "object"
                    }
                },
                {
                    "name": "helius_get_asset",
                    "description": "Get details of a digital asset by its ID",
                    "inputSchema": {
                        "properties": {
                            "id": {"type": "string"}
                        },
                        "required": ["id"],
                        "type": "object"
                    }
                }
            ]
        });

        // Extract the tools array
        let tools_array = tools_json["tools"]
            .as_array()
            .expect("tools should be an array");
        let server_id = "test_server".to_string();

        // Parse each tool individually
        let tools: Vec<ServerToolInfo> = tools_array
            .iter()
            .map(|tool_value| ServerToolInfo::from_value(tool_value.clone(), server_id.clone()))
            .collect::<Result<_, _>>()
            .expect("Failed to parse tools");

        assert_eq!(tools.len(), 3, "Expected 3 tools");

        // Verify first tool (helius_get_balance)
        let balance_tool = &tools[0];
        assert_eq!(balance_tool.name, "helius_get_balance");
        assert_eq!(
            balance_tool.description,
            "Get the balance of a Solana address"
        );
        let schema = balance_tool
            .input_schema
            .as_ref()
            .expect("Should have input schema");
        assert!(schema.properties.contains_key("publicKey"));
        assert!(schema.properties.contains_key("commitment"));
        assert_eq!(schema.required, vec!["publicKey"]);

        // Verify second tool (helius_get_block_height)
        let block_height_tool = &tools[1];
        assert_eq!(block_height_tool.name, "helius_get_block_height");
        assert_eq!(
            block_height_tool.description,
            "Get the block height of the Solana blockchain"
        );
        let schema = block_height_tool
            .input_schema
            .as_ref()
            .expect("Should have input schema");
        assert!(schema.properties.contains_key("commitment"));
        assert!(schema.required.is_empty());

        // Verify third tool (helius_get_asset)
        let asset_tool = &tools[2];
        assert_eq!(asset_tool.name, "helius_get_asset");
        assert_eq!(
            asset_tool.description,
            "Get details of a digital asset by its ID"
        );
        let schema = asset_tool
            .input_schema
            .as_ref()
            .expect("Should have input schema");
        assert!(schema.properties.contains_key("id"));
        assert_eq!(schema.required, vec!["id"]);
    }
}
