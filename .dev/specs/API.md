# EchoVoice API 规范

> 内部模块接口定义（与实际代码一致）+ Tauri 命令（前后端通信）

---

## 一、后端层：内部模块接口

### AudioRecorder

```rust
pub struct AudioRecorder {
    stream: Option<Stream>,
    buffer: Arc<Mutex<Vec<f32>>>,
    sample_rate: u32,
}

impl AudioRecorder {
    /// 创建新的录制器
    pub fn new() -> Result<Self, AudioError>

    /// 开始录音
    pub fn start(&mut self) -> Result<()>

    /// 停止录音并返回音频数据
    pub fn stop(&mut self) -> Result<Vec<f32>>

    /// 检查是否正在录音
    pub fn is_recording(&self) -> bool

    /// 获取采样率
    pub fn sample_rate(&self) -> u32

    /// 清空缓冲区
    pub fn clear_buffer(&mut self)
}
```

---

## WhisperASR

```rust
pub struct WhisperASR {
    ctx: WhisperContext,
}

impl WhisperASR {
    /// 创建 ASR 引擎
    ///
    /// # Arguments
    /// * `model_path` - 模型文件路径
    pub fn new(model_path: impl AsRef<Path>) -> Result<Self, ASRError>

    /// 转录音频为文本
    pub fn transcribe(&self, audio: &[f32]) -> Result<String, ASRError>
}
```

---

## SmolLM2

```rust
pub struct SmolLM2 {
    model: LlamaModel,
    n_ctx: u32,
}

impl SmolLM2 {
    /// 创建 LLM 引擎
    ///
    /// # Arguments
    /// * `model_path` - 模型文件路径
    pub fn new(model_path: impl AsRef<Path>) -> Result<Self, LLMError>

    /// 润色文本
    pub fn polish(&self, text: &str) -> Result<String, LLMError>
}
```

---

## HotkeyManager

```rust
pub struct HotkeyManager {
    callback: Arc<Mutex<Box<dyn Fn() + Send + 'static>>>,
}

impl HotkeyManager {
    /// 创建新的管理器
    ///
    /// # Arguments
    /// * `callback` - 按键触发时的回调函数
    pub fn new<F>(callback: F) -> Self
    where
        F: Fn() + Send + 'static

    /// 启动热键监听
    pub fn start(&self) -> Result<(), HotkeyError>
}
```

---

## TrayManager

```rust
pub struct TrayManager {}

impl TrayManager {
    /// 创建托盘管理器
    pub fn new() -> Result<Self, TrayError>

    /// 显示托盘图标
    pub fn show(&self)

    /// 隐藏托盘图标
    pub fn hide(&self)
}
```

---

## 错误类型

### AudioError

```rust
pub enum AudioError {
    NoInputDevice,
    NoOutputDevice,
    PermissionDenied,
    UnsupportedSampleRate(u32),
    StreamError(String),
    DeviceError(String),
}
```

### ASRError

```rust
pub enum ASRError {
    ModelNotFound(String),
    WhisperError(String),
    InvalidAudio,
}
```

### LLMError

```rust
pub enum LLMError {
    ModelNotFound(String),
    BackendError(String),
    ModelLoadError(String),
    ContextError(String),
    TokenizationError(String),
    GenerationError(String),
    InvalidInput,
}
```

### HotkeyError

```rust
pub enum HotkeyError {
    ListenError(String),
    PermissionDenied,
}
```

### TrayError

```rust
pub enum TrayError {
    CreateError(String),
}
```

---

## 二、API 层：Tauri 命令（前后端通信）

### 录音控制

```rust
#[tauri::command]
fn start_recording() -> Result<(), String>

#[tauri::command]
fn stop_recording() -> Result<String, String>  // 返回识别结果

#[tauri::command]
fn get_recording_state() -> Result<RecordingState, String>

pub enum RecordingState {
    Idle,
    Recording,
    Processing,
    Success,
    Error(String),
}
```

### 胶囊控制

```rust
#[tauri::command]
fn show_capsule(app: AppHandle) -> Result<(), String>

#[tauri::command]
fn hide_capsule(app: AppHandle) -> Result<(), String>

#[tauri::command]
fn update_capsule_state(app: AppHandle, state: CapsuleState) -> Result<(), String>

#[tauri::command]
fn update_capsule_waveform(app: AppHandle, levels: Vec<f32>) -> Result<(), String>
```

### 设置控制

```rust
#[tauri::command]
fn get_config() -> Result<Config, String>

#[tauri::command]
fn save_config(config: Config) -> Result<(), String>

#[tauri::command]
fn set_output_method(method: String) -> Result<(), String>  // "clipboard" | "keyboard"

#[tauri::command]
fn set_auto_start(enabled: bool) -> Result<(), String>

#[tauri::command]
fn play_sound(sound_type: String) -> Result<(), String>
```

### 事件定义

```rust
// 前端监听的事件
enum FrontendEvent {
    "capsule-show",
    "capsule-hide",
    "capsule-state",
    "recording-volume",
    "recording-result",
    "config-changed",
}

// 后端监听的事件
enum BackendEvent {
    "start-recording",   // 前端请求开始录音
    "stop-recording",    // 前端请求停止录音
    "open-settings",     // 打开设置面板
}
```

---

*文档版本: 0.3.0*
*更新日期: 2026-03-12*