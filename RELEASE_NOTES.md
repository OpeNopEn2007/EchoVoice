# EchoVoice v0.1.0 发布说明

## 概述

EchoVoice 是一个轻量级 AI 语音输入法，支持本地运行，保护隐私。

## 功能特性

- 🎤 **语音输入**：按住热键录音，松开自动识别
- 🧠 **本地 AI**：集成 Whisper + SmolLM2，无需联网
- 🔒 **隐私保护**：所有处理在本地完成
- ⚡ **快速响应**：优化的本地模型，实时处理
- 🎨 **精美 UI**：悬浮胶囊显示，仿微信输入法效果
- ⚙️ **可配置**：支持自定义快捷键、提示音、模型选择

## 平台支持

| 平台 | 状态 | 说明 |
|------|------|------|
| macOS | ✅ 可用 | 完整功能，可直接使用 |
| Windows | 🔄 代码完成 | 需 Windows 环境编译运行 |

## 快速开始

### macOS

```bash
# 1. 克隆仓库
git clone https://github.com/OpeNopEn2007/EchoVoice.git
cd EchoVoice

# 2. 下载模型
./scripts/download-models.sh

# 3. 构建并运行
cargo build --release
./target/release/echovoice
```

### Windows

```powershell
# 1. 克隆仓库
git clone https://github.com/OpeNopEn2007/EchoVoice.git
cd EchoVoice

# 2. 下载模型
.\scripts\download-models.ps1

# 3. 构建并运行（需安装 Rust、LLVM、CMake）
cargo build --release
.\target\release\echovoice.exe
```

详细 Windows 安装指南见 [.dev/WINDOWS_DEV_GUIDE.md](.dev/WINDOWS_DEV_GUIDE.md)

## 使用说明

1. **启动程序**：运行 `echovoice`（macOS）或 `echovoice.exe`（Windows）
2. **按住热键**：默认 F9（可在设置中修改）
3. **说话**：看到悬浮胶囊显示"正在听"
4. **松开热键**：胶囊显示"思考中"
5. **完成**：文本自动复制到剪贴板，胶囊显示"✓ 已复制"

## 配置

配置文件位置：
- macOS: `~/Library/Application Support/echovoice/config.yaml`
- Windows: `%APPDATA%\echovoice\config.yaml`

或通过托盘菜单 → 设置打开配置面板。

## 构建包

### macOS DMG

```bash
./scripts/build-dmg.sh 0.1.0
# 输出: target/EchoVoice-0.1.0.dmg
```

### Windows MSI

```powershell
# 需安装 WiX Toolset: choco install wixtoolset
.\scripts\build-msi.bat
# 输出: target\EchoVoice-0.1.0.msi
```

## 技术栈

- **Rust** - 核心语言
- **whisper.cpp** - 语音识别（ASR）
- **llama.cpp** - 文本润色（LLM）
- **cpal** - 跨平台音频
- **rdev** - 全局热键
- **Tauri** - 设置面板
- **Core Animation** / **Direct2D** - 悬浮胶囊动画

## 系统要求

### macOS
- macOS 11.0+ (Big Sur)
- 4GB 内存
- 500MB 磁盘空间

### Windows
- Windows 10 2004+ / Windows 11
- 4GB 内存
- 500MB 磁盘空间

## 许可证

MIT License

## 致谢

- [whisper.cpp](https://github.com/ggerganov/whisper.cpp) - OpenAI Whisper 的 C++ 实现
- [llama.cpp](https://github.com/ggerganov/llama.cpp) - LLM 推理引擎
- [SmolLM2](https://huggingface.co/HuggingFaceTB/SmolLM2-360M-Instruct-GGUF) - 轻量级语言模型

---

*发布时间: 2026-03-13*
*版本: v0.1.0*
