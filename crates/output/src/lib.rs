//! Output 模块 - 统一的文本输出接口
//!
//! 支持两种输出方式：
//! 1. 剪贴板（默认）- 将文本复制到剪贴板
//! 2. 模拟键盘输入 - 直接模拟键盘输入文本（需要 enigo 特性）

use arboard::Clipboard;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum OutputError {
    #[error("Clipboard error: {0}")]
    ClipboardError(String),
    #[error("Keyboard simulation error: {0}")]
    KeyboardError(String),
    #[error("Invalid output mode")]
    InvalidMode,
    #[error("Keyboard output not available")]
    KeyboardNotAvailable,
}

/// 输出模式
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OutputMode {
    /// 复制到剪贴板
    Clipboard,
    /// 模拟键盘输入
    Keyboard,
    /// 两者都执行（先键盘输入，再复制到剪贴板）
    Both,
}

impl Default for OutputMode {
    fn default() -> Self {
        OutputMode::Clipboard
    }
}

/// 输出管理器
pub struct OutputManager {
    mode: OutputMode,
    clipboard: Clipboard,
}

impl OutputManager {
    /// 创建新的输出管理器（使用默认剪贴板模式）
    pub fn new() -> Result<Self, OutputError> {
        Self::with_mode(OutputMode::default())
    }

    /// 使用指定模式创建输出管理器
    pub fn with_mode(mode: OutputMode) -> Result<Self, OutputError> {
        let clipboard = Clipboard::new()
            .map_err(|e| OutputError::ClipboardError(e.to_string()))?;

        Ok(Self {
            mode,
            clipboard,
        })
    }

    /// 设置输出模式
    pub fn set_mode(&mut self, mode: OutputMode) {
        self.mode = mode;
    }

    /// 获取当前输出模式
    pub fn mode(&self) -> OutputMode {
        self.mode
    }

    /// 输出文本
    ///
    /// 根据当前模式，将文本输出到剪贴板或模拟键盘输入
    pub fn output(&mut self, text: &str) -> Result<(), OutputError> {
        match self.mode {
            OutputMode::Clipboard => self.output_clipboard(text),
            OutputMode::Keyboard => self.output_keyboard(text),
            OutputMode::Both => {
                self.output_keyboard(text)?;
                self.output_clipboard(text)
            }
        }
    }

    /// 仅输出到剪贴板
    pub fn output_clipboard(&mut self, text: &str) -> Result<(), OutputError> {
        self.clipboard
            .set_text(text)
            .map_err(|e| OutputError::ClipboardError(e.to_string()))?;
        Ok(())
    }

    /// 仅模拟键盘输入（暂不支持）
    pub fn output_keyboard(&mut self, _text: &str) -> Result<(), OutputError> {
        // TODO: 实现键盘模拟输入
        // 当前版本暂不支持，返回错误
        Err(OutputError::KeyboardNotAvailable)
    }

    /// 模拟粘贴操作（Ctrl+V 或 Cmd+V）（暂不支持）
    pub fn paste(&mut self) -> Result<(), OutputError> {
        // TODO: 实现粘贴操作
        Err(OutputError::KeyboardNotAvailable)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_output_manager_creation() {
        let manager = OutputManager::new();
        assert!(manager.is_ok());
    }

    #[test]
    fn test_output_mode_default() {
        let mode = OutputMode::default();
        assert_eq!(mode, OutputMode::Clipboard);
    }
}
