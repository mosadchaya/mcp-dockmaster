/**
 * MCP Stdio Adapter
 * 
 * This library provides a simple adapter for MCP tools to use stdin/stdout
 * for communication instead of HTTP requests.
 */

const readline = require('readline');

class MCPStdioAdapter {
  constructor(options = {}) {
    this.toolId = options.toolId || process.argv.find(arg => arg.startsWith('--tool-id=')).split('=')[1];
    this.tools = options.tools || [];
    this.handlers = options.handlers || {};
    
    // Create interface for stdin/stdout
    this.rl = readline.createInterface({
      input: process.stdin,
      output: process.stdout,
      terminal: false
    });
    
    // Setup line handler
    this.rl.on('line', (line) => this.handleCommand(line));
    
    // Log startup information
    console.error(`MCP tool ${this.toolId} started and listening on stdin`);
  }
  
  /**
   * Register a tool with the adapter
   */
  registerTool(tool) {
    this.tools.push(tool);
  }
  
  /**
   * Register a command handler
   */
  registerHandler(command, handler) {
    this.handlers[command] = handler;
  }
  
  /**
   * Handle incoming commands
   */
  async handleCommand(line) {
    try {
      const command = JSON.parse(line);
      
      // Check if we have a handler for this command
      if (command.command === 'discover_tools') {
        // Respond with the list of tools
        this.sendResponse({
          request_id: command.request_id,
          tools: this.tools
        });
      } else if (command.command === 'execute_tool') {
        // Find the tool
        const toolId = command.tool_id;
        const tool = this.tools.find(t => t.id === toolId);
        
        if (!tool) {
          this.sendError(command.request_id, `Tool ${toolId} not found`);
          return;
        }
        
        // Execute the tool
        if (this.handlers.execute) {
          try {
            const result = await this.handlers.execute(toolId, command.parameters);
            this.sendResponse({
              request_id: command.request_id,
              result
            });
          } catch (error) {
            this.sendError(command.request_id, error.message || String(error));
          }
        } else {
          this.sendError(command.request_id, 'No execute handler registered');
        }
      } else if (this.handlers[command.command]) {
        // Execute custom handler
        try {
          const result = await this.handlers[command.command](command);
          this.sendResponse({
            request_id: command.request_id,
            result
          });
        } catch (error) {
          this.sendError(command.request_id, error.message || String(error));
        }
      } else {
        this.sendError(command.request_id, `Unknown command: ${command.command}`);
      }
    } catch (error) {
      console.error('Error handling command:', error);
      // Try to send a response even if we can't parse the command
      process.stdout.write(JSON.stringify({
        error: `Failed to handle command: ${error.message || String(error)}`
      }) + '\n');
    }
  }
  
  /**
   * Send a response back through stdout
   */
  sendResponse(response) {
    process.stdout.write(JSON.stringify(response) + '\n');
  }
  
  /**
   * Send an error response
   */
  sendError(requestId, errorMessage) {
    this.sendResponse({
      request_id: requestId,
      error: errorMessage
    });
  }
}

module.exports = MCPStdioAdapter; 