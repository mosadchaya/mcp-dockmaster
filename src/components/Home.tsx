import React, { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/core';

interface PrerequisiteStatus {
  name: string;
  installed: boolean;
  loading: boolean;
}

const Home: React.FC = () => {
  const [prerequisites, setPrerequisites] = useState<PrerequisiteStatus[]>([
    { name: 'Node.js', installed: false, loading: true },
    { name: 'uv', installed: false, loading: true },
    { name: 'Docker', installed: false, loading: true },
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
        { name: 'Node.js', installed: nodeInstalled, loading: false },
        { name: 'uv', installed: uvInstalled, loading: false },
        { name: 'Docker', installed: dockerInstalled, loading: false },
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
      
      <div style={styles.prerequisitesContainer}>
        <div style={styles.prerequisitesHeader}>
          <h3>Tool Support</h3>
          <button 
            onClick={checkPrerequisites} 
            disabled={isChecking}
            style={styles.refreshButton}
          >
            {isChecking ? 'Checking...' : 'Refresh'}
          </button>
        </div>
        
        <div style={styles.prerequisitesList}>
          {prerequisites.map((prerequisite) => (
            <div key={prerequisite.name} style={styles.prerequisiteItem}>
              <span style={styles.prerequisiteName}>{prerequisite.name}</span>
              {prerequisite.loading ? (
                <span style={styles.loadingIndicator}>Checking...</span>
              ) : (
                <span style={styles.statusIndicator}>
                  {prerequisite.installed ? (
                    <div style={styles.statusLight.success} title="Installed and running"></div>
                  ) : (
                    <div style={styles.statusLight.error} title="Not installed or not running"></div>
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

const styles = {
  prerequisitesContainer: {
    marginTop: '2rem',
    padding: '1.5rem',
    borderRadius: '8px',
    backgroundColor: '#f8f9fa',
    boxShadow: '0 1px 3px rgba(0, 0, 0, 0.1)',
  },
  prerequisitesHeader: {
    display: 'flex',
    justifyContent: 'space-between',
    alignItems: 'center',
    marginBottom: '1rem',
  },
  refreshButton: {
    backgroundColor: '#007bff',
    color: 'white',
    border: 'none',
    borderRadius: '4px',
    padding: '0.375rem 0.75rem',
    cursor: 'pointer',
    fontSize: '0.875rem',
  },
  prerequisitesList: {
    display: 'flex',
    flexDirection: 'column' as const,
    gap: '0.75rem',
  },
  prerequisiteItem: {
    display: 'flex',
    justifyContent: 'space-between',
    alignItems: 'center',
    padding: '0.75rem',
    borderRadius: '4px',
    backgroundColor: 'white',
    border: '1px solid #dee2e6',
  },
  prerequisiteName: {
    fontWeight: 500 as const,
  },
  loadingIndicator: {
    color: '#6c757d',
    fontStyle: 'italic' as const,
  },
  statusIndicator: {
    display: 'flex',
    alignItems: 'center',
  },
  statusLight: {
    success: {
      width: '16px',
      height: '16px',
      borderRadius: '50%',
      backgroundColor: '#28a745',
      boxShadow: '0 0 10px #28a745',
    },
    error: {
      width: '16px',
      height: '16px',
      borderRadius: '50%',
      backgroundColor: '#dc3545',
      boxShadow: '0 0 10px #dc3545',
    }
  }
};

export default Home; 