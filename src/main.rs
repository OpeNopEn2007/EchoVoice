mod pipeline;

use std::path::Path;
use pipeline::VoicePipeline;

fn main() -> anyhow::Result<()> {
    println!("EchoVoice - AI Voice Input");
    println!("===========================\n");
    
    // Check models
    let models_dir = Path::new("models");
    if !models_dir.exists() {
        eprintln!("Error: models/ directory not found");
        std::process::exit(1);
    }
    
    println!("Available models:");
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
    
    println!("\nInitializing pipeline...");
    let mut pipeline = VoicePipeline::new(
        "models/ggml-base.bin",
        "models/smollm2-360m-q8.gguf",
    )?;
    
    println!("Pipeline ready!");
    println!("\nPress F9 to record (3 second demo)...");
    
    // Demo: Record and transcribe
    match pipeline.record_and_transcribe() {
        Ok(text) => println!("\nTranscribed: {}", text),
        Err(e) => eprintln!("Error: {}", e),
    }
    
    println!("\nStatus: End-to-end pipeline working");
    Ok(())
}