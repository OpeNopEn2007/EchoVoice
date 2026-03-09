//! 音频录制模块
//! 
//! 使用 CPAL 进行跨平台音频录制

use anyhow::{anyhow, Result};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use std::sync::{Arc, Mutex};

/// 音频录制器
pub struct AudioRecorder {
    /// 录制的音频数据
    audio_data: Arc<Mutex<Vec<f32>>>,
    /// 音频流
    stream: Option<Box<dyn StreamTrait>>,
    /// 采样率
    sample_rate: u32,
}

impl AudioRecorder {
    /// 创建新的音频录制器
    pub fn new() -> Result<Self> {
        let host = cpal::default_host();
        let device = host
            .default_input_device()
            .map_err(|_| anyhow!("找不到默认输入设备"))?;
        
        let config = device.default_input_config()?;
        let sample_rate = config.sample_rate().0;
        
        info!("音频设备: {:?}, 采样率: {}", device.name(), sample_rate);
        
        Ok(Self {
            audio_data: Arc::new(Mutex::new(Vec::new())),
            stream: None,
            sample_rate,
        })
    }
    
    /// 开始录音
    pub fn start(&mut self) -> Result<()> {
        if self.stream.is_some() {
            return Err(anyhow!("已经在录音中"));
        }
        
        let host = cpal::default_host();
        let device = host
            .default_input_device()
            .map_err(|_| anyhow!("找不到默认输入设备"))?;
        
        let config = device.default_input_config()?;
        let audio_data = self.audio_data.clone();
        
        // 清空之前的录音数据
        {
            let mut data = audio_data.lock().unwrap();
            data.clear();
        }
        
        let stream = match config.sample_format() {
            cpal::SampleFormat::F32 => device.build_input_stream(
                &config.into(),
                move |data: &[f32], _: &cpal::InputCallbackInfo| {
                    let mut audio = audio_data.lock().unwrap();
                    audio.extend_from_slice(data);
                },
                move |err| {
                    warn!("音频输入错误: {}", err);
                },
                None,
            )?,
            _ => return Err(anyhow!("不支持的音频格式")),
        };
        
        stream.play()?;
        self.stream = Some(Box::new(stream));
        
        info!("开始录音");
        Ok(())
    }
    
    /// 停止录音并返回音频数据
    pub fn stop(&mut self) -> Result<Vec<f32>> {
        if self.stream.is_none() {
            return Err(anyhow!("没有正在进行的录音"));
        }
        
        // 停止流
        self.stream = None;
        
        // 获取录音数据
        let data = {
            let audio = self.audio_data.lock().unwrap();
            audio.clone()
        };
        
        info!("停止录音，共 {} 样本", data.len());
        Ok(data)
    }
    
    /// 获取采样率
    pub fn sample_rate(&self) -> u32 {
        self.sample_rate
    }
}

use tracing::{info, warn};
