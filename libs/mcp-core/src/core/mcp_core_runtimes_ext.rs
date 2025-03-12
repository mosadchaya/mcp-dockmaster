use crate::utils::command::CommandWrappedInShellBuilder;

use super::mcp_core::MCPCore;

pub trait McpCoreRuntimesExt {
    fn is_nodejs_installed() -> impl std::future::Future<Output = Result<bool, String>> + Send;
    fn is_uv_installed() -> impl std::future::Future<Output = Result<bool, String>> + Send;
    fn is_docker_installed() -> impl std::future::Future<Output = Result<bool, String>> + Send;
}

impl McpCoreRuntimesExt for MCPCore {
    async fn is_nodejs_installed() -> Result<bool, String> {
        let mut command_builder = CommandWrappedInShellBuilder::new("node");
        command_builder.arg("-v");
        let mut command = command_builder.build();
        let output = command.output().await;
        match output {
            Ok(output) => Ok(output.status.success()),
            Err(e) => Err(e.to_string()),
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
