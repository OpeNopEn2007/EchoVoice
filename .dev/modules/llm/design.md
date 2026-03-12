# LLM 模块设计文档

## 模块职责

负责将 ASR 识别的文本进行润色、结构化处理，是 EchoVoice 的智能处理环节。

## 功能需求

1. **文本润色**
   - 保持原意
   - 优化表达（更自然、更流畅）
   - 标点规范化

2. **模型管理**
   - 加载本地 GGUF 模型
   - 支持多种模型（SmolLM2-360M、SmolLM2-1.7B）

3. **错误处理**
   - 模型文件不存在
   - 模型加载失败
   - 推理错误

## 技术选型

- **库**: `llama-cpp-2` (Rust 绑定)
- **模型**: GGUF 格式（SmolLM2）
- **上下文长度**: 512 tokens（可配置）

## 接口设计

```rust
#[derive(Error, Debug)]
pub enum LLMError {
    #[error("Model not found: {0}")]
    ModelNotFound(String),
    #[error("Backend initialization failed: {0}")]
    BackendError(String),
    #[error("Model load failed: {0}")]
    ModelLoadError(String),
    #[error("Context creation failed: {0}")]
    ContextError(String),
    #[error("Tokenization failed: {0}")]
    TokenizationError(String),
    #[error("Generation failed: {0}")]
    GenerationError(String),
    #[error("Invalid input")]
    InvalidInput,
}

pub struct SmolLM2 {
    model: LlamaModel,
    n_ctx: u32,
}

impl SmolLM2 {
    /// 创建 LLM 引擎，加载模型
    pub fn new(model_path: impl AsRef<Path>) -> Result<Self, LLMError>

    /// 润色文本
    pub fn polish(&self, text: &str) -> Result<String, LLMError>
}

/// LLM 引擎 trait，便于扩展
pub trait LLMEngine {
    fn polish(&self, text: &str) -> Result<String, LLMError>;
}
```

## 提示词设计

当前使用的提示词（ChatML 格式）：

```
<|im_start|>system
You are a helpful assistant that polishes and improves text. Keep the meaning but make it clearer and more natural.
<|im_end|>
<|im_start|>user
Please polish this text: {用户输入}
<|im_end|>
<|im_start|>assistant
```

> **⚠️ 待解耦**：提示词目前硬编码在代码中，需要移到 `prompts/` 目录

## 生成参数

| 参数 | 值 | 说明 |
|------|-----|------|
| temperature | 0.7 | 创造性平衡 |
| repeat_penalty | 1.1 | 避免重复 |
| max_tokens | 256 | 最大生成长度 |
| n_ctx | 512 | 上下文长度 |

## 错误处理

| 错误类型 | 触发条件 | 处理方式 |
|----------|----------|----------|
| ModelNotFound | 模型文件不存在 | 返回错误，提示用户下载模型 |
| BackendError | Llama 后端初始化失败 | 返回错误详情 |
| ModelLoadError | 模型加载失败 | 返回错误详情 |
| TokenizationError | 分词失败 | 返回错误详情 |
| GenerationError | 生成失败 | 返回错误详情 |
| InvalidInput | 输入为空 | 返回错误 |

## 测试策略

1. 模型不存在时的错误处理
2. 空文本的错误处理
3. 正常润色流程测试
4. 集成测试（需要真实模型文件）

## 依赖

```toml
[dependencies]
llama-cpp-2 = "0.2"
```

## 下一步

1. **提示词解耦**：移到 `prompts/llm-polish.md`
2. 支持更多模型
3. 性能优化（减少内存占用）
4. 流式输出支持

---

*文档版本: 0.1.0*
*关联文档: [接口定义](./interface.md) · [提示词工程](./prompt-engineering.md)*