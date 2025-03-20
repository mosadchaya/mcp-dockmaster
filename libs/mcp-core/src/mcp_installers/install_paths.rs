use std::env;
use std::fs;
use std::io;
use std::path::Path;

use super::install_errors::ClaudeError;

pub fn get_claude_config_path() -> Result<String, ClaudeError> {
    match env::consts::OS {
        "windows" => {
            let appdata = env::var("APPDATA").map_err(|_| {
                ClaudeError::IoError(std::io::Error::new(
                    std::io::ErrorKind::NotFound,
                    "APPDATA not found",
                ))
            })?;
            Ok(format!("{}\\Claude\\claude_desktop_config.json", appdata))
        }
        "macos" => {
            let home = env::var("HOME").map_err(|_| {
                ClaudeError::IoError(std::io::Error::new(
                    std::io::ErrorKind::NotFound,
                    "HOME not found",
                ))
            })?;
            Ok(format!(
                "{}/Library/Application Support/Claude/claude_desktop_config.json",
                home
            ))
        }
        os => Err(ClaudeError::UnsupportedOS(os.to_string())),
    }
}

pub fn backup_file(file_path: &str) -> Result<String, io::Error> {
    let path = Path::new(file_path);
    if !path.exists() {
        return Ok(file_path.to_string());
    }
    let timestamp = std::time::UNIX_EPOCH.elapsed().unwrap().as_millis();
    let backup_path = format!("{}.backup_{}", file_path, timestamp);

    fs::copy(file_path, &backup_path)?;
    Ok(backup_path)
}

pub fn get_generic_config(binary_path: &str) -> String {
    format!(
        "
For any MCP client, configure with:

```
Server Name: mcp-dockmaster
Command: {}
Arguments: []
```
        ",
        binary_path
    )
}
