# Config 模块接口定义

## Config

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub hotkey: HotkeyConfig,
    pub asr: AsrConfig,
    pub llm: LlmConfig,
    pub ui: UiConfig,
}

impl Config {
    /// 从文件加载配置
    ///
    /// # Returns
    /// * `Ok(Config)` - 成功加载
    /// * `Err(ConfigError)` - 加载失败
    pub fn load() -> Result<Self, ConfigError>

    /// 保存配置到文件
    ///
    /// # Returns
    /// * `Ok(())` - 保存成功
    /// * `Err(ConfigError)` - 保存失败
    pub fn save(&self) -> Result<(), ConfigError>

    /// 获取默认配置
    pub fn default() -> Self
}
```

## HotkeyConfig

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HotkeyConfig {
    /// 主快捷键（桌面端）
    pub primary: String,
    /// 备用快捷键（桌面端）
    pub secondary: Option<String>,
}
```

## AsrConfig

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AsrConfig {
    /// ASR 模型名称
    pub model: String,
    /// 识别语言 (auto/zh/en)
    pub language: String,
}
```

## LlmConfig

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LlmConfig {
    /// LLM 模型名称
    pub model: String,
    /// 系统提示词
    pub system_prompt: String,
}
```

## UiConfig

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UiConfig {
    /// 悬浮球透明度 (0.0-1.0)
    pub float_ball_opacity: f32,
    /// 主题 (auto/light/dark)
    pub theme: String,
}
```

## ConfigError

```rust
pub enum ConfigError {
    /// IO 错误
    Io(#[from] std::io::Error),
    /// YAML 解析错误
    Yaml(#[from] serde_yaml::Error),
    /// 配置目录未找到
    ConfigDirNotFound,
}
```

---

*接口版本: 0.1.0*