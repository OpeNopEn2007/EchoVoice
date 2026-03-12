# Hotkey 模块设计文档

## 模块职责

负责监听全局快捷键，触发语音输入，是 EchoVoice 的触发机制。

## 功能需求

1. **全局热键监听**
   - 监听 F9 键（默认）
   - 支持自定义快捷键
   - 后台运行

2. **事件回调**
   - 按下时触发回调
   - 释放时停止录音
   - 支持"按住说话，释放停止"

3. **快捷键配置 UI**
   - 在设置面板中提供快捷键选择器
   - 支持多快捷键（F9 + CapsLock）
   - 显示当前快捷键配置
   - 支持快捷键冲突检测

4. **错误处理**
   - 权限被拒绝
   - 监听失败
   - 快捷键冲突

## 技术选型

- **库**: `rdev` (跨平台键盘事件监听)
- **后端**: 平台原生 API

## 接口设计

```rust
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
    /// 创建热键管理器
    pub fn new<F>(callback: F) -> Self
    where
        F: Fn() + Send + 'static

    /// 启动监听
    pub fn start(&self) -> Result<(), HotkeyError>
}
```

## 当前行为

1. 启动新线程监听键盘事件
2. 当按下 F9 时触发回调
3. **待改进**：目前只监听按下，未实现"长按触发，释放停止"的功能

## 错误处理

| 错误类型 | 触发条件 | 处理方式 |
|----------|----------|----------|
| ListenError | 系统监听失败 | 返回错误详情 |
| PermissionDenied | 无权限访问键盘 | 提示用户授权 |

## 测试策略

1. 管理器创建测试
2. 集成测试（需要真实键盘事件）

## 依赖

```toml
[dependencies]
rdev = "0.5"
```

## 下一步

1. 支持快捷键自定义配置
2. 实现"长按触发，释放停止"
3. 支持多个快捷键（F9 + Capslock）
4. 支持移动端悬浮球触发

---

*文档版本: 0.1.0*
*关联文档: [接口定义](./interface.md)*