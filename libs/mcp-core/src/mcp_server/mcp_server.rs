use std::sync::Arc;

use rmcp::{
    model::{
        Annotated, CallToolRequestParam, CallToolResult, ErrorCode, ListToolsResult,
        PaginatedRequestParam, RawContent, ServerCapabilities, ServerInfo,
    },
    service::RequestContext,
    tool, Error as McpError, RoleServer, ServerHandler,
};

use crate::core::mcp_core::MCPCore;

use super::mcp_tools_service::MCPToolsService;

pub struct McpServer {
    mcp_core: MCPCore,
    tools_service: Arc<MCPToolsService>,
}

#[tool(tool_box)]
impl McpServer {
    pub fn new(mcp_core: MCPCore, tools_service: Arc<MCPToolsService>) -> Self {
        Self {
            mcp_core,
            tools_service,
        }
    }
}

impl ServerHandler for McpServer {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            instructions: Some("Dockmaster MCP Server".into()),
            capabilities: ServerCapabilities::builder()
                .enable_tools()
                .enable_tool_list_changed()
                .enable_prompts()
                .enable_resources()
                .build(),
            ..Default::default()
        }
    }

    // TODO: Implement pagination
    async fn list_tools(
        &self,
        _request: Option<PaginatedRequestParam>,
        _context: RequestContext<RoleServer>,
    ) -> Result<ListToolsResult, McpError> {
        Ok(ListToolsResult {
            tools: self.tools_service.list_tools(),
            next_cursor: None,
        })
    }

    async fn call_tool(
        &self,
        request: CallToolRequestParam,
        _: RequestContext<RoleServer>,
    ) -> Result<CallToolResult, McpError> {
        self.tools_service
            .execute_tool(&request.name, request.arguments.into())
            .await
            .map_err(|e| {
                McpError::new(
                    ErrorCode::INTERNAL_ERROR,
                    "Tool execution failed",
                    Some(e.to_string().into()),
                )
            })
            .map(|result| CallToolResult {
                content: Vec::from([Annotated::new(
                    RawContent::Text(rmcp::model::RawTextContent {
                        text: result.to_string(),
                    }),
                    None,
                )]),
                is_error: None,
            })
    }
}
