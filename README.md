# EchoVoice

> 轻量级 AI 语音输入法 - 自由说，智能润色

[English](./i18n/README.en.md) | [日本語](./i18n/README.ja.md) (计划中)

---

## 一句话介绍

EchoVoice 是一个**纯本地运行**的 AI 语音输入法，让你自由表达，无需担心"一口气说完美"的压力。录音 → 识别 → 润色 → 上屏，一气呵成。

---

## 核心特性

| 特性 | 说明 |
|------|------|
| 🎯 **自由表达** | 边说边整理，不必一口气说完美 |
| 🔒 **纯本地** | 所有模型本地运行，语音数据永不离开设备 |
| 📱 **跨平台** | iOS / Android / Windows / macOS / Linux |
| ⚡ **轻量级** | 手机端 <600MB 内存，桌面端 <2.5GB |
| 🎨 **Apple风格** | 极简设计，无缝集成系统 |
| 🔧 **可扩展** | 支持自定义模型和云端API |

---

## 快速开始

### 安装

```bash
# macOS
brew install echovoice

# Windows (Winget)
winget install EchoVoice

# Linux
curl -fsSL https://echovoice.dev/install.sh | sh
```

### 使用

**桌面端**：
1. 按下 `F9` 或 `Capslock` 开始录音
2. 自由说话，不必担心停顿
3. 释放按键，自动识别并润色
4. 文本自动上屏到当前光标位置

**移动端**：
1. 长按悬浮球开始录音
2. 说完后松开，自动处理
3. 文本自动输入

---

## 技术架构

```
用户按键 → 录音 → ASR识别 → LLM润色 → 键盘上屏
```

| 组件 | 技术 | 模型 |
|------|------|------|
| 音频录制 | cpal | - |
| ASR识别 | whisper.cpp | Whisper tiny/base |
| LLM润色 | llama.cpp | SmolLM2 360M/1.7B |
| 快捷键 | global-hotkey | - |
| 键盘输入 | enigo | - |

---

## 多端支持

| 平台 | 触发方式 | 模型大小 | 内存占用 |
|------|---------|---------|---------|
| 桌面端 | F9 / Capslock | 1.1GB | ~2.5GB |
| 移动端 | 悬浮球长按 | 280MB | ~600MB |

---

## 配置

```yaml
# ~/.config/echovoice/config.yaml
hotkey:
  primary: "F9"
  secondary: "CapsLock"

asr:
  model: "whisper-tiny"  # 或 whisper-base
  language: "auto"

llm:
  model: "smollm2-360m"  # 或 smollm2-1.7b
  system_prompt: |
    你是一个智能文本助手...

mobile:
  float_ball_opacity: 0.8
  float_ball_size: 56
```

---

## 开发

参见 [开发文档](./.dev/README.md)

### 快速构建

```bash
git clone https://github.com/openopen/echovoice.git
cd echovoice

# 桌面端
cd desktop
cargo build --release

# 移动端
cd mobile
flutter build
```

---

## 路线图

- [x] 架构设计
- [x] 技术选型
- [ ] 桌面端 MVP
- [ ] 移动端 MVP
- [ ] 配置系统
- [ ] 云端API支持
- [ ] 插件系统

---

## 贡献

参见 [贡献指南](./CONTRIBUTING.md)

---

## 许可证

[MIT](./LICENSE)

---

*Made with ❤️ by OpenOpen*
