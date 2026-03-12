use std::sync::{Arc, Mutex};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{Stream, StreamConfig};
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
        // 验证输入设备存在
        let host = cpal::default_host();
        let _device = host
            .default_input_device()
            .ok_or(AudioError::NoInputDevice)?;

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

        // 清空缓冲区
        if let Ok(mut buf) = self.buffer.lock() {
            buf.clear();
        }

        Ok(())
    }

    pub fn stop(&mut self) -> Result<(), AudioError> {
        if let Some(stream) = self.stream.take() {
            stream.pause().map_err(|e| AudioError::StreamError(e.to_string()))?;
        }

        Ok(())
    }

    /// 获取录音数据（不清空缓冲区）
    pub fn get_recorded_data(&self) -> Result<Vec<f32>, AudioError> {
        let buffer = Arc::clone(&self.buffer);
        let data = buffer
            .lock()
            .map_err(|_| AudioError::StreamError("Lock poisoned".to_string()))?;

        Ok(data.clone())
    }

    /// 获取录音数据并清空缓冲区
    pub fn get_recorded_data_and_clear(&mut self) -> Result<Vec<f32>, AudioError> {
        let buffer = Arc::clone(&self.buffer);
        let data = buffer
            .lock()
            .map_err(|_| AudioError::StreamError("Lock poisoned".to_string()))?;

        let result = data.clone();
        Ok(result)
    }

    /// 清空录音缓冲区
    pub fn clear_buffer(&mut self) {
        if let Ok(mut buf) = self.buffer.lock() {
            buf.clear();
        }
    }

    pub fn is_recording(&self) -> bool {
        self.stream.is_some()
    }

    pub fn sample_rate(&self) -> u32 {
        self.sample_rate
    }
}

pub struct AudioPlayer {
    #[allow(dead_code)]
    stream: Option<Stream>,
}

impl AudioPlayer {
    pub fn new() -> Result<Self, AudioError> {
        // 验证输出设备存在
        let host = cpal::default_host();
        let _device = host
            .default_output_device()
            .ok_or(AudioError::NoOutputDevice)?;

        Ok(Self { stream: None })
    }

    /// 播放音频样本
    pub fn play(&mut self, samples: &[f32]) -> Result<(), AudioError> {
        if samples.is_empty() {
            return Ok(());
        }

        let host = cpal::default_host();
        let device = host
            .default_output_device()
            .ok_or(AudioError::NoOutputDevice)?;

        let config = StreamConfig {
            channels: CHANNELS,
            sample_rate: cpal::SampleRate(SAMPLE_RATE),
            buffer_size: cpal::BufferSize::Default,
        };

        // 克隆样本数据供回调使用
        let samples = samples.to_vec();
        let samples_len = samples.len();

        let stream = device
            .build_output_stream(
                &config,
                move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
                    let len = std::cmp::min(data.len(), samples_len);
                    data[..len].copy_from_slice(&samples[..len]);
                    // 如果样本比缓冲区短，填充剩余空间
                    for item in data.iter_mut().skip(len) {
                        *item = 0.0;
                    }
                },
                move |err| {
                    eprintln!("Playback error: {}", err);
                },
                None,
            )
            .map_err(|e| AudioError::StreamError(e.to_string()))?;

        stream.play().map_err(|e| AudioError::StreamError(e.to_string()))?;

        // 播放完成后自动释放
        std::thread::sleep(std::time::Duration::from_millis(
            (samples_len as u64 * 1000) / SAMPLE_RATE as u64 + 50,
        ));

        Ok(())
    }

    /// 播放提示音（短 beep）
    pub fn play_beep(&mut self) -> Result<(), AudioError> {
        // 生成 800Hz 正弦波，持续 100ms
        let frequency = 800.0;
        let duration_ms = 100;
        let num_samples = (SAMPLE_RATE as f32 * duration_ms as f32 / 1000.0) as usize;

        let mut samples = Vec::with_capacity(num_samples);
        for i in 0..num_samples {
            let t = i as f32 / SAMPLE_RATE as f32;
            let sample = (2.0 * std::f32::consts::PI * frequency * t).sin() * 0.3;
            samples.push(sample);
        }

        self.play(&samples)
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