use once_cell::sync::Lazy;
use std::process::{Command, Child};
use std::sync::Mutex;
use tauri::Emitter;
use std::io::{BufRead, BufReader};
use std::thread;

static UXPLAY_PROCESS: Lazy<Mutex<Option<Child>>> = Lazy::new(|| Mutex::new(None));

#[derive(serde::Deserialize)]
struct StartArgs {
    device_name: String,
    port: Option<i32>,
    verbose: bool,
    no_video: bool,
    fs: bool,
}

#[tauri::command]
fn start_uxplay(app: tauri::AppHandle, args: StartArgs) -> Result<String, String> {
    let mut process_guard = UXPLAY_PROCESS.lock().map_err(|e| e.to_string())?;
    
    if process_guard.is_some() {
        return Err("UxPlay is already running".to_string());
    }

    let mut cmd = Command::new("uxplay");
    cmd.arg("-n").arg(&args.device_name);

    if let Some(port) = args.port {
        cmd.arg("-p").arg(port.to_string());
    }

    if args.verbose {
        cmd.arg("-v");
    }

    if args.no_video {
        cmd.arg("-novideo");
    }

    if args.fs {
        cmd.arg("-fs");
    }

    cmd.stdout(std::process::Stdio::piped());
    cmd.stderr(std::process::Stdio::piped());

    let mut child = cmd.spawn().map_err(|e| format!("Failed to start uxplay: {}. Make sure uxplay is installed and in your PATH.", e))?;
    
    // Capture stdout and stderr
    let stdout = child.stdout.take();
    let stderr = child.stderr.take();
    
    let app_clone = app.clone();
    if let Some(stdout) = stdout {
        thread::spawn(move || {
            let reader = BufReader::new(stdout);
            for line in reader.lines() {
                if let Ok(line) = line {
                    let _ = app_clone.emit("uxplay-log", line);
                }
            }
        });
    }
    
    let app_clone = app.clone();
    if let Some(stderr) = stderr {
        thread::spawn(move || {
            let reader = BufReader::new(stderr);
            for line in reader.lines() {
                if let Ok(line) = line {
                    let _ = app_clone.emit("uxplay-log", line);
                }
            }
        });
    }

    *process_guard = Some(child);

    app.emit("uxplay-status", "Running").map_err(|e| e.to_string())?;
    
    Ok(format!("Started UxPlay with device name: {}", args.device_name))
}

#[tauri::command]
fn stop_uxplay(app: tauri::AppHandle) -> Result<String, String> {
    let mut process_guard = UXPLAY_PROCESS.lock().map_err(|e| e.to_string())?;
    
    if let Some(mut child) = process_guard.take() {
        #[cfg(target_os = "windows")]
        {
            use std::process::Command as StdCommand;
            // On Windows, we need to kill the process tree
            StdCommand::new("taskkill")
                .args(["/F", "/T", "/PID"])
                .arg(child.id().to_string())
                .output()
                .map_err(|e| format!("Failed to kill process: {}", e))?;
        }
        
        #[cfg(not(target_os = "windows"))]
        {
            child.kill().map_err(|e| format!("Failed to kill process: {}", e))?;
        }
        
        app.emit("uxplay-status", "Stopped").map_err(|e| e.to_string())?;
        Ok("UxPlay stopped successfully".to_string())
    } else {
        Err("UxPlay is not running".to_string())
    }
}

#[tauri::command]
fn get_status() -> Result<String, String> {
    let process_guard = UXPLAY_PROCESS.lock().map_err(|e| e.to_string())?;
    
    if process_guard.is_some() {
        Ok("Running".to_string())
    } else {
        Ok("Stopped".to_string())
    }
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![start_uxplay, stop_uxplay, get_status])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
