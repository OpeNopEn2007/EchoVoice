# EchoVoice

轻量级 AI 语音输入法 - 本地运行，隐私优先

## 功能

- 🎤 语音输入：按住 F9 录音，自动识别并润色文本
- 🧠 本地 AI：使用 Whisper + SmolLM2，无需联网
- 🔒 隐私保护：所有处理在本地完成
- ⚡ 快速响应：优化的本地模型，实时处理

## 安装

### 下载模型

```bash
# Whisper base (141MB)
curl -L -o models/ggml-base.bin \
  "https://huggingface.co/ggerganov/whisper.cpp/resolve/main/ggml-base.bin"

# SmolLM2 360M (368MB)
curl -L -o models/smollm2-360m-q8.gguf \
  "https://huggingface.co/HuggingFaceTB/SmolLM2-360M-Instruct-GGUF/resolve/main/smollm2-360m-instruct-q8_0.gguf"
```

### 编译

```bash
# macOS 需要 Accessibility 权限
cargo build --release
```

## 使用

```bash
./target/release/echovoice
```

1. 按 F9 开始录音
2. 说话（3秒自动停止）
3. 自动识别并润色
4. 文本自动复制到剪贴板

## 配置

配置文件：`~/.config/echovoice/config.yaml`

```yaml
hotkey:
  primary: "F9"

asr:
  model: "whisper-base"
  language: "auto"

llm:
  model: "smollm2-360m"
```

## 技术栈

- Rust + Cargo
- whisper.cpp (ASR)
- llama.cpp (LLM)
- cpal (音频)
- rdev (全局热键)
- tray-icon (系统托盘)

## 许可证

MIT