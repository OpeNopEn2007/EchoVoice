//! EchoVoice 桌面端
//! 
//! 轻量级 AI 语音输入法
//! 快捷键触发 → 录音 → ASR识别 → LLM润色 → 自动上屏

use anyhow::Result;
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::{info, warn};

mod audio;
mod asr;
mod enigo;
mod hotkey;
mod llm;
mod tray;

use audio::AudioRecorder;
use asr::WhisperASR;
use hotkey::HotkeyManager;
use llm::LLMEngine;

/// 应用状态
struct AppState {
    /// 音频录制器
    recorder: Arc<Mutex<AudioRecorder>>,
    /// ASR引擎
    asr: Arc<Mutex<WhisperASR>>,
    /// LLM引擎
    llm: Arc<Mutex<LLMEngine>>,
    /// 是否正在录音
    is_recording: Arc<Mutex<bool>>,
}

#[tokio::main]
async fn main() -> Result<()> {
    // 初始化日志
    tracing_subscriber::fmt::init();
    info!("EchoVoice 桌面端启动");

    // 初始化应用状态
    let state = Arc::new(AppState {
        recorder: Arc::new(Mutex::new(AudioRecorder::new()?)),
        asr: Arc::new(Mutex::new(WhisperASR::new()?)),
        llm: Arc::new(Mutex::new(LLMEngine::new()?)),
        is_recording: Arc::new(Mutex::new(false)),
    });

    // 初始化系统托盘
    let _tray = tray::init_tray()?;
    info!("系统托盘已初始化");

    // 初始化全局快捷键
    let mut hotkey_manager = HotkeyManager::new();
    
    // 注册 F9 快捷键
    let state_clone = state.clone();
    hotkey_manager.register("F9", move || {
        let state = state_clone.clone();
        tokio::spawn(async move {
            if let Err(e) = handle_hotkey(state).await {
                warn!("快捷键处理失败: {}", e);
            }
        });
    })?;
    
    // 注册 Capslock 快捷键
    let state_clone = state.clone();
    hotkey_manager.register("Capslock", move || {
        let state = state_clone.clone();
        tokio::spawn(async move {
            if let Err(e) = handle_hotkey(state).await {
                warn!("快捷键处理失败: {}", e);
            }
        });
    })?;
    
    info!("快捷键已注册: F9, Capslock");

    // 运行事件循环
    hotkey_manager.run().await?;

    Ok(())
}

/// 处理快捷键触发
async fn handle_hotkey(state: Arc<AppState>) -> Result<()> {
    let mut is_recording = state.is_recording.lock().await;
    
    if *is_recording {
        // 停止录音，开始处理
        info!("停止录音，开始处理...");
        *is_recording = false;
        drop(is_recording);
        
        // 停止录音并获取音频数据
        let audio_data = {
            let mut recorder = state.recorder.lock().await;
            recorder.stop()?
        };
        
        info!("录音完成，音频长度: {} 样本", audio_data.len());
        
        // ASR 识别
        let text = {
            let mut asr = state.asr.lock().await;
            asr.transcribe(&audio_data).await?
        };
        
        info!("ASR 识别结果: {}", text);
        
        // LLM 润色
        let polished = {
            let mut llm = state.llm.lock().await;
            llm.polish(&text).await?
        };
        
        info!("LLM 润色结果: {}", polished);
        
        // 模拟键盘输入上屏
        enigo::simulate_key_input(&polished)?;
        
        info!("文本已上屏");
    } else {
        // 开始录音
        info!("开始录音...");
        *is_recording = true;
        drop(is_recording);
        
        let mut recorder = state.recorder.lock().await;
        recorder.start()?;
    }
    
    Ok(())
}
