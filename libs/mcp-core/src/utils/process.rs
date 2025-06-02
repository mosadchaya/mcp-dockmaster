#[cfg(windows)]
use std::os::windows::process::CommandExt;
use std::process::Command;

use log::info;
use sysinfo::System;

use super::command::CommandWrappedInShellBuilder;

#[cfg(windows)]
use super::command::CREATE_NO_WINDOW;

fn adapted_process_name(process_name: &str) -> String {
    let name = if cfg!(target_os = "windows") {
        format!("{process_name}.exe").to_string()
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
            println!("Process not found {process_name}");
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
        if n.to_lowercase().eq(&target_name.to_lowercase()) {
            println!("Found process '{process:?}' (PID: {pid}) -- terminating...");
            let command = process
                .cmd()
                .iter()
                .map(|s| s.to_str().unwrap())
                .collect::<Vec<&str>>();
            return Ok(command.first().unwrap_or(&"").to_string());
        }
    }
    print!("Process not found {process_name}");
    Err("Process not found".to_string())
}

pub fn kill_process_by_name(process_name: &str) {
    let adapted_process_name = adapted_process_name(process_name);
    // Windows: Use taskkill command
    #[cfg(windows)]
    let output = std::process::Command::new("taskkill")
        .creation_flags(CREATE_NO_WINDOW)
        .args(["/F", "/T", "/IM", &adapted_process_name])
        .output();

    // Unix-like systems: Use pkill command
    #[cfg(not(windows))]
    let output = std::process::Command::new("pkill")
        .args(["-9", &adapted_process_name])
        .output();

    match output {
        Ok(output) => {
            if output.status.success() {
                log::info!("existing process '{adapted_process_name}' has been terminated.");
            } else {
                log::warn!(
                    "failed to terminate process '{}'. Error: {}",
                    adapted_process_name,
                    String::from_utf8_lossy(&output.stderr)
                );
            }
        }
        Err(e) => {
            log::error!("failed to execute command to terminate process: {e}");
        }
    }
}

pub async fn kill_process_by_pid(process_id: &str) -> Result<(), String> {
    let mut command = if cfg!(target_os = "windows") {
        CommandWrappedInShellBuilder::new("taskkill")
            .args(["/F", "/T", "/PID", process_id])
            .clone()
            .build()
    } else {
        CommandWrappedInShellBuilder::new("kill")
            .args(["-15", process_id])
            .clone()
            .build()
    };
    if let Ok(output) = command.output().await {
        if output.status.success() {
            log::info!("process with PID '{process_id}' has been terminated.");
        } else {
            log::warn!(
                "failed to terminate process with PID '{}'. error: {}",
                process_id,
                String::from_utf8_lossy(&output.stderr)
            );
            return Err(String::from_utf8_lossy(&output.stderr).to_string());
        }
    } else {
        log::error!("failed to execute command to terminate process");
        return Err("failed to execute command to terminate process".to_string());
    }
    Ok(())
}

pub async fn kill_all_processes_by_name(process_name: &str) {
    info!("kill_all_processes_by_name name:{process_name} ");
    let adapted_process_name = adapted_process_name(process_name);
    let mut system = System::new_all();
    system.refresh_all();

    let process_name_as_os_str = std::ffi::OsStr::new(adapted_process_name.as_str());

    let processes: Vec<_> = system.processes_by_name(process_name_as_os_str).collect();

    if processes.is_empty() {
        info!("no process found with name:{process_name}");
        return;
    }

    let futures = processes
        .into_iter()
        .map(|process| async move {
            let pid = process.pid();
            let name = process.name();
            info!(
                "found process id:{} name:{}",
                pid.as_u32(),
                name.to_string_lossy()
            );

            info!(
                "sending kill signal to process id:{} name:{}",
                pid.as_u32(),
                name.to_string_lossy()
            );
            process.kill();

            info!(
                "waiting for exit process id:{} name:{}",
                pid.as_u32(),
                name.to_string_lossy()
            );
            process.wait();

            info!(
                "process id:{} name:{} has been terminated",
                pid.as_u32(),
                name.to_string_lossy()
            );
        })
        .collect::<Vec<_>>();

    futures::future::join_all(futures).await;
}
