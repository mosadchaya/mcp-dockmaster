/**
 * MCP Proxy Server
 * This server proxies MCP Protocol commands to a server running on localhost:3000
 */
import { Server } from "npm:@modelcontextprotocol/sdk/server/index.js";
import { StdioServerTransport } from "npm:@modelcontextprotocol/sdk/server/stdio.js";
import {
  CallToolRequestSchema,
  ListResourcesRequestSchema,
  ListToolsRequestSchema,
  ReadResourceRequestSchema,
  ListPromptsRequestSchema,
  GetPromptRequestSchema,
} from "npm:@modelcontextprotocol/sdk/types.js";
import { z } from "npm:zod";
import { Tools } from './types.ts';
import { proxyRequest } from "./proxyRequest.ts";
import { debugLog } from "./logger.ts";
import { initInternalTools, injectInternalTools, runInternalTool } from "./internal-tools/index.ts";

debugLog('Starting MCP Proxy Server script');

/**
 * Create an MCP server with all capabilities
 */
const server = new Server(
  {
    name: "MCP Proxy Server",
    version: "0.1.0",
  },
  {
    capabilities: {
      resources: {},
      tools: {
        "listChanged": true,
      },
      prompts: {},
    },
  }
);

debugLog('Server instance created');
debugLog('Setting up request handlers');

// Handler for listing resources
server.setRequestHandler(ListResourcesRequestSchema, async () => {
  debugLog('ListResourcesRequestSchema handler called');
  try {
    const resources = await proxyRequest('resources/list', {});
    return resources as any; // The backend should already return { resources: [...] }
  } catch (error) {
    console.error('Error fetching resources list:', error);
    return { resources: [] } as any;
  }
});

// Handler for reading resources
server.setRequestHandler(ReadResourceRequestSchema, async (request: any) => {
  debugLog('ReadResourceRequestSchema handler called', request);
  try {
    const result = await proxyRequest('resources/read', request.params);
    return result as any;
  } catch (error) {
    console.error('Error fetching resource:', error);
    return { error: 'Resource not found' } as any;
  }
});

// Handler for listing tools
server.setRequestHandler(ListToolsRequestSchema, async () => {
  debugLog('ListToolsRequestSchema handler called');
  try {
    const result = await proxyRequest<Tools>('tools/list', {});
    
    // Function to process tool schema
    // PATCH FOR CURSOR
    const processToolSchema = (tool: any) => {
      if (tool.inputSchema?.properties) {
        const properties = Object.keys(tool.inputSchema.properties);
        
        // Check if there's only one property and required is empty
        if (properties.length === 1 && 
            (!tool.inputSchema.required || tool.inputSchema.required.length === 0)) {
          const propertyName = properties[0];
          const property = tool.inputSchema.properties[propertyName];
          
          // Add property name to required array
          tool.inputSchema.required = [propertyName];
          
          // Append (Optional) to description if not already present
          if (property.description && !property.description.includes('(Optional)')) {
            property.description += ' (Optional) leave it empty if optional';
          }
        }
      }
      return tool;
    };
    // END PATCH FOR CURSOR

    // Function to clean tool fields
    const cleanTool = (tool: any) => {
      const cleanedTool = { ...tool };
      // Remove specified fields
      delete cleanedTool.proxy_id;
      delete cleanedTool.server_id;
      delete cleanedTool.categories;
      delete cleanedTool.tags;
      delete cleanedTool.is_active;
      delete cleanedTool.id;
      
      // Clean the name
      cleanedTool.name = cleanedTool.name.replace(/[^a-zA-Z0-9_-]/g, '_');
      cleanedTool.name = cleanedTool.name.substring(0, 64);
      
      return cleanedTool;
    };

    // Ensure the result has the expected format
    if (result && typeof result === 'object') {
      // If the result already has a tools array
      if (Array.isArray(result.tools)) {
        injectInternalTools(result);
        // Filter out inactive tools, process schemas, and clean remaining ones
        result.tools = result.tools
          .filter(tool => tool.is_active !== false)
          // PATCH FOR CURSOR
          .map(tool => processToolSchema(tool))
          // END PATCH FOR CURSOR
          .map(cleanTool);
        
        debugLog('Received tools list with correct format');
        // debugLog('Tools list:', JSON.stringify(result.tools, null, 2));
        return result as any;
      }
      
      // If the result is an array
      if (Array.isArray(result)) {
        injectInternalTools({ tools: result });
        debugLog('Received tools as array, converting to expected format');
        const filteredTools = result
          .filter(tool => tool.is_active !== false)
          .map(cleanTool);
          // injectInternalTools(wrappedResult);
        return { tools: filteredTools } as any;
      }
    }
    
    // If we got here, the format is unexpected
    debugLog('Unexpected tools list format, returning empty list');
    const emptyResult = { tools: [] } as any;
    injectInternalTools(emptyResult);
    return emptyResult;
  } catch (error) {
    console.error('Error fetching tools list:', error);
    const emptyResult = { tools: [] } as any;
    injectInternalTools(emptyResult);
    return emptyResult;
  }
});

// Handler for calling tools
server.setRequestHandler(CallToolRequestSchema, async (request: any) => {
  debugLog('CallToolRequestSchema handler called', request);
  const params: { name: string, arguments: Record<string, any> } = request.params;
  const result = await runInternalTool(params);

  if (result.isInternalTool) {
    return result.result;
  }

  try {
    const callResult = await proxyRequest('tools/call', request.params);
    return callResult as any;
  } catch (error) {
    console.error('Error calling tool:', error);
    return { error: 'Tool call failed' } as any;
  }
});

// Handler for listing prompts
server.setRequestHandler(ListPromptsRequestSchema, async () => {
  debugLog('ListPromptsRequestSchema handler called');
  try {
    const prompts = await proxyRequest('prompts/list', {});
    return prompts as any;  // The backend should already return { prompts: [...] }
  } catch (error) {
    console.error('Error fetching prompts list:', error);
    return { prompts: [] } as any;
  }
});

// Handler for getting prompts
server.setRequestHandler(GetPromptRequestSchema, async (request: any) => {
  debugLog('GetPromptRequestSchema handler called', request);
  try {
    const result = await proxyRequest('prompts/get', request.params);
    return result as any;
  } catch (error) {
    console.error('Error fetching prompt:', error);
    return { error: 'Prompt not found' } as any;
  }
});

debugLog('All request handlers set up');

// Add a ping handler for testing
server.setRequestHandler(z.object({ method: z.literal('ping') }), async () => {
  debugLog('Ping handler called');
  return { message: 'pong' };
});

/**
 * Start the server using stdio transport.
 * This allows the server to communicate via standard input/output streams.
 */
async function main() {
  console.error('Starting MCP Proxy Server...');

  const transport = new StdioServerTransport();
  
  // Add transport event handlers
  transport.onmessage = (message) => {
    debugLog('Transport received message:', JSON.stringify(message));
  };
  
  transport.onerror = (error) => {
    debugLog('Transport error:', error);
  };
  
  transport.onclose = () => {
    debugLog('Transport closed');
  };
  
  await server.connect(transport);
  
  console.error('MCP Proxy Server started');

  // Initialize internal tools;
  await initInternalTools();

}

main().catch((error) => {
  console.error("Server error:", error);
  Deno.exit(1);
});
