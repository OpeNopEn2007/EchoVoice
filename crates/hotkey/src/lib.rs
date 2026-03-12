use rdev::{listen, EventType, Key};
use std::sync::{Arc, Mutex};
use std::thread;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum HotkeyError {
    #[error("Failed to listen: {0}")]
    ListenError(String),
    #[error("Permission denied")]
    PermissionDenied,
}

/// 热键事件类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HotkeyEvent {
    Pressed,
    Released,
}

/// 热键回调
pub type HotkeyCallback = Arc<Mutex<dyn Fn(HotkeyEvent) + Send + 'static>>;

pub struct HotkeyManager {
    callback: HotkeyCallback,
    target_key: Key,
}

impl HotkeyManager {
    /// 创建新的热键管理器
    ///
    /// # Arguments
    /// * `callback` - 热键事件回调，参数为按下或释放
    /// * `key` - 要监听的热键（默认 F9）
    pub fn new<F>(callback: F, key: Key) -> Self
    where
        F: Fn(HotkeyEvent) + Send + 'static,
    {
        Self {
            callback: Arc::new(Mutex::new(Box::new(callback))),
            target_key: key,
        }
    }

    /// 创建使用默认 F9 键的热键管理器
    pub fn new_default<F>(callback: F) -> Self
    where
        F: Fn(HotkeyEvent) + Send + 'static,
    {
        Self::new(callback, Key::F9)
    }

    /// 启动热键监听
    ///
    /// # Returns
    /// * `Ok(())` - 启动成功
    /// * `Err(HotkeyError::ListenError)` - 监听失败
    pub fn start(&self) -> Result<(), HotkeyError> {
        let callback = Arc::clone(&self.callback);
        let target_key = self.target_key;

        thread::spawn(move || {
            if let Err(e) = listen(move |event| {
                match event.event_type {
                    EventType::KeyPress(key) => {
                        if key == target_key {
                            if let Ok(cb) = callback.lock() {
                                cb(HotkeyEvent::Pressed);
                            }
                        }
                    }
                    EventType::KeyRelease(key) => {
                        if key == target_key {
                            if let Ok(cb) = callback.lock() {
                                cb(HotkeyEvent::Released);
                            }
                        }
                    }
                    _ => {}
                }
            }) {
                eprintln!("Hotkey listen error: {:?}", e);
            }
        });

        Ok(())
    }

    /// 获取当前监听的热键
    pub fn key(&self) -> Key {
        self.target_key
    }
}

/// 解析字符串为 Key
///
/// # Arguments
/// * `key_name` - 键名，如 "F9", "CapsLock", "Space" 等
///
/// # Returns
/// * `Some(Key)` - 解析成功
/// * `None` - 解析失败
pub fn parse_key(key_name: &str) -> Option<Key> {
    match key_name.to_uppercase().as_str() {
        "F1" => Some(Key::F1),
        "F2" => Some(Key::F2),
        "F3" => Some(Key::F3),
        "F4" => Some(Key::F4),
        "F5" => Some(Key::F5),
        "F6" => Some(Key::F6),
        "F7" => Some(Key::F7),
        "F8" => Some(Key::F8),
        "F9" => Some(Key::F9),
        "F10" => Some(Key::F10),
        "F11" => Some(Key::F11),
        "F12" => Some(Key::F12),
        "CAPSLOCK" | "CAPS LOCK" => Some(Key::CapsLock),
        "SPACE" => Some(Key::Space),
        "ENTER" | "RETURN" => Some(Key::Return),
        "ESCAPE" | "ESC" => Some(Key::Escape),
        "TAB" => Some(Key::Tab),
        "BACKSPACE" | "BACK SPACE" => Some(Key::Backspace),
        "DELETE" => Some(Key::Delete),
        "CONTROL" | "CTRL" => Some(Key::ControlLeft),
        "SHIFT" => Some(Key::ShiftLeft),
        "A" => Some(Key::KeyA),
        "B" => Some(Key::KeyB),
        "C" => Some(Key::KeyC),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hotkey_manager_creation() {
        let manager = HotkeyManager::new(|_| {}, Key::F9);
        assert_eq!(manager.key(), Key::F9);
    }

    #[test]
    fn test_parse_key() {
        assert_eq!(parse_key("F9"), Some(Key::F9));
        assert_eq!(parse_key("f9"), Some(Key::F9));
        assert_eq!(parse_key("CapsLock"), Some(Key::CapsLock));
        assert_eq!(parse_key("Unknown"), None);
    }
}