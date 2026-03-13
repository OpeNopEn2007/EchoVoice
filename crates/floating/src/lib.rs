//! Floating Capsule 模块 - 悬浮胶囊窗口
//!
//! 提供录音时的可视化反馈，在任务栏上方显示胶囊形悬浮窗。
//! 支持 Windows 和 macOS 双平台原生实现。

use thiserror::Error;

#[cfg(target_os = "macos")]
#[macro_use]
extern crate objc;

#[derive(Error, Debug)]
pub enum FloatingError {
    #[error("Window creation failed: {0}")]
    WindowCreationFailed(String),
    #[error("Platform not supported")]
    PlatformNotSupported,
    #[error("Render failed: {0}")]
    RenderFailed(String),
    #[error("Animation error: {0}")]
    AnimationError(String),
}

/// 胶囊窗口状态
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CapsuleState {
    /// 空闲状态（隐藏）
    Idle,
    /// 录音中
    Recording,
    /// 处理中（ASR识别）
    Processing,
    /// 成功完成
    Success,
    /// 无音频
    NoAudio,
    /// 识别失败
    Error(String),
}

/// 胶囊窗口接口（跨平台抽象）
pub trait CapsuleWindow {
    /// 创建新窗口
    fn new() -> Result<Self, FloatingError>
    where
        Self: Sized;

    /// 显示窗口在指定位置
    fn show(&self, x: i32, y: i32) -> Result<(), FloatingError>;

    /// 隐藏窗口
    fn hide(&self) -> Result<(), FloatingError>;

    /// 设置窗口状态（更新文字和动画）
    fn set_state(&mut self, state: CapsuleState) -> Result<(), FloatingError>;

    /// 更新音量波纹数据（用于动画）
    fn update_waveform(&mut self, levels: &[f32]) -> Result<(), FloatingError>;

    /// 运行消息循环（平台特定）
    fn run_loop(&mut self) -> Result<(), FloatingError>;

    /// 关闭窗口
    fn close(&mut self) -> Result<(), FloatingError>;
}

// 平台特定实现
#[cfg(target_os = "windows")]
pub mod windows;

#[cfg(target_os = "macos")]
pub mod macos;

// 平台特定导出
#[cfg(target_os = "windows")]
pub use windows::{get_screen_size, get_taskbar_height, WindowsCapsule as NativeCapsule};

#[cfg(target_os = "macos")]
pub use macos::{get_menu_bar_height, get_screen_size, MacOSCapsule as NativeCapsule};

/// 创建平台原生胶囊窗口
pub fn create_capsule() -> Result<NativeCapsule, FloatingError> {
    NativeCapsule::new()
}

/// 获取胶囊窗口尺寸
pub const CAPSULE_WIDTH: i32 = 120;
pub const CAPSULE_HEIGHT: i32 = 32;

/// 计算胶囊在屏幕上的位置（水平居中，任务栏上方）
pub fn calculate_position(screen_width: i32, screen_height: i32, taskbar_height: i32) -> (i32, i32) {
    let x = (screen_width - CAPSULE_WIDTH) / 2;
    let y = screen_height - taskbar_height - CAPSULE_HEIGHT;
    (x, y)
}
