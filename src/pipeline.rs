use echovoice_audio::{AudioRecorder, AudioPlayer};
use echovoice_asr::WhisperASR;
use echovoice_llm::SmolLM2;
use std::path::Path;

pub struct VoicePipeline {
    recorder: AudioRecorder,
    player: AudioPlayer,
    asr: WhisperASR,
    llm: SmolLM2,
}

impl VoicePipeline {
    pub fn new(
        asr_model: impl AsRef<Path>,
        llm_model: impl AsRef<Path>,
    ) -> anyhow::Result<Self> {
        Ok(Self {
            recorder: AudioRecorder::new()?,
            player: AudioPlayer::new()?,
            asr: WhisperASR::new(asr_model)?,
            llm: SmolLM2::new(llm_model)?,
        })
    }

    pub fn record_and_transcribe(&mut self) -> anyhow::Result<String> {
        // Start recording
        self.recorder.start()?;
        
        // TODO: Wait for hotkey release or timeout
        std::thread::sleep(std::time::Duration::from_secs(3));
        
        // Stop recording
        let audio = self.recorder.stop()?;
        
        // Transcribe
        let text = self.asr.transcribe(&audio)?;
        
        Ok(text)
    }

    pub fn polish(&self, text: &str) -> anyhow::Result<String> {
        Ok(self.llm.polish(text)?)
    }

    pub fn process(&mut self) -> anyhow::Result<String> {
        let text = self.record_and_transcribe()?;
        let polished = self.polish(&text)?;
        Ok(polished)
    }
}