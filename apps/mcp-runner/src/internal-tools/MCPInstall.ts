import { proxyRequest } from "../proxyRequest.ts";
import { Tool } from "../types.ts";

export class MCPInstall {
  public static name = 'install_mcp_servers_and_tools';
  public static isInitialized = false;

  public static async install(tool_id: string) {
    if (!MCPInstall.isInitialized) {
      throw new Error("MCPInstall is not initialized");
    }
    const result = await proxyRequest('registry/install', {
      tool_id: tool_id
    });
    return {
      content: [{
        type: 'text',
        text: JSON.stringify(result),
      }]
    };
  }
  
  private static setInitialized() {
    MCPInstall.isInitialized = true;
  }

  public static async init() {
    MCPInstall.setInitialized();
  } 

  static tool: Tool = {
    "fullDescription": "Installs MCP Servers & Tools available to be installed.",
    "description": "Installs MCP Servers & Tools available to be installed.",
      "inputSchema": {
        "description": "Name of the MCP Server or Tool to install.",
        "properties": {
          "tool_id": {
            "description": "The tool 'id' of the MCP Server or Tool to install.",
            "title": "Tool ID",
            "type": "string"
          }
        },
        "required": [
          "tool_id"
        ],
        "title": "Install MCP Server",
        "type": "object"
      },
      "name": MCPInstall.name,
      "server_id": MCPInstall.name,
      "installed": true,
      "categories": ["mcp-dockmaster", "install"]
    };
}