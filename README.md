# EchoVoice

轻量级 AI 语音输入法 - 本地运行，隐私优先

## 功能

- 🎤 语音输入：按住 F9 录音，自动识别并润色文本
- 🧠 本地 AI：使用 Whisper + SmolLM2，无需联网
- 🔒 隐私保护：所有处理在本地完成
- ⚡ 快速响应：优化的本地模型，实时处理

## 平台支持

| 平台 | 状态 | 说明 |
|------|------|------|
| macOS | ✅ 可用 | 完整功能，可直接编译运行 |
| Windows | 🔄 代码完成 | 需 Windows 环境编译测试 |

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

或使用脚本：
```bash
# macOS/Linux
./scripts/download-models.sh

# Windows
.\scripts\download-models.ps1
```

### 编译

**macOS：**
```bash
cargo build --release
./target/release/echovoice
```

**Windows：**
```powershell
cargo build --release
.\target\release\echovoice.exe
```

详细安装说明见 [INSTALL.md](INSTALL.md)。

## 使用

```bash
./target/release/echovoice
```

1. 按住 F9 开始录音
2. 说话
3. 松开 F9 停止录音
4. 自动识别、润色并复制到剪贴板

## Windows 端开发协作

由于 Windows 和 macOS 系统差异，采用以下协作方式：

1. **macOS 端**：完成核心功能和 macOS UI
2. **Windows 端**：参考 [WINDOWS_DEV_GUIDE.md](.dev/WINDOWS_DEV_GUIDE.md) 在 Windows 机器上完成编译测试

### Windows 快速开始

```powershell
# 1. 克隆仓库
git clone https://github.com/OpeNopEn2007/EchoVoice.git
cd EchoVoice

# 2. 阅读 Windows 开发指南
code .dev\WINDOWS_DEV_GUIDE.md

# 3. 运行 Claude Code
claude

# 4. 在 Claude Code 中执行
# "读取 PLAN.md 检查进度，完成 Windows 端任务"
```

## 配置

配置文件：`~/.config/echovoice/config.yaml` (macOS) 或 `%APPDATA%\echovoice\config.yaml` (Windows)

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
- Tauri (设置面板)
- Direct2D / Core Animation (悬浮胶囊)

## 许可证

MIT