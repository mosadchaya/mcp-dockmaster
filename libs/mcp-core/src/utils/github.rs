use log::info;
use regex::Regex;
use reqwest::Client;
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

    Err(format!("Invalid GitHub URL: {url}"))
}

/// Fetch a file from a GitHub repository
pub async fn fetch_github_file(
    client: &Client,
    owner: &str,
    repo: &str,
    path: &str,
) -> Result<String, String> {
    let url = format!("https://raw.githubusercontent.com/{owner}/{repo}/main/{path}");

    info!("Fetching file from GitHub: {url}");

    match client.get(&url).send().await {
        Ok(response) => {
            if response.status().is_success() {
                match response.text().await {
                    Ok(content) => Ok(content),
                    Err(e) => Err(format!("Failed to read response content: {e}")),
                }
            } else {
                // Try with master branch if main fails
                let master_url =
                    format!("https://raw.githubusercontent.com/{owner}/{repo}/master/{path}");

                match client.get(&master_url).send().await {
                    Ok(master_response) => {
                        if master_response.status().is_success() {
                            match master_response.text().await {
                                Ok(content) => Ok(content),
                                Err(e) => Err(format!("Failed to read response content: {e}")),
                            }
                        } else {
                            Err(format!(
                                "Failed to fetch file: HTTP {}",
                                master_response.status()
                            ))
                        }
                    }
                    Err(e) => Err(format!("Failed to fetch file: {e}")),
                }
            }
        }
        Err(e) => Err(format!("Failed to fetch file: {e}")),
    }
}

/// Extract environment variables from README.md content
pub fn extract_env_vars_from_readme(readme_content: &str) -> HashSet<String> {
    let mut env_vars = HashSet::new();

    // Common patterns for environment variables in READMEs
    let patterns = vec![
        // Match export statements: export VAR_NAME="value"
        r"export\s+([A-Z][A-Z0-9_]+)=",
        // Match env vars in JSON configuration
        r#""([A-Z][A-Z0-9_]+)":\s*"[^"]*""#,
        // Match env vars in environment sections with clear markers
        r#"env.*?["']([A-Z][A-Z0-9_]+)["']"#,
        // Match env vars in markdown code blocks
        r"`([A-Z][A-Z0-9_]+)`",
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
                        if !name.contains("HTTP")
                            && !name.contains("JSON")
                            && !name.contains("API")
                            && !name.contains("URL")
                            && !name.contains("HTML")
                            && !name.contains("CSS")
                            && !name.contains("README")
                            && !name.contains("TODO")
                        {
                            env_vars.insert(name);
                        }
                    }
                }
            }
        }
    }

    // Additional specific patterns for common env vars in MCP servers
    let specific_patterns = vec![
        "API_KEY",
        "TOKEN",
        "SECRET",
        "CREDENTIAL",
        "AUTH",
        "PASSWORD",
        "KEY",
        "BUNDLE",
        "ACCESS",
        "APIKEY",
    ];

    // Look for common environment variable names directly
    for pattern in specific_patterns {
        for line in readme_content.lines() {
            if line.contains(pattern) {
                // Extract words that look like environment variables
                for word in line.split_whitespace() {
                    let word = word.trim_matches(|c: char| !c.is_alphanumeric() && c != '_');
                    if word
                        .chars()
                        .all(|c| c.is_ascii_uppercase() || c.is_ascii_digit() || c == '_')
                        && word.len() > 2
                        && word.chars().next().is_some_and(|c| c.is_ascii_uppercase())
                    {
                        env_vars.insert(word.to_string());
                    }
                }
            }
        }
    }

    env_vars
}

/// Analyze the context around an environment variable to determine description and required status
pub fn analyze_env_var_context(var_name: &str, readme_content: &str) -> (String, bool) {
    // Patterns that indicate a variable is required
    let required_indicators = [
        "required", "must", "need", "mandatory", "essential", "necessary"
    ];
    
    // Patterns that indicate a variable is optional
    let optional_indicators = [
        "optional", "default", "fallback", "if not set", "leave empty"
    ];
    
    // Common descriptions for known environment variable patterns
    let (mut default_description, mut required) = match var_name {
        name if name.contains("API_KEY") || name.contains("APIKEY") => {
            ("API authentication key".to_string(), true)
        },
        name if name.contains("TOKEN") => {
            ("Authentication token".to_string(), true)
        },
        name if name.contains("SECRET") => {
            ("Secret key for authentication or encryption".to_string(), true)
        },
        name if name.contains("URL") && name.contains("API") => {
            ("API endpoint URL".to_string(), false)
        },
        name if name.contains("PORT") => {
            ("Server port number".to_string(), false)
        },
        name if name.contains("HOST") => {
            ("Server hostname or IP address".to_string(), false)
        },
        name if name.contains("DB") || name.contains("DATABASE") => {
            ("Database connection string or URL".to_string(), true)
        },
        name if name.contains("PATH") => {
            ("File or directory path".to_string(), false)
        },
        name if name.contains("DEBUG") => {
            ("Enable debug mode (true/false)".to_string(), false)
        },
        name if name.contains("LOG") => {
            ("Logging configuration".to_string(), false)
        },
        _ => {
            (format!("Environment variable for {}", var_name.to_lowercase().replace('_', " ")), false)
        }
    };
    
    // Look for context in the README content
    for line in readme_content.lines() {
        let line_lower = line.to_lowercase();
        if line_lower.contains(&var_name.to_lowercase()) {
            // Check for required indicators
            for indicator in &required_indicators {
                if line_lower.contains(indicator) {
                    required = true;
                    break;
                }
            }
            
            // Check for optional indicators (overrides required if found)
            for indicator in &optional_indicators {
                if line_lower.contains(indicator) {
                    required = false;
                    break;
                }
            }
            
            // Try to extract a better description from the line
            if line.len() > var_name.len() + 10 {
                // Clean up the line and try to extract meaningful description
                let clean_line = line
                    .chars()
                    .filter(|&c| c != '`' && c != '*' && c != '#')
                    .collect::<String>()
                    .trim()
                    .to_string();
                    
                if clean_line.len() > var_name.len() + 20 && clean_line.len() < 200 {
                    // Find the part after the variable name
                    if let Some(pos) = clean_line.find(var_name) {
                        let after_var = &clean_line[pos + var_name.len()..];
                        if let Some(colon_pos) = after_var.find(':') {
                            let potential_desc = after_var[colon_pos + 1..].trim();
                            if potential_desc.len() > 5 && potential_desc.len() < 150 {
                                default_description = potential_desc.to_string();
                            }
                        } else if let Some(dash_pos) = after_var.find('-') {
                            let potential_desc = after_var[dash_pos + 1..].trim();
                            if potential_desc.len() > 5 && potential_desc.len() < 150 {
                                default_description = potential_desc.to_string();
                            }
                        }
                    }
                }
            }
        }
    }
    
    (default_description, required)
}
