use std::{
    future::Future,
    pin::Pin,
    sync::{Arc, Mutex},
};

use crate::server::mcp_client::McpClientProxy;
use mcp_client::McpClientTrait;
use mcp_core::{
    Content, Resource, Tool, ToolError,
    handler::{PromptError, ResourceError},
    prompt::Prompt,
    protocol::ServerCapabilities,
};
use mcp_server::router::CapabilitiesBuilder;
use serde_json::Value;

pub struct DockmasterRouter {
    tools_cache: Arc<Mutex<Vec<Tool>>>,
    mcp_client: McpClientProxy,
}

impl Clone for DockmasterRouter {
    fn clone(&self) -> Self {
        Self {
            tools_cache: self.tools_cache.clone(),
            mcp_client: self.mcp_client.clone(),
        }
    }
}

impl Default for DockmasterRouter {
    fn default() -> Self {
        Self::new()
    }
}

impl DockmasterRouter {
    pub fn new() -> DockmasterRouter {
        DockmasterRouter {
            tools_cache: Arc::new(Mutex::new(vec![])),
            mcp_client: McpClientProxy::new(&Self::get_target_server_url()),
        }
    }

    pub fn get_target_server_url() -> String {
        let port = std::env::var("DOCKMASTER_HTTP_SERVER_PORT")
            .unwrap_or_else(|_| "11011".to_string())
            .parse::<u16>()
            .unwrap_or(11011);
        format!("http://localhost:{}/mcp/sse", port)
    }

    pub async fn initialize(&mut self) {
        // Create client instance
        let _ = self.mcp_client.init().await;

        // Save tools to cache
        let tools = self.mcp_client.get_client().unwrap().list_tools(None).await;
        if let Ok(mut cache) = self.tools_cache.lock() {
            *cache = tools.unwrap().tools;
        }

        // Spawn background task to update tools cache
        let tools_cache = self.tools_cache.clone();
        let client = self.mcp_client.get_client().unwrap();
        tokio::spawn(async move {
            loop {
                let tools = client.list_tools(None).await;
                if let Ok(mut cache) = tools_cache.lock() {
                    *cache = tools.unwrap().tools;
                }
                tokio::time::sleep(tokio::time::Duration::from_secs(60)).await;
            }
        });
    }

    async fn execute_tool(
        &self,
        tool_name: &str,
        arguments: Value,
    ) -> Result<Vec<Content>, ToolError> {
        tracing::info!(
            "Executing tool: {} with arguments: {}",
            tool_name,
            arguments
        );

        let result = self
            .mcp_client
            .get_client()
            .unwrap()
            .call_tool(tool_name, arguments)
            .await;
        match result {
            Ok(response) => Ok(response.content),
            Err(e) => {
                tracing::error!("Error executing tool: {}", e);
                Err(ToolError::ExecutionError(e.to_string()))
            }
        }
    }
}

impl mcp_server::Router for DockmasterRouter {
    fn name(&self) -> String {
        "dockmaster".to_string()
    }

    fn instructions(&self) -> String {
        "This server is a proxy to the MCP Dockmaster Application.".to_string()
    }

    fn capabilities(&self) -> ServerCapabilities {
        CapabilitiesBuilder::new()
            .with_tools(true)
            .with_prompts(true)
            .with_resources(true, true)
            .build()
    }

    fn list_tools(&self) -> Vec<Tool> {
        // Use a synchronous lock to get the cached tools
        self.tools_cache
            .lock()
            .map(|cache| cache.clone())
            .unwrap_or_else(|_| {
                tracing::error!("Failed to acquire tools cache lock");
                vec![]
            })
    }

    fn call_tool(
        &self,
        tool_name: &str,
        arguments: Value,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<Content>, ToolError>> + Send + 'static>> {
        let tool_name = tool_name.to_string();
        let arguments = arguments.clone();
        let router = self.clone();

        Box::pin(async move { router.execute_tool(&tool_name, arguments).await })
    }

    fn list_resources(&self) -> Vec<Resource> {
        vec![]
    }

    fn read_resource(
        &self,
        uri: &str,
    ) -> Pin<Box<dyn Future<Output = Result<String, ResourceError>> + Send + 'static>> {
        let uri = uri.to_string();
        Box::pin(async move {
            Err(ResourceError::NotFound(format!(
                "Resource {} not found",
                uri
            )))
        })
    }

    fn list_prompts(&self) -> Vec<Prompt> {
        vec![]
    }

    fn get_prompt(
        &self,
        prompt_name: &str,
    ) -> Pin<Box<dyn Future<Output = Result<String, PromptError>> + Send + 'static>> {
        let prompt_name = prompt_name.to_string();
        Box::pin(async move {
            Err(PromptError::NotFound(format!(
                "Prompt {} not found",
                prompt_name
            )))
        })
    }
}
