use log::{error, info};

use crate::utils::command::CommandWrappedInShellBuilder;

use super::mcp_core::MCPCore;

pub trait McpCoreRuntimesExt {
    fn is_nodejs_installed() -> impl std::future::Future<Output = Result<bool, String>> + Send;
    fn is_uv_installed() -> impl std::future::Future<Output = Result<bool, String>> + Send;
    fn is_docker_installed() -> impl std::future::Future<Output = Result<bool, String>> + Send;
}

impl McpCoreRuntimesExt for MCPCore {
    async fn is_nodejs_installed() -> Result<bool, String> {
        info!("Checking if Node.js is installed");
        let mut command_builder = CommandWrappedInShellBuilder::new("echo $PATH && node");
        command_builder.arg("-v");
        let mut command = command_builder.build();
        info!("Executing command: node -v");
        let output = command.output().await;
        match output {
            Ok(output) => {
                let success = output.status.success();
                if success {
                    info!("Node.js is installed");
                } else {
                    info!(
                        "Node.js is not installed {} - {}",
                        String::from_utf8_lossy(&output.stdout),
                        String::from_utf8_lossy(&output.stderr)
                    );
                }
                Ok(success)
            }
            Err(e) => {
                error!("Failed to check Node.js installation: {}", e);
                Err(e.to_string())
            }
        }
    }
    async fn is_uv_installed() -> Result<bool, String> {
        let mut command_builder = CommandWrappedInShellBuilder::new("uv");
        command_builder.arg("version");
        let mut command = command_builder.build();
        let output = command.output().await;
        match output {
            Ok(output) => Ok(output.status.success()),
            Err(e) => Err(e.to_string()),
        }
    }
    async fn is_docker_installed() -> Result<bool, String> {
        let mut command_builder = CommandWrappedInShellBuilder::new("docker");
        command_builder.arg("-v");
        let mut command = command_builder.build();
        let output = command.output().await;
        match output {
            Ok(output) => Ok(output.status.success()),
            Err(e) => Err(e.to_string()),
        }
    }
}
