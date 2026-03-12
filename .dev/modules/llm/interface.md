# LLM 模块接口定义

## SmolLM2

```rust
pub struct SmolLM2 {
    // 内部状态
    model: LlamaModel,
    n_ctx: u32,
}

impl SmolLM2 {
    /// 创建新的 LLM 引擎
    ///
    /// # Arguments
    /// * `model_path` - 模型文件路径（如 "models/smollm2-360m-q8.gguf"）
    ///
    /// # Returns
    /// * `Ok(Self)` - 成功创建
    /// * `Err(LLMError::ModelNotFound)` - 模型文件不存在
    /// * `Err(LLMError::BackendError)` - 后端初始化失败
    /// * `Err(LLMError::ModelLoadError)` - 模型加载失败
    pub fn new(model_path: impl AsRef<Path>) -> Result<Self, LLMError>

    /// 润色文本
    ///
    /// # Arguments
    /// * `text` - 待润色的文本
    ///
    /// # Returns
    /// * `Ok(String)` - 润色后的文本
    /// * `Err(LLMError::InvalidInput)` - 输入为空
    /// * `Err(LLMError::*)?` - 生成过程中的错误
    pub fn polish(&self, text: &str) -> Result<String, LLMError>
}
```

## LLMError

```rust
pub enum LLMError {
    /// 模型文件未找到
    ModelNotFound(String),

    /// Llama 后端初始化失败
    BackendError(String),

    /// 模型加载失败
    ModelLoadError(String),

    /// 上下文创建失败
    ContextError(String),

    /// 分词失败
    TokenizationError(String),

    /// 生成失败
    GenerationError(String),

    /// 无效输入
    InvalidInput,
}
```

## LLMEngine Trait

```rust
/// LLM 引擎抽象接口
pub trait LLMEngine {
    /// 润色文本
    fn polish(&self, text: &str) -> Result<String, LLMError>;
}
```

---

*接口版本: 0.1.0*