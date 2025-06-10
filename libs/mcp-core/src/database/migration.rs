use anyhow::Result;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;

use crate::models::types::{ServerConfiguration, ServerEnvironment};
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
            .or_insert_with(HashMap::new)
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