use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use diesel::sqlite::SqliteConnection;
use serde_json;
use std::env;
use std::path::Path;

type Pool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

#[derive(QueryableByName)]
struct TableName {
    #[diesel(sql_type = diesel::sql_types::Text)]
    name: String,
}

#[derive(QueryableByName, Debug)]
struct ItemTableRow {
    #[diesel(sql_type = diesel::sql_types::Text)]
    key: String,
    #[diesel(sql_type = diesel::sql_types::Text)]
    value: String,
}

fn get_cursor_db_path() -> Result<String, Box<dyn std::error::Error>> {
    match env::consts::OS {
        "windows" => {
            let appdata = env::var("APPDATA")?;
            Ok(format!(
                "{}\\Cursor\\User\\globalStorage\\state.vscdb",
                appdata
            ))
        }
        "macos" => {
            let home = env::var("HOME")?;
            Ok(format!(
                "{}/Library/Application Support/Cursor/User/globalStorage/state.vscdb",
                home
            ))
        }
        os => Err(format!("Unsupported operating system: {}", os).into()),
    }
}

fn open_and_find_tables() -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let db_path = get_cursor_db_path()?;

    // Ensure the database file exists
    if !Path::new(&db_path).exists() {
        return Err("Database file does not exist".into());
    }

    // Set up connection manager and pool
    let manager = ConnectionManager::<SqliteConnection>::new(db_path);
    let pool = Pool::builder()
        .build(manager)
        .expect("Failed to create pool");

    // Get a connection from the pool
    let mut conn = pool.get()?;

    // Query to get all table names
    let tables: Vec<TableName> = diesel::sql_query(
        "
        SELECT name FROM sqlite_master 
        WHERE type='table' 
        ORDER BY name;
    ",
    )
    .load(&mut *conn)?;

    let table_names: Vec<String> = tables.into_iter().map(|t| t.name).collect();

    println!("Found tables:");
    for table in &table_names {
        println!("- {}", table);
    }

    Ok(table_names)
}

pub fn get_item_value(key: &str) -> Result<Option<String>, Box<dyn std::error::Error>> {
    let db_path = get_cursor_db_path()?;

    if !Path::new(&db_path).exists() {
        return Err("Database file does not exist".into());
    }

    let manager = ConnectionManager::<SqliteConnection>::new(db_path);
    let pool = Pool::builder()
        .build(manager)
        .expect("Failed to create pool");

    let mut conn = pool.get()?;

    let items: Vec<ItemTableRow> =
        diesel::sql_query("SELECT key, value FROM ItemTable WHERE key = ?")
            .bind::<diesel::sql_types::Text, _>(key)
            .load(&mut *conn)?;

    if let Some(item) = items.first() {
        println!("Found item:");
        println!("Key: {}", item.key);
        println!("Value: {}", item.value);
        Ok(Some(item.value.clone()))
    } else {
        println!("No item found with the specified key");
        Ok(None)
    }
}

// Example usage
pub fn is_cursor_installed() -> Result<bool, Box<dyn std::error::Error>> {
    let key = "src.vs.platform.reactivestorage.browser.reactiveStorageServiceImpl.persistentStorage.applicationUser";
    let v = get_item_value(key)?;

    if let Some(value) = v {
        // Parse the JSON string
        let parsed: serde_json::Value = serde_json::from_str(&value)?;

        // Extract the mcpServers array
        if let Some(servers) = parsed.get("mcpServers") {
            // Check if mcp-dockmaster exists in the servers array
            let is_dockmaster_installed = servers
                .as_array()
                .map(|arr| {
                    arr.iter().any(|server| {
                        server
                            .get("name")
                            .and_then(|name| name.as_str())
                            .map(|name| name == "mcp-dockmaster")
                            .unwrap_or(false)
                    })
                })
                .unwrap_or(false);

            if is_dockmaster_installed {
                println!("✅ MCP Dockmaster installed in CURSOR");
                return Ok(true);
            } else {
                println!("❌ MCP Dockmaster not installed in CURSOR");
                println!("mcp-dockmaster missing from list");
            }
        } else {
            println!("❌ MCP Dockmaster not installed in CURSOR");
            println!("No MCP servers found in the configuration");
        }
    } else {
        println!("❌ MCP Dockmaster not installed in CURSOR");
        println!("src.vs.platform.reactivestorage.browser.reactiveStorageServiceImpl.persistentStorage.applicationUser not found");
    }

    Ok(false)
}
