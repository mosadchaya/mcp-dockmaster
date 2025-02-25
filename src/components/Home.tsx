import React, { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/core';
import './Home.css';

// Import runner icons
import dockerIcon from '../assets/docker.svg';
import nodeIcon from '../assets/node.svg';
import pythonIcon from '../assets/python.svg';

interface PrerequisiteStatus {
  name: string;
  installed: boolean;
  loading: boolean;
  icon: string;
}

const Home: React.FC = () => {
  const [prerequisites, setPrerequisites] = useState<PrerequisiteStatus[]>([
    { name: 'Node.js', installed: false, loading: true, icon: nodeIcon },
    { name: 'UV (Python)', installed: false, loading: true, icon: pythonIcon },
    { name: 'Docker', installed: false, loading: true, icon: dockerIcon },
  ]);
  const [isChecking, setIsChecking] = useState(false);

  const checkPrerequisites = async () => {
    setIsChecking(true);
    setPrerequisites(prev => prev.map(item => ({ ...item, loading: true })));
    
    try {
      // Check if Node.js is installed
      const checkNode = async () => {
        try {
          const installed = await invoke<boolean>('check_node_installed');
          return installed;
        } catch (error) {
          console.error('Failed to check Node.js:', error);
          return false;
        }
      };

      // Check if uv is installed
      const checkUv = async () => {
        try {
          const installed = await invoke<boolean>('check_uv_installed');
          return installed;
        } catch (error) {
          console.error('Failed to check uv:', error);
          return false;
        }
      };

      // Check if Docker is installed
      const checkDocker = async () => {
        try {
          const installed = await invoke<boolean>('check_docker_installed');
          return installed;
        } catch (error) {
          console.error('Failed to check Docker:', error);
          return false;
        }
      };

      const [nodeInstalled, uvInstalled, dockerInstalled] = await Promise.all([
        checkNode(),
        checkUv(),
        checkDocker()
      ]);
      
      setPrerequisites([
        { name: 'Node.js', installed: nodeInstalled, loading: false, icon: nodeIcon },
        { name: 'UV (Python)', installed: uvInstalled, loading: false, icon: pythonIcon },
        { name: 'Docker', installed: dockerInstalled, loading: false, icon: dockerIcon },
      ]);
    } catch (error) {
      console.error('Failed to check prerequisites:', error);
      setPrerequisites(prev => 
        prev.map(item => ({ ...item, loading: false }))
      );
    } finally {
      setIsChecking(false);
    }
  };

  useEffect(() => {
    checkPrerequisites();
  }, []);

  return (
    <div className="home-container">
      <h2>Welcome to Shinkai AI App Manager</h2>
      <p>Select an option from the sidebar to get started.</p>
      
      <div className="claude-instructions-container">
        <h3>Claude MCP Integration</h3>
        <p>Claude is automatically connected to Shinkai AI App Manager. Whenever you enable, disable or make changes to your apps and tools in Shinkai AI App Manager, you may refresh Claude
          by pressing the following keyboard shortcut:</p>
        <div className="keyboard-shortcuts">
          <div className="keyboard-key">⌘</div>
          <div className="keyboard-key">+</div>
          <div className="keyboard-key">R</div>
        </div>
        <p>in the Claude app. If this does not work, you may click the refresh button in the Claude app.</p>
      </div>
      
      <div className="prerequisites-container">
        <div className="prerequisites-header">
          <h3>Runner Environment Support</h3>
          <button 
            onClick={checkPrerequisites} 
            disabled={isChecking}
            className="refresh-button"
          >
            {isChecking ? 'Checking...' : 'Refresh'}
          </button>
        </div>
        
        <div className="prerequisites-list">
          {prerequisites.map((prerequisite) => (
            <div key={prerequisite.name} className="prerequisite-item">
              <div className="prerequisite-info">
                <img 
                  src={prerequisite.icon} 
                  alt={prerequisite.name} 
                  className="runner-icon"
                />
                <span className="prerequisite-name">{prerequisite.name}</span>
              </div>
              {prerequisite.loading ? (
                <span className="loading-indicator">Checking...</span>
              ) : (
                <span className="status-indicator">
                  {prerequisite.installed ? (
                    <div className="status-light success" title="Installed and running"></div>
                  ) : (
                    <div className="status-light error" title="Not installed or not running"></div>
                  )}
                </span>
              )}
            </div>
          ))}
        </div>
      </div>
    </div>
  );
};

export default Home; 