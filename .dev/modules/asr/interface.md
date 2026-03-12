# ASR 模块接口定义

## WhisperASR

```rust
pub struct WhisperASR {
    // 内部状态
    ctx: WhisperContext,
}

impl WhisperASR {
    /// 创建新的 ASR 引擎
    ///
    /// # Arguments
    /// * `model_path` - 模型文件路径（如 "models/ggml-base.bin"）
    ///
    /// # Returns
    /// * `Ok(Self)` - 成功创建
    /// * `Err(ASRError::ModelNotFound)` - 模型文件不存在
    /// * `Err(ASRError::WhisperError)` - 模型加载失败
    pub fn new(model_path: impl AsRef<Path>) -> Result<Self, ASRError>

    /// 转录音频数据为文本
    ///
    /// # Arguments
    /// * `audio` - 音频样本（f32 数组，16kHz 采样率，单声道）
    ///
    /// # Returns
    /// * `Ok(String)` - 识别出的文本
    /// * `Err(ASRError::InvalidAudio)` - 音频为空
    /// * `Err(ASRError::WhisperError)` - 推理失败
    pub fn transcribe(&self, audio: &[f32]) -> Result<String, ASRError>
}
```

## ASRError

```rust
pub enum ASRError {
    /// 模型文件未找到
    ModelNotFound(String),

    /// Whisper 运行时错误
    WhisperError(String),

    /// 无效的音频数据
    InvalidAudio,
}
```

## ASREngine Trait

```rust
/// ASR 引擎抽象接口
pub trait ASREngine {
    /// 转录音频为文本
    fn transcribe(&self, audio: &[f32]) -> Result<String, ASRError>;
}
```

---

*接口版本: 0.1.0*