# Floating Capsule 模块接口定义

> 悬浮胶囊与后端通信的 API 定义

---

## 窗口管理

### 显示胶囊

```rust
#[tauri::command]
fn show_capsule(app: AppHandle) -> Result<(), String>
```

**说明**: 在任务栏上方显示录音胶囊

### 隐藏胶囊

```rust
#[tauri::command]
fn hide_capsule(app: AppHandle) -> Result<(), String>
```

**说明**: 隐藏胶囊窗口

### 更新状态

```rust
#[tauri::command]
fn update_capsule_state(app: AppHandle, state: CapsuleState) -> Result<(), String>
```

**参数**:

```rust
pub enum CapsuleState {
    /// 录音中，显示波纹动画
    Recording,

    /// 处理中（ASR 识别）
    Processing,

    /// 识别成功
    Success,

    /// 无音频
    NoAudio,

    /// 识别失败
    Error(String),
}
```

### 更新波纹数据

```rust
#[tauri::command]
fn update_capsule_waveform(app: AppHandle, levels: Vec<f32>) -> Result<(), String>
```

**参数**: `levels` - 音量级别数组，长度 5-7，值范围 0.0-1.0

---

## 前端 → 后端事件

前端监听后端事件：

| 事件名 | 参数 | 说明 |
|--------|------|------|
| `capsule-show` | - | 显示胶囊 |
| `capsule-hide` | - | 隐藏胶囊 |
| `capsule-state` | `CapsuleState` | 更新状态 |
| `recording-volume` | `Vec<f32>` | 实时音量 |

---

## 后端 → 前端事件

后端向前端发送：

| 事件名 | 参数 | 说明 |
|--------|------|------|
| `start-recording` | - | 开始录音 |
| `stop-recording` | - | 停止录音 |
| `recording-volume` | `Vec<f32>` | 实时音量数据 |
| `recording-result` | `String` | 识别结果 |

---

## 示例

### 前端 JavaScript

```javascript
// 监听后端事件
window.__TAURI__.event.listen('capsule-show', () => {
    // 显示胶囊
});

window.__TAURI__.event.listen('recording-volume', (event) => {
    // 更新波纹动画
    const levels = event.payload;
    updateWaveform(levels);
});

// 发送命令
await window.__TAURI__.core.invoke('update_capsule_state', {
    state: 'Recording'
});
```

### 后端 Rust

```rust
// 触发显示
app.emit("capsule-show", ()).unwrap();

// 发送音量数据
app.emit("recording-volume", vec![0.1, 0.3, 0.5, 0.4, 0.2]).unwrap();

// 触发状态更新
app.emit("capsule-state", CapsuleState::Recording).unwrap();
```

---

## 错误处理

| 错误码 | 说明 |
|--------|------|
| WINDOW_CREATE_FAILED | 窗口创建失败 |
| WINDOW_NOT_FOUND | 窗口未找到 |

---

*文档版本: 0.1.0*
*关联文档: [设计文档](./design.md)*