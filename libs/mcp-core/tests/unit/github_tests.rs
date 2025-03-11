#[cfg(test)]
mod tests {
    use mcp_core::utils::github::{extract_env_vars_from_readme, parse_github_url};

    #[test]
    fn test_parse_github_url() {
        // Test valid URL
        let url = "https://github.com/dcSpark/mcp-dockmaster";
        let result = parse_github_url(url);
        assert!(result.is_ok());
        let repo = result.unwrap();
        assert_eq!(repo.owner, "dcSpark");
        assert_eq!(repo.repo, "mcp-dockmaster");

        // Test URL with trailing slash
        let url = "https://github.com/dcSpark/mcp-dockmaster/";
        let result = parse_github_url(url);
        assert!(result.is_ok());
        let repo = result.unwrap();
        assert_eq!(repo.owner, "dcSpark");
        assert_eq!(repo.repo, "mcp-dockmaster");

        // Test invalid URL
        let url = "https://example.com/dcSpark/mcp-dockmaster";
        let result = parse_github_url(url);
        assert!(result.is_err());
    }

    #[test]
    fn test_extract_env_vars_from_readme() {
        let readme = r#"
        # Test README
        
        ## Configuration
        
        You need to set the following environment variables:
        
        ```bash
        export API_KEY="your_api_key_here"
        export USER_TOKEN="your_token"
        ```
        
        In your configuration file:
        
        ```json
        {
          "env": {
            "API_KEY": "your_api_key",
            "USER_TOKEN": "your_token",
            "BUNDLE_NAME": "your_bundle"
          }
        }
        ```
        "#;

        let env_vars = extract_env_vars_from_readme(readme);
        assert!(env_vars.contains("API_KEY"));
        assert!(env_vars.contains("USER_TOKEN"));
        assert!(env_vars.contains("BUNDLE_NAME"));
    }

    #[test]
    fn test_extract_env_vars_from_replicate_readme() {
        let readme = r#"# Replicate MCP Server

A [Model Context Protocol](https://github.com/mcp-sdk/mcp) server implementation for Replicate. Run Replicate models through a simple tool-based interface.

## Quickstart

1. Install the server:

```bash
npm install -g mcp-replicate
```

2. Get your Replicate API token:
   - Go to [Replicate API tokens page](https://replicate.com/account/api-tokens)
   - Create a new token if you don't have one
   - Copy the token for the next step

3. Configure Claude Desktop:
   - Open Claude Desktop Settings
   - Select the "Developer" section in the sidebar
   - Click "Edit Config" to open the configuration file
   - Add the following configuration, replacing `your_token_here` with your actual Replicate API token:

```json
{
  "mcpServers": {
    "replicate": {
      "command": "mcp-replicate",
      "env": {
        "REPLICATE_API_TOKEN": "your_token_here"
      }
    }
  }
}
```

### 2. As Environment Variable

Alternatively, you can set it as an environment variable if you're using another MCP client:

```bash
export REPLICATE_API_TOKEN=your_token_here
```"#;

        let env_vars = extract_env_vars_from_readme(readme);
        assert!(
            env_vars.contains("REPLICATE_API_TOKEN"),
            "Should extract REPLICATE_API_TOKEN"
        );
        assert_eq!(
            env_vars.len(),
            1,
            "Should only extract one environment variable"
        );
    }

    #[test]
    fn test_extract_env_vars_from_ollama_readme() {
        let readme = r#"# MCP Ollama

A Model Context Protocol (MCP) server for integrating Ollama with Claude Desktop or other MCP clients.

### Configure Claude Desktop

Add to your Claude Desktop configuration (`~/Library/Application Support/Claude/claude_desktop_config.json` on macOS, `%APPDATA%\Claude\claude_desktop_config.json` on Windows):

```json
{
  "mcpServers": {
    "ollama": {
      "command": "uvx",
      "args": [
        "mcp-ollama"
      ]
    }
  }
}
```"#;

        let env_vars = extract_env_vars_from_readme(readme);
        assert!(
            env_vars.is_empty(),
            "Should not extract any environment variables"
        );
    }
}
