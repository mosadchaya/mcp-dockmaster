use std::path::Path;

use diesel::{
    prelude::QueryableByName,
    r2d2::{self, ConnectionManager},
    RunQueryDsl, SqliteConnection,
};

use crate::{mcp_installers::install_paths, utils::process::kill_process_by_name};

use super::{install_errors::CursorError, install_paths::get_cursor_db_path};

type Pool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

#[derive(QueryableByName)]
struct TableName {
    #[diesel(sql_type = diesel::sql_types::Text)]
    name: String,
}

#[derive(QueryableByName)]
struct ItemTableRow {
    #[diesel(sql_type = diesel::sql_types::Text)]
    key: String,
    #[diesel(sql_type = diesel::sql_types::Text)]
    value: String,
}

pub fn get_item_value(key: &str) -> Result<Option<String>, CursorError> {
    let db_path = get_cursor_db_path()?;

    if !Path::new(&db_path).exists() {
        return Err(CursorError::DatabaseNotFound(db_path));
    }

    let manager = ConnectionManager::<SqliteConnection>::new(db_path);
    let pool = Pool::builder()
        .build(manager)
        .expect("Failed to create pool");

    let mut conn = pool.get().map_err(|e| {
        println!("❌ Cannot install: Failed to connect to database");
        CursorError::DatabaseCorrupt(e.to_string())
    })?;

    let items: Vec<ItemTableRow> =
        diesel::sql_query("SELECT key, value FROM ItemTable WHERE key = ?")
            .bind::<diesel::sql_types::Text, _>(key)
            .load(&mut *conn)?;

    if let Some(item) = items.first() {
        println!("Found item for key: {}", item.key);
        Ok(Some(item.value.clone()))
    } else {
        println!("No item found with the specified key");
        Ok(None)
    }
}

fn get_key() -> String {
    "src.vs.platform.reactivestorage.browser.reactiveStorageServiceImpl.persistentStorage.applicationUser".to_string()
}

pub fn install_cursor(app_name: &str, binary_path: &str) -> Result<(), CursorError> {
    // First check if already installed
    if is_cursor_installed(app_name).is_ok() {
        return Ok(());
    }

    kill_process_by_name("Cursor");

    let db_path = get_cursor_db_path()?;

    // Check if database exists and create backup
    if Path::new(&db_path).exists() {
        println!("ℹ️ Creating backup of Cursor database...");
        match install_paths::backup_file(&db_path) {
            Ok(backup_path) => println!("✅ Backup created at: {}", backup_path),
            Err(e) => println!("⚠️ Failed to create backup: {}", e),
        }
    } else {
        println!("❌ Cannot install: Cursor database not found");
        return Err(CursorError::DatabaseNotFound(db_path));
    }

    // Set up connection manager and pool
    let manager = ConnectionManager::<SqliteConnection>::new(&db_path);
    let pool = Pool::builder().build(manager).map_err(|e| {
        println!("❌ Cannot install: Cursor database is corrupt");
        CursorError::DatabaseCorrupt(e.to_string())
    })?;

    // Get a connection from the pool
    let mut conn = pool.get().map_err(|e| {
        println!("❌ Cannot install: Failed to connect to database");
        CursorError::DatabaseCorrupt(e.to_string())
    })?;

    // Check if ItemTable exists
    let tables: Vec<TableName> = diesel::sql_query(
        "SELECT name FROM sqlite_master WHERE type='table' AND name='ItemTable' ORDER BY name;",
    )
    .load(&mut *conn)?;

    if tables.is_empty() {
        println!("❌ Cannot install: ItemTable not found in database");
        return Err(CursorError::TableNotFound("ItemTable".to_string()));
    }

    tables.iter().for_each(|table| {
        println!("Table: {}", table.name);
    });

    let key = get_key();
    let existing_value = get_item_value(&key)?;

    // Start with existing config or create new one
    let mut config: serde_json::Value = if let Some(value) = existing_value {
        match serde_json::from_str(&value) {
            Ok(parsed) => parsed,
            Err(e) => {
                println!("❌ Cannot install: Existing configuration is corrupt");
                return Err(CursorError::InvalidJson(e.to_string()));
            }
        }
    } else {
        println!("ℹ️ Creating new configuration");
        serde_json::json!({})
    };

    // Get or create mcpServers array
    let servers = if let Some(existing_servers) = config.get_mut("mcpServers") {
        if !existing_servers.is_array() {
            println!("ℹ️ Replacing invalid mcpServers with array");
            config["mcpServers"] = serde_json::json!([]);
            config["mcpServers"].as_array_mut().unwrap()
        } else {
            existing_servers.as_array_mut().unwrap()
        }
    } else {
        println!("ℹ️ Creating mcpServers array");
        config["mcpServers"] = serde_json::json!([]);
        config["mcpServers"].as_array_mut().unwrap()
    };

    // Add mcp-dockmaster if not present
    if !servers.iter().any(|server| {
        server
            .get("name")
            .and_then(|name| name.as_str())
            .map(|name| name == app_name)
            .unwrap_or(false)
    }) {
        println!("ℹ️ Adding mcp-dockmaster to configuration");
        servers.push(serde_json::json!({
            "command": binary_path,
            "identifier": "769f94c1-0076-47df-a0aa-6a394ab263cd",
            "name": app_name,
            "type": "stdio"
        }));
    }

    // Update the value in the database
    match diesel::sql_query("INSERT OR REPLACE INTO ItemTable (key, value) VALUES (?, ?)")
        .bind::<diesel::sql_types::Text, _>(key)
        .bind::<diesel::sql_types::Text, _>(serde_json::to_string(&config)?)
        .execute(&mut *conn)
    {
        Ok(_) => {
            // Verify the installation was successful
            is_cursor_installed(app_name)
        }
        Err(e) => {
            println!("❌ Failed to update database configuration");
            Err(CursorError::DatabaseCorrupt(e.to_string()))
        }
    }
}

pub fn get_cursor_config(app_name: &str, binary_path: &str) -> Result<String, CursorError> {
    Ok(format!(
        "
Cursor Settings -> MCP Servers -> Add new MCP server

```
name:         {app_name}
type:         command
command_path: {binary_path}
```
    "
    ))
}

// Example usage
pub fn is_cursor_installed(app_name: &str) -> Result<(), CursorError> {
    let key = get_key();
    let v = get_item_value(&key)?;

    if v.is_none() {
        println!("❌ {} not installed in CURSOR", app_name);
        println!("src.vs.platform.reactivestorage.browser.reactiveStorageServiceImpl.persistentStorage.applicationUser not found");
        return Err(CursorError::KeyNotFound(key));
    }
    let value = v.unwrap();

    // Parse the JSON string
    let parsed: serde_json::Value = serde_json::from_str(&value)?;

    // Extract the mcpServers array
    let servers = parsed.get("mcpServers");
    if servers.is_none() {
        println!("❌ {} not installed in CURSOR", app_name);
        println!("mcpServers not found in the configuration");
        return Err(CursorError::NoMcpServers);
    }

    let servers = servers.unwrap();

    // Check if mcp-dockmaster exists in the servers array
    let is_dockmaster_installed = servers
        .as_array()
        .map(|arr| {
            arr.iter().any(|server| {
                server
                    .get("name")
                    .and_then(|name| name.as_str())
                    .map(|name| name == app_name)
                    .unwrap_or(false)
            })
        })
        .unwrap_or(false);

    if is_dockmaster_installed {
        println!("✅ {} installed in CURSOR", app_name);
        Ok(())
    } else {
        println!("❌ {} not installed in CURSOR", app_name);
        println!("mcp-dockmaster missing from list");
        Err(CursorError::NoDockmaster)
    }
}
