/**
 * Example MCP Tool using stdio adapter
 * This is a simple MCP tool that demonstrates how to use the MCPStdioAdapter
 */

const MCPStdioAdapter = require('../lib/mcpStdioAdapter');

// Create the adapter
const adapter = new MCPStdioAdapter({
  tools: [
    {
      id: 'hello-world',
      name: 'Hello World',
      description: 'A simple greeting tool',
      parameters: {
        type: 'object',
        properties: {
          name: {
            type: 'string',
            description: 'The name to greet'
          }
        }
      }
    },
    {
      id: 'echo',
      name: 'Echo',
      description: 'Echo back what was sent',
      parameters: {
        type: 'object',
        properties: {
          message: {
            type: 'string',
            description: 'The message to echo back'
          }
        }
      }
    }
  ]
});

// Register execute handler
adapter.registerHandler('execute', async (toolId, parameters) => {
  // Respond based on which tool was called
  switch (toolId) {
    case 'hello-world':
      const name = parameters.name || 'World';
      return {
        greeting: `Hello, ${name}!`
      };
    
    case 'echo':
      return {
        echo: parameters.message || ''
      };
    
    default:
      throw new Error(`Unknown tool: ${toolId}`);
  }
});

// The adapter automatically sets up stdin/stdout communication
console.error('MCP Example Tool started'); 