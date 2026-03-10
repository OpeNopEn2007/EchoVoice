use std::sync::{Arc, Mutex};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{SampleFormat, Stream, StreamConfig};
use thiserror::Error;

pub const SAMPLE_RATE: u32 = 16000;
pub const CHANNELS: u16 = 1;

#[derive(Error, Debug)]
pub enum AudioError {
    #[error("No input device found")]
    NoInputDevice,
    #[error("No output device found")]
    NoOutputDevice,
    #[error("Permission denied")]
    PermissionDenied,
    #[error("Unsupported sample rate: {0}")]
    UnsupportedSampleRate(u32),
    #[error("Stream error: {0}")]
    StreamError(String),
    #[error("Device error: {0}")]
    DeviceError(String),
}

pub struct AudioRecorder {
    stream: Option<Stream>,
    buffer: Arc<Mutex<Vec<f32>>>,
    sample_rate: u32,
}

impl AudioRecorder {
    pub fn new() -> Result<Self, AudioError> {
        let host = cpal::default_host();
        let device = host
            .default_input_device()
            .ok_or(AudioError::NoInputDevice)?;

        let config = StreamConfig {
            channels: CHANNELS,
            sample_rate: cpal::SampleRate(SAMPLE_RATE),
            buffer_size: cpal::BufferSize::Default,
        };

        Ok(Self {
            stream: None,
            buffer: Arc::new(Mutex::new(Vec::new())),
            sample_rate: SAMPLE_RATE,
        })
    }

    pub fn start(&mut self) -> Result<(), AudioError> {
        if self.stream.is_some() {
            return Ok(());
        }

        let host = cpal::default_host();
        let device = host
            .default_input_device()
            .ok_or(AudioError::NoInputDevice)?;

        let config = StreamConfig {
            channels: CHANNELS,
            sample_rate: cpal::SampleRate(SAMPLE_RATE),
            buffer_size: cpal::BufferSize::Default,
        };

        let buffer = Arc::clone(&self.buffer);
        let stream = device
            .build_input_stream(
                &config,
                move |data: &[f32], _: &cpal::InputCallbackInfo| {
                    if let Ok(mut buf) = buffer.lock() {
                        buf.extend_from_slice(data);
                    }
                },
                move |err| {
                    eprintln!("Stream error: {}", err);
                },
                None,
            )
            .map_err(|e| AudioError::StreamError(e.to_string()))?;

        stream.play().map_err(|e| AudioError::StreamError(e.to_string()))?;
        self.stream = Some(stream);

        Ok(())
    }

    pub fn stop(&mut self) -> Result<Vec<f32>, AudioError> {
        if let Some(stream) = self.stream.take() {
            stream.pause().map_err(|e| AudioError::StreamError(e.to_string()))?;
        }

        let buffer = Arc::clone(&self.buffer);
        let data = buffer
            .lock()
            .map_err(|_| AudioError::StreamError("Lock poisoned".to_string()))?;
        
        Ok(data.clone())
    }

    pub fn is_recording(&self) -> bool {
        self.stream.is_some()
    }

    pub fn sample_rate(&self) -> u32 {
        self.sample_rate
    }

    pub fn clear_buffer(&mut self) {
        if let Ok(mut buf) = self.buffer.lock() {
            buf.clear();
        }
    }
}

pub struct AudioPlayer {
    _stream: Option<Stream>,
}

impl AudioPlayer {
    pub fn new() -> Result<Self, AudioError> {
        Ok(Self { _stream: None })
    }

    pub fn play(&mut self, _samples: &[f32]) -> Result<(), AudioError> {
        // TODO: Implement playback
        Ok(())
    }

    pub fn play_beep(&mut self) -> Result<(), AudioError> {
        // TODO: Generate and play beep sound
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_recorder_creation() {
        let recorder = AudioRecorder::new();
        assert!(recorder.is_ok());
    }

    #[test]
    fn test_player_creation() {
        let player = AudioPlayer::new();
        assert!(player.is_ok());
    }
}