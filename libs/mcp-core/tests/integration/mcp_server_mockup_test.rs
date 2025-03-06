use serde_json::json;
use std::sync::Arc;

#[cfg(test)]
mod tests {
    use mcp_core::{mcp_proxy, models::models::ToolExecutionRequest, AppContext};

    use super::*;

    #[tokio::test]
    async fn test_mcp_core_with_registry() -> Result<(), String> {
        // Initialize AppContext
        let app_context = Arc::new(AppContext::new()?);

        // Get the absolute path to the script
        let current_dir = std::env::current_dir().map_err(|e| e.to_string())?;
        let script_path = current_dir
            .join("tests/integration/resources/mcp-server-hello-world/build/index.js")
            .to_string_lossy()
            .into_owned();

        eprintln!("Script path: {}", script_path);

        // Create registration request
        let request = json!({
            "tool_name": "hello_world",
            "description": "A simple hello world tool",
            "tool_type": "node",
            "authentication": null,
            "configuration": {
                "command": "node",
                "args": ["--experimental-modules", "--no-warnings", script_path]
            },
            "distribution": null
        });

        eprintln!(
            "Registering tool with configuration: {}",
            serde_json::to_string_pretty(&request).unwrap()
        );

        // Register tool
        let response =
            mcp_proxy::register_tool(&app_context, serde_json::from_value(request).unwrap())
                .await?;
        let tool_id = response.tool_id.ok_or("No tool ID returned")?;

        eprintln!("Received tool_id from registration: {}", tool_id);

        // List all available tools
        let all_tools = mcp_proxy::list_all_server_tools(&app_context).await?;
        eprintln!(
            "Available tools: {}",
            serde_json::to_string_pretty(&all_tools).unwrap()
        );

        // Execute tool
        let request = ToolExecutionRequest {
            tool_id: format!("{}:{}", tool_id, "hello_world"),
            parameters: json!({}),
        };

        let result = mcp_core::mcp_proxy::execute_proxy_tool(&app_context, request).await?;

        // Print the execution result
        eprintln!(
            "Tool execution result: {}",
            serde_json::to_string_pretty(&result).unwrap()
        );

        // Verify result
        if !result.success {
            return Err(result.error.unwrap_or_else(|| "Unknown error".to_string()));
        }

        // Verify content matches expected
        let result_value = result.result.ok_or("No result found")?;
        let content = result_value
            .get("content")
            .and_then(|c| c.as_array())
            .ok_or("Content is not an array")?;

        if content.len() != 1 {
            return Err(format!("Expected 1 content item, got {}", content.len()));
        }

        let first_content = &content[0];
        let text = first_content
            .get("text")
            .and_then(|v| v.as_str())
            .ok_or("Content text not found or not a string")?;

        if text != "hello world" {
            return Err(format!("Expected content 'hello world', got '{}'", text));
        }

        // Test hello_world_with_input
        let request = ToolExecutionRequest {
            tool_id: format!("{}:{}", tool_id, "hello_world_with_input"),
            parameters: json!({
                "message": "custom message"
            }),
        };

        let result = mcp_core::mcp_proxy::execute_proxy_tool(&app_context, request).await?;

        eprintln!(
            "Tool execution result (with input): {}",
            serde_json::to_string_pretty(&result).unwrap()
        );

        if !result.success {
            return Err(result.error.unwrap_or_else(|| "Unknown error".to_string()));
        }

        let result_value = result.result.ok_or("No result found")?;
        let content = result_value
            .get("content")
            .and_then(|c| c.as_array())
            .ok_or("Content is not an array")?;

        if content.len() != 1 {
            return Err(format!("Expected 1 content item, got {}", content.len()));
        }

        let first_content = &content[0];
        let text = first_content
            .get("text")
            .and_then(|v| v.as_str())
            .ok_or("Content text not found or not a string")?;

        if text != "hello world custom message" {
            return Err(format!(
                "Expected content 'hello world custom message', got '{}'",
                text
            ));
        }

        // Test hello_world_with_config
        let request = ToolExecutionRequest {
            tool_id: format!("{}:{}", tool_id, "hello_world_with_config"),
            parameters: json!({
                "config": "test-config"
            }),
        };

        let result = mcp_core::mcp_proxy::execute_proxy_tool(&app_context, request).await?;

        eprintln!(
            "Tool execution result (with config): {}",
            serde_json::to_string_pretty(&result).unwrap()
        );

        if !result.success {
            return Err(result.error.unwrap_or_else(|| "Unknown error".to_string()));
        }

        let result_value = result.result.ok_or("No result found")?;
        let content = result_value
            .get("content")
            .and_then(|c| c.as_array())
            .ok_or("Content is not an array")?;

        if content.len() != 1 {
            return Err(format!("Expected 1 content item, got {}", content.len()));
        }

        let first_content = &content[0];
        let text = first_content
            .get("text")
            .and_then(|v| v.as_str())
            .ok_or("Content text not found or not a string")?;

        if text != "hello configuration test-config" {
            return Err(format!(
                "Expected content 'hello configuration test-config', got '{}'",
                text
            ));
        }

        // Cleanup
        app_context.kill_all_processes().await;

        Ok(())
    }
}
