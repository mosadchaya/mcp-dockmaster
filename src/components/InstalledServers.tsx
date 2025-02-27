import React, { useState, useEffect } from 'react';
import MCPClient from '../lib/mcpClient';
import { dispatchToolUninstalled, dispatchToolStatusChanged } from '../lib/events';
import './InstalledServers.css';

interface InstalledTool {
  id: string;
  name: string;
  description: string;
  enabled: boolean;
  server_id?: string;
  process_running?: boolean;
  server_name?: string;
}

interface Server {
  id: string;
  name: string;
  tool_count: number;
  enabled: boolean;
  process_running: boolean;
}

interface ServerTool {
  id: string;
  name: string;
  description: string;
  server_id: string;
  proxy_id: string;
}

const InstalledServers: React.FC = () => {
  const [installedTools, setInstalledTools] = useState<InstalledTool[]>([]);
  const [servers, setServers] = useState<Server[]>([]);
  const [serverTools, setServerTools] = useState<ServerTool[]>([]);
  const [loading, setLoading] = useState(true);
  const [claudeConfig, setClaudeConfig] = useState<any>(null);
  const [showClaudeConfig, setShowClaudeConfig] = useState(false);
  const [expandedToolId, setExpandedToolId] = useState<string | null>(null);

  useEffect(() => {
    loadData();

    // Reload when the window regains focus
    window.addEventListener('focus', loadData);

    return () => {
      window.removeEventListener('focus', loadData);
    };
  }, []);

  // Effect to handle expanded tool changes
  useEffect(() => {
    if (expandedToolId) {
      console.log('Tool expanded:', expandedToolId);
      // Find the tool that was expanded
      const tool = installedTools.find(t => t.id === expandedToolId);
      if (tool?.server_id) {
        console.log('Expanded tool has server_id:', tool.server_id);
        // Instead of calling loadData which would trigger another render,
        // just ensure we have the server tools for this server
        const server = servers.find(s => s.id === tool.server_id);
        if (server && server.process_running) {
          discoverToolsForServer(tool.server_id);
        }
      }
    }
  // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [expandedToolId]);

  const loadData = async () => {
    setLoading(true);
    try {
      // Get all data from MCP client
      const allData = await MCPClient.getAllServerData();
      
      // Set servers (filtered to only active servers)
      const activeServers = allData.servers.filter((server: any) => server.process_running);
      setServers(activeServers);
      
      // Set server tools - make sure we have all tools from all servers
      const allServerTools = allData.tools || [];
      setServerTools(allServerTools);
      
      // Set Claude configuration
      setClaudeConfig(allData);

      // Get installed tools
      const tools = await MCPClient.listTools();
      
      // Transform to our internal format with enabled state
      // Ensure we don't have duplicates by using a Map with tool name as key
      const toolsMap = new Map();
      
      
      tools.forEach((tool: any) => {
        const toolId = tool.id || `tool_${Math.random().toString(36).substr(2, 9)}_${Date.now()}`;
        // Find server info for this tool
        const serverTool = allServerTools.find((st: any) => st.server_id === tool.id);
        const server = serverTool ? activeServers.find((s: any) => s.id === serverTool.server_id) : null;
        
        // Only add if not already in the map (avoid duplicates)
        if (!toolsMap.has(tool.name.toLowerCase())) {
          toolsMap.set(tool.name.toLowerCase(), {
            id: toolId,
            name: tool.name,
            description: tool.description,
            enabled: tool.enabled !== undefined ? tool.enabled : true,
            server_id: serverTool?.server_id,
            server_name: server?.name,
            process_running: server?.process_running
          });
        }
      });
      
      // Convert map values to array
      const installedTools = Array.from(toolsMap.values());
      
      setInstalledTools(installedTools);
    } catch (error) {
      console.error('Failed to load data:', error);
    } finally {
      setLoading(false);
    }
  };

  const toggleToolStatus = async (id: string) => {
    try {
      // Find the current tool to get its current enabled state
      const tool = installedTools.find(tool => tool.id === id);
      if (!tool) return;
      
      // Update the UI optimistically
      setInstalledTools(prev => 
        prev.map(tool => 
          tool.id === id ? { ...tool, enabled: !tool.enabled } : tool
        )
      );
      
      // Call the backend API to update the tool status
      const response = await MCPClient.updateToolStatus({
        tool_id: id,
        enabled: !tool.enabled
      });
      
      if (response.success) {
        // Dispatch event that a tool's status was changed
        dispatchToolStatusChanged(id);
      } else {
        // If the API call fails, revert the UI change
        console.error('Failed to update tool status:', response.message);
        setInstalledTools(prev => 
          prev.map(tool => 
            tool.id === id ? { ...tool, enabled: tool.enabled } : tool
          )
        );
      }
    } catch (error) {
      console.error('Error toggling tool status:', error);
      // Refresh the list to ensure UI is in sync with backend
      loadData();
    }
  };

  const uninstallTool = async (id: string) => {
    try {
      // Update the UI optimistically
      setInstalledTools(prev => prev.filter(tool => tool.id !== id));
      
      // Call the backend API to uninstall the tool
      const response = await MCPClient.uninstallTool({
        tool_id: id
      });
      
      if (response.success) {
        // Dispatch event that a tool was uninstalled
        dispatchToolUninstalled(id);
      } else {
        // If the API call fails, reload the tools to restore the UI
        console.error('Failed to uninstall tool:', response.message);
        loadData();
      }
    } catch (error) {
      console.error('Error uninstalling tool:', error);
      // Refresh the list to ensure UI is in sync with backend
      loadData();
    }
  };

  const discoverToolsForServer = async (serverId: string, e?: React.MouseEvent) => {
    if (e) {
      e.stopPropagation(); // Prevent the click from toggling the expanded state
    }
    
    try {
      await MCPClient.discoverTools({ server_id: serverId });
      // Reload all data after discovery
      loadData();
    } catch (error) {
      console.error(`Failed to discover tools for server ${serverId}:`, error);
    }
  };

  const copyToClipboard = (text: string) => {
    navigator.clipboard.writeText(text)
      .then(() => {
        alert('Configuration copied to clipboard!');
      })
      .catch(err => {
        console.error('Failed to copy text: ', err);
      });
  };

  const toggleExpandTool = (toolId: string, e: React.MouseEvent) => {
    // Don't toggle if clicking on status toggle or uninstall button
    const target = e.target as HTMLElement;
    if (
      target.closest('.app-status-indicator') || 
      target.closest('.uninstall-button')
    ) {
      return;
    }
    
    console.log('Toggling tool expansion for:', toolId);
    
    // If the tool is already expanded, collapse it
    if (expandedToolId === toolId) {
      console.log('Collapsing tool');
      setExpandedToolId(null);
    } else {
      console.log('Expanding tool');
      setExpandedToolId(toolId);
      
      // If the tool has a server_id and the server is running, refresh the tools
      const tool = installedTools.find(t => t.id === toolId);
      console.log('Tool found:', tool);
      
      if (tool?.server_id) {
        console.log('Tool has server_id:', tool.server_id);
        // Force refresh tools for this server regardless of running state
        discoverToolsForServer(tool.server_id);
      } else {
        console.log('Tool does not have server_id');
      }
    }
  };

  const renderServerInfo = (tool: InstalledTool) => {
    // Only check for expandedToolId match, not server_id
    if (expandedToolId !== tool.id) {
      return null;
    }
    
    // If the tool doesn't have a server_id, show a fallback message
    if (!tool.server_id) {
      return (
        <div className="server-info-container">
          <div className="empty-tools-message">
            <p>This application is not associated with an MCP server. It may be a standalone application or a built-in tool.</p>
          </div>
        </div>
      );
    }
    
    console.log('Rendering server info for tool:', tool.id, 'with server_id:', tool.server_id);
    
    const server = servers.find(s => s.id === tool.server_id);
    if (!server) {
      console.log('Server not found for id:', tool.server_id);
      return (
        <div className="server-info-container">
          <div className="empty-tools-message">
            <p>Server information not available. The server may not be running.</p>
          </div>
        </div>
      );
    }
    
    console.log('Server found:', server.name, 'Running:', server.process_running);
    const toolsForServer = serverTools.filter(t => t.server_id === tool.server_id);
    console.log('Tools for server:', toolsForServer.length);
    
    return (
      <div className="server-info-container">
        <div className="server-header">
          <h4>Server Information</h4>
          <div className="server-status-badge">
            <span className={server.process_running ? 'running' : 'stopped'}>
              {server.process_running ? 'Running' : 'Stopped'}
            </span>
          </div>
          <button 
            className="discover-button"
            onClick={(e) => {
              e.stopPropagation();
              discoverToolsForServer(server.id, e);
            }}
          >
            Refresh Tools
          </button>
        </div>
        
        {toolsForServer.length > 0 ? (
          <div className="server-tools">
            <h5>Available Tools</h5>
            <div className="server-tools-grid">
              {toolsForServer.map(tool => (
                <div key={tool.proxy_id} className="server-tool-card">
                  <h6>{tool.name}</h6>
                  <p>{tool.description}</p>
                </div>
              ))}
            </div>
          </div>
        ) : (
          <div className="empty-tools-message">
            <p>
              {server.process_running 
                ? "No tools discovered from this server yet. Click \"Refresh Tools\" to discover available tools." 
                : "Server is not running. Start the server to discover available tools."}
            </p>
          </div>
        )}
      </div>
    );
  };

  return (
    <div className="installed-servers-container">
      <div className="installed-servers-header">
        <h2>My Applications</h2>
        <p>Manage your installed AI applications and MCP tools.</p>
        
        <button 
          className="config-button"
          onClick={() => setShowClaudeConfig(!showClaudeConfig)}
        >
          {showClaudeConfig ? 'Hide' : 'Show'} Claude Configuration
        </button>
        
        {showClaudeConfig && claudeConfig && (
          <div className="claude-config">
            <h3>Claude Configuration</h3>
            <p>Use this configuration to connect Claude to your MCP servers:</p>
            <pre className="config-code">
              {JSON.stringify(claudeConfig, null, 2)}
            </pre>
            <button 
              className="copy-button"
              onClick={() => copyToClipboard(JSON.stringify(claudeConfig, null, 2))}
            >
              Copy to Clipboard
            </button>
          </div>
        )}
      </div>
      
      {loading ? (
        <div className="loading-message">Loading installed applications...</div>
      ) : installedTools.length === 0 ? (
        <div className="empty-state">
          <p>You don't have any applications installed yet.</p>
          <p>Visit the AI App Store to discover and install applications.</p>
        </div>
      ) : (
        <div className="tools-grid">
          {installedTools.map(tool => (
            <div 
              key={tool.id} 
              className={`tool-card ${tool.enabled ? 'enabled' : 'disabled'} ${expandedToolId === tool.id ? 'expanded' : ''}`}
              onClick={(e) => toggleExpandTool(tool.id, e)}
            >
              <div className="tool-header">
                <h3 className="tool-title">{tool.name}</h3>
                <div className="tool-status">
                  <span 
                    className={`app-status-indicator ${tool.enabled ? 'active' : 'inactive'}`}
                    onClick={(e) => {
                      e.stopPropagation();
                      toggleToolStatus(tool.id);
                    }}
                    role="switch"
                    aria-checked={tool.enabled}
                    tabIndex={0}
                  ></span>
                  <span>{tool.enabled ? 'Enabled' : 'Disabled'}</span>
                </div>
              </div>
              
              <p className="tool-description">{tool.description}</p>
              
              <div className="server-status-indicator">
                {tool.server_id ? (
                  <>
                    <span className={`server-status-dot ${tool.process_running ? 'running' : 'stopped'}`}></span>
                    <span className="server-status-text">
                      Server: {tool.server_name || 'Unknown'} ({tool.process_running ? 'Running' : 'Stopped'})
                    </span>
                  </>
                ) : (
                  <span className="server-status-text">
                    Click to view details
                  </span>
                )}
                <span className="expand-indicator">
                  {expandedToolId === tool.id ? '▼ Hide details' : '▶ Show details'}
                </span>
              </div>
              
              {renderServerInfo(tool)}
              
              <div className="tool-actions">
                <button 
                  className="uninstall-button"
                  onClick={(e) => {
                    e.stopPropagation();
                    uninstallTool(tool.id);
                  }}
                >
                  Uninstall
                </button>
              </div>
            </div>
          ))}
        </div>
      )}
    </div>
  );
};

export default InstalledServers; 