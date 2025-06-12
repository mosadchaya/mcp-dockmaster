import { invoke } from '@tauri-apps/api/core';

export interface Registry {
  count: number;
  version: number;
  tools: RegistryServer[];
  categories: Record<string, number>;
  tags: Record<string, number>;
}

export interface RegistryServer {
  id: string;
  name: string;
  description: string;
  short_description: string;
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
  tools?: {
    signature: string;
    description: string;
  }[];
  weight: number;
  featured: boolean;
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

export type ServerStatus = 'running' | 'stopped' | 'starting' | string;

export interface RuntimeServer extends ServerDefinition {
  id: string;  // Using string instead of ToolId since we don't need the full Rust implementation
  status: ServerStatus;
  tool_count: number;
  sourceUrl?: string;
  colorTags?: string[]; // Add this line to store color tags
}

export interface ServerToolInfo {
  id: string;
  name: string;
  description: string;
  inputSchema?: InputSchema;
  server_id: string;
  proxy_id?: string;
}

export interface ServerRegistrationResponse {
  success: boolean;
  message: string;
  server_id?: string;
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

interface ServerUpdateRequest {
  server_id: string;
  enabled: boolean;
}

interface ServerUpdateResponse {
  success: boolean;
  message: string;
}

interface ServerConfigUpdateRequest {
  server_id: string;
  config: Record<string, string>;
}

interface ServerConfigUpdateResponse {
  success: boolean;
  message: string;
}

interface ServerUninstallRequest {
  server_id: string;
}

interface ServerUninstallResponse {
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
  static async updateServerStatus(request: ServerUpdateRequest): Promise<ServerUpdateResponse> {
    return await invoke<ServerUpdateResponse>('update_server_status', { request });
  }

  /**
   * Update a tool's configuration (environment variables)
   */
  static async updateServerConfig(request: ServerConfigUpdateRequest): Promise<ServerConfigUpdateResponse> {
    return await invoke<ServerConfigUpdateResponse>('update_server_config', { request });
  }

  static async restartTool(serverId: string): Promise<ServerUpdateResponse> {
    return await invoke<ServerUpdateResponse>('restart_server_command', { serverId });
  }

  /**
   * Uninstall a registered tool
   */
  static async uninstallServer(request: ServerUninstallRequest): Promise<ServerUninstallResponse> {
    return await invoke<ServerUninstallResponse>('uninstall_server', { request });
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
   * Import a server from a GitHub repository URL
   */
  static async importServerFromUrl(url: string): Promise<ServerRegistrationResponse> {
    return await invoke<ServerRegistrationResponse>('import_server_from_url', { 
      url 
    });
  }
  
  /**
   * Analyze a GitHub repository to extract environment variables
   */
  static async analyzeGitHubRepository(url: string): Promise<Array<{key: string, value: string, description: string, required: boolean}>> {
    return await invoke<Array<{key: string, value: string, description: string, required: boolean}>>('analyze_github_repository', { 
      url 
    });
  }

  /**
   * Analyze a local directory to extract environment variables
   */
  static async analyzeLocalDirectory(directory: string): Promise<Array<{key: string, value: string, description: string, required: boolean}>> {
    return await invoke<Array<{key: string, value: string, description: string, required: boolean}>>('analyze_local_directory', { 
      directory 
    });
  }
  

  /**
   * Get Claude configuration for MCP servers
   */
  static async getClaudeConfig(): Promise<any> {
    return await invoke<any>('get_claude_config');
  }

  /**
   * Get the tool visibility state
   */
  static async getToolsVisibilityState(): Promise<boolean> {
    return await invoke<boolean>('get_tools_visibility_state');
  }

  /**
   * Set the tool visibility state
   */
  static async setToolsHidden(hidden: boolean): Promise<void> {
    return await invoke<void>('set_tools_hidden', { hidden });
  }
  
  // Add a function to update server color tags
  // This is a client-side only function since we're not persisting to backend
  static updateServerColorTags(serverId: string, colorTags: string[]): string[] {
    // Get servers from localStorage or initialize empty array
    const storedServers = localStorage.getItem('serverColorTags') || '{}';
    const serverTags = JSON.parse(storedServers);
    
    // Update tags for this server
    serverTags[serverId] = colorTags;
    
    // Save back to localStorage
    localStorage.setItem('serverColorTags', JSON.stringify(serverTags));
    
    return colorTags;
  }
  
  // Add a function to get server color tags
  static getServerColorTags(): Record<string, string[]> {
    const storedServers = localStorage.getItem('serverColorTags') || '{}';
    return JSON.parse(storedServers);
  }
  
  // Add a function to get color tags for a specific server
  static getColorTagsForServer(serverId: string): string[] {
    const allTags = this.getServerColorTags();
    return allTags[serverId] || [];
  }
  
  // Add a function to save available colors
  static saveAvailableColors(colors: Record<string, string>): void {
    localStorage.setItem('availableColors', JSON.stringify(colors));
  }
  
  // Add a function to get available colors
  static getAvailableColors(): Record<string, string> {
    const storedColors = localStorage.getItem('availableColors');
    return storedColors ? JSON.parse(storedColors) : {};
  }
  
  // Add a function to add a new color to all servers
  static addColorToAllServers(colorKey: string): void {
    const serverTags = this.getServerColorTags();
    Object.keys(serverTags).forEach(serverId => {
      if (!serverTags[serverId].includes(colorKey)) {
        serverTags[serverId].push(colorKey);
      }
    });
    localStorage.setItem('serverColorTags', JSON.stringify(serverTags));
  }
}

export default MCPClient;                                                                                                                                                                                                