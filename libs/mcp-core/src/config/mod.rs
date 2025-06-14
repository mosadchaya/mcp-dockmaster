use std::env;

/// Configuration for MCP tool names with namespace support
#[derive(Debug, Clone)]
pub struct ToolConfig {
    pub prefix: String,
    pub namespace_enabled: bool,
}

impl ToolConfig {
    /// Create ToolConfig from environment variables
    pub fn from_env() -> Self {
        let namespace_enabled = env::var("DOCKMASTER_NAMESPACE_MODE")
            .unwrap_or_else(|_| "enabled".to_string())
            .to_lowercase() != "disabled";

        let prefix = if namespace_enabled {
            env::var("DOCKMASTER_TOOL_PREFIX")
                .unwrap_or_else(|_| "dockmaster_".to_string())
        } else {
            String::new()
        };

        Self {
            prefix,
            namespace_enabled,
        }
    }

    /// Generate tool name with optional namespace prefix
    pub fn tool_name(&self, base_name: &str) -> String {
        if self.namespace_enabled {
            format!("{}{}", self.prefix, base_name)
        } else {
            base_name.to_string()
        }
    }

    /// Check if namespace is enabled
    pub fn is_namespace_enabled(&self) -> bool {
        self.namespace_enabled
    }
}

impl Default for ToolConfig {
    fn default() -> Self {
        Self {
            prefix: "dockmaster_".to_string(),
            namespace_enabled: true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn test_default_config() {
        let config = ToolConfig::default();
        assert_eq!(config.tool_name("register_server"), "dockmaster_register_server");
        assert!(config.is_namespace_enabled());
    }

    #[test]
    fn test_custom_prefix() {
        env::set_var("DOCKMASTER_TOOL_PREFIX", "dm_");
        let config = ToolConfig::from_env();
        assert_eq!(config.tool_name("register_server"), "dm_register_server");
        env::remove_var("DOCKMASTER_TOOL_PREFIX");
    }

    #[test]
    fn test_disabled_namespace() {
        env::set_var("DOCKMASTER_NAMESPACE_MODE", "disabled");
        let config = ToolConfig::from_env();
        assert_eq!(config.tool_name("register_server"), "register_server");
        assert!(!config.is_namespace_enabled());
        env::remove_var("DOCKMASTER_NAMESPACE_MODE");
    }
}