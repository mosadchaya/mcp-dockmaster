import { invoke } from '@tauri-apps/api/core';

interface ToolRegistrationRequest {
  tool_id: string;
  tool_name: string;
  description: string;
  authentication?: any;
  tool_type: string;  // "nodejs", "python", "docker"
  configuration?: {
    command: string;
    args: string[];
    env: Record<string, string>;
  };
  distribution?: {
    type: string;
    package: string;
  };
}

// new code
interface RuntimeEnvConfig {
  default: string;
  description: string;
  required: boolean;
}

interface RuntimeState {
  enabled: boolean;
  process_running: boolean;
  tool_count: number;
}

type RuntimeConfiguration = NonNullable<ToolRegistrationRequest['configuration']> & {
  env: Record<string, RuntimeEnvConfig>;
};

type ToolInstance = Omit<ToolRegistrationRequest, 'tool_id' | 'tool_name' | 'configuration'> & RuntimeState & {
  id: string;  // renamed from tool_id
  name: string;  // renamed from tool_name
  configuration: RuntimeConfiguration;  // enhanced configuration
  distribution: NonNullable<ToolRegistrationRequest['distribution']>;  // make required
}
// end new code

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

interface ToolConfigUpdateRequest {
  tool_id: string;
  config: Record<string, string>;
}

interface ToolConfigUpdateResponse {
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

interface DiscoverServerToolsRequest {
  server_id: string;
}

interface DiscoverServerToolsResponse {
  success: boolean;
  tools?: any[];
  error?: string;
}

interface ServerToolInfo {
  id: string;
  name: string;
  description: string;
  parameters?: Record<string, unknown>;
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
  static async listServers(): Promise<ToolInstance[]> {
    return await invoke<ToolInstance[]>('list_servers');
  }

  /**
   * List all available tools from all running MCP servers
   */
  static async listAllServerTools(): Promise<any[]> {
    return await invoke<any[]>('list_all_server_tools');
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
   * Update a tool's configuration (environment variables)
   */
  static async updateToolConfig(request: ToolConfigUpdateRequest): Promise<ToolConfigUpdateResponse> {
    return await invoke<ToolConfigUpdateResponse>('update_tool_config', { request });
  }

  static async restartTool(toolId: string): Promise<ToolUpdateResponse> {
    return await invoke<ToolUpdateResponse>('restart_tool_command', { toolId });
  }

  /**
   * Uninstall a registered tool
   */
  static async uninstallTool(request: ToolUninstallRequest): Promise<ToolUninstallResponse> {
    return await invoke<ToolUninstallResponse>('uninstall_tool', { request });
  }

  /**
   * Discover tools from a specific MCP server
   */
  static async discoverTools(request: DiscoverServerToolsRequest): Promise<ServerToolInfo[]> {
    return await invoke<ServerToolInfo[]>('discover_tools', { request });
  }
  
  /**
   * Execute a tool from an MCP server through the proxy
   */
  static async executeProxyTool(request: ToolExecutionRequest): Promise<ToolExecutionResponse> {
    return await invoke<ToolExecutionResponse>('execute_proxy_tool', { request });
  }
  
  /**
   * Get Claude configuration for MCP servers
   */
  static async getClaudeConfig(): Promise<any> {
    return await invoke<any>('get_claude_config');
  }
}

export default MCPClient; 