//! macOS 悬浮胶囊实现
//!
//! 使用 NSWindow + NSVisualEffectView 实现毛玻璃效果

use crate::{CapsuleState, CapsuleWindow, FloatingError, CAPSULE_HEIGHT, CAPSULE_WIDTH};

pub struct MacOSCapsule {
    // 简化的 macOS 实现
    // 实际实现需要使用 cocoa 和 objc crate
    state: CapsuleState,
    visible: bool,
}

impl MacOSCapsule {
    pub fn new() -> Result<Self, FloatingError> {
        println!("[MacOSCapsule] Creating new capsule window");

        Ok(Self {
            state: CapsuleState::Idle,
            visible: false,
        })
    }
}

impl CapsuleWindow for MacOSCapsule {
    fn new() -> Result<Self, FloatingError>
    where
        Self: Sized,
    {
        Self::new()
    }

    fn show(&self, x: i32, y: i32) -> Result<(), FloatingError> {
        println!(
            "[MacOSCapsule] Showing at position ({}, {})",
            x, y
        );
        Ok(())
    }

    fn hide(&self) -> Result<(), FloatingError> {
        println!("[MacOSCapsule] Hiding");
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

        println!("[MacOSCapsule] State changed: {:?} - {}", state, state_text);
        self.state = state;
        Ok(())
    }

    fn update_waveform(&mut self, _levels: &[f32]) -> Result<(), FloatingError> {
        // 简化的波形更新
        Ok(())
    }

    fn run_loop(&mut self) -> Result<(), FloatingError> {
        // macOS 的消息循环由系统管理
        Ok(())
    }

    fn close(&mut self) -> Result<(), FloatingError> {
        println!("[MacOSCapsule] Closing");
        Ok(())
    }
}

/// 获取屏幕尺寸（简化实现）
pub fn get_screen_size() -> (i32, i32) {
    // 默认返回常见分辨率
    (1920, 1080)
}

/// 获取菜单栏高度（简化实现）
pub fn get_menu_bar_height() -> i32 {
    // macOS 菜单栏默认高度
    24
}
