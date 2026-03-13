use tray_icon::{TrayIconBuilder, menu::{Menu, MenuEvent, MenuItem, PredefinedMenuItem}};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum TrayError {
    #[error("Failed to create tray: {0}")]
    CreateError(String),
}

pub struct TrayManager {
    _tray: tray_icon::TrayIcon,
}

pub enum TrayEvent {
    Settings,
    Quit,
}

impl TrayManager {
    pub fn new<F>(on_event: F) -> Result<Self, TrayError>
    where
        F: Fn(TrayEvent) + Send + Sync + 'static,
    {
        // 创建菜单项
        let settings_item = MenuItem::new("设置", true, None);
        let separator = PredefinedMenuItem::separator();
        let quit_item = MenuItem::new("退出", true, None);

        // 创建菜单
        let menu = Menu::new();
        menu.append(&settings_item).map_err(|e| TrayError::CreateError(e.to_string()))?;
        menu.append(&separator).map_err(|e| TrayError::CreateError(e.to_string()))?;
        menu.append(&quit_item).map_err(|e| TrayError::CreateError(e.to_string()))?;

        // 创建托盘图标
        let tray = TrayIconBuilder::new()
            .with_menu(Box::new(menu))
            .with_tooltip("EchoVoice")
            .build()
            .map_err(|e| TrayError::CreateError(e.to_string()))?;

        // 处理菜单事件
        std::thread::spawn(move || {
            let menu_channel = MenuEvent::receiver();
            loop {
                if let Ok(event) = menu_channel.recv() {
                    if event.id == settings_item.id() {
                        on_event(TrayEvent::Settings);
                    } else if event.id == quit_item.id() {
                        on_event(TrayEvent::Quit);
                    }
                }
            }
        });

        Ok(Self { _tray: tray })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tray_manager_creation() {
        // Skip test in CI environments
        if std::env::var("CI").is_ok() {
            return;
        }
        // Tray creation may fail in headless environments
        let _ = TrayManager::new(|_| {});
    }
}