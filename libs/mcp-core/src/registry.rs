use crate::error::{MCPError, MCPResult};
use crate::models::models::{
    ToolId, ToolMetadata, ToolRegistrationRequest, ToolRegistrationResponse,
};
use crate::process::ProcessManager;
use log::{error, info};
use serde_json::{json, Value};
use std::collections::HashMap;
use tokio::time::Duration;

#[derive(Default)]
pub struct ToolRegistry {
    pub tools: HashMap<ToolId, ToolMetadata>,
    pub processes: HashMap<ToolId, Option<ProcessManager>>,
    pub server_tools: HashMap<ToolId, Vec<Value>>,
}

impl ToolRegistry {
    pub async fn register_tool(
        &mut self,
        request: ToolRegistrationRequest,
    ) -> MCPResult<ToolRegistrationResponse> {
        info!("Starting registration of tool: {}", request.tool_name);

        // Generate tool ID
        let tool_id = ToolId::new(format!("tool_{}", self.tools.len() + 1));

        // Create tool metadata
        let metadata = ToolMetadata {
            name: request.tool_name.clone(),
            description: request.description.clone(),
            tool_type: request.tool_type.clone(),
            enabled: true,
            configuration: request.configuration.clone(),
            process_running: false,
            tool_count: 0,
        };

        // Store the tool metadata
        self.tools.insert(tool_id.clone(), metadata);

        // Create empty tools list
        self.server_tools.insert(tool_id.clone(), Vec::new());

        // Extract environment variables if they exist
        let env_vars = if let Some(auth) = &request.authentication {
            if let Some(env) = auth.get("env") {
                let mut env_map = HashMap::new();
                if let Some(env_obj) = env.as_object() {
                    for (key, value) in env_obj {
                        if let Some(value_str) = value.as_str() {
                            env_map.insert(key.clone(), value_str.to_string());
                        }
                    }
                }
                Some(env_map)
            } else {
                None
            }
        } else {
            None
        };

        // Get the configuration
        let config = request
            .configuration
            .ok_or_else(|| MCPError::ConfigurationError("Configuration is required".to_string()))?;

        // Spawn the process
        let process_manager =
            ProcessManager::new(&tool_id, &request.tool_type, &config, env_vars.as_ref()).await?;

        // Store the process
        self.processes
            .insert(tool_id.clone(), Some(process_manager));

        // Wait for server to initialize
        tokio::time::sleep(Duration::from_secs(2)).await;

        // Try to discover tools
        match self.discover_tools(&tool_id).await {
            Ok(tools) => {
                self.server_tools.insert(tool_id.clone(), tools);
            }
            Err(e) => {
                error!("Error discovering tools: {}", e);
                // Add default tool
                self.server_tools.insert(
                    tool_id.clone(),
                    vec![json!({
                        "id": "main",
                        "name": request.tool_name,
                        "description": request.description
                    })],
                );
            }
        }

        Ok(ToolRegistrationResponse {
            success: true,
            message: "Tool registered successfully".to_string(),
            tool_id: Some(tool_id),
        })
    }

    pub async fn discover_tools(&mut self, tool_id: &ToolId) -> MCPResult<Vec<Value>> {
        let process = self
            .processes
            .get_mut(tool_id)
            .ok_or_else(|| MCPError::ServerNotFound(tool_id.to_string()))?
            .as_mut()
            .ok_or_else(|| MCPError::ServerNotFound(tool_id.to_string()))?;

        let result = process.send_command("tools/list", json!({})).await?;

        if let Some(tools) = result.as_array() {
            Ok(tools.clone())
        } else {
            Ok(Vec::new())
        }
    }

    pub async fn restart_all_tools(&mut self) -> MCPResult<Vec<MCPResult<()>>> {
        // Collect tool IDs first
        let tool_ids: Vec<_> = self.tools.keys().cloned().collect();

        // Process each tool sequentially to avoid borrow checker issues
        let mut results = Vec::new();
        for id in tool_ids {
            let result = tokio::time::timeout(Duration::from_secs(30), self.restart_tool(&id))
                .await
                .unwrap_or_else(|_| Err(MCPError::TimeoutError(id.to_string())));
            results.push(result);
        }

        Ok(results)
    }

    pub async fn restart_tool(&mut self, tool_id: &ToolId) -> MCPResult<()> {
        // Kill existing process if running
        if let Some(Some(process)) = self.processes.get_mut(tool_id) {
            process.kill().await?;
        }

        // Get tool metadata
        let metadata = self
            .tools
            .get(tool_id)
            .ok_or_else(|| MCPError::ToolError(format!("Tool not found: {}", tool_id)))?;

        // Skip if disabled
        if !metadata.enabled {
            return Ok(());
        }

        // Get configuration
        let config = metadata
            .configuration
            .as_ref()
            .ok_or_else(|| MCPError::ConfigurationError("Configuration missing".to_string()))?;

        // Spawn new process
        let process_manager =
            ProcessManager::new(tool_id, &metadata.tool_type, config, config.env.as_ref()).await?;

        // Store new process
        self.processes
            .insert(tool_id.clone(), Some(process_manager));

        // Wait for initialization
        tokio::time::sleep(Duration::from_secs(2)).await;

        // Rediscover tools
        match self.discover_tools(tool_id).await {
            Ok(tools) => {
                self.server_tools.insert(tool_id.clone(), tools);
            }
            Err(e) => {
                error!("Error rediscovering tools: {}", e);
            }
        }

        Ok(())
    }

    pub async fn kill_all_processes(&mut self) -> MCPResult<()> {
        for (tool_id, process_opt) in self.processes.iter_mut() {
            if let Some(process) = process_opt {
                if let Err(e) = process.kill().await {
                    error!("Failed to kill process for tool {}: {}", tool_id, e);
                }
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::models::models::{ToolConfiguration, ToolType};

    use super::*;

    #[tokio::test]
    async fn test_tool_registration() {
        let mut registry = ToolRegistry::default();
        let request = ToolRegistrationRequest {
            tool_name: "test_tool".to_string(),
            description: "Test tool".to_string(),
            tool_type: ToolType::Node,
            authentication: None,
            configuration: Some(ToolConfiguration {
                command: "node".to_string(),
                args: Some(vec!["test.js".to_string()]),
                env: None,
            }),
            distribution: None,
        };

        let result = registry.register_tool(request).await;
        assert!(result.is_ok());
    }

    // Add more tests...
}
