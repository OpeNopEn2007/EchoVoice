use rdev::{listen, EventType, Key};
use std::sync::{Arc, Mutex};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum HotkeyError {
    #[error("Failed to listen: {0}")]
    ListenError(String),
    #[error("Permission denied")]
    PermissionDenied,
}

pub struct HotkeyManager {
    callback: Arc<Mutex<Box<dyn Fn() + Send + 'static>>>,
}

impl HotkeyManager {
    pub fn new<F>(callback: F) -> Self
    where
        F: Fn() + Send + 'static,
    {
        Self {
            callback: Arc::new(Mutex::new(Box::new(callback))),
        }
    }

    pub fn start(&self) -> Result<(), HotkeyError> {
        let callback = Arc::clone(&self.callback);
        
        std::thread::spawn(move || {
            if let Err(e) = listen(move |event| {
                if let EventType::KeyPress(key) = event.event_type {
                    if key == Key::F9 {
                        if let Ok(cb) = callback.lock() {
                            cb();
                        }
                    }
                }
            }) {
                eprintln!("Error: {:?}", e);
            }
        });

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hotkey_manager_creation() {
        let manager = HotkeyManager::new(|| {});
        // Just test creation, don't actually start listening in tests
        assert!(true);
    }
}