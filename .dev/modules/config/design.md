# Config 模块设计文档

## 模块职责

管理 EchoVoice 的配置文件，支持用户自定义设置。

## 配置文件位置

- macOS: `~/.config/echovoice/config.yaml`
- Linux: `~/.config/echovoice/config.yaml`

## 配置项

```yaml
hotkey:
  primary: "F9"
  secondary: "CapsLock"

asr:
  model: "whisper-base"
  language: "auto"
  
llm:
  model: "smollm2-360m"
  system_prompt: "你是一个文本润色助手..."

ui:
  float_ball_opacity: 0.8
  theme: "auto"
```

## 接口设计

```rust
pub struct Config {
    hotkey: HotkeyConfig,
    asr: AsrConfig,
    llm: LlmConfig,
    ui: UiConfig,
}

impl Config {
    pub fn load() -> Result<Self>;
    pub fn save(&self) -> Result<()>;
    pub fn default() -> Self;
}
```

## 依赖

```toml
[dependencies]
serde = { version = "1.0", features = ["derive"] }
serde_yaml = "0.9"
dirs = "5.0"
```