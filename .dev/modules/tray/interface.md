# Tray 模块接口定义

## TrayManager

```rust
pub struct TrayManager {
    // 托盘图标实例
    state: TrayState,
}

impl TrayManager {
    /// 创建新的托盘管理器
    ///
    /// # Returns
    /// * `Ok(Self)` - 成功创建
    /// * `Err(TrayError::CreateError)` - 创建失败
    pub fn new() -> Result<Self, TrayError>

    /// 显示托盘图标
    pub fn show(&self)

    /// 隐藏托盘图标
    pub fn hide(&self)

    /// 更新托盘状态
    ///
    /// # Arguments
    /// * `state` - 新的托盘状态
    pub fn set_state(&mut self, state: TrayState)

    /// 设置设置面板打开回调
    pub fn on_settings_click<F>(&mut self, callback: F)
    where
        F: Fn() + Send + 'static

    /// 设置开始录音回调
    pub fn on_record_click<F>(&mut self, callback: F)
    where
        F: Fn() + Send + 'static

    /// 设置退出回调
    pub fn on_quit_click<F>(&mut self, callback: F)
    where
        F: Fn() + Send + 'static
}
```

## TrayState

```rust
#[derive(Clone, Debug)]
pub enum TrayState {
    /// 空闲状态
    Idle,
    /// 录音中
    Recording,
}
```

## TrayError

```rust
pub enum TrayError {
    /// 托盘创建失败
    CreateError(String),
    /// 图标加载失败
    IconError(String),
}
```

---

*接口版本: 0.2.0*
*关联文档: [设计文档](./design.md)*