use reqwest::Client;
use log::info;
use regex::Regex;
use std::collections::HashSet;

/// GitHub repository information
pub struct GitHubRepo {
    pub owner: String,
    pub repo: String,
}

/// Parse a GitHub URL to extract owner and repository name
pub fn parse_github_url(url: &str) -> Result<GitHubRepo, String> {
    // Handle different GitHub URL formats
    let url = url.trim_end_matches('/');
    
    // Extract owner and repo from URL
    if let Some(github_path) = url.strip_prefix("https://github.com/") {
        let parts: Vec<&str> = github_path.split('/').collect();
        if parts.len() >= 2 {
            return Ok(GitHubRepo {
                owner: parts[0].to_string(),
                repo: parts[1].to_string(),
            });
        }
    }
    
    Err(format!("Invalid GitHub URL: {}", url))
}

/// Fetch a file from a GitHub repository
pub async fn fetch_github_file(client: &Client, owner: &str, repo: &str, path: &str) -> Result<String, String> {
    let url = format!(
        "https://raw.githubusercontent.com/{}/{}/main/{}",
        owner, repo, path
    );
    
    info!("Fetching file from GitHub: {}", url);
    
    match client.get(&url).send().await {
        Ok(response) => {
            if response.status().is_success() {
                match response.text().await {
                    Ok(content) => Ok(content),
                    Err(e) => Err(format!("Failed to read response content: {}", e)),
                }
            } else {
                // Try with master branch if main fails
                let master_url = format!(
                    "https://raw.githubusercontent.com/{}/{}/master/{}",
                    owner, repo, path
                );
                
                match client.get(&master_url).send().await {
                    Ok(master_response) => {
                        if master_response.status().is_success() {
                            match master_response.text().await {
                                Ok(content) => Ok(content),
                                Err(e) => Err(format!("Failed to read response content: {}", e)),
                            }
                        } else {
                            Err(format!("Failed to fetch file: HTTP {}", master_response.status()))
                        }
                    }
                    Err(e) => Err(format!("Failed to fetch file: {}", e)),
                }
            }
        }
        Err(e) => Err(format!("Failed to fetch file: {}", e)),
    }
}

/// Extract environment variables from README.md content
pub fn extract_env_vars_from_readme(readme_content: &str) -> HashSet<String> {
    let mut env_vars = HashSet::new();
    
    // Common patterns for environment variables in READMEs
    let patterns = vec![
        // Match export statements: export VAR_NAME="value"
        r"export\s+([A-Z][A-Z0-9_]+)=",
        // Match env vars in code blocks or configuration examples
        r"['\"]?([A-Z][A-Z0-9_]+)['\"]?\s*:",
        // Match env vars in environment sections
        r"['\"']([A-Z][A-Z0-9_]+)['\"']\s*:",
        // Match env vars in configuration sections
        r"['\"]?env['\"]?\s*:\s*\{\s*['\"]?([A-Z][A-Z0-9_]+)['\"]?\s*:",
        // Match env vars in JSON/YAML examples
        r"['\"]?([A-Z][A-Z0-9_]+)['\"]?\s*:\s*['\"].*?['\"]",
        // Match env vars in markdown code blocks
        r"`([A-Z][A-Z0-9_]+)`",
        // Match env vars in inline code
        r"\$([A-Z][A-Z0-9_]+)",
        // Match env vars in angle brackets
        r"<([A-Z][A-Z0-9_]+)>",
    ];
    
    // Apply each pattern to the README content
    for pattern in patterns {
        if let Ok(regex) = Regex::new(pattern) {
            for cap in regex.captures_iter(readme_content) {
                if cap.len() > 1 {
                    if let Some(var_name) = cap.get(1) {
                        let name = var_name.as_str().to_string();
                        // Filter out common false positives
                        if !name.contains("HTTP") && !name.contains("JSON") && 
                           !name.contains("API") && !name.contains("URL") &&
                           !name.contains("HTML") && !name.contains("CSS") &&
                           !name.contains("README") && !name.contains("TODO") {
                            env_vars.insert(name);
                        }
                    }
                }
            }
        }
    }
    
    // Additional specific patterns for common env vars in MCP servers
    let specific_patterns = vec![
        r"API[_\s]+[Kk]ey",
        r"[Tt]oken",
        r"[Ss]ecret",
        r"[Cc]redential",
        r"[Aa]uth",
        r"[Pp]assword",
        r"[Kk]ey",
        r"[Bb]undle",
    ];
    
    for pattern in specific_patterns {
        if let Ok(regex) = Regex::new(&format!(r"(?i)([A-Z][A-Z0-9_]+)(?:[_\s]+)?{}", pattern)) {
            for cap in regex.captures_iter(readme_content) {
                if cap.len() > 1 {
                    if let Some(var_name) = cap.get(1) {
                        env_vars.insert(var_name.as_str().to_string());
                    }
                }
            }
        }
    }
    
    env_vars
}

#[cfg(test)]
mod tests {
    use super::*;

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
