use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager, Pool};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use directories::ProjectDirs;
use log::info;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::sync::Arc;

use crate::models::types::Tool;
use crate::schema::tools;
use crate::schema::server_tools;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations/sqlite");

type SqlitePool = Pool<ConnectionManager<SqliteConnection>>;

/// Database manager for persisting application state
pub struct DBManager {
    pool: Arc<SqlitePool>,
}

impl DBManager {
    /// Initialize the database manager with the default database path
    pub fn new() -> Result<Self, String> {
        let db_path = get_database_path()?;
        Self::with_path(db_path)
    }

    /// Initialize the database manager with a custom database path
    pub fn with_path(db_path: PathBuf) -> Result<Self, String> {
        // Ensure the parent directory exists
        if let Some(parent) = db_path.parent() {
            if !parent.exists() {
                fs::create_dir_all(parent)
                    .map_err(|e| format!("Failed to create database directory: {}", e))?;
            }
        }

        // Create the database URL - use file: prefix for SQLite
        let database_url = format!("sqlite://{}", db_path.to_string_lossy());

        // Ensure the database file exists
        if !db_path.exists() {
            std::fs::File::create(&db_path)
                .map_err(|e| format!("Failed to create database file: {}", e))?;
        }

        // Create the connection manager
        let manager = ConnectionManager::<SqliteConnection>::new(database_url);

        // Create the connection pool with more conservative settings
        let pool = r2d2::Pool::builder()
            .max_size(5)
            .connection_timeout(std::time::Duration::from_secs(5))
            .build(manager)
            .map_err(|e| format!("Failed to create connection pool: {}", e))?;

        // Get a connection to initialize the database
        let mut conn = pool
            .get()
            .map_err(|e| format!("Failed to get database connection: {}", e))?;

        // Run migrations
        conn.run_pending_migrations(MIGRATIONS)
            .map_err(|e| format!("Failed to run migrations: {}", e))?;

        // Set pragmas
        diesel::sql_query("PRAGMA journal_mode=WAL")
            .execute(&mut conn)
            .map_err(|e| format!("Failed to set journal_mode: {}", e))?;
        
        diesel::sql_query("PRAGMA synchronous=FULL")
            .execute(&mut conn)
            .map_err(|e| format!("Failed to set synchronous: {}", e))?;
        
        diesel::sql_query("PRAGMA temp_store=MEMORY")
            .execute(&mut conn)
            .map_err(|e| format!("Failed to set temp_store: {}", e))?;
        
        diesel::sql_query("PRAGMA optimize")
            .execute(&mut conn)
            .map_err(|e| format!("Failed to optimize: {}", e))?;
        
        diesel::sql_query("PRAGMA busy_timeout = 5000")
            .execute(&mut conn)
            .map_err(|e| format!("Failed to set busy_timeout: {}", e))?;
        
        diesel::sql_query("PRAGMA mmap_size=262144000")
            .execute(&mut conn)
            .map_err(|e| format!("Failed to set mmap_size: {}", e))?;
        
        diesel::sql_query("PRAGMA foreign_keys = ON")
            .execute(&mut conn)
            .map_err(|e| format!("Failed to enable foreign keys: {}", e))?;

        let db_manager = Self {
            pool: Arc::new(pool),
        };

        info!("Database initialized at: {:?}", db_path);
        Ok(db_manager)
    }

    /// Get a tool by ID
    pub fn get_tool(&self, tool_id: &str) -> Result<Tool, String> {
        let mut conn = self
            .pool
            .get()
            .map_err(|e| format!("Failed to get database connection: {}", e))?;

        let result = tools::table
            .filter(tools::id.eq(tool_id))
            .select(tools::data)
            .first::<String>(&mut conn)
            .map_err(|e| format!("Failed to get tool {}: {}", tool_id, e))?;

        serde_json::from_str(&result).map_err(|e| format!("Failed to deserialize tool data: {}", e))
    }

    /// Get all tools
    pub fn get_all_tools(&self) -> Result<HashMap<String, Tool>, String> {
        let mut conn = self
            .pool
            .get()
            .map_err(|e| format!("Failed to get database connection: {}", e))?;

        let results = tools::table
            .select((tools::id, tools::data))
            .load::<(String, String)>(&mut conn)
            .map_err(|e| format!("Failed to query tools: {}", e))?;

        let mut tools_map = HashMap::new();
        for (tool_id, tool_data_str) in results {
            let tool: Tool = serde_json::from_str(&tool_data_str)
                .map_err(|e| format!("Failed to deserialize tool data: {}", e))?;
            tools_map.insert(tool_id, tool);
        }

        Ok(tools_map)
    }

    /// Save or update a tool
    pub fn save_tool(&self, tool_id: &str, tool: &Tool) -> Result<(), String> {
        let mut conn = self
            .pool
            .get()
            .map_err(|e| format!("Failed to get database connection: {}", e))?;

        let tool_json = serde_json::to_string(tool)
            .map_err(|e| format!("Failed to serialize tool data: {}", e))?;

        diesel::insert_into(tools::table)
            .values((tools::id.eq(tool_id), tools::data.eq(&tool_json)))
            .on_conflict(tools::id)
            .do_update()
            .set(tools::data.eq(&tool_json))
            .execute(&mut conn)
            .map_err(|e| format!("Failed to save tool: {}", e))?;

        Ok(())
    }

    /// Delete a tool by ID
    pub fn delete_tool(&self, tool_id: &str) -> Result<(), String> {
        let mut conn = self
            .pool
            .get()
            .map_err(|e| format!("Failed to get database connection: {}", e))?;

        diesel::delete(tools::table.filter(tools::id.eq(tool_id)))
            .execute(&mut conn)
            .map_err(|e| format!("Failed to delete tool: {}", e))?;

        Ok(())
    }

    /// Clear the database
    pub fn clear_database(&mut self) -> Result<(), String> {
        let mut conn = self
            .pool
            .get()
            .map_err(|e| format!("Failed to get database connection: {}", e))?;

        conn.transaction::<_, diesel::result::Error, _>(|conn| {
            diesel::delete(tools::table)
                .execute(conn)?;

            diesel::delete(server_tools::table)
                .execute(conn)?;

            Ok(())
        })
        .map_err(|e| format!("Transaction failed: {}", e))?;

        info!("Database cleared successfully");
        Ok(())
    }

    /// Check if the database exists and has data
    pub fn check_exists(&self) -> Result<bool, String> {
        let mut conn = self
            .pool
            .get()
            .map_err(|e| format!("Failed to get database connection: {}", e))?;

        let count: i64 = tools::table
            .count()
            .get_result(&mut conn)
            .map_err(|e| format!("Failed to check database: {}", e))?;

        Ok(count > 0)
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
