import { proxyRequest } from "../proxyRequest.ts";
import { Tool } from "../types.ts";

export class MCPConfig {
  public static name = 'configure_mcp_tool';
  public static isInitialized = false;

  public static async configure(tool_id: string, config: Record<string, any>) {
    if (!MCPConfig.isInitialized) {
      throw new Error("MCPConfig is not initialized");
    }
    const result = await proxyRequest('server/config', {
      tool_id,
      config
    });
    return {
      content: [{
        type: 'text',
        text: JSON.stringify(result),
      }]
    };
  }
  
  private static setInitialized() {
    MCPConfig.isInitialized = true;
  }

  public static async init() {
    MCPConfig.setInitialized();
  } 

  static tool: Tool = {
    "fullDescription": "Configure MCP Tool settings and parameters.",
    "description": "Configure MCP Tool settings and parameters.",
    "inputSchema": {
      "description": "Configuration parameters for the MCP Tool.",
      "properties": {
        "tool_id": {
          "description": "ID of the MCP Tool to configure.",
          "title": "Tool ID",
          "type": "string"
        },
        "config": {
          "description": "Configuration object with key-value pairs.",
          "title": "Configuration",
          "type": "object",
          "additionalProperties": true
        }
      },
      "required": [
        "tool_id",
        "config"
      ],
      "title": "Configure MCP Tool",
      "type": "object"
    },
    "name": MCPConfig.name,
    "server_id": MCPConfig.name,
    "installed": true,
    "categories": ["mcp-dockmaster", "config"]
  };
}
