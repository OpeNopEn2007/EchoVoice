#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{SystemTray, SystemTrayEvent, SystemTrayMenu, SystemTrayMenuItem, Manager};
use std::sync::{Arc, Mutex};
use echovoice_audio::AudioRecorder;
use echovoice_asr::WhisperASR;
use echovoice_llm::SmolLM2;
use echovoice_config::Config;

struct AppState {
    recorder: Arc<Mutex<AudioRecorder>>,
    asr: Arc<Mutex<WhisperASR>>,
    llm: Arc<Mutex<SmolLM2>>,
}

#[tauri::command]
fn start_recording(state: tauri::State<AppState>) -> Result<(), String> {
    if let Ok(mut rec) = state.recorder.lock() {
        rec.start().map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[tauri::command]
fn stop_and_transcribe(state: tauri::State<AppState>) -> Result<String, String> {
    let audio = if let Ok(mut rec) = state.recorder.lock() {
        rec.stop().map_err(|e| e.to_string())?
    } else {
        return Err("Failed to lock recorder".to_string());
    };
    
    let text = if let Ok(asr) = state.asr.lock() {
        asr.transcribe(&audio).map_err(|e| e.to_string())?
    } else {
        return Err("Failed to lock ASR".to_string());
    };
    
    Ok(text)
}

#[tauri::command]
fn polish_text(state: tauri::State<AppState>, text: String) -> Result<String, String> {
    if let Ok(llm) = state.llm.lock() {
        llm.polish(&text).map_err(|e| e.to_string())
    } else {
        Err("Failed to lock LLM".to_string())
    }
}

#[tauri::command]
fn copy_to_clipboard(text: String) {
    #[cfg(target_os = "macos")]
    {
        use std::process::Command;
        let _ = Command::new("pbcopy")
            .arg(&text)
            .spawn();
    }
}

fn main() {
    // Load config
    let config = Config::load().expect("Failed to load config");
    
    // Initialize components
    let recorder = Arc::new(Mutex::new(
        AudioRecorder::new().expect("Failed to create recorder")
    ));
    let asr = Arc::new(Mutex::new(
        WhisperASR::new("models/ggml-base.bin").expect("Failed to load ASR model")
    ));
    let llm = Arc::new(Mutex::new(
        SmolLM2::new("models/smollm2-360m-q8.gguf").expect("Failed to load LLM model")
    ));
    
    let state = AppState {
        recorder,
        asr,
        llm,
    };

    // System tray menu
    let tray_menu = SystemTrayMenu::new()
        .add_item(SystemTrayMenuItem::new("开始录音", "record"))
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(SystemTrayMenuItem::new("设置", "settings"))
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(SystemTrayMenuItem::new("退出", "quit"));

    let system_tray = SystemTray::new().with_menu(tray_menu);

    tauri::Builder::default()
        .manage(state)
        .invoke_handler(tauri::generate_handler![
            start_recording,
            stop_and_transcribe,
            polish_text,
            copy_to_clipboard
        ])
        .system_tray(system_tray)
        .on_system_tray_event(|app, event| {
            match event {
                SystemTrayEvent::LeftClick { .. } => {
                    let window = app.get_window("main").unwrap();
                    window.show().unwrap();
                    window.set_focus().unwrap();
                }
                SystemTrayEvent::MenuItemClick { id, .. } => {
                    match id.as_str() {
                        "record" => {
                            // Trigger recording
                        }
                        "settings" => {
                            let window = app.get_window("main").unwrap();
                            window.show().unwrap();
                            window.set_focus().unwrap();
                        }
                        "quit" => {
                            std::process::exit(0);
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}