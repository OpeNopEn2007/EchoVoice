# Tray 模块设计文档

## 模块职责

负责系统托盘管理，提供后台运行时的交互入口，是 EchoVoice 桌面端的用户交互核心。

## 功能需求

### 1. 托盘图标
- 显示托盘图标
- 隐藏/显示托盘
- 提示文本
- 状态指示（录音中/空闲）

### 2. 托盘菜单
右击托盘图标显示菜单：

```
┌─────────────────────┐
│ 🎤 EchoVoice        │ ← 状态：空闲/录音中
├─────────────────────┤
│ 🎤 开始录音          │ ← 快捷触发
├─────────────────────┤
│ ⚙️ 设置...           │ ← 打开设置面板
│ 📊 状态              │
├─────────────────────┤
│ ❌ 退出              │
└─────────────────────┘
```

### 3. 设置面板入口
- 点击"设置..."打开设置窗口
- 设置窗口通过 Tauri Webview 实现

### 4. 错误处理
- 创建失败
- 图标加载失败

## 技术选型

- **库**: `tray-icon` (Tauri 官方 crate)
- **平台**: Windows / macOS / Linux

## 接口设计

```rust
#[derive(Error, Debug)]
pub enum TrayError {
    #[error("Failed to create tray: {0}")]
    CreateError(String),
    #[error("Icon load failed: {0}")]
    IconError(String),
}

pub enum TrayState {
    Idle,
    Recording,
}

pub struct TrayManager {
    // Tray icon instance
    state: TrayState,
}

impl TrayManager {
    /// 创建托盘管理器
    pub fn new() -> Result<Self, TrayError>

    /// 显示托盘图标
    pub fn show(&self)

    /// 隐藏托盘图标
    pub fn hide(&self)

    /// 更新托盘状态
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

## 当前状态

- [ ] 托盘菜单为空，未添加菜单项
- [ ] show/hide 方法是空实现

## 错误处理

| 错误类型 | 触发条件 | 处理方式 |
|----------|----------|----------|
| CreateError | 托盘创建失败 | 返回错误详情 |

## 测试策略

1. 创建测试（在 CI 环境中跳过）
2. 集成测试（需要图形环境）

## 依赖

```toml
[dependencies]
tray-icon = "0.5"
```

## 下一步

1. ✅ 完善托盘菜单（显示/设置/退出）
2. ✅ 实现 show/hide 功能
3. ✅ 添加设置面板入口
4. 实现状态指示（录音中/空闲）
5. 添加托盘图标点击事件

---

## 关联文档

- [设置面板设计](../settings/design.md)
- [接口定义](./interface.md)