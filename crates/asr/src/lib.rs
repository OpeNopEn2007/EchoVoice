use std::path::Path;
use thiserror::Error;
use whisper_rs::{WhisperContext, FullParams, SamplingStrategy};

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
    ctx: WhisperContext,
}

impl WhisperASR {
    pub fn new(model_path: impl AsRef<Path>) -> Result<Self, ASRError> {
        if !model_path.as_ref().exists() {
            return Err(ASRError::ModelNotFound(
                model_path.as_ref().to_string_lossy().to_string()
            ));
        }

        let ctx = WhisperContext::new(
            model_path.as_ref().to_str().unwrap(),
        )
        .map_err(|e| ASRError::WhisperError(format!("{:?}", e)))?;

        Ok(Self { ctx })
    }

    pub fn transcribe(&self, audio: &[f32]) -> Result<String, ASRError> {
        if audio.is_empty() {
            return Err(ASRError::InvalidAudio);
        }

        let mut params = FullParams::new(SamplingStrategy::default());
        params.set_language(Some("zh"));
        params.set_translate(false);
        params.set_print_special(false);
        params.set_print_progress(false);
        params.set_print_realtime(false);
        params.set_print_timestamps(false);

        let mut state = self.ctx.create_state()
            .map_err(|e| ASRError::WhisperError(format!("{:?}", e)))?;

        state.full(params, audio)
            .map_err(|e| ASRError::WhisperError(format!("{:?}", e)))?;

        let num_segments = state.full_n_segments()
            .map_err(|e| ASRError::WhisperError(format!("{:?}", e)))?;

        let mut text = String::new();
        for i in 0..num_segments {
            let segment = state.full_get_segment_text(i)
                .map_err(|e| ASRError::WhisperError(format!("{:?}", e)))?;
            text.push_str(&segment);
        }

        Ok(text.trim().to_string())
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
        let result = WhisperASR::new("/nonexistent/model.bin");
        assert!(result.is_err());
    }

    #[test]
    fn test_transcribe_empty() {
        let model_path = std::path::Path::new("../../models/ggml-base.bin");
        if !model_path.exists() {
            return;
        }
        
        let asr = WhisperASR::new(model_path).unwrap();
        let result = asr.transcribe(&[]);
        assert!(result.is_err());
    }
}