use echovoice_audio::{AudioRecorder, AudioPlayer};
use echovoice_asr::WhisperASR;
use echovoice_llm::SmolLM2;
use echovoice_config::Config;
use echovoice_hotkey::HotkeyManager;
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::sync::mpsc::channel;

enum Command {
    Record,
}

fn main() -> anyhow::Result<()> {
    println!("EchoVoice - AI Voice Input");
    println!("===========================\n");
    
    // Load config
    let config = Config::load()?;
    println!("Config loaded: hotkey = {}", config.hotkey.primary);
    
    // Check models
    let models_dir = Path::new("models");
    if !models_dir.exists() {
        eprintln!("Error: models/ directory not found");
        std::process::exit(1);
    }
    
    println!("\nAvailable models:");
    if let Ok(entries) = std::fs::read_dir(models_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_file() {
                let size = entry.metadata().map(|m| m.len()).unwrap_or(0);
                println!("  - {} ({} MB)", 
                    path.file_name().unwrap_or_default().to_string_lossy(),
                    size / 1024 / 1024
                );
            }
        }
    }
    
    // Initialize components
    println!("\nInitializing...");
    let mut recorder = AudioRecorder::new()?;
    let _player = AudioPlayer::new()?;
    let asr = Arc::new(Mutex::new(WhisperASR::new("models/ggml-base.bin")?));
    let llm = Arc::new(Mutex::new(SmolLM2::new("models/smollm2-360m-q8.gguf")?));
    
    println!("Components ready!");
    println!("\nPress F9 to start recording...");
    
    // Setup channel
    let (tx, rx) = channel::<Command>();
    
    // Setup hotkey
    let hotkey = HotkeyManager::new(move || {
        let _ = tx.send(Command::Record);
    });
    
    hotkey.start()?;
    
    // Main loop
    println!("\nEchoVoice is running. Press Ctrl+C to exit.");
    loop {
        if let Ok(cmd) = rx.try_recv() {
            match cmd {
                Command::Record => {
                    println!("\n[F9] Recording...");
                    
                    // Start recording
                    let _ = recorder.start();
                    
                    // Record for 3 seconds
                    std::thread::sleep(std::time::Duration::from_secs(3));
                    
                    // Stop recording
                    let audio = recorder.stop().unwrap_or_default();
                    
                    println!("Recording complete. Transcribing...");
                    
                    // Transcribe
                    let text = if let Ok(asr) = asr.lock() {
                        asr.transcribe(&audio).unwrap_or_default()
                    } else {
                        String::new()
                    };
                    
                    println!("Transcribed: {}", text);
                    
                    // Polish
                    if !text.is_empty() {
                        println!("Polishing...");
                        let polished = if let Ok(llm) = llm.lock() {
                            llm.polish(&text).unwrap_or(text.clone())
                        } else {
                            text.clone()
                        };
                        
                        println!("Polished: {}", polished);
                        
                        // Copy to clipboard
                        #[cfg(target_os = "macos")]
                        {
                            use std::process::Command;
                            let _ = Command::new("pbcopy")
                                .arg(&polished)
                                .spawn();
                        }
                        
                        println!("(Copied to clipboard)");
                    }
                }
            }
        }
        
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
}