#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::Manager;
use std::sync::{Arc, Mutex};
use echovoice_asr::WhisperASR;
use echovoice_llm::SmolLM2;
use echovoice_config::Config;
use std::sync::mpsc::{channel, Sender, Receiver};

enum AudioCommand {
    StartRecording,
    StopRecording,
}

struct AppState {
    audio_tx: Sender<AudioCommand>,
    audio_rx: Arc<Mutex<Receiver<Vec<f32>>>>,
    asr: Arc<Mutex<WhisperASR>>,
    llm: Arc<Mutex<SmolLM2>>,
}

#[tauri::command]
fn start_recording(state: tauri::State<AppState>) -> Result<(), String> {
    state.audio_tx.send(AudioCommand::StartRecording)
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
fn stop_and_transcribe(state: tauri::State<AppState>) -> Result<String, String> {
    state.audio_tx.send(AudioCommand::StopRecording)
        .map_err(|e| e.to_string())?;
    
    // Wait for audio data
    let audio = if let Ok(rx) = state.audio_rx.lock() {
        rx.recv().map_err(|e| e.to_string())?
    } else {
        return Err("Failed to lock audio receiver".to_string());
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
    let _config = Config::load().expect("Failed to load config");
    
    // Setup audio channel
    let (audio_tx, audio_rx) = channel::<AudioCommand>();
    let (data_tx, data_rx) = channel::<Vec<f32>>();
    
    // Start audio thread
    std::thread::spawn(move || {
        use echovoice_audio::AudioRecorder;
        let mut recorder = AudioRecorder::new().expect("Failed to create recorder");
        
        loop {
            if let Ok(cmd) = audio_rx.recv() {
                match cmd {
                    AudioCommand::StartRecording => {
                        let _ = recorder.start();
                    }
                    AudioCommand::StopRecording => {
                        let audio = recorder.stop().unwrap_or_default();
                        let _ = data_tx.send(audio);
                    }
                }
            }
        }
    });
    
    // Initialize components
    let asr = Arc::new(Mutex::new(
        WhisperASR::new("models/ggml-base.bin").expect("Failed to load ASR model")
    ));
    let llm = Arc::new(Mutex::new(
        SmolLM2::new("models/smollm2-360m-q8.gguf").expect("Failed to load LLM model")
    ));
    
    let state = AppState {
        audio_tx,
        audio_rx: Arc::new(Mutex::new(data_rx)),
        asr,
        llm,
    };

    tauri::Builder::default()
        .manage(state)
        .invoke_handler(tauri::generate_handler![
            start_recording,
            stop_and_transcribe,
            polish_text,
            copy_to_clipboard
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}