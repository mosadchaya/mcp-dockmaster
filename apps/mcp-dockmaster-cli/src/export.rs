use anyhow::Result;
use std::path::PathBuf;
use mcp_core::database::migration::export_servers_to_file;

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