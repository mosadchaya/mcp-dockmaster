use anyhow::Result;
use std::path::PathBuf;
use mcp_core::database::migration::{export_servers_to_file, import_claude_desktop_config, ExportData};

pub fn export_servers(db_path: Option<PathBuf>, output_path: PathBuf) -> Result<()> {
    // Use default database path if not provided
    let db_path = db_path.unwrap_or_else(|| {
        let mut path = dirs::data_dir()
            .expect("Could not find data directory");
        path.push("com.mcp-dockmaster.desktop");
        path.push("mcp_dockmaster.db");
        path
    });
    
    println!("Exporting servers from: {}", db_path.display());
    println!("Output file: {}", output_path.display());
    
    export_servers_to_file(&db_path, &output_path)?;
    
    println!("Export completed successfully!");
    Ok(())
}

pub fn import_claude_config(config_path: PathBuf, output_path: PathBuf) -> Result<()> {
    println!("Importing Claude Desktop config from: {}", config_path.display());
    println!("Output file: {}", output_path.display());
    
    let servers = import_claude_desktop_config(&config_path)?;
    
    let export_data = ExportData {
        version: "1.0".to_string(),
        servers,
    };
    
    let json = serde_json::to_string_pretty(&export_data)?;
    std::fs::write(&output_path, json)?;
    
    println!("Import completed successfully!");
    println!("Found {} custom servers", export_data.servers.len());
    Ok(())
}