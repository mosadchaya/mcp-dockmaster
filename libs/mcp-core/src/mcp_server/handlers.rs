use std::sync::Arc;
use async_trait::async_trait;
use serde_json::{Value};
use std::future::Future;
use std::pin::Pin;
use log::info;

// Import SDK modules
use mcp_sdk_core::{Tool, Content, Resource};
use mcp_sdk_core::protocol;
use mcp_sdk_core::handler::{ToolError, ResourceError, PromptError};
use mcp_sdk_core::prompt;
use mcp_sdk_server::{Router, Server, ByteTransport};
use mcp_sdk_server::router::{RouterService, CapabilitiesBuilder};

/// Trait for client managers to implement
#[async_trait]
pub trait ClientManagerTrait: Send + Sync {
    /// Handle a tool call
    async fn handle_tool_call(&self, tool_name: String, arguments: Value) -> Result<Value, String>;
    
    /// List available tools
    async fn list_tools(&self) -> Result<Vec<Tool>, String>;
    
    /// Synchronous version of list_tools for use in non-async contexts
    fn list_tools_sync(&self) -> Vec<Tool>;
    
    /// Update the tool cache
    async fn update_tools_cache(&self);
}

/// Default implementation of ClientManager
pub struct ClientManager {}

#[async_trait]
impl ClientManagerTrait for ClientManager {
    async fn handle_tool_call(&self, tool_name: String, _arguments: Value) -> Result<Value, String> {
        // Default implementation just returns an error
        Err(format!("Tool '{}' not implemented", tool_name))
    }
    
    async fn list_tools(&self) -> Result<Vec<Tool>, String> {
        // Default implementation returns an empty list
        Ok(vec![])
    }
    
    fn list_tools_sync(&self) -> Vec<Tool> {
        // Default implementation returns an empty list
        vec![]
    }
    
    async fn update_tools_cache(&self) {
        // Default implementation does nothing
    }
}

/// MCP Router implementation that uses a ClientManager
#[derive(Clone)]
struct MCPRouter {
    name: String,
    client_manager: Arc<dyn ClientManagerTrait>,
    capabilities: protocol::ServerCapabilities,
}

impl MCPRouter {
    fn new(name: String, client_manager: Arc<dyn ClientManagerTrait>) -> Self {
        // Create capabilities
        let mut builder = CapabilitiesBuilder::new();
        builder = builder.with_tools(true);
        
        Self {
            name,
            client_manager,
            capabilities: builder.build(),
        }
    }
}

impl Router for MCPRouter {
    fn name(&self) -> String {
        self.name.clone()
    }
    
    fn instructions(&self) -> String {
        "MCP Dockmaster Server".to_string()
    }
    
    fn capabilities(&self) -> protocol::ServerCapabilities {
        self.capabilities.clone()
    }
    
    fn list_tools(&self) -> Vec<Tool> {
        // Use the synchronous version of list_tools to avoid runtime panic
        self.client_manager.list_tools_sync()
    }
    
    fn call_tool(
        &self,
        tool_name: &str,
        arguments: Value,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<Content>, ToolError>> + Send + 'static>> {
        let client_manager = self.client_manager.clone();
        let tool_name = tool_name.to_string();
        
        Box::pin(async move {
            match client_manager.handle_tool_call(tool_name, arguments).await {
                Ok(result) => {
                    // Convert the result to Content using the text method
                    let result_str = serde_json::to_string(&result).unwrap_or_default();
                    let content = Content::text(result_str);
                    Ok(vec![content])
                },
                Err(err) => Err(ToolError::ExecutionError(err)),
            }
        })
    }
    
    fn list_resources(&self) -> Vec<Resource> {
        vec![]
    }
    
    fn read_resource(
        &self,
        _uri: &str,
    ) -> Pin<Box<dyn Future<Output = Result<String, ResourceError>> + Send + 'static>> {
        Box::pin(async {
            Err(ResourceError::NotFound("Resource not found".to_string()))
        })
    }
    
    fn list_prompts(&self) -> Vec<prompt::Prompt> {
        vec![]
    }
    
    fn get_prompt(
        &self,
        _prompt_name: &str,
    ) -> Pin<Box<dyn Future<Output = Result<String, PromptError>> + Send + 'static>> {
        Box::pin(async {
            Err(PromptError::NotFound("Prompt not found".to_string()))
        })
    }
}

/// Start the MCP server with the provided client manager
pub async fn start_mcp_server(client_manager: Arc<dyn ClientManagerTrait>) -> Result<(), Box<dyn std::error::Error>> {
    // Update the tool cache before starting the server
    info!("Updating tool cache before starting server...");
    client_manager.update_tools_cache().await;
    
    // Create our router implementation
    let router = MCPRouter::new("MCP Dockmaster Server".to_string(), client_manager);
    
    // Wrap it in a RouterService
    let router_service = RouterService(router);
    
    // Create the server
    let server = Server::new(router_service);
    
    // Create stdin/stdout transport for the MCP server
    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();
    let transport = ByteTransport::new(stdin, stdout);
    
    // Run the server
    server.run(transport).await?;
    
    Ok(())
} 