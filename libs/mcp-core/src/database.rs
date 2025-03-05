use crate::mcp_proxy::ToolRegistry;
use directories::ProjectDirs;
use log::info;
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::params;
use serde_json::Value;
use std::fs;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Duration;

/// Database manager for persisting application state
pub struct DatabaseManager {
    pool: Arc<Pool<SqliteConnectionManager>>,
}

impl DatabaseManager {
    /// Initialize the database manager
    pub fn new() -> Result<Self, String> {
        let db_path = get_database_path()?;

        // Ensure the parent directory exists
        if let Some(parent) = db_path.parent() {
            if !parent.exists() {
                fs::create_dir_all(parent)
                    .map_err(|e| format!("Failed to create database directory: {}", e))?;
            }
        }

        // Create the connection manager
        let manager = SqliteConnectionManager::file(&db_path);
        
        // Create the connection pool
        let pool = Pool::builder()
            .max_size(10)
            .connection_timeout(Duration::from_secs(60))
            .build(manager)
            .map_err(|e| format!("Failed to create connection pool: {}", e))?;
        
        // Get a connection to initialize the database
        let conn = pool
            .get()
            .map_err(|e| format!("Failed to get database connection: {}", e))?;
        
        // Enable WAL mode and set optimizations
        conn.execute_batch(
            "PRAGMA journal_mode=WAL;
             PRAGMA synchronous=FULL;
             PRAGMA temp_store=MEMORY;
             PRAGMA optimize;
             PRAGMA busy_timeout = 5000;
             PRAGMA mmap_size=262144000; -- 250 MB in bytes (250 * 1024 * 1024)
             PRAGMA foreign_keys = ON;", // Enable foreign key support
        ).map_err(|e| format!("Failed to set PRAGMA configurations: {}", e))?;

        let db_manager = Self { pool: Arc::new(pool) };
        db_manager.initialize_tables()?;

        info!("Database initialized at: {:?}", db_path);
        Ok(db_manager)
    }

    /// Initialize database tables
    fn initialize_tables(&self) -> Result<(), String> {
        let conn = self.pool.get().map_err(|e| format!("Failed to get database connection: {}", e))?;
        
        conn.execute(
            "CREATE TABLE IF NOT EXISTS tools (
                id TEXT PRIMARY KEY,
                data TEXT NOT NULL
            )",
            [],
        )
        .map_err(|e| format!("Failed to create tools table: {}", e))?;

        conn.execute(
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
        // Get a connection from the pool
        let mut conn = self.pool.get().map_err(|e| format!("Failed to get database connection: {}", e))?;
        
        // Begin transaction
        let tx = conn
            .transaction()
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
        
        // Get a connection from the pool
        let conn = self.pool.get().map_err(|e| format!("Failed to get database connection: {}", e))?;

        // Load tools
        let mut stmt = conn
            .prepare("SELECT id, data FROM tools")
            .map_err(|e| format!("Failed to prepare tools query: {}", e))?;

        let tool_rows = stmt
            .query_map([], |row| {
                let id: String = row.get(0)?;
                let data_str: String = row.get(1)?;
                let data: Value = serde_json::from_str(&data_str).map_err(|e| {
                    rusqlite::Error::InvalidParameterName(format!("Invalid JSON: {}", e))
                })?;
                Ok((id, data))
            })
            .map_err(|e| format!("Failed to query tools: {}", e))?;

        for tool_result in tool_rows {
            let (id, data) = tool_result.map_err(|e| format!("Failed to read tool row: {}", e))?;
            registry.tools.insert(id, data);
        }

        // Load server tools
        let mut stmt = conn
            .prepare("SELECT server_id, tool_data FROM server_tools")
            .map_err(|e| format!("Failed to prepare server_tools query: {}", e))?;

        let server_rows = stmt
            .query_map([], |row| {
                let server_id: String = row.get(0)?;
                let tools_str: String = row.get(1)?;
                let tools: Vec<Value> = serde_json::from_str(&tools_str).map_err(|e| {
                    rusqlite::Error::InvalidParameterName(format!("Invalid JSON: {}", e))
                })?;
                Ok((server_id, tools))
            })
            .map_err(|e| format!("Failed to query server tools: {}", e))?;

        for server_result in server_rows {
            let (server_id, tools) =
                server_result.map_err(|e| format!("Failed to read server tools row: {}", e))?;
            registry.server_tools.insert(server_id, tools);
        }

        info!("Tool registry loaded from database");
        Ok(registry)
    }

    /// Clear the database
    pub fn clear_database(&mut self) -> Result<(), String> {
        // Get a connection from the pool
        let mut conn = self.pool.get().map_err(|e| format!("Failed to get database connection: {}", e))?;
        
        // Begin transaction
        let tx = conn
            .transaction()
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

    /// Safely close the database connection
    pub fn close(self) -> Result<(), String> {
        // The connection pool will be dropped when self is dropped
        // No explicit handling needed
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
        }
        Err(e) => {
            return Err(format!("Data directory is not writable: {}", e));
        }
    }

    let db_path = data_dir.join("mcp_dockmaster.db");
    info!("Database path: {:?}", db_path);
    Ok(db_path)
}

/// Check if the database exists and has data
pub fn check_database_exists() -> Result<bool, String> {
    let db_path = get_database_path()?;

    // Check if the database file exists
    if !db_path.exists() {
        return Ok(false);
    }

    // Create a temporary connection manager and pool
    let manager = SqliteConnectionManager::file(&db_path);
    let pool = Pool::builder()
        .max_size(1)
        .connection_timeout(Duration::from_secs(5))
        .build(manager)
        .map_err(|e| format!("Failed to create connection pool: {}", e))?;
        
    // Get a connection to check the database
    let conn = pool.get().map_err(|e| format!("Failed to get database connection: {}", e))?;

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
