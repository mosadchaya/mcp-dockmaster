use std::path::{Path, PathBuf};
use std::collections::HashMap;
use anyhow::{Result, anyhow};

/// Validation result for server configuration
#[derive(Debug)]
pub struct ValidationResult {
    pub valid: bool,
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
}

impl ValidationResult {
    pub fn new() -> Self {
        Self {
            valid: true,
            errors: Vec::new(),
            warnings: Vec::new(),
        }
    }

    pub fn add_error(&mut self, error: String) {
        self.valid = false;
        self.errors.push(error);
    }

    pub fn add_warning(&mut self, warning: String) {
        self.warnings.push(warning);
    }
}

/// Validate a custom server configuration
pub async fn validate_custom_server(
    server_type: &str,
    runtime: &str,
    command: &Option<String>,
    executable_path: &Option<String>,
    _args: &Option<Vec<String>>,
    working_directory: &Option<String>,
    env_vars: &Option<HashMap<String, String>>,
) -> ValidationResult {
    let mut result = ValidationResult::new();

    // Validate server type
    match server_type {
        "local" | "custom" => {},
        _ => result.add_error(format!("Invalid server_type '{}'. Must be 'local' or 'custom'", server_type)),
    }

    // Validate runtime and check dependencies
    match runtime {
        "node" => {
            if let Err(e) = validate_node_runtime().await {
                result.add_error(format!("Node.js runtime validation failed: {}", e));
            }
        },
        "python" => {
            if let Err(e) = validate_python_runtime().await {
                result.add_error(format!("Python runtime validation failed: {}", e));
            }
        },
        "docker" => {
            if let Err(e) = validate_docker_runtime().await {
                result.add_error(format!("Docker runtime validation failed: {}", e));
            }
        },
        "custom" => {
            // Custom runtime - no specific validation
        },
        _ => result.add_error(format!("Invalid runtime '{}'. Must be 'node', 'python', 'docker', or 'custom'", runtime)),
    }

    // Validate paths
    if let Some(exec_path) = executable_path {
        if let Err(e) = validate_executable_path(exec_path).await {
            result.add_error(format!("Executable path validation failed: {}", e));
        }
    }

    if let Some(work_dir) = working_directory {
        if let Err(e) = validate_working_directory(work_dir).await {
            result.add_error(format!("Working directory validation failed: {}", e));
        }
    }

    // Validate environment variables
    if let Some(env) = env_vars {
        validate_environment_variables(env, &mut result).await;
    }

    // Validate configuration consistency
    validate_configuration_consistency(server_type, runtime, command, executable_path, &mut result);

    result
}

/// Validate that Node.js is installed and accessible
async fn validate_node_runtime() -> Result<()> {
    use crate::core::{mcp_core::MCPCore, mcp_core_runtimes_ext::McpCoreRuntimesExt};
    
    if MCPCore::is_nodejs_installed().await.unwrap_or(false) {
        Ok(())
    } else {
        Err(anyhow!("Node.js is not installed or not accessible in PATH"))
    }
}

/// Validate that Python is installed and accessible
async fn validate_python_runtime() -> Result<()> {
    use crate::core::{mcp_core::MCPCore, mcp_core_runtimes_ext::McpCoreRuntimesExt};
    
    // Check if uv is available (preferred for Python MCP servers)
    if MCPCore::is_uv_installed().await.unwrap_or(false) {
        return Ok(());
    }

    // Fall back to checking basic Python installation
    match tokio::process::Command::new("python3")
        .arg("--version")
        .output()
        .await
    {
        Ok(output) if output.status.success() => Ok(()),
        _ => {
            match tokio::process::Command::new("python")
                .arg("--version")
                .output()
                .await
            {
                Ok(output) if output.status.success() => Ok(()),
                _ => Err(anyhow!("Python is not installed or not accessible in PATH")),
            }
        }
    }
}

/// Validate that Docker is installed and accessible
async fn validate_docker_runtime() -> Result<()> {
    use crate::core::{mcp_core::MCPCore, mcp_core_runtimes_ext::McpCoreRuntimesExt};
    
    if MCPCore::is_docker_installed().await.unwrap_or(false) {
        Ok(())
    } else {
        Err(anyhow!("Docker is not installed or not accessible in PATH"))
    }
}

/// Validate that an executable path exists and is executable
async fn validate_executable_path(path: &str) -> Result<()> {
    let resolved_path = resolve_template_variables(path)?;
    let path_buf = Path::new(&resolved_path);

    if !path_buf.exists() {
        return Err(anyhow!("Executable path does not exist: {}", resolved_path));
    }

    if !path_buf.is_file() {
        return Err(anyhow!("Executable path is not a file: {}", resolved_path));
    }

    // Check if the file is executable (Unix-specific)
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let metadata = std::fs::metadata(&path_buf)?;
        let permissions = metadata.permissions();
        if permissions.mode() & 0o111 == 0 {
            return Err(anyhow!("File is not executable: {}", resolved_path));
        }
    }

    Ok(())
}

/// Validate that a working directory exists
async fn validate_working_directory(path: &str) -> Result<()> {
    let resolved_path = resolve_template_variables(path)?;
    let path_buf = Path::new(&resolved_path);

    if !path_buf.exists() {
        return Err(anyhow!("Working directory does not exist: {}", resolved_path));
    }

    if !path_buf.is_dir() {
        return Err(anyhow!("Working directory path is not a directory: {}", resolved_path));
    }

    Ok(())
}

/// Validate environment variables and resolve templates
async fn validate_environment_variables(env_vars: &HashMap<String, String>, result: &mut ValidationResult) {
    for (key, value) in env_vars {
        // Check for empty keys
        if key.is_empty() {
            result.add_error("Environment variable key cannot be empty".to_string());
            continue;
        }

        // Validate key format (basic check for valid environment variable names)
        if !key.chars().all(|c| c.is_alphanumeric() || c == '_') || key.chars().next().map_or(false, |c| c.is_numeric()) {
            result.add_warning(format!("Environment variable key '{}' may not be valid", key));
        }

        // Try to resolve template variables in the value
        match resolve_template_variables(value) {
            Ok(resolved) => {
                // Check if the resolved value points to a file that should exist
                if value.contains("PATH") || value.contains(".json") || value.contains(".key") {
                    let path = Path::new(&resolved);
                    if path.is_absolute() && !path.exists() {
                        result.add_warning(format!("Environment variable '{}' points to non-existent path: {}", key, resolved));
                    }
                }
            },
            Err(e) => {
                result.add_warning(format!("Could not resolve template in environment variable '{}': {}", key, e));
            }
        }
    }
}

/// Validate that the configuration is consistent
fn validate_configuration_consistency(
    server_type: &str,
    runtime: &str,
    command: &Option<String>,
    executable_path: &Option<String>,
    result: &mut ValidationResult,
) {
    match server_type {
        "local" => {
            // Local servers should have either executable_path or command
            if executable_path.is_none() && command.is_none() {
                result.add_error("Local servers must specify either executable_path or command".to_string());
            }

            // If runtime is specified, command should be consistent
            if let Some(cmd) = command {
                match runtime {
                    "node" => {
                        if cmd != "node" {
                            result.add_warning(format!("Runtime is 'node' but command is '{}' - this may be intentional", cmd));
                        }
                    },
                    "python" => {
                        if !["python", "python3", "uv"].contains(&cmd.as_str()) {
                            result.add_warning(format!("Runtime is 'python' but command is '{}' - this may be intentional", cmd));
                        }
                    },
                    _ => {}
                }
            }
        },
        "custom" => {
            // Custom servers are more flexible but should have some way to execute
            if executable_path.is_none() && command.is_none() {
                result.add_error("Custom servers must specify either executable_path or command".to_string());
            }
        },
        _ => {}
    }
}

/// Resolve template variables in a string (e.g., $HOME, $USER)
pub fn resolve_template_variables(input: &str) -> Result<String> {
    let mut result = input.to_string();

    // Common template variables
    let templates = [
        ("$HOME", std::env::var("HOME").unwrap_or_default()),
        ("$USER", std::env::var("USER").unwrap_or_default()),
        ("$PWD", std::env::current_dir().map(|p| p.to_string_lossy().to_string()).unwrap_or_default()),
    ];

    for (template, value) in templates {
        if !value.is_empty() {
            result = result.replace(template, &value);
        }
    }

    // Handle ${VAR} syntax
    while let Some(start) = result.find("${") {
        if let Some(end) = result[start..].find('}') {
            let var_name = &result[start + 2..start + end];
            if let Ok(var_value) = std::env::var(var_name) {
                let full_template = format!("${{{}}}", var_name);
                result = result.replace(&full_template, &var_value);
            } else {
                return Err(anyhow!("Environment variable '{}' not found", var_name));
            }
        } else {
            break;
        }
    }

    Ok(result)
}

/// Convert relative paths to absolute paths
pub fn resolve_relative_path(path: &str, working_dir: Option<&str>) -> Result<PathBuf> {
    let path_buf = Path::new(path);
    
    if path_buf.is_absolute() {
        Ok(path_buf.to_path_buf())
    } else {
        let base_dir = if let Some(work_dir) = working_dir {
            PathBuf::from(work_dir)
        } else {
            std::env::current_dir()?
        };
        Ok(base_dir.join(path_buf))
    }
}