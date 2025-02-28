# MCP Stdio Tools

This folder contains examples of MCP tools that communicate via stdin/stdout instead of HTTP.

## Understanding MCP Stdio Communication

MCP Dockmaster spawns Node.js processes and communicates with them via stdin/stdout. This means:

1. Your tool receives commands as JSON objects via stdin
2. Your tool responds with JSON objects via stdout
3. Debug logs and errors should go to stderr (console.error)

## Using the MCPStdioAdapter

We provide a simple adapter (`mcpStdioAdapter.js`) that handles the stdin/stdout communication for you:

```javascript
const MCPStdioAdapter = require('../lib/mcpStdioAdapter');

// Create the adapter with your tools
const adapter = new MCPStdioAdapter({
  tools: [
    {
      id: 'my-tool',
      name: 'My Tool',
      description: 'A tool that does something',
      parameters: {
        // JSON Schema for the parameters
      }
    }
  ]
});

// Register an execute handler
adapter.registerHandler('execute', async (toolId, parameters) => {
  // Execute the tool and return a result
  return { result: 'Success!' };
});
```

## Command Protocol

The command protocol is simple:

1. **Tool Discovery**:
   ```json
   { "command": "discover_tools", "request_id": "discover_123" }
   ```
   Response:
   ```json
   { "request_id": "discover_123", "tools": [...] }
   ```

2. **Tool Execution**:
   ```json
   { 
     "command": "execute_tool", 
     "request_id": "execute_123", 
     "tool_id": "my-tool",
     "parameters": { ... } 
   }
   ```
   Response:
   ```json
   { "request_id": "execute_123", "result": { ... } }
   ```

## Example Tool

Check out `mcp-example-tool.js` for a simple example that implements two tools:
- A "Hello World" tool that greets a name
- An "Echo" tool that returns whatever was sent to it

## Running the Example

To run the example tool directly:

```bash
node mcp-example-tool.js --tool-id=example-tool
```

Then you can communicate with it by sending JSON to stdin:

```bash
echo '{"command":"discover_tools","request_id":"123"}' | node mcp-example-tool.js --tool-id=example-tool
```

In the MCP Dockmaster app, the tool will be spawned automatically when registered.

## Notes

- Always make sure your tool responds to each command with a JSON object
- Add a newline to the end of each JSON response
- Use stderr for debug logs, not stdout
- Handle errors gracefully and return appropriate error messages 