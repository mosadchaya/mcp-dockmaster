#!/usr/bin/env node

/**
 * MCP Proxy Server
 * This server proxies MCP Protocol commands to a server running on localhost:3000
 */

import { Server } from "@modelcontextprotocol/sdk/server/index.js";
import { StdioServerTransport } from "@modelcontextprotocol/sdk/server/stdio.js";
import {
  CallToolRequestSchema,
  ListResourcesRequestSchema,
  ListToolsRequestSchema,
  ReadResourceRequestSchema,
  ListPromptsRequestSchema,
  GetPromptRequestSchema,
} from "@modelcontextprotocol/sdk/types.js";
// @ts-ignore - node-fetch is an ESM module
import fetch from 'node-fetch';

// Target server URL
const TARGET_SERVER_URL = 'http://localhost:3000/mcp-proxy';

// Enable debug logging
const DEBUG = true;

function debugLog(...args: any[]) {
  if (DEBUG) {
    console.error(`[DEBUG ${new Date().toISOString()}]`, ...args);
  }
}

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
      tools: {},
      prompts: {},
    },
  }
);

debugLog('Server instance created');

/**
 * Generic proxy function to forward requests to the target server
 * @param method The JSON-RPC method to call
 * @param params The parameters to pass to the method
 * @returns The response from the target server
 */
async function proxyRequest(method: string, params: any): Promise<any> {
  debugLog(`proxyRequest called with method: ${method}`);
  try {
    console.error(`Proxying request: ${method} to ${TARGET_SERVER_URL}`);
    console.error(`Request params: ${JSON.stringify(params)}`);
    
    // The server only accepts POST requests with JSON-RPC format
    const requestBody = JSON.stringify({
      jsonrpc: '2.0',
      id: 1,
      method,
      params: params || {},
    });
    
    console.error(`Request body: ${requestBody}`);
    
    const response = await fetch(TARGET_SERVER_URL, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: requestBody,
    });

    if (!response.ok) {
      const responseText = await response.text();
      console.error(`HTTP error! status: ${response.status}, response: ${responseText}`);
      throw new Error(`HTTP error! status: ${response.status}, response: ${responseText}`);
    }

    const data = await response.json() as { result: any; error?: { message: string } };
    
    if (data.error) {
      console.error(`Server error: ${JSON.stringify(data.error)}`);
      throw new Error(`Server error: ${JSON.stringify(data.error)}`);
    }

    console.error(`Received response for: ${method}`);
    console.error(`Response data: ${JSON.stringify(data.result)}`);
    return data.result;
  } catch (error) {
    console.error(`Error proxying request: ${error}`);
    throw error;
  }
}

debugLog('Setting up request handlers');

// Handler for listing resources
server.setRequestHandler(ListResourcesRequestSchema, async () => {
  debugLog('ListResourcesRequestSchema handler called');
  const resources = await proxyRequest('resources/list', {});
  return resources; // The backend should already return { resources: [...] }
});

// Handler for reading resources
server.setRequestHandler(ReadResourceRequestSchema, async (request: any) => {
  debugLog('ReadResourceRequestSchema handler called', request);
  return await proxyRequest('resources/read', request.params);
});

// Handler for listing tools
server.setRequestHandler(ListToolsRequestSchema, async () => {
  debugLog('ListToolsRequestSchema handler called');
  try {
    const result = await proxyRequest('tools/list', {});
    
    // Ensure the result has the expected format
    if (result && typeof result === 'object') {
      // If the result already has a tools array, return it directly
      if (Array.isArray(result.tools)) {
        debugLog('Received tools list with correct format');
        return result;
      }
      
      // If the result is an array, wrap it in a tools object
      if (Array.isArray(result)) {
        debugLog('Received tools as array, converting to expected format');
        return { tools: result };
      }
    }
    
    // If we got here, the format is unexpected
    debugLog('Unexpected tools list format, returning empty list');
    return { tools: [] };
  } catch (error) {
    console.error('Error fetching tools list:', error);
    return { tools: [] };
  }
});

// Handler for calling tools
server.setRequestHandler(CallToolRequestSchema, async (request: any) => {
  debugLog('CallToolRequestSchema handler called', request);
  return await proxyRequest('tools/call', request.params);
});

// Handler for listing prompts
server.setRequestHandler(ListPromptsRequestSchema, async () => {
  debugLog('ListPromptsRequestSchema handler called');
  const prompts = await proxyRequest('prompts/list', {});
  return prompts; // The backend should already return { prompts: [...] }
});

// Handler for getting prompts
server.setRequestHandler(GetPromptRequestSchema, async (request: any) => {
  debugLog('GetPromptRequestSchema handler called', request);
  return await proxyRequest('prompts/get', request.params);
});

debugLog('All request handlers set up');

// Add a ping handler for testing
server.setRequestHandler({ shape: { method: { value: 'ping' } } }, async () => {
  debugLog('Ping handler called');
  return { message: 'pong' };
});

/**
 * Start the server using stdio transport.
 * This allows the server to communicate via standard input/output streams.
 */
async function main() {
  console.error('Starting MCP Proxy Server...');
  console.error(`Target server: ${TARGET_SERVER_URL}`);

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
}

main().catch((error) => {
  console.error("Server error:", error);
  process.exit(1);
});
