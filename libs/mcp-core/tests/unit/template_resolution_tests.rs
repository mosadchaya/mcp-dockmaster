#[cfg(test)]
mod tests {
    use mcp_core::validation::resolve_template_variables;
    use std::env;

    #[test]
    fn test_home_template_resolution() {
        let input = "$HOME/config/servers";
        let result = resolve_template_variables(input).unwrap();
        let expected = format!("{}/config/servers", env::var("HOME").unwrap());
        assert_eq!(result, expected);
    }

    #[test]
    fn test_user_template_resolution() {
        let input = "/Users/$USER/Documents";
        let result = resolve_template_variables(input).unwrap();
        let expected = format!("/Users/{}/Documents", env::var("USER").unwrap());
        assert_eq!(result, expected);
    }

    #[test]
    fn test_multiple_template_resolution() {
        let input = "$HOME/projects/$USER/config";
        let result = resolve_template_variables(input).unwrap();
        let expected = format!("{}/projects/{}/config", 
            env::var("HOME").unwrap(), 
            env::var("USER").unwrap()
        );
        assert_eq!(result, expected);
    }

    #[test]
    fn test_curly_brace_template_resolution() {
        let input = "${HOME}/servers/${USER}/data";
        let result = resolve_template_variables(input).unwrap();
        let expected = format!("{}/servers/{}/data", 
            env::var("HOME").unwrap(), 
            env::var("USER").unwrap()
        );
        assert_eq!(result, expected);
    }

    #[test]
    fn test_no_template_passthrough() {
        let input = "/usr/local/bin/server";
        let result = resolve_template_variables(input).unwrap();
        assert_eq!(result, input);
    }

    #[test]
    fn test_missing_env_var_error() {
        let input = "${NONEXISTENT_VAR}/path";
        let result = resolve_template_variables(input);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("NONEXISTENT_VAR"));
    }
}