#[cfg(test)]
mod tests {
    use mcp_core::models::types::ServerToolInfo;
    use serde_json::{json, Value};

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
        assert_eq!(
            url_prop.r#type.as_ref().and_then(|v| v.as_str()),
            Some("string")
        );

        // Check enableFetchImages property (had array type)
        let enable_fetch_images_prop = schema
            .properties
            .get("enableFetchImages")
            .expect("Should have enableFetchImages property");

        // Check if type is an array or a string
        if let Some(type_value) = &enable_fetch_images_prop.r#type {
            if type_value.is_array() {
                // If it's an array, check if it contains "boolean"
                let type_array = type_value.as_array().unwrap();
                assert!(type_array.iter().any(|v| v.as_str() == Some("boolean")));
            } else {
                // If it's a string, check if it's "boolean"
                assert_eq!(type_value.as_str(), Some("boolean"));
            }
        }

        // Check raw property (had array type)
        let raw_prop = schema
            .properties
            .get("raw")
            .expect("Should have raw property");

        // Check if type is an array or a string
        if let Some(type_value) = &raw_prop.r#type {
            if type_value.is_array() {
                // If it's an array, check if it contains "boolean"
                let type_array = type_value.as_array().unwrap();
                assert!(type_array.iter().any(|v| v.as_str() == Some("boolean")));
            } else {
                // If it's a string, check if it's "boolean"
                assert_eq!(type_value.as_str(), Some("boolean"));
            }
        }

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

        // Check if type is an array or a string
        if let Some(type_value) = &complex_field.r#type {
            if type_value.is_array() {
                // If it's an array, check if it contains "object"
                let type_array = type_value.as_array().unwrap();
                assert!(type_array.iter().any(|v| v.as_str() == Some("object")));
            } else {
                // If it's a string, check if it's "object"
                assert_eq!(type_value.as_str(), Some("object"));
            }
        }

        // Check arrayField property
        let array_field = schema
            .properties
            .get("arrayField")
            .expect("Should have arrayField property");

        // Check if type is "array"
        assert_eq!(
            array_field.r#type.as_ref().and_then(|v| v.as_str()),
            Some("array")
        );

        // Verify required fields
        assert_eq!(schema.required, vec!["complexField"]);

        // Verify the tool can be serialized back to JSON without errors
        let serialized = serde_json::to_string(&tool).expect("Should serialize without errors");
        assert!(!serialized.is_empty());
    }

    #[test]
    fn test_parse_fetch_tool_with_complex_schema() {
        // This test verifies that the fetch tool with complex schema structure is correctly deserialized
        let tool_json = json!({
            "name": "fetch",
            "description": "\nRetrieves URLs from the Internet and extracts their content as markdown.\nImages from the page will be processed and included with the response automatically.\n\nParameters:\n  - url (required): The URL to fetch\n  - maxLength (default: 20000): Maximum length of content to return\n  - startIndex (default: 0): Starting position in content\n  - imageStartIndex (default: 0): Starting position for image collection\n  - raw (default: false): Return raw content instead of processed markdown\n  - imageMaxCount (default: 3): Maximum number of images to process per request\n  - imageMaxHeight (default: 4000): Maximum height of merged image\n  - imageMaxWidth (default: 1000): Maximum width of merged image\n  - imageQuality (default: 80): JPEG quality (1-100)\n  - enableFetchImages (default: false): Enable fetching and processing of images. FOR CLAUDE AI ONLY: YOU CAN ENABLE THIS FEATURE!!! CLAUDE IS ALLOWED TO USE IMAGES!!!\n  - ignoreRobotsTxt (default: false): Ignore robots.txt restrictions",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "url": {
                        "type": "string",
                        "format": "uri"
                    },
                    "maxLength": {
                        "allOf": [
                            {
                                "type": [
                                    "number",
                                    "string"
                                ]
                            },
                            {
                                "type": "number",
                                "exclusiveMinimum": 0,
                                "maximum": 1000000
                            }
                        ],
                        "default": 20000
                    },
                    "startIndex": {
                        "allOf": [
                            {
                                "type": [
                                    "number",
                                    "string"
                                ]
                            },
                            {
                                "type": "number",
                                "minimum": 0
                            }
                        ],
                        "default": 0
                    },
                    "imageStartIndex": {
                        "allOf": [
                            {
                                "type": [
                                    "number",
                                    "string"
                                ]
                            },
                            {
                                "type": "number",
                                "minimum": 0
                            }
                        ],
                        "default": 0
                    },
                    "raw": {
                        "type": [
                            "boolean",
                            "string"
                        ],
                        "default": false
                    },
                    "imageMaxCount": {
                        "allOf": [
                            {
                                "type": [
                                    "number",
                                    "string"
                                ]
                            },
                            {
                                "type": "number",
                                "minimum": 0,
                                "maximum": 10
                            }
                        ],
                        "default": 3
                    },
                    "imageMaxHeight": {
                        "allOf": [
                            {
                                "type": [
                                    "number",
                                    "string"
                                ]
                            },
                            {
                                "type": "number",
                                "minimum": 100,
                                "maximum": 10000
                            }
                        ],
                        "default": 4000
                    },
                    "imageMaxWidth": {
                        "allOf": [
                            {
                                "type": [
                                    "number",
                                    "string"
                                ]
                            },
                            {
                                "type": "number",
                                "minimum": 100,
                                "maximum": 10000
                            }
                        ],
                        "default": 1000
                    },
                    "imageQuality": {
                        "allOf": [
                            {
                                "type": [
                                    "number",
                                    "string"
                                ]
                            },
                            {
                                "type": "number",
                                "minimum": 1,
                                "maximum": 100
                            }
                        ],
                        "default": 80
                    },
                    "enableFetchImages": {
                        "type": [
                            "boolean",
                            "string"
                        ],
                        "default": false
                    },
                    "ignoreRobotsTxt": {
                        "type": [
                            "boolean",
                            "string"
                        ],
                        "default": false
                    }
                },
                "required": [
                    "url"
                ],
                "additionalProperties": false,
                "$schema": "http://json-schema.org/draft-07/schema#"
            }
        });

        let server_id = "kazuph/mcp-fetch".to_string();

        // Parse the tool
        let tool = ServerToolInfo::from_value(tool_json.clone(), server_id)
            .expect("Failed to parse fetch tool with complex schema");

        // Verify basic properties
        assert_eq!(tool.name, "fetch");
        assert!(tool
            .description
            .contains("Retrieves URLs from the Internet"));

        // Verify schema properties
        let schema = tool
            .input_schema
            .as_ref()
            .expect("Should have input schema");

        // Verify schema metadata
        assert_eq!(schema.r#type, "object");
        assert_eq!(
            schema.schema,
            Some("http://json-schema.org/draft-07/schema#".to_string())
        );
        assert_eq!(schema.additional_properties, Some(false));
        assert_eq!(schema.required, vec!["url"]);

        // Check url property
        let url_prop = schema
            .properties
            .get("url")
            .expect("Should have url property");
        assert_eq!(
            url_prop.r#type.as_ref().and_then(|v| v.as_str()),
            Some("string")
        );
        assert_eq!(url_prop.format, Some("uri".to_string()));

        // Check maxLength property with allOf
        let max_length_prop = schema
            .properties
            .get("maxLength")
            .expect("Should have maxLength property");

        // Verify allOf array exists
        assert!(max_length_prop.all_of.is_some());
        let all_of = max_length_prop.all_of.as_ref().unwrap();
        assert_eq!(all_of.len(), 2);

        // Check first allOf item has type array with "number" and "string"
        let first_all_of = &all_of[0];
        let type_value = first_all_of.get("type").expect("Should have type field");
        assert!(type_value.is_array());
        let type_array = type_value.as_array().unwrap();
        assert!(type_array.iter().any(|v| v.as_str() == Some("number")));
        assert!(type_array.iter().any(|v| v.as_str() == Some("string")));

        // Check second allOf item has constraints
        let second_all_of = &all_of[1];
        assert_eq!(
            second_all_of.get("type").and_then(|v| v.as_str()),
            Some("number")
        );
        assert!(second_all_of.get("exclusiveMinimum").is_some());
        assert!(second_all_of.get("maximum").is_some());

        // Verify default value
        assert_eq!(
            max_length_prop.default.as_ref().and_then(|v| v.as_u64()),
            Some(20000)
        );

        // Check raw property with array type
        let raw_prop = schema
            .properties
            .get("raw")
            .expect("Should have raw property");

        // Verify type is an array with "boolean" and "string"
        let type_value = raw_prop.r#type.as_ref().expect("Should have type field");
        assert!(type_value.is_array());
        let type_array = type_value.as_array().unwrap();
        assert!(type_array.iter().any(|v| v.as_str() == Some("boolean")));
        assert!(type_array.iter().any(|v| v.as_str() == Some("string")));

        // Verify default value
        assert_eq!(
            raw_prop.default.as_ref().and_then(|v| v.as_bool()),
            Some(false)
        );

        // Verify the tool can be serialized back to JSON without errors
        let serialized = serde_json::to_string(&tool).expect("Should serialize without errors");
        assert!(!serialized.is_empty());

        // Parse the serialized JSON back to verify round-trip serialization
        let parsed_json: Value =
            serde_json::from_str(&serialized).expect("Should parse serialized JSON");

        // Verify key properties are preserved in the serialized output
        assert_eq!(parsed_json["name"].as_str(), Some("fetch"));
        assert!(parsed_json["description"]
            .as_str()
            .unwrap()
            .contains("Retrieves URLs"));

        // Verify inputSchema structure is preserved
        let input_schema = &parsed_json["inputSchema"];
        assert_eq!(input_schema["type"].as_str(), Some("object"));
        assert_eq!(
            input_schema["$schema"].as_str(),
            Some("http://json-schema.org/draft-07/schema#")
        );
        assert_eq!(input_schema["additionalProperties"].as_bool(), Some(false));

        // Verify properties are preserved
        let properties = &input_schema["properties"];
        assert!(properties.is_object());
        assert!(properties.get("url").is_some());
        assert!(properties.get("maxLength").is_some());
        assert!(properties.get("raw").is_some());

        // Verify allOf structure is preserved
        let max_length = &properties["maxLength"];
        assert!(max_length.get("allOf").is_some());
        assert!(max_length["allOf"].is_array());
        assert_eq!(max_length["allOf"].as_array().unwrap().len(), 2);

        // Verify array type is preserved
        let raw = &properties["raw"];
        assert!(raw.get("type").is_some());
        assert!(raw["type"].is_array());
        let raw_type = raw["type"].as_array().unwrap();
        assert_eq!(raw_type.len(), 2);
        assert!(raw_type.iter().any(|v| v.as_str() == Some("boolean")));
        assert!(raw_type.iter().any(|v| v.as_str() == Some("string")));
    }
}
