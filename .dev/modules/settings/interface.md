# Settings 模块接口定义

> Settings 模块与前端 Webview 通信的 API 定义

---

## 窗口管理

### 打开设置窗口

```rust
#[tauri::command]
fn open_settings_window(app: AppHandle) -> Result<(), String>
```

### 关闭设置窗口

```rust
#[tauri::command]
fn close_settings_window(app: AppHandle) -> Result<(), String>
```

---

## 配置读写

### 获取当前配置

```rust
#[tauri::command]
fn get_config() -> Result<Config, String>
```

**返回**:
```json
{
  "hotkey": {
    "primary": "F9",
    "secondary": "CapsLock"
  },
  "sound": {
    "enabled": true,
    "on_start": true,
    "on_stop": true,
    "on_complete": true,
    "volume": 0.8
  },
  "asr": {
    "model": "whisper-base",
    "language": "auto",
    "sensitivity": 0.5
  },
  "llm": {
    "model": "smollm2-360m",
    "temperature": 0.7
  },
  "ui": {
    "theme": "auto",
    "window_opacity": 0.95
  },
  "system": {
    "auto_start": false,
    "run_in_background": true
  }
}
```

### 保存配置

```rust
#[tauri::command]
fn save_config(config: Config) -> Result<(), String>
```

**参数**: 同 `get_config` 返回的 JSON 结构

**返回**: 成功返回空，失败返回错误信息

---

## 快捷键录制

### 开始录制快捷键

```rust
#[tauri::command]
fn start_recording_hotkey() -> Result<(), String>
```

### 停止录制快捷键

```rust
#[tauri::command]
fn stop_recording_hotkey() -> Result<String, String>
```

**返回**: 录制到的快捷键名称，如 `"F9"` 或 `"Ctrl+Shift+A"`

### 检测快捷键冲突

```rust
#[tauri::command]
fn check_hotkey_conflict(key: String) -> Result<bool, String>
```

**返回**: `true` 表示有冲突，`false` 表示无冲突

---

## 提示音

### 播放提示音

```rust
#[tauri::command]
fn play_sound(sound_type: String) -> Result<(), String>
```

**参数**:
- `"start"` - 开始录音
- `"stop"` - 停止录音
- `"complete"` - 处理完成

---

## 系统操作

### 设置开机自启动

```rust
#[tauri::command]
fn set_auto_start(enabled: bool) -> Result<(), String>
```

### 最小化到托盘

```rust
#[tauri::command]
fn minimize_to_tray(app: AppHandle) -> Result<(), String>
```

---

## 前端事件监听

前端需要监听的事件：

| 事件名 | 触发时机 |
|--------|----------|
| `config-changed` | 配置被修改 |
| `hotkey-triggered` | 热键被按下 |
| `recording-state-changed` | 录音状态变化 |

---

## 错误响应格式

所有接口的错误响应：

```json
{
  "error": "错误描述信息",
  "code": "ERROR_CODE"
}
```

**错误码**:
- `CONFIG_NOT_FOUND` - 配置文件不存在
- `CONFIG_INVALID` - 配置文件格式错误
- `HOTKEY_CONFLICT` - 快捷键冲突
- `PERMISSION_DENIED` - 权限不足
- `SAVE_FAILED` - 保存失败

---

## 示例：前端调用

```javascript
// 获取配置
const config = await window.__TAURI__.core.invoke('get_config');

// 保存配置
await window.__TAURI__.core.invoke('save_config', { config: newConfig });

// 播放提示音
await window.__TAURI__.core.invoke('play_sound', { soundType: 'start' });
```

---

*文档版本: 0.1.0*
*关联文档: [设计文档](./design.md)*