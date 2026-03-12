# Config 模块设计文档

## 模块职责

管理 EchoVoice 的配置文件，支持用户自定义设置。

## 配置文件位置

- macOS: `~/.config/echovoice/config.yaml`
- Linux: `~/.config/echovoice/config.yaml`
- Windows: `%APPDATA%/echovoice/config.yaml`

## 配置项

```yaml
# 热键配置
hotkey:
  primary: "F9"           # 主快捷键
  secondary: "CapsLock"   # 备用快捷键

# 语音识别配置
asr:
  model: "whisper-base"
  language: "auto"       # auto = 自动检测
  sensitivity: 0.5        # 录音灵敏度 0.0-1.0

# LLM 润色配置
llm:
  model: "smollm2-360m"
  system_prompt: "你是一个文本润色助手..."
  temperature: 0.7

# 提示音配置
sound:
  enabled: true           # 是否启用提示音
  on_start: true          # 开始录音提示音
  on_stop: true           # 停止录音提示音
  on_complete: true       # 完成处理提示音
  volume: 0.8             # 音量 0.0-1.0

# UI 配置
ui:
  float_ball_opacity: 0.8
  theme: "auto"           # auto / light / dark
  window_opacity: 0.95

# 系统配置
system:
  auto_start: false       # 开机自启动
  run_in_background: true # 后台运行
  minimize_to_tray: true  # 最小化到托盘

# 输出配置
output:
  method: "clipboard"     # clipboard / keyboard
  auto_paste: true        # 复制后自动粘贴
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