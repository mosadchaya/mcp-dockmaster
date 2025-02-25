import { invoke } from '@tauri-apps/api/core';

interface ToolRegistrationRequest {
  tool_name: string;
  description: string;
  authentication?: any;
}

interface ToolRegistrationResponse {
  success: boolean;
  message: string;
  tool_id?: string;
}

interface ToolExecutionRequest {
  tool_id: string;
  parameters: any;
}

interface ToolExecutionResponse {
  success: boolean;
  result?: any;
  error?: string;
}

interface ToolUpdateRequest {
  tool_id: string;
  enabled: boolean;
}

interface ToolUpdateResponse {
  success: boolean;
  message: string;
}

interface ToolUninstallRequest {
  tool_id: string;
}

interface ToolUninstallResponse {
  success: boolean;
  message: string;
}

/**
 * MCP Client for interacting with the MCP Server Proxy
 */
export class MCPClient {
  /**
   * Register a new tool with the MCP server
   */
  static async registerTool(request: ToolRegistrationRequest): Promise<ToolRegistrationResponse> {
    return await invoke<ToolRegistrationResponse>('register_tool', { request });
  }

  /**
   * List all registered tools
   */
  static async listTools(): Promise<any[]> {
    return await invoke<any[]>('list_tools');
  }

  /**
   * Execute a registered tool
   */
  static async executeTool(request: ToolExecutionRequest): Promise<ToolExecutionResponse> {
    return await invoke<ToolExecutionResponse>('execute_tool', { request });
  }

  /**
   * Update a tool's status (enabled/disabled)
   */
  static async updateToolStatus(request: ToolUpdateRequest): Promise<ToolUpdateResponse> {
    return await invoke<ToolUpdateResponse>('update_tool_status', { request });
  }

  /**
   * Uninstall a registered tool
   */
  static async uninstallTool(request: ToolUninstallRequest): Promise<ToolUninstallResponse> {
    return await invoke<ToolUninstallResponse>('uninstall_tool', { request });
  }

  /**
   * Test the MCP server connection with a hello world request
   */
  static async helloWorld(): Promise<any> {
    return await invoke<any>('mcp_hello_world');
  }
}

export default MCPClient; 