//! 配置热加载模块
//!
//! 监控配置文件变化，自动重新加载

use crate::{Config, ConfigError};
use notify::{Config as NotifyConfig, Event, RecommendedWatcher, RecursiveMode, Watcher};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

/// 配置变化回调
pub type ConfigChangeCallback = Box<dyn Fn(&Config) + Send + 'static>;

/// 配置监控器
pub struct ConfigWatcher {
    running: Arc<AtomicBool>,
    config: Arc<Mutex<Config>>,
    callback: Arc<Mutex<Option<ConfigChangeCallback>>>,
}

impl ConfigWatcher {
    /// 创建新的配置监控器
    ///
    /// 启动后台线程监控配置文件变化
    pub fn new() -> Result<Self, ConfigError> {
        let config = Config::load()?;
        let config_path = Config::config_path()?;
        let config_dir = config_path.parent()
            .map(|p| p.to_path_buf())
            .unwrap_or_else(|| PathBuf::from("."));

        let config_arc = Arc::new(Mutex::new(config));
        let running = Arc::new(AtomicBool::new(true));
        let callback: Arc<Mutex<Option<ConfigChangeCallback>>> = Arc::new(Mutex::new(None));

        let config_clone = Arc::clone(&config_arc);
        let running_clone = Arc::clone(&running);
        let callback_clone = Arc::clone(&callback);
        let config_path_clone = config_path;

        // 启动监控线程
        thread::spawn(move || {
            let (tx, rx) = std::sync::mpsc::channel();

            let mut watcher: RecommendedWatcher = Watcher::new(
                tx,
                NotifyConfig::default().with_poll_interval(Duration::from_secs(1)),
            )
            .expect("Failed to create file watcher");

            watcher
                .watch(&config_dir, RecursiveMode::NonRecursive)
                .expect("Failed to watch config directory");

            println!("[ConfigWatcher] Watching: {:?}", config_dir);

            while running_clone.load(Ordering::SeqCst) {
                match rx.recv_timeout(Duration::from_millis(100)) {
                    Ok(Ok(event)) => {
                        if Self::is_config_file_changed(&event, &config_path_clone) {
                            // 等待文件写入完成
                            thread::sleep(Duration::from_millis(100));

                            match Config::load() {
                                Ok(new_config) => {
                                    // 更新配置
                                    if let Ok(mut cfg) = config_clone.lock() {
                                        *cfg = new_config.clone();
                                    }

                                    // 触发回调
                                    if let Ok(cb_mutex) = callback_clone.lock() {
                                        if let Some(ref cb) = *cb_mutex {
                                            cb(&new_config);
                                        }
                                    }

                                    println!("[ConfigWatcher] Config reloaded");
                                }
                                Err(e) => {
                                    eprintln!("[ConfigWatcher] Failed to reload config: {}", e);
                                }
                            }
                        }
                    }
                    Ok(Err(e)) => {
                        eprintln!("[ConfigWatcher] Watch error: {}", e);
                    }
                    Err(_) => {
                        // Timeout, continue
                    }
                }
            }

            let _ = watcher.unwatch(&config_dir);
            println!("[ConfigWatcher] Stopped");
        });

        Ok(Self {
            running,
            config: config_arc,
            callback: callback,
        })
    }

    /// 检查是否是配置文件变化
    fn is_config_file_changed(event: &Event, config_path: &Path) -> bool {
        let config_file_name = config_path.file_name().unwrap_or_default();

        event.paths.iter().any(|p| {
            p.file_name().unwrap_or_default() == config_file_name
        })
    }

    /// 设置配置变化回调
    pub fn on_change<F>(&self, callback: F)
    where
        F: Fn(&Config) + Send + 'static,
    {
        if let Ok(mut cb) = self.callback.lock() {
            *cb = Some(Box::new(callback));
        }
    }

    /// 获取当前配置
    pub fn config(&self) -> Config {
        self.config
            .lock()
            .map(|cfg| cfg.clone())
            .unwrap_or_default()
    }

    /// 停止监控
    pub fn stop(&self) {
        self.running.store(false, Ordering::SeqCst);
    }
}

impl Drop for ConfigWatcher {
    fn drop(&mut self) {
        self.stop();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_watcher_creation() {
        // 注意：这个测试会实际创建文件监控，在 CI 环境中可能需要跳过
        if std::env::var("CI").is_ok() {
            return;
        }

        let watcher = ConfigWatcher::new();
        // 可能失败（如果配置目录不存在），但不 panic
        if let Ok(w) = watcher {
            // 立即停止
            w.stop();
        }
    }
}
