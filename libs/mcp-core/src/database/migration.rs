use anyhow::Result;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;

// Removed unused imports
use crate::schema::{server_env, servers};

#[derive(Debug, Serialize, Deserialize)]
pub struct ExportedServer {
    pub id: String,
    pub name: String,
    pub description: String,
    pub tools_type: String,
    pub enabled: bool,
    pub entry_point: Option<String>,
    pub command: Option<String>,
    pub args: Option<Vec<String>>,
    pub distribution_type: Option<String>,
    pub distribution_package: Option<String>,
    pub server_type: Option<String>,
    pub working_directory: Option<String>,
    pub executable_path: Option<String>,
    pub env_vars: HashMap<String, ExportedEnvVar>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExportedEnvVar {
    pub value: String,
    pub description: String,
    pub required: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExportData {
    pub version: String,
    pub servers: Vec<ExportedServer>,
}

#[derive(Queryable, Debug)]
struct ServerRecord {
    pub id: String,
    pub name: String,
    pub description: String,
    pub tools_type: String,
    pub enabled: bool,
    pub entry_point: Option<String>,
    pub command: Option<String>,
    pub args: Option<String>,
    pub distribution_type: Option<String>,
    pub distribution_package: Option<String>,
    pub server_type: Option<String>,
    pub working_directory: Option<String>,
    pub executable_path: Option<String>,
}

#[derive(Queryable, Debug)]
struct ServerEnvRecord {
    pub server_id: String,
    pub env_key: String,
    pub env_value: String,
    pub env_description: String,
    pub env_required: bool,
}

pub fn export_servers_to_json(db_path: &Path) -> Result<String> {
    use diesel::sqlite::SqliteConnection;
    
    // Connect to the database
    let database_url = format!("sqlite://{}", db_path.display());
    let mut connection = SqliteConnection::establish(&database_url)?;
    
    // Query all servers
    let server_records = servers::table
        .select((
            servers::id,
            servers::name,
            servers::description,
            servers::tools_type,
            servers::enabled,
            servers::entry_point,
            servers::command,
            servers::args,
            servers::distribution_type,
            servers::distribution_package,
            servers::server_type,
            servers::working_directory,
            servers::executable_path,
        ))
        .load::<ServerRecord>(&mut connection)?;
    
    // Query all environment variables
    let env_records = server_env::table
        .select((
            server_env::server_id,
            server_env::env_key,
            server_env::env_value,
            server_env::env_description,
            server_env::env_required,
        ))
        .load::<ServerEnvRecord>(&mut connection)?;
    
    // Group env vars by server_id
    let mut env_by_server: HashMap<String, HashMap<String, ExportedEnvVar>> = HashMap::new();
    for env in env_records {
        env_by_server
            .entry(env.server_id.clone())
            .or_default()
            .insert(
                env.env_key,
                ExportedEnvVar {
                    value: env.env_value,
                    description: env.env_description,
                    required: env.env_required,
                },
            );
    }
    
    // Convert to export format
    let mut exported_servers = Vec::new();
    for server in server_records {
        let args = server.args.as_ref().map(|args_str| {
            serde_json::from_str::<Vec<String>>(args_str).unwrap_or_else(|_| vec![args_str.clone()])
        });
        
        exported_servers.push(ExportedServer {
            id: server.id.clone(),
            name: server.name,
            description: server.description,
            tools_type: server.tools_type,
            enabled: server.enabled,
            entry_point: server.entry_point,
            command: server.command,
            args,
            distribution_type: server.distribution_type,
            distribution_package: server.distribution_package,
            server_type: server.server_type,
            working_directory: server.working_directory,
            executable_path: server.executable_path,
            env_vars: env_by_server.remove(&server.id).unwrap_or_default(),
        });
    }
    
    let export_data = ExportData {
        version: "1.0".to_string(),
        servers: exported_servers,
    };
    
    // Serialize to JSON
    serde_json::to_string_pretty(&export_data).map_err(Into::into)
}

pub fn export_servers_to_file(db_path: &Path, output_path: &Path) -> Result<()> {
    let json = export_servers_to_json(db_path)?;
    std::fs::write(output_path, json)?;
    Ok(())
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ClaudeDesktopConfig {
    #[serde(rename = "mcpServers")]
    pub mcp_servers: HashMap<String, ClaudeServerConfig>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ClaudeServerConfig {
    pub command: String,
    #[serde(default)]
    pub args: Vec<String>,
    #[serde(default)]
    pub env: HashMap<String, String>,
}

pub fn import_claude_desktop_config(config_path: &Path) -> Result<Vec<ExportedServer>> {
    let config_str = std::fs::read_to_string(config_path)?;
    let config: ClaudeDesktopConfig = serde_json::from_str(&config_str)?;
    
    let mut servers = Vec::new();
    
    for (server_name, server_config) in config.mcp_servers {
        // Skip the mcp-dockmaster proxy server itself
        if server_name == "mcp-dockmaster" {
            continue;
        }
        
        // Determine server type based on configuration patterns
        let (server_type, tools_type, working_directory, executable_path) = analyze_server_config(&server_config);
        
        // Convert env vars to our format
        let mut env_vars = HashMap::new();
        for (key, value) in server_config.env {
            env_vars.insert(key, ExportedEnvVar {
                value,
                description: "Imported from Claude Desktop config".to_string(),
                required: false,
            });
        }
        
        servers.push(ExportedServer {
            id: format!("imported/{}", server_name),
            name: server_name.clone(),
            description: format!("Imported from Claude Desktop: {}", server_name),
            tools_type,
            enabled: true,
            entry_point: None,
            command: Some(server_config.command),
            args: if server_config.args.is_empty() { None } else { Some(server_config.args) },
            distribution_type: None,
            distribution_package: None,
            server_type: Some(server_type),
            working_directory,
            executable_path,
            env_vars,
        });
    }
    
    Ok(servers)
}

fn analyze_server_config(config: &ClaudeServerConfig) -> (String, String, Option<String>, Option<String>) {
    let command = &config.command;
    let args = &config.args;
    
    // Determine if this is a local server based on patterns
    if command == "node" && args.len() == 1 && args[0].starts_with('/') {
        // Local Node.js server (like clanki)
        return (
            "local".to_string(),
            "node".to_string(),
            None,
            Some(args[0].clone()),
        );
    }
    
    if command == "uv" && args.contains(&"run".to_string()) && args.contains(&"--directory".to_string()) {
        // Local Python project with uv (like mcp-google-sheets-local)
        let dir_index = args.iter().position(|x| x == "--directory").unwrap_or(0);
        let working_dir = if dir_index + 1 < args.len() {
            Some(args[dir_index + 1].clone())
        } else {
            None
        };
        
        return (
            "local".to_string(),
            "python".to_string(),
            working_dir,
            None,
        );
    }
    
    // Check if command is an absolute path
    if command.starts_with('/') {
        return (
            "local".to_string(),
            "custom".to_string(),
            None,
            Some(command.clone()),
        );
    }
    
    // Default to custom type for other patterns
    (
        "custom".to_string(),
        "custom".to_string(),
        None,
        None,
    )
}