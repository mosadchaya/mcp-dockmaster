#[cfg(test)]
mod tests {
    use mcp_core::core::{mcp_core::MCPCore, mcp_core_runtimes_ext::McpCoreRuntimesExt};

    #[tokio::test]
    async fn test_is_nodejs_installed() {
        let result = MCPCore::is_nodejs_installed().await;
        match result {
            Ok(installed) => {
                assert!(installed);
            }
            Err(e) => panic!("error checking Node.js installation: {}", e),
        }
    }

    #[tokio::test]
    async fn test_is_uv_installed() {
        let result = MCPCore::is_uv_installed().await;
        match result {
            Ok(installed) => {
                assert!(installed);
            }
            Err(e) => panic!("error checking uv installation: {}", e),
        }
    }

    #[tokio::test]
    async fn test_is_docker_installed() {
        let result = MCPCore::is_docker_installed().await;
        match result {
            Ok(installed) => {
                assert!(installed);
            }
            Err(e) => panic!("error checking docker installation: {}", e),
        }
    }
}
