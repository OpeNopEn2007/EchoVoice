//! EchoVoice 主程序
//!
//! 完整流程：按下热键 → 录音 → 释放热键 → ASR识别 → LLM润色 → 输出文本

use echovoice_audio::{AudioPlayer, AudioRecorder};
use echovoice_asr::WhisperASR;
use echovoice_config::Config;
use echovoice_hotkey::{parse_key, HotkeyEvent, HotkeyManager};
use echovoice_llm::SmolLM2;
use echovoice_output::OutputManager;
use rdev::Key;
use std::path::Path;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

/// 应用状态
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum AppState {
    Idle,
    Recording,
    Processing,
}

/// 复制文本到剪贴板（跨平台）- 兼容旧代码
fn copy_to_clipboard(text: &str) -> bool {
    match OutputManager::new() {
        Ok(mut output) => output.output_clipboard(text).is_ok(),
        Err(e) => {
            eprintln!("Output error: {}", e);
            false
        }
    }
}

fn main() -> anyhow::Result<()> {
    println!("EchoVoice - AI Voice Input");
    println!("===========================\n");

    // 加载配置
    let config = Config::load()?;
    println!("Config loaded:");
    println!("  - Hotkey: {}", config.hotkey.primary);

    // 检查模型目录
    let models_dir = Path::new("models");
    if !models_dir.exists() {
        eprintln!("Error: models/ directory not found");
        std::process::exit(1);
    }

    // 显示可用模型
    println!("\nAvailable models:");
    if let Ok(entries) = std::fs::read_dir(models_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_file() {
                let size = entry.metadata().map(|m| m.len()).unwrap_or(0);
                println!(
                    "  - {} ({} MB)",
                    path.file_name().unwrap_or_default().to_string_lossy(),
                    size / 1024 / 1024
                );
            }
        }
    }

    // 确定模型路径（从配置读取，或使用默认值）
    let asr_model_path = format!("models/{}.bin", config.asr.model);
    let llm_model_path = format!("models/{}.gguf", config.llm.model);

    // 检查模型文件是否存在
    if !Path::new(&asr_model_path).exists() {
        eprintln!("Error: ASR model not found: {}", asr_model_path);
        eprintln!("Please download a Whisper model to the models/ directory");
        std::process::exit(1);
    }

    if !Path::new(&llm_model_path).exists() {
        eprintln!("Error: LLM model not found: {}", llm_model_path);
        eprintln!("Please download a SmolLM2 model to the models/ directory");
        std::process::exit(1);
    }

    // 初始化组件
    println!("\nInitializing...");

    // 音频录制器（在主线程中使用，不是Send的）
    let mut recorder = AudioRecorder::new()?;
    println!("  ✓ AudioRecorder");

    // 音频播放器
    let mut player = AudioPlayer::new()?;
    // 播放提示音表示程序启动
    if config.ui.sound.enabled && config.ui.sound.startup {
        let _ = player.play_beep();
    }
    println!("  ✓ AudioPlayer");

    // ASR 引擎
    let asr = Arc::new(std::sync::Mutex::new(WhisperASR::new(&asr_model_path)?));
    println!("  ✓ WhisperASR: {}", asr_model_path);

    // LLM 引擎
    let llm = Arc::new(std::sync::Mutex::new(SmolLM2::new(&llm_model_path)?));
    println!("  ✓ SmolLM2: {}", llm_model_path);

    println!("\nComponents ready!");

    // 解析热键
    let hotkey_key = parse_key(&config.hotkey.primary).unwrap_or(Key::F9);
    println!("\nPress {} to start recording, release to stop...", config.hotkey.primary);

    // 共享状态（原子操作，线程安全）
    let is_pressed = Arc::new(AtomicBool::new(false));
    let should_start_recording = Arc::new(AtomicBool::new(false));
    let should_stop_recording = Arc::new(AtomicBool::new(false));

    let is_pressed_clone = Arc::clone(&is_pressed);
    let should_start_clone = Arc::clone(&should_start_recording);
    let should_stop_clone = Arc::clone(&should_stop_recording);

    // 创建热键管理器 - 只设置标志位
    let hotkey_manager = HotkeyManager::new(
        move |event| match event {
            HotkeyEvent::Pressed => {
                if !is_pressed_clone.load(Ordering::SeqCst) {
                    is_pressed_clone.store(true, Ordering::SeqCst);
                    should_start_clone.store(true, Ordering::SeqCst);
                }
            }
            HotkeyEvent::Released => {
                if is_pressed_clone.load(Ordering::SeqCst) {
                    is_pressed_clone.store(false, Ordering::SeqCst);
                    should_stop_clone.store(true, Ordering::SeqCst);
                }
            }
        },
        hotkey_key,
    );

    // 启动热键监听
    hotkey_manager.start()?;

    // 主循环 - 处理所有录音逻辑
    println!("\nEchoVoice is running. Press Ctrl+C to exit.");

    let mut is_recording = false;

    loop {
        // 检查是否需要开始录音
        if should_start_recording.load(Ordering::SeqCst) {
            should_start_recording.store(false, Ordering::SeqCst);

            if !is_recording {
                // 开始录音
                if let Err(e) = recorder.start() {
                    eprintln!("Failed to start recording: {}", e);
                } else {
                    is_recording = true;
                    println!("\n[Recording...]");
                    // 播放开始提示音（高频）
                    if config.ui.sound.enabled && config.ui.sound.recording_start {
                        let _ = player.play_recording_start();
                    }
                }
            }
        }

        // 检查是否需要停止录音
        if should_stop_recording.load(Ordering::SeqCst) {
            should_stop_recording.store(false, Ordering::SeqCst);

            if is_recording {
                // 停止录音
                if let Err(e) = recorder.stop() {
                    eprintln!("Failed to stop recording: {}", e);
                } else {
                    is_recording = false;
                    println!("[Stopped]");

                    // 播放停止提示音（中频）
                    if config.ui.sound.enabled {
                        let _ = player.play_recording_stop();
                    }

                    // 获取录音数据并处理
                    let audio = recorder.get_recorded_data().unwrap_or_default();
                    recorder.clear_buffer();

                    if !audio.is_empty() {
                        process_recording(audio, &asr, &llm, &mut player, &config.ui.sound);
                    } else {
                        println!("No audio recorded");
                    }
                }
            }
        }

        thread::sleep(Duration::from_millis(10));
    }
}

/// 处理录音后的完整流程
fn process_recording(
    audio: Vec<f32>,
    asr: &Arc<std::sync::Mutex<WhisperASR>>,
    llm: &Arc<std::sync::Mutex<SmolLM2>>,
    player: &mut AudioPlayer,
    sound_config: &echovoice_config::SoundConfig,
) {
    let duration_secs = audio.len() as f32 / 16000.0;
    println!("Recording done ({}s)", duration_secs);

    // 识别
    println!("Transcribing...");
    let text = match asr.lock() {
        Ok(asr) => match asr.transcribe(&audio) {
            Ok(t) => t,
            Err(e) => {
                eprintln!("ASR error: {}", e);
                return;
            }
        },
        Err(_) => return,
    };

    if text.is_empty() {
        println!("No text recognized");
        return;
    }

    println!("  → \"{}\"", text);

    // 润色
    println!("Polishing...");
    let polished = match llm.lock() {
        Ok(llm) => match llm.polish(&text) {
            Ok(t) => t,
            Err(e) => {
                eprintln!("LLM error: {}, using original text", e);
                text.clone()
            }
        },
        Err(_) => text.clone(),
    };

    if polished != text {
        println!("  → \"{}\"", polished);
    }

    // 复制到剪贴板
    if copy_to_clipboard(&polished) {
        println!("✓ Copied to clipboard");
    } else {
        eprintln!("Failed to copy to clipboard");
    }

    // 播放完成提示音（上升双音调）
    if sound_config.enabled && sound_config.processing_done {
        let _ = player.play_processing_done();
    }
}
