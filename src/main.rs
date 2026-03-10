use std::path::Path;

fn main() {
    println!("EchoVoice - AI Voice Input");
    println!("===========================");
    
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
    
    println!("\nModules loaded:");
    println!("  - audio: Audio recording and playback");
    println!("  - asr: Whisper speech recognition");
    println!("  - llm: SmolLM2 text polishing");
    
    println!("\nStatus: Core modules ready");
    println!("Next: Integrate modules and implement end-to-end flow");
}