# ASR 模块设计文档

## 模块职责

负责将音频转换为文本（Speech-to-Text），是 EchoVoice 的核心输入环节。

## 功能需求

1. **语音识别**
   - 加载本地 Whisper 模型
   - 识别中文和英文语音
   - 返回结构化文本

2. **错误处理**
   - 模型文件不存在
   - 音频数据无效
   - Whisper 推理错误

## 技术选型

- **库**: `whisper-rs` (Rust 绑定)
- **模型**: whisper.cpp GGML 格式

## 接口设计

```rust
#[derive(Error, Debug)]
pub enum ASRError {
    #[error("Model not found: {0}")]
    ModelNotFound(String),
    #[error("Whisper error: {0}")]
    WhisperError(String),
    #[error("Invalid audio data")]
    InvalidAudio,
}

pub struct WhisperASR {
    ctx: WhisperContext,
}

impl WhisperASR {
    /// 创建 ASR 引擎，加载模型
    pub fn new(model_path: impl AsRef<Path>) -> Result<Self, ASRError>

    /// 转录音频为文本
    pub fn transcribe(&self, audio: &[f32]) -> Result<String, ASRError>
}

/// ASR 引擎 trait，便于扩展
pub trait ASREngine {
    fn transcribe(&self, audio: &[f32]) -> Result<String, ASRError>;
}
```

## 错误处理

| 错误类型 | 触发条件 | 处理方式 |
|----------|----------|----------|
| ModelNotFound | 模型文件不存在 | 返回错误，提示用户下载模型 |
| WhisperError | Whisper 推理失败 | 返回错误详情 |
| InvalidAudio | 音频数据为空 | 返回错误 |

## 测试策略

1. 模型不存在时的错误处理
2. 空音频数据的错误处理
3. 集成测试（需要真实模型文件）

## 依赖

```toml
[dependencies]
whisper-rs = "0.8"
```

## 下一步

1. 支持多语言识别（当前硬编码中文）
2. 添加语言自动检测
3. 优化模型加载速度

---

*文档版本: 0.1.0*
*关联文档: [接口定义](./interface.md)*