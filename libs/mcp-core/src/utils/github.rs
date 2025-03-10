use reqwest::Client;
use log::info;

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
}
