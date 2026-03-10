use std::path::Path;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ASRError {
    #[error("Model not found: {0}")]
    ModelNotFound(String),
    #[error("Whisper error: {0}")]
    WhisperError(String),
    #[error("Invalid audio data")]
    InvalidAudio,
}

pub struct WhisperASR {
    model_path: String,
}

impl WhisperASR {
    pub fn new(model_path: impl AsRef<Path>) -> Result<Self, ASRError> {
        let path = model_path.as_ref().to_string_lossy().to_string();
        if !model_path.as_ref().exists() {
            return Err(ASRError::ModelNotFound(path));
        }
        Ok(Self { model_path: path })
    }

    pub fn transcribe(&self, audio: &[f32]) -> Result<String, ASRError> {
        // TODO: Integrate whisper-rs
        // Placeholder implementation
        if audio.is_empty() {
            return Err(ASRError::InvalidAudio);
        }
        Ok("Transcription placeholder".to_string())
    }

    pub fn model_path(&self) -> &str {
        &self.model_path
    }
}

pub trait ASREngine {
    fn transcribe(&self, audio: &[f32]) -> Result<String, ASRError>;
}

impl ASREngine for WhisperASR {
    fn transcribe(&self, audio: &[f32]) -> Result<String, ASRError> {
        self.transcribe(audio)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_asr_creation() {
        // This will fail if model doesn't exist, which is expected in tests
        let result = WhisperASR::new("/nonexistent/model.bin");
        assert!(result.is_err());
    }

    #[test]
    fn test_transcribe_empty() {
        // Skip if model doesn't exist
        let model_path = std::path::Path::new("../../models/ggml-base.bin");
        if !model_path.exists() {
            return;
        }
        
        let asr = WhisperASR::new(model_path).unwrap();
        let result = asr.transcribe(&[]);
        assert!(result.is_err());
    }
}