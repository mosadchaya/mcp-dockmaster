import { useEffect, useState } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import { dispatchToolStatusChanged } from '../lib/events';

interface LoadingOverlayProps {
  onInitializationComplete: () => void;
}

const LoadingOverlay: React.FC<LoadingOverlayProps> = ({ onInitializationComplete }) => {
  const [progress, setProgress] = useState(0);
  
  useEffect(() => {
    // Set up a listener for the initialization complete event
    const unlisten = listen('mcp-initialization-complete', () => {
      console.log('Received initialization complete event');
      
      // Trigger a refresh of all tools
      dispatchToolStatusChanged('all');
      
      onInitializationComplete();
    });
    
    // Poll for initialization status
    const interval = setInterval(async () => {
      try {
        const isComplete = await invoke<boolean>('check_initialization_complete');
        if (isComplete) {
          console.log('Initialization is complete');
          
          // Trigger a refresh of all tools
          dispatchToolStatusChanged('all');
          
          onInitializationComplete();
          clearInterval(interval);
        } else {
          // Increment progress for visual feedback
          setProgress(prev => Math.min(prev + 5, 90));
        }
      } catch (error) {
        console.error('Error checking initialization status:', error);
      }
    }, 500);
    
    return () => {
      unlisten.then(fn => fn());
      clearInterval(interval);
    };
  }, [onInitializationComplete]);
  
  return (
    <div className="loading-overlay">
      <div className="loading-content">
        <h2>Starting MCP Services</h2>
        <p>Please wait while we initialize your MCP servers...</p>
        <div className="progress-bar-container">
          <div className="progress-bar" style={{ width: `${progress}%` }}></div>
        </div>
      </div>
    </div>
  );
};

export default LoadingOverlay; 