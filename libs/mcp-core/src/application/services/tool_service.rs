use std::collections::HashMap;
use std::sync::Arc;
use serde_json::{json, Value};
use log::{info, error};
use tokio::time::Duration;

use crate::domain::traits::{ProcessManager, ToolRepository};
use crate::domain::entities::tool::{
    Tool, ToolConfiguration, ToolConfig, ToolConfigUpdate,
    ToolRegistrationRequest, 
    ToolExecutionRequest, 
    ToolUpdateRequest, 
    ToolConfigUpdateRequest, 
    ToolUninstallRequest, 
    DiscoverServerToolsRequest
};
use crate::domain::errors::DomainError;
use crate::application::dto::{
    ToolRegistrationDTO, ToolRegistrationResponseDTO,
    ToolExecutionDTO, ToolExecutionResponseDTO,
    ToolUpdateDTO, ToolUpdateResponseDTO,
    ToolConfigUpdateDTO, ToolConfigUpdateResponseDTO,
    ToolUninstallDTO, ToolUninstallResponseDTO,
    DiscoverServerToolsDTO, DiscoverServerToolsResponseDTO
};

/// Application service that handles tool operations
pub struct ToolService {
    tool_repository: Arc<dyn ToolRepository>,
    process_manager: Arc<dyn ProcessManager>,
}

impl ToolService {
    pub fn new(
        tool_repository: Arc<dyn ToolRepository>,
        process_manager: Arc<dyn ProcessManager>,
    ) -> Self {
        Self { tool_repository, process_manager }
    }

    /// Register a new tool
    pub async fn register_tool(
        &self,
        request: ToolRegistrationDTO,
    ) -> Result<ToolRegistrationResponseDTO, DomainError> {
        info!("Starting registration of tool: {}", request.tool_name);
        
        // Convert DTO to domain request
        let domain_request = ToolRegistrationRequest {
            tool_name: request.tool_name,
            description: request.description,
            authentication: request.authentication,
            tool_type: request.tool_type,
            configuration: request.configuration,
            distribution: request.distribution,
        };
        
        // Generate a tool ID
        let tools = self.tool_repository.get_all_tools().await?;
        let tool_id = format!("tool_{}", tools.len() + 1);
        
        // Create the tool
        let tool = Tool {
            name: domain_request.tool_name.clone(),
            description: domain_request.description.clone(),
            enabled: true,
            tool_type: domain_request.tool_type.clone(),
            entry_point: None,
            configuration: domain_request.configuration.as_ref().and_then(|config| {
                if let Some(cmd) = config.get("command").and_then(|c| c.as_str()) {
                    Some(ToolConfiguration {
                        command: cmd.to_string(),
                        args: config.get("args").and_then(|a| a.as_array()).map(|args| {
                            args.iter()
                                .filter_map(|a| a.as_str().map(|s| s.to_string()))
                                .collect()
                        }),
                    })
                } else {
                    None
                }
            }),
            distribution: domain_request.distribution,
            config: None,
            authentication: domain_request.authentication,
        };
        
        // Save the tool
        self.tool_repository.save_tool(&tool_id, &tool).await?;
        
        // Return success response
        Ok(ToolRegistrationResponseDTO {
            success: true,
            message: format!("Tool '{}' registered successfully", domain_request.tool_name),
            tool_id: Some(tool_id),
        })
    }

    /// List all tools
    pub async fn list_tools(&self) -> Result<Vec<Value>, DomainError> {
        let tools = self.tool_repository.get_all_tools().await?;
        let server_tools = self.tool_repository.get_server_tools().await?;
        
        let mut all_tools = Vec::new();
        
        // Add local tools
        for (tool_id, tool) in tools {
            let tool_json = json!({
                "id": tool_id,
                "name": tool.name,
                "description": tool.description,
                "enabled": tool.enabled,
                "tool_type": tool.tool_type,
                "configuration": tool.configuration,
                "distribution": tool.distribution,
                "config": tool.config,
                "authentication": tool.authentication,
            });
            
            all_tools.push(tool_json);
        }
        
        // Add server tools
        for (server_id, tools) in server_tools {
            for tool in tools {
                let mut tool_info = serde_json::Map::new();
                
                if let Some(obj) = tool.as_object() {
                    for (key, value) in obj {
                        tool_info.insert(key.clone(), value.clone());
                    }
                }
                
                tool_info.insert("server_id".to_string(), json!(server_id));
                
                if !tool_info.contains_key("description") {
                    tool_info.insert("description".to_string(), json!("Tool from server"));
                }
                
                if !tool_info.contains_key("inputSchema") {
                    tool_info.insert(
                        "inputSchema".to_string(),
                        json!({
                            "type": "object",
                            "properties": {}
                        }),
                    );
                }
                
                all_tools.push(json!(tool_info));
            }
        }
        
        Ok(all_tools)
    }

    /// Discover tools from an MCP server
    pub async fn discover_tools(
        &self,
        request: DiscoverServerToolsDTO,
    ) -> Result<DiscoverServerToolsResponseDTO, DomainError> {
        // Convert DTO to domain request
        let domain_request = DiscoverServerToolsRequest {
            server_id: request.server_id,
        };
        
        // Get all tools from the server
        let mut process_ios = HashMap::new();
        let tools = self.process_manager.discover_server_tools(
            &domain_request.server_id,
            &mut process_ios,
        ).await?;
        
        // Return success response
        Ok(DiscoverServerToolsResponseDTO {
            success: true,
            tools: Some(tools),
            error: None,
        })
    }

    /// Execute a tool
    pub async fn execute_tool(
        &self,
        request: ToolExecutionDTO,
    ) -> Result<ToolExecutionResponseDTO, DomainError> {
        // Convert DTO to domain request
        let domain_request = ToolExecutionRequest {
            tool_id: request.tool_id,
            parameters: request.parameters,
        };
        
        // Execute the tool
        let result = self.tool_repository.execute_tool(domain_request).await?;
        
        // Convert domain response to DTO
        Ok(ToolExecutionResponseDTO {
            success: result.success,
            result: result.result,
            error: result.error,
        })
    }

    /// Update tool status (enabled/disabled)
    pub async fn update_tool(
        &self,
        request: ToolUpdateDTO,
    ) -> Result<ToolUpdateResponseDTO, DomainError> {
        // Convert DTO to domain request
        let domain_request = ToolUpdateRequest {
            tool_id: request.tool_id,
            enabled: request.enabled,
        };
        
        // Get the tool
        let mut tool = self.tool_repository.get_tool(&domain_request.tool_id).await?;
        
        // Update the tool
        tool.enabled = domain_request.enabled;
        
        // Save the tool
        self.tool_repository.save_tool(&domain_request.tool_id, &tool).await?;
        
        // Return success response
        Ok(ToolUpdateResponseDTO {
            success: true,
            message: format!(
                "Tool '{}' {} successfully",
                domain_request.tool_id,
                if domain_request.enabled { "enabled" } else { "disabled" }
            ),
        })
    }

    /// Update tool configuration
    pub async fn update_tool_config(
        &self,
        request: ToolConfigUpdateDTO,
    ) -> Result<ToolConfigUpdateResponseDTO, DomainError> {
        // Convert DTO to domain request
        let domain_request = ToolConfigUpdateRequest {
            tool_id: request.tool_id,
            config: ToolConfigUpdate {
                env: request.config.env.clone(),
            },
        };
        
        // Get the tool
        let mut tool = self.tool_repository.get_tool(&domain_request.tool_id).await?;
        
        // Update the tool config
        // Convert ToolConfigUpdate to ToolConfig
        let config = ToolConfig {
            env: domain_request.config.env,
            command: None,
            args: None,
        };
        tool.config = Some(config);
        
        // Save the tool
        self.tool_repository.save_tool(&domain_request.tool_id, &tool).await?;
        
        // Return success response
        Ok(ToolConfigUpdateResponseDTO {
            success: true,
            message: format!("Tool '{}' configuration updated successfully", domain_request.tool_id),
        })
    }

    /// Uninstall a tool
    pub async fn uninstall_tool(
        &self,
        request: ToolUninstallDTO,
    ) -> Result<ToolUninstallResponseDTO, DomainError> {
        // Convert DTO to domain request
        let domain_request = ToolUninstallRequest {
            tool_id: request.tool_id,
        };
        
        // Delete the tool
        self.tool_repository.delete_tool(&domain_request.tool_id).await?;
        
        // Return success response
        Ok(ToolUninstallResponseDTO {
            success: true,
            message: format!("Tool '{}' uninstalled successfully", domain_request.tool_id),
        })
    }
}
