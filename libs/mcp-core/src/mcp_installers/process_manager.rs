use std::process::Command;
use sysinfo::System;

fn adapted_process_name(process_name: &str) -> String {
    let name = if cfg!(target_os = "windows") {
        format!("{}.exe", process_name).to_string()
    } else if cfg!(target_os = "linux") {
        // For linux pkill pattern just supports 15 characters
        process_name.chars().take(15).collect::<String>()
    } else {
        process_name.to_string()
    };
    name
}

pub fn is_process_running(process_name: &str) -> bool {
    find_process_by_name(process_name).is_ok()
}

pub fn restart_process(process_name: &str) -> Result<bool, String> {
    match find_process_by_name(process_name) {
        Ok(command) => {
            kill_process_by_name(process_name);
            Command::new(command).spawn().map_err(|e| e.to_string())?;
            Ok(true)
        }
        Err(e) => {
            println!("Process not found {}", process_name);
            Err(e)
        }
    }
}

pub fn find_process_by_name(process_name: &str) -> Result<String, String> {
    // Create a System object and refresh process information
    let mut system = System::new_all();
    system.refresh_all();
    let target_name = adapted_process_name(process_name);
    // Iterate through all processes and find matches by name
    for (pid, process) in system.processes() {
        let n = process.name().to_str().unwrap().to_string();
        if n == target_name {
            println!(
                "Found process '{:?}' (PID: {}) -- terminating...",
                process, pid
            );
            let command = process
                .cmd()
                .iter()
                .map(|s| s.to_str().unwrap())
                .collect::<Vec<&str>>();
            return Ok(command.first().unwrap_or(&"").to_string());
        }
    }
    print!("Process not found {}", process_name);
    Err("Process not found".to_string())
}

pub fn kill_process_by_name(process_name: &str) {
    let adapted_process_name = adapted_process_name(process_name);
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
