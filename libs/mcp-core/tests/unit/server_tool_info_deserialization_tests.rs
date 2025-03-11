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

    #[test]
    fn test_parse_tool_with_array_types() {
        // This test verifies that tools with array types in their schema are correctly deserialized
        let tool_json = json!({
            "name": "fetch",
            "description": "Retrieves URLs from the Internet and extracts their content as markdown.",
            "inputSchema": {
                "$schema": "http://json-schema.org/draft-07/schema#",
                "additionalProperties": false,
                "properties": {
                    "url": {
                        "format": "uri",
                        "type": "string"
                    },
                    "maxLength": {
                        "allOf": [
                            {
                                "type": ["number", "string"]
                            },
                            {
                                "exclusiveMinimum": 0,
                                "maximum": 1000000,
                                "type": "number"
                            }
                        ],
                        "default": 20000
                    },
                    "enableFetchImages": {
                        "default": false,
                        "type": ["boolean", "string"]
                    },
                    "raw": {
                        "default": false,
                        "type": ["boolean", "string"]
                    }
                },
                "required": ["url"],
                "type": "object"
            }
        });

        let server_id = "kazuph/mcp-fetch".to_string();

        // Parse the tool
        let tool = ServerToolInfo::from_value(tool_json.clone(), server_id)
            .expect("Failed to parse tool with array types");

        // Verify basic properties
        assert_eq!(tool.name, "fetch");
        assert_eq!(
            tool.description,
            "Retrieves URLs from the Internet and extracts their content as markdown."
        );

        // Verify schema properties with array types
        let schema = tool
            .input_schema
            .as_ref()
            .expect("Should have input schema");

        // Check url property (simple string type)
        let url_prop = schema
            .properties
            .get("url")
            .expect("Should have url property");
        assert_eq!(url_prop.r#type, "string");

        // Check enableFetchImages property (had array type)
        let enable_fetch_images_prop = schema
            .properties
            .get("enableFetchImages")
            .expect("Should have enableFetchImages property");
        assert_eq!(enable_fetch_images_prop.r#type, "boolean");

        // Check raw property (had array type)
        let raw_prop = schema
            .properties
            .get("raw")
            .expect("Should have raw property");
        assert_eq!(raw_prop.r#type, "boolean");

        // Verify required fields
        assert_eq!(schema.required, vec!["url"]);
    }

    #[test]
    fn test_parse_tool_with_complex_nested_types() {
        // This test verifies that tools with complex nested structures and array types are correctly deserialized
        let tool_json = json!({
            "name": "complex_tool",
            "description": "A tool with complex nested type structures",
            "inputSchema": {
                "properties": {
                    "complexField": {
                        "type": ["object", "null"],
                        "properties": {
                            "nestedField": {
                                "type": ["string", "number", "boolean"],
                                "description": "A field that can be multiple types"
                            },
                            "deeplyNested": {
                                "type": "object",
                                "properties": {
                                    "veryDeep": {
                                        "type": ["array", "string"],
                                        "items": {
                                            "type": ["string", "number"]
                                        }
                                    }
                                }
                            }
                        }
                    },
                    "arrayField": {
                        "type": "array",
                        "items": {
                            "type": ["string", "number"]
                        }
                    }
                },
                "required": ["complexField"],
                "type": "object"
            }
        });

        let server_id = "test/complex-server".to_string();

        // Parse the tool
        let tool = ServerToolInfo::from_value(tool_json.clone(), server_id)
            .expect("Failed to parse tool with complex nested types");

        // Verify basic properties
        assert_eq!(tool.name, "complex_tool");
        assert_eq!(
            tool.description,
            "A tool with complex nested type structures"
        );

        // Verify schema properties
        let schema = tool
            .input_schema
            .as_ref()
            .expect("Should have input schema");

        // Check complexField property (had array type)
        let complex_field = schema
            .properties
            .get("complexField")
            .expect("Should have complexField property");
        assert_eq!(complex_field.r#type, "object");

        // Check arrayField property
        let array_field = schema
            .properties
            .get("arrayField")
            .expect("Should have arrayField property");
        assert_eq!(array_field.r#type, "array");

        // Verify required fields
        assert_eq!(schema.required, vec!["complexField"]);

        // Verify the tool can be serialized back to JSON without errors
        let serialized = serde_json::to_string(&tool).expect("Should serialize without errors");
        assert!(!serialized.is_empty());
    }
}
