//! 全局快捷键模块
//! 
//! 使用 global-hotkey 库实现系统级快捷键监听

use anyhow::{anyhow, Result};
use global_hotkey::{GlobalHotKeyManager, HotKeyState};

/// 快捷键管理器
pub struct HotkeyManager {
    manager: GlobalHotKeyManager,
}

impl HotkeyManager {
    /// 创建新的快捷键管理器
    pub fn new() -> Self {
        let manager = GlobalHotKeyManager::new().expect("创建快捷键管理器失败");
        Self { manager }
    }
    
    /// 注册快捷键
    /// 
    /// # Arguments
    /// * `key` - 按键名称 (如 "F9", "Capslock")
    /// * `callback` - 回调函数
    pub fn register<F>(&mut self, key: &str, callback: F) -> Result<()>
    where
        F: Fn() + Send + 'static,
    {
        // 解析按键
        let hotkey = parse_hotkey(key)?;
        
        // 注册到管理器
        self.manager.register(hotkey)?;
        
        // 启动监听线程
        std::thread::spawn(move || {
            loop {
                if let Ok(events) = GlobalHotKeyManager::receiver().try_recv() {
                    for event in events {
                        if event.state == HotKeyState::Pressed {
                            callback();
                        }
                    }
                }
                std::thread::sleep(std::time::Duration::from_millis(10));
            }
        });
        
        info!("已注册快捷键: {}", key);
        Ok(())
    }
    
    /// 运行事件循环
    pub async fn run(&self) -> Result<()> {
        // 保持程序运行
        loop {
            tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        }
    }
}

/// 解析快捷键字符串
fn parse_hotkey(key: &str) -> Result<global_hotkey::HotKey> {
    use global_hotkey::hotkey::{Code, HotKey, Modifiers};
    
    let code = match key.to_lowercase().as_str() {
        "f1" => Code::F1,
        "f2" => Code::F2,
        "f3" => Code::F3,
        "f4" => Code::F4,
        "f5" => Code::F5,
        "f6" => Code::F6,
        "f7" => Code::F7,
        "f8" => Code::F8,
        "f9" => Code::F9,
        "f10" => Code::F10,
        "f11" => Code::F11,
        "f12" => Code::F12,
        "capslock" => Code::CapsLock,
        "space" => Code::Space,
        "enter" => Code::Enter,
        _ => return Err(anyhow!("不支持的按键: {}", key)),
    };
    
    Ok(HotKey::new(None, code))
}

use tracing::info;
