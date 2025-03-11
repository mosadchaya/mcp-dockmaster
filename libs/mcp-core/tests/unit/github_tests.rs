#[cfg(test)]
mod tests {
    use mcp_core::utils::github::{parse_github_url, extract_env_vars_from_readme, GitHubRepo};

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
}
