use directories::ProjectDirs;
use log::{error, info};
use rusqlite::{params, Connection};
use serde_json::Value;
use crate::features::mcp_proxy::{MCPState, ToolRegistry};
use std::fs;
use std::path::PathBuf;

/// Database manager for persisting application state
pub struct DatabaseManager {
    conn: Connection,
}

impl DatabaseManager {
    /// Initialize the database manager
    pub fn new() -> Result<Self, String> {
        let db_path = get_database_path()?;
        
        // Ensure the parent directory exists
        if let Some(parent) = db_path.parent() {
            if !parent.exists() {
                fs::create_dir_all(parent).map_err(|e| format!("Failed to create database directory: {}", e))?;
            }
        }
        
        // Open or create the database
        let conn = Connection::open(&db_path)
            .map_err(|e| format!("Failed to open database: {}", e))?;
        
        let db_manager = Self { conn };
        db_manager.initialize_tables()?;
        
        info!("Database initialized at: {:?}", db_path);
        Ok(db_manager)
    }
    
    /// Initialize database tables
    fn initialize_tables(&self) -> Result<(), String> {
        self.conn
            .execute(
                "CREATE TABLE IF NOT EXISTS tools (
                    id TEXT PRIMARY KEY,
                    data TEXT NOT NULL
                )",
                [],
            )
            .map_err(|e| format!("Failed to create tools table: {}", e))?;
            
        self.conn
            .execute(
                "CREATE TABLE IF NOT EXISTS server_tools (
                    server_id TEXT,
                    tool_data TEXT NOT NULL,
                    PRIMARY KEY (server_id)
                )",
                [],
            )
            .map_err(|e| format!("Failed to create server_tools table: {}", e))?;
            
        Ok(())
    }
    
    /// Save tool registry to database
    pub fn save_tool_registry(&mut self, registry: &ToolRegistry) -> Result<(), String> {
        // Begin transaction
        let tx = self.conn.transaction()
            .map_err(|e| format!("Failed to begin transaction: {}", e))?;
            
        // Clear existing tools
        tx.execute("DELETE FROM tools", [])
            .map_err(|e| format!("Failed to clear tools table: {}", e))?;
            
        // Insert tools
        for (tool_id, tool_data) in &registry.tools {
            let tool_json = serde_json::to_string(tool_data)
                .map_err(|e| format!("Failed to serialize tool data: {}", e))?;
                
            tx.execute(
                "INSERT INTO tools (id, data) VALUES (?1, ?2)",
                params![tool_id, tool_json],
            )
            .map_err(|e| format!("Failed to insert tool: {}", e))?;
        }
        
        // Clear existing server tools
        tx.execute("DELETE FROM server_tools", [])
            .map_err(|e| format!("Failed to clear server_tools table: {}", e))?;
            
        // Insert server tools
        for (server_id, tools) in &registry.server_tools {
            let tools_json = serde_json::to_string(tools)
                .map_err(|e| format!("Failed to serialize server tools: {}", e))?;
                
            tx.execute(
                "INSERT INTO server_tools (server_id, tool_data) VALUES (?1, ?2)",
                params![server_id, tools_json],
            )
            .map_err(|e| format!("Failed to insert server tools: {}", e))?;
        }
        
        // Commit transaction
        tx.commit()
            .map_err(|e| format!("Failed to commit transaction: {}", e))?;
            
        info!("Tool registry saved to database");
        Ok(())
    }
    
    /// Load tool registry from database
    pub fn load_tool_registry(&self) -> Result<ToolRegistry, String> {
        let mut registry = ToolRegistry::default();
        
        // Load tools
        let mut stmt = self.conn
            .prepare("SELECT id, data FROM tools")
            .map_err(|e| format!("Failed to prepare tools query: {}", e))?;
            
        let tool_rows = stmt
            .query_map([], |row| {
                let id: String = row.get(0)?;
                let data_str: String = row.get(1)?;
                let data: Value = serde_json::from_str(&data_str)
                    .map_err(|e| rusqlite::Error::InvalidParameterName(format!("Invalid JSON: {}", e)))?;
                Ok((id, data))
            })
            .map_err(|e| format!("Failed to query tools: {}", e))?;
            
        for tool_result in tool_rows {
            let (id, data) = tool_result
                .map_err(|e| format!("Failed to read tool row: {}", e))?;
            registry.tools.insert(id, data);
        }
        
        // Load server tools
        let mut stmt = self.conn
            .prepare("SELECT server_id, tool_data FROM server_tools")
            .map_err(|e| format!("Failed to prepare server_tools query: {}", e))?;
            
        let server_rows = stmt
            .query_map([], |row| {
                let server_id: String = row.get(0)?;
                let tools_str: String = row.get(1)?;
                let tools: Vec<Value> = serde_json::from_str(&tools_str)
                    .map_err(|e| rusqlite::Error::InvalidParameterName(format!("Invalid JSON: {}", e)))?;
                Ok((server_id, tools))
            })
            .map_err(|e| format!("Failed to query server tools: {}", e))?;
            
        for server_result in server_rows {
            let (server_id, tools) = server_result
                .map_err(|e| format!("Failed to read server tools row: {}", e))?;
            registry.server_tools.insert(server_id, tools);
        }
        
        info!("Tool registry loaded from database");
        Ok(registry)
    }
    
    /// Safely close the database connection
    pub fn close(self) -> Result<(), String> {
        // The connection will be closed automatically when self is dropped,
        // but we can add explicit handling if needed
        info!("Database connection closed");
        Ok(())
    }
}

/// Get the path to the database file
fn get_database_path() -> Result<PathBuf, String> {
    let proj_dirs = ProjectDirs::from("com", "mcp", "dockmaster")
        .ok_or_else(|| "Failed to determine project directories".to_string())?;
        
    let data_dir = proj_dirs.data_dir();
    
    // Ensure the data directory exists
    if !data_dir.exists() {
        fs::create_dir_all(data_dir)
            .map_err(|e| format!("Failed to create data directory: {}", e))?;
    }
    
    // Check if the directory is writable
    let test_file = data_dir.join(".write_test");
    match fs::File::create(&test_file) {
        Ok(_) => {
            // Clean up the test file
            let _ = fs::remove_file(&test_file);
        },
        Err(e) => {
            return Err(format!("Data directory is not writable: {}", e));
        }
    }
    
    let db_path = data_dir.join("mcp_dockmaster.db");
    info!("Database path: {:?}", db_path);
    Ok(db_path)
}

/// Initialize the MCP state from the database
pub async fn initialize_mcp_state() -> MCPState {
    let mcp_state = MCPState::default();
    
    let db_manager = match DatabaseManager::new() {
        Ok(manager) => manager,
        Err(e) => {
            error!("Failed to initialize database: {}", e);
            return mcp_state;
        }
    };

    // Load the tool registry
    match db_manager.load_tool_registry() {
        Ok(registry_data) => {
            // Explicitly close the database connection
            let _ = db_manager.close();
            
            // Update the mcp_state.tool_registry with the loaded data
            let mut registry = mcp_state.tool_registry.write().await;
            *registry = registry_data;
            info!("Successfully loaded MCP state from database");
            
            // Restart enabled tools
            drop(registry); // Release the lock before calling restart_enabled_tools
            restart_enabled_tools(&mcp_state).await;
        }
        Err(e) => {
            // Explicitly close the database connection
            let _ = db_manager.close();
            
            error!("Failed to load MCP state from database: {}", e);
        }
    }
    
    mcp_state
}

/// Restart all enabled tools after loading from database
async fn restart_enabled_tools(mcp_state: &MCPState) {
    info!("Restarting enabled tools...");
    
    // Get a list of all enabled tools with their data
    let enabled_tools = {
        let registry = mcp_state.tool_registry.read().await;
        let mut tools = Vec::new();
        
        for (tool_id, tool_data) in &registry.tools {
            // Check if the tool is enabled
            if let Some(enabled) = tool_data.get("enabled").and_then(|v| v.as_bool()) {
                if enabled {
                    info!("Found enabled tool: {}", tool_id);
                    tools.push((tool_id.clone(), tool_data.clone()));
                }
            }
        }
        
        tools
    };
    
    info!("Found {} enabled tools to restart", enabled_tools.len());
    
    if enabled_tools.is_empty() {
        info!("No enabled tools to restart");
        return;
    }
    
    // Restart each enabled tool
    for (tool_id, tool_data) in enabled_tools {
        info!("Attempting to restart tool: {}", tool_id);
        
        // Get tool type and entry point
        let tool_type = tool_data.get("tool_type")
            .and_then(|v| v.as_str())
            .unwrap_or("");
        
        info!("Tool {} is type {}", tool_id, tool_type);
        
        // Get a write lock on the registry to restart the tool
        let restart_result = {
            let mut registry = mcp_state.tool_registry.write().await;
            registry.restart_tool(&tool_id).await
        };
        
        match restart_result {
            Ok(_) => {
                info!("Successfully restarted tool: {}", tool_id);
            },
            Err(e) => {
                error!("Failed to restart tool {}: {}", tool_id, e);
            }
        }
        
        // Add a small delay between tool starts to avoid resource contention
        tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
    }
    
    info!("Finished restarting enabled tools");
}

/// Save the current MCP state to the database
pub async fn save_mcp_state(mcp_state: &MCPState) -> Result<(), String> {
    // First, get a clone of the registry data to avoid holding the lock for too long
    let registry_data = {
        let registry = mcp_state.tool_registry.read().await;
        
        // Create clones of the data we need to save
        let tools_clone = registry.tools.clone();
        let server_tools_clone = registry.server_tools.clone();
        
        // Create a temporary registry with just the data we need
        let mut temp_registry = ToolRegistry::default();
        temp_registry.tools = tools_clone;
        temp_registry.server_tools = server_tools_clone;
        temp_registry
    };
    
    // Now save the cloned data to the database
    match DatabaseManager::new() {
        Ok(mut db_manager) => {
            let result = db_manager.save_tool_registry(&registry_data);
            
            // Explicitly close the database connection
            let _ = db_manager.close();
            
            match result {
                Ok(_) => {
                    info!("Successfully saved MCP state to database");
                    Ok(())
                },
                Err(e) => {
                    error!("Failed to save tool registry: {}", e);
                    Err(e)
                }
            }
        }
        Err(e) => {
            error!("Failed to initialize database for saving: {}", e);
            Err(e)
        }
    }
}

/// Check if the database exists and has data
pub fn check_database_exists() -> Result<bool, String> {
    let db_path = get_database_path()?;
    
    // Check if the database file exists
    if !db_path.exists() {
        return Ok(false);
    }
    
    // Try to open the database and check if it has data
    let conn = Connection::open(&db_path)
        .map_err(|e| format!("Failed to open database: {}", e))?;
        
    // Check if the tools table exists and has data
    let count: i64 = conn
        .query_row(
            "SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name='tools'",
            [],
            |row| row.get(0),
        )
        .map_err(|e| format!("Failed to check if tools table exists: {}", e))?;
        
    if count == 0 {
        return Ok(false);
    }
    
    // Check if there are any tools in the database
    let tool_count: i64 = conn
        .query_row("SELECT COUNT(*) FROM tools", [], |row| row.get(0))
        .unwrap_or(0);
        
    Ok(tool_count > 0)
}

/// Clear all data from the database
pub fn clear_database() -> Result<(), String> {
    let db_path = get_database_path()?;
    
    // Check if the database file exists
    if !db_path.exists() {
        return Ok(());  // Nothing to clear
    }
    
    // Try to open the database and clear all tables
    let mut conn = Connection::open(&db_path)
        .map_err(|e| format!("Failed to open database: {}", e))?;
        
    // Begin transaction
    let tx = conn.transaction()
        .map_err(|e| format!("Failed to begin transaction: {}", e))?;
        
    // Clear tools table
    tx.execute("DELETE FROM tools", [])
        .map_err(|e| format!("Failed to clear tools table: {}", e))?;
        
    // Clear server_tools table
    tx.execute("DELETE FROM server_tools", [])
        .map_err(|e| format!("Failed to clear server_tools table: {}", e))?;
        
    // Commit transaction
    tx.commit()
        .map_err(|e| format!("Failed to commit transaction: {}", e))?;
        
    info!("Database cleared successfully");
    Ok(())
} 