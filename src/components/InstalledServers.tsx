import React, { useState, useEffect } from 'react';
import MCPClient from '../lib/mcpClient';
import { dispatchToolUninstalled, dispatchToolStatusChanged } from '../lib/events';
import './InstalledServers.css';

interface InstalledTool {
  id: string;
  name: string;
  description: string;
  enabled: boolean;
}

const InstalledServers: React.FC = () => {
  const [installedTools, setInstalledTools] = useState<InstalledTool[]>([]);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    loadInstalledTools();
  }, []);

  const loadInstalledTools = async () => {
    setLoading(true);
    try {
      // Get tools from MCP client
      const tools = await MCPClient.listTools();
      
      // Transform to our internal format with enabled state
      // Ensure we don't have duplicates by using a Map with tool name as key
      const toolsMap = new Map();
      
      tools.forEach(tool => {
        const toolId = tool.id || `tool_${Math.random().toString(36).substr(2, 9)}`;
        // Only add if not already in the map (avoid duplicates)
        if (!toolsMap.has(tool.name.toLowerCase())) {
          toolsMap.set(tool.name.toLowerCase(), {
            id: toolId,
            name: tool.name,
            description: tool.description,
            enabled: tool.enabled !== undefined ? tool.enabled : true
          });
        }
      });
      
      // Convert map values to array
      const installedTools = Array.from(toolsMap.values());
      
      setInstalledTools(installedTools);
    } catch (error) {
      console.error('Failed to load installed tools:', error);
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
      loadInstalledTools();
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
        loadInstalledTools();
      }
    } catch (error) {
      console.error('Error uninstalling tool:', error);
      // Refresh the list to ensure UI is in sync with backend
      loadInstalledTools();
    }
  };

  return (
    <div className="installed-servers-container">
      <h2>My Applications</h2>
      <p>Manage your installed AI applications and MCP tools.</p>
      
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
            <div key={tool.id} className={`tool-card ${tool.enabled ? 'enabled' : 'disabled'}`}>
              <div className="tool-header">
                <h3>{tool.name}</h3>
                <div className="tool-status">
                  <span 
                    className={`app-status-indicator ${tool.enabled ? 'active' : 'inactive'}`}
                    onClick={() => toggleToolStatus(tool.id)}
                    role="switch"
                    aria-checked={tool.enabled}
                    tabIndex={0}
                  ></span>
                  <span>{tool.enabled ? 'Enabled' : 'Disabled'}</span>
                </div>
              </div>
              
              <p className="tool-description">{tool.description}</p>
              
              <div className="tool-actions">
                <button 
                  className="uninstall-button"
                  onClick={() => uninstallTool(tool.id)}
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