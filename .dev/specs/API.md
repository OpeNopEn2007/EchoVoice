# EchoVoice API 规范

> 内部模块接口定义

---

## AudioRecorder

```rust
pub struct AudioRecorder {
    // ...
}

impl AudioRecorder {
    /// 创建新的录制器
    pub fn new() -> Result<Self>
    
    /// 开始录音
    pub fn start(&mut self) -> Result<()>
    
    /// 停止录音并返回音频数据
    pub fn stop(&mut self) -> Result<Vec<f32>>
    
    /// 获取采样率
    pub fn sample_rate(&self) -> u32
}
```

---

## WhisperASR

```rust
pub struct WhisperASR {
    // ...
}

impl WhisperASR {
    /// 创建新的ASR引擎
    pub fn new() -> Result<Self>
    
    /// 识别音频
    pub async fn transcribe(&mut self, audio_data: &[f32]) -> Result<String>
    
    /// 检查模型是否存在
    pub fn check_model(&self) -> bool
}
```

---

## LLMEngine

```rust
pub struct LLMEngine {
    // ...
}

impl LLMEngine {
    /// 创建新的LLM引擎
    pub fn new() -> Result<Self>
    
    /// 润色文本
    pub async fn polish(&mut self, text: &str) -> Result<String>
    
    /// 检查模型是否存在
    pub fn check_model(&self) -> bool
}
```

---

## HotkeyManager

```rust
pub struct HotkeyManager {
    // ...
}

impl HotkeyManager {
    /// 创建新的管理器
    pub fn new() -> Self
    
    /// 注册快捷键
    pub fn register<F>(&mut self, key: &str, callback: F) -> Result<()>
    where F: Fn() + Send + 'static
    
    /// 运行事件循环
    pub async fn run(&self) -> Result<()>
}
```

---

## 错误处理

所有API返回 `anyhow::Result<T>`，包含详细错误信息。

---

*文档版本: 0.1.0*
