//! Windows 悬浮胶囊实现
//!
//! 使用 WS_EX_LAYERED + Direct2D 实现

use crate::{CapsuleState, CapsuleWindow, FloatingError, CAPSULE_HEIGHT, CAPSULE_WIDTH};

pub struct WindowsCapsule {
    state: CapsuleState,
    visible: bool,
}

impl WindowsCapsule {
    pub fn new() -> Result<Self, FloatingError> {
        println!("[WindowsCapsule] Creating new capsule window (placeholder)");

        // TODO: 实现完整的 Windows 原生窗口
        // 1. 注册窗口类
        // 2. 创建 WS_EX_LAYERED 窗口
        // 3. 初始化 Direct2D
        // 4. 设置定时器动画

        Ok(Self {
            state: CapsuleState::Idle,
            visible: false,
        })
    }
}

impl CapsuleWindow for WindowsCapsule {
    fn new() -> Result<Self, FloatingError>
    where
        Self: Sized,
    {
        Self::new()
    }

    fn show(&self, x: i32, y: i32) -> Result<(), FloatingError> {
        println!("[WindowsCapsule] Showing at position ({}, {})", x, y);
        Ok(())
    }

    fn hide(&self) -> Result<(), FloatingError> {
        println!("[WindowsCapsule] Hiding");
        Ok(())
    }

    fn set_state(&mut self, state: CapsuleState) -> Result<(), FloatingError> {
        let state_text = match state {
            CapsuleState::Recording => "正在听",
            CapsuleState::Processing => "思考中",
            CapsuleState::Success => "✓ 已复制",
            CapsuleState::NoAudio => "未检测到声音",
            CapsuleState::Error(_) => "识别失败",
            CapsuleState::Idle => "",
        };

        println!("[WindowsCapsule] State changed: {:?} - {}", state, state_text);
        self.state = state;
        Ok(())
    }

    fn update_waveform(&mut self, _levels: &[f32]) -> Result<(), FloatingError> {
        Ok(())
    }

    fn run_loop(&mut self) -> Result<(), FloatingError> {
        Ok(())
    }

    fn close(&mut self) -> Result<(), FloatingError> {
        println!("[WindowsCapsule] Closing");
        Ok(())
    }
}

/// 获取屏幕尺寸
pub fn get_screen_size() -> (i32, i32) {
    // TODO: 使用 Win32 API GetSystemMetrics
    (1920, 1080)
}

/// 获取任务栏高度
pub fn get_taskbar_height() -> i32 {
    // TODO: 使用 Win32 API 获取任务栏高度
    40
}
