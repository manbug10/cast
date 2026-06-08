// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::fs;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::sync::Mutex;
use tauri::State;

struct AppState {
    process: Mutex<Option<std::process::Child>>,
    uxplay_path: Mutex<Option<PathBuf>,
}

#[derive(serde::Deserialize)]
struct StartParams {
    name: String,
    port: Option<u16>,
    verbose: bool,
    audio_only: bool,
    fullscreen: bool,
}

#[tauri::command]
fn start_server(state: State<AppState>, params: StartParams) -> Result<String, String> {
    let mut lock = state.process.lock().map_err(|e| e.to_string())?;
    
    if lock.is_some() {
        return Err("El servidor ya está en ejecución".to_string());
    }

    // Extraer el binario embebido
    let exe_path = std::env::current_exe().map_err(|e| e.to_string())?;
    let exe_dir = exe_path.parent().ok_or("No se pudo determinar el directorio")?;
    let temp_uxplay = exe_dir.join("uxplay_temp.exe");

    // Si existe el binario embebido (en producción), lo copiamos
    // En desarrollo, asumimos que está en la carpeta src-tauri/uxplay_windows.exe
    let source_path = if cfg!(debug_assertions) {
        let dev_path = exe_dir.join("uxplay_windows.exe");
        if dev_path.exists() {
            dev_path
        } else {
            // Fallback para desarrollo si no se ha compilado UxPlay aún
            return Err("No se encontró uxplay_windows.exe en el directorio de desarrollo. Ejecuta el workflow de GitHub o compílalo manualmente.".to_string());
        }
    } else {
        // En producción, el archivo debería haber sido extraído o estar junto al exe
        // Para simplificar, asumimos que el build script lo dejó ahí
        let prod_path = exe_dir.join("uxplay_windows.exe");
        if !prod_path.exists() {
             return Err("Error crítico: No se encontró el motor UxPlay integrado.".to_string());
        }
        prod_path
    };

    // Construir argumentos
    let mut args = Vec::new();
    
    if !params.name.is_empty() {
        args.push("-n");
        args.push(&params.name);
    }
    
    if let Some(port) = params.port {
        args.push("-p");
        args.push(&port.to_string());
    }
    
    if params.verbose {
        args.push("-v");
    }
    
    if params.audio_only {
        args.push("-a");
    }
    
    if params.fullscreen {
        args.push("-f");
    }

    // Iniciar proceso
    let mut cmd = Command::new(source_path);
    cmd.args(&args);
    cmd.stdout(Stdio::piped());
    cmd.stderr(Stdio::piped());

    let mut child = cmd.spawn().map_err(|e| format!("Error al iniciar UxPlay: {}", e))?;
    
    // Leer salida en un hilo separado
    let stdout = child.stdout.take().ok_or("No se pudo capturar la salida")?;
    let stderr = child.stderr.take().ok_or("No se pudo capturar el error")?;
    
    let app_handle = tauri::AppHandle::from(state.inner()); // Esto requiere acceso al handle, lo simplificaremos
    
    std::thread::spawn(move || {
        let reader = BufReader::new(stdout);
        for line in reader.lines() {
            if let Ok(l) = line {
                // Emitir evento a la GUI (simplificado)
                println!("UXPLAY: {}", l);
            }
        }
    });

    std::thread::spawn(move || {
        let reader = BufReader::new(stderr);
        for line in reader.lines() {
            if let Ok(l) = line {
                println!("UXPLAY ERR: {}", l);
            }
        }
    });

    *lock = Some(child);
    Ok("Servidor iniciado correctamente".to_string())
}

#[tauri::command]
fn stop_server(state: State<AppState>) -> Result<String, String> {
    let mut lock = state.process.lock().map_err(|e| e.to_string())?;
    
    if let Some(mut child) = lock.take() {
        child.kill().map_err(|e| format!("Error al detener: {}", e))?;
        child.wait().ok();
        Ok("Servidor detenido".to_string())
    } else {
        Err("El servidor no está en ejecución".to_string())
    }
}

#[tauri::command]
fn is_running(state: State<AppState>) -> Result<bool, String> {
    let lock = state.process.lock().map_err(|e| e.to_string())?;
    Ok(lock.is_some())
}

fn main() {
    tauri::Builder::default()
        .manage(AppState {
            process: Mutex::new(None),
            uxplay_path: Mutex::new(None),
        })
        .invoke_handler(tauri::generate_handler![start_server, stop_server, is_running])
        .run(tauri::generate_context!())
        .expect("Error al ejecutar Tauri");
}
