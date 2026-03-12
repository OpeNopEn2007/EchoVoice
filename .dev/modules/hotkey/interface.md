# Hotkey 模块接口定义

## HotkeyManager

```rust
pub struct HotkeyManager {
    // 内部状态
    callback: Arc<Mutex<Box<dyn Fn() + Send + 'static>>>,
}

impl HotkeyManager {
    /// 创建新的热键管理器
    ///
    /// # Arguments
    /// * `callback` - 按键触发时的回调函数
    ///
    /// # Example
    /// ```rust
    /// let manager = HotkeyManager::new(|| {
    ///     println!("F9 pressed!");
    /// });
    /// ```
    pub fn new<F>(callback: F) -> Self
    where
        F: Fn() + Send + 'static

    /// 启动热键监听
    ///
    /// # Returns
    /// * `Ok(())` - 启动成功
    /// * `Err(HotkeyError::ListenError)` - 监听失败
    /// * `Err(HotkeyError::PermissionDenied)` - 权限被拒绝
    pub fn start(&self) -> Result<(), HotkeyError>
}
```

## HotkeyError

```rust
pub enum HotkeyError {
    /// 监听失败
    ListenError(String),

    /// 权限被拒绝
    PermissionDenied,
}
```

---

*接口版本: 0.1.0*