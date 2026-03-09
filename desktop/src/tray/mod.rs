//! 系统托盘模块
//! 
//! 提供系统托盘图标和菜单

use anyhow::Result;
use tray_icon::{TrayIcon, TrayIconBuilder};
use tao::event_loop::EventLoop;

/// 初始化系统托盘
pub fn init_tray() -> Result<TrayIcon> {
    // TODO: 加载图标
    // let icon = load_icon()?;
    
    let tray_icon = TrayIconBuilder::new()
        .with_tooltip("EchoVoice - AI语音输入")
        // .with_icon(icon)
        .build()?;
    
    info!("系统托盘已初始化");
    Ok(tray_icon)
}

use tracing::info;
