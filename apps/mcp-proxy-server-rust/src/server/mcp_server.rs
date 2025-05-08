use rmcp::{
    RoleClient, ServerHandler, ServiceError,
    handler::server::wrapper::Json,
    model::{
        CallToolRequestParam, CallToolResult, InitializeRequestParam, ServerCapabilities,
        ServerInfo,
    },
    schemars,
    service::RunningService,
    tool,
};
use serde_json::Value;

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct SumRequest {
    #[schemars(description = "the left hand side number")]
    pub a: i32,
    pub b: i32,
}
#[derive(Debug, Clone)]
pub struct McpServer {
    mcp_client: RunningService<RoleClient, InitializeRequestParam>,
}

#[tool(tool_box)]
impl McpServer {
    pub fn new(mcp_client: RunningService<RoleClient, InitializeRequestParam>) -> Self {
        Self { mcp_client }
    }

    #[tool(description = "Call a tool from Dockmaster")]
    async fn call_tool(
        &self,
        #[tool(param)] name: String,
        #[tool(param)] arguments: Value,
    ) -> Result<CallToolResult, ServiceError> {
        self.mcp_client
            .call_tool(CallToolRequestParam {
                name: name.into(),
                arguments: Some(
                    arguments
                        .as_object()
                        .unwrap_or(&serde_json::Map::new())
                        .clone(),
                ),
            })
            .await
            .map(|r| CallToolResult::success(r.content))
    }
}

#[tool(tool_box)]
impl ServerHandler for McpServer {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            instructions: Some("Dockmaster MCP Proxy Server".into()),
            capabilities: ServerCapabilities::builder()
                .enable_tools()
                .enable_prompts()
                .enable_resources()
                .build(),
            ..Default::default()
        }
    }
}
