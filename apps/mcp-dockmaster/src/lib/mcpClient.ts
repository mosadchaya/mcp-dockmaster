import { invoke } from '@tauri-apps/api/core';

export interface RegistryServer {
  id: string;
  name: string;
  description: string;
  fullDescription: string;
  publisher: {
    id: string;
    name: string;
    url: string;
  };
  runtime: string;
  installed: boolean;
  isOfficial?: boolean;
  sourceUrl?: string;
  distribution?: {
    type: string;
    package: string;
  };
  config?: {
    command: string;
    args: string[];
    env: Record<string, any>;
  };
  license?: string;
  categories?: string[];
}

export interface ServerRegistrationRequest {
  server_id: string;
  server_name: string;
  description: string;
  authentication?: any;
  tools_type: string;  // "nodejs", "python", "docker"
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

// new code needs adjusting
export interface RuntimeEnvConfig {
  default: string;
  description: string;
  required: boolean;
}

export interface InputSchemaProperty {
  description: string;
  type: string;
}

export interface InputSchema {
  properties: Record<string, InputSchemaProperty>;
  required: string[];
  type: string;
}

export interface ToolConfiguration {
  command?: string;
  args?: string[];
  env?: Record<string, RuntimeEnvConfig>;
}

export interface Distribution {
  type: string;
  package: string;
}

export interface ServerDefinition {
  name: string;
  description: string;
  enabled: boolean;
  tools_type: string;
  entry_point?: string;
  configuration?: ToolConfiguration;
  distribution?: Distribution;
}

export interface RuntimeServer extends ServerDefinition {
  id: string;  // Using string instead of ToolId since we don't need the full Rust implementation
  process_running: boolean;
  tool_count: number;
}

export interface ServerToolInfo {
  id: string;
  name: string;
  description: string;
  inputSchema?: InputSchema;
  server_id: string;
  proxy_id?: string;
}

interface ServerRegistrationResponse {
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

/**
 * MCP Client for interacting with the MCP Server Proxy
 */
export class MCPClient {
  /**
   * Register a new tool with the MCP server
   */
  static async registerServer(request: ServerRegistrationRequest): Promise<ServerRegistrationResponse> {
    return await invoke<ServerRegistrationResponse>('register_server', { request });
  }

  /**
   * List all registered tools
   */
  static async listServers(): Promise<RuntimeServer[]> {
    return await invoke<RuntimeServer[]>('list_servers');
  }

  /**
   * List all available tools from all running MCP servers
   */
  static async listAllServerTools(): Promise<ServerToolInfo[]> {
    return await invoke<ServerToolInfo[]>('list_all_server_tools');
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
  static async updateServerStatus(request: ToolUpdateRequest): Promise<ToolUpdateResponse> {
    return await invoke<ToolUpdateResponse>('update_server_status', { request });
  }

  /**
   * Update a tool's configuration (environment variables)
   */
  static async updateServerConfig(request: ToolConfigUpdateRequest): Promise<ToolConfigUpdateResponse> {
    return await invoke<ToolConfigUpdateResponse>('update_server_config', { request });
  }

  static async restartTool(toolId: string): Promise<ToolUpdateResponse> {
    return await invoke<ToolUpdateResponse>('restart_tool_command', { toolId });
  }

  /**
   * Uninstall a registered tool
   */
  static async uninstallServer(request: ToolUninstallRequest): Promise<ToolUninstallResponse> {
    return await invoke<ToolUninstallResponse>('uninstall_server', { request });
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