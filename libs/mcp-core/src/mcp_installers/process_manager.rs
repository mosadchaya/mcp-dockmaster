pub fn kill_process_by_name(process_name: &str) {
    let adapted_process_name = if cfg!(target_os = "windows") {
        format!("{}.exe", process_name).to_string()
    } else if cfg!(target_os = "linux") {
        // For linux pkill pattern just supports 15 characters
        process_name.chars().take(15).collect::<String>()
    } else {
        process_name.to_string()
    };

    let output = if cfg!(target_os = "windows") {
        // Windows: Use taskkill command
        std::process::Command::new("taskkill")
            .args(["/F", "/T", "/IM", &adapted_process_name])
            .output()
    } else {
        // Unix-like systems: Use pkill command
        std::process::Command::new("pkill")
            .args(["-9", &adapted_process_name])
            .output()
    };

    match output {
        Ok(output) => {
            if output.status.success() {
                log::info!(
                    "existing process '{}' has been terminated.",
                    adapted_process_name
                );
            } else {
                log::warn!(
                    "failed to terminate process '{}'. Error: {}",
                    adapted_process_name,
                    String::from_utf8_lossy(&output.stderr)
                );
            }
        }
        Err(e) => {
            log::error!("failed to execute command to terminate process: {}", e);
        }
    }
}
