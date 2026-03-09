# EchoVoice

> Lightweight AI Voice Input - Speak Freely, Polish Smartly

[中文](../README.md) | [日本語](./README.ja.md) (Planned)

---

## One Sentence Intro

EchoVoice is a **purely local** AI voice input method that lets you express freely without worrying about "getting it perfect in one breath". Record → Recognize → Polish → Input, all in one go.

---

## Core Features

| Feature | Description |
|---------|-------------|
| 🎯 **Free Expression** | Speak and organize simultaneously, no need to say everything perfectly at once |
| 🔒 **Purely Local** | All models run locally, voice data never leaves your device |
| 📱 **Cross-Platform** | iOS / Android / Windows / macOS / Linux |
| ⚡ **Lightweight** | Mobile <600MB RAM, Desktop <2.5GB |
| 🎨 **Apple Style** | Minimalist design, seamless system integration |
| 🔧 **Extensible** | Support custom models and cloud APIs |

---

## Quick Start

### Installation

```bash
# macOS
brew install echovoice

# Windows (Winget)
winget install EchoVoice

# Linux
curl -fsSL https://echovoice.dev/install.sh | sh
```

### Usage

**Desktop**:
1. Press `F9` or `Capslock` to start recording
2. Speak freely, don't worry about pauses
3. Release key, automatic recognition and polishing
4. Text automatically inputs at current cursor position

**Mobile**:
1. Long press the floating ball to start recording
2. Release after speaking, automatic processing
3. Text automatically inputs

---

## Technical Architecture

```
User Key → Record → ASR → LLM Polish → Keyboard Input
```

| Component | Technology | Model |
|-----------|------------|-------|
| Audio Recording | cpal | - |
| ASR | whisper.cpp | Whisper tiny/base |
| LLM Polish | llama.cpp | SmolLM2 360M/1.7B |
| Hotkey | global-hotkey | - |
| Keyboard Input | enigo | - |

---

## Multi-Platform Support

| Platform | Trigger | Model Size | RAM Usage |
|----------|---------|------------|-----------|
| Desktop | F9 / Capslock | 1.1GB | ~2.5GB |
| Mobile | Long press floating ball | 280MB | ~600MB |

---

## Configuration

```yaml
# ~/.config/echovoice/config.yaml
hotkey:
  primary: "F9"
  secondary: "CapsLock"

asr:
  model: "whisper-tiny"  # or whisper-base
  language: "auto"

llm:
  model: "smollm2-360m"  # or smollm2-1.7b
  system_prompt: |
    You are an intelligent text assistant...

mobile:
  float_ball_opacity: 0.8
  float_ball_size: 56
```

---

## Development

See [Development Docs](../.dev/README.md)

### Quick Build

```bash
git clone https://github.com/openopen/echovoice.git
cd echovoice

# Desktop
cd desktop
cargo build --release

# Mobile
cd mobile
flutter build
```

---

## Roadmap

- [x] Architecture Design
- [x] Technology Selection
- [ ] Desktop MVP
- [ ] Mobile MVP
- [ ] Configuration System
- [ ] Cloud API Support
- [ ] Plugin System

---

## Contributing

See [Contributing Guide](../CONTRIBUTING.md)

---

## License

[MIT](../LICENSE)

---

*Made with ❤️ by OpenOpen*
