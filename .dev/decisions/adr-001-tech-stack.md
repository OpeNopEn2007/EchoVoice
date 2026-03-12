# ADR-001: 技术栈选择

> 决定 EchoVoice 的技术选型

## 状态

已通过 ✅

## 背景

EchoVoice 需要做一个跨平台（iOS/Android/Windows/macOS/Linux）的本地 AI 语音输入法。需要在性能、隐私、开发效率之间取得平衡。

## 决策

### 桌面端

| 技术 | 选择 | 理由 |
|------|------|------|
| 框架 | **Tauri + Rust** | 轻量（~10MB）、安全、跨平台、性能好 |
| UI | HTML/CSS/JS | Tauri 内置支持，开发效率高 |
| 后端 | Rust | 与 Tauri 天然集成，性能好 |

### 移动端

| 技术 | 选择 | 理由 |
|------|------|------|
| 框架 | **Flutter** | 一套代码双端、性能好、社区活跃 |
| 后端 | Dart + Rust (FFI) | 性能关键逻辑用 Rust |

### AI 模型

| 组件 | 选择 | 理由 |
|------|------|------|
| ASR | **Whisper.cpp** | 本地运行、多语言支持、效果好 |
| LLM | **llama.cpp + SmolLM2** | 边缘优化、支持 GGUF、性能好 |

## 替代方案

### 被否决的方案

| 方案 | 理由 |
|------|------|
| Electron | 体积太大（~150MB），不适合轻量工具 |
| React Native | 性能不如 Flutter，跨平台支持一般 |
| PyQt | Python 运行时重，不适合轻量部署 |
| 云端 API | 违反"纯本地"的隐私要求 |

## 后果

### 正面

- Tauri 打包体积小，启动快
- 一套 Rust 核心代码，桌面端可复用
- Flutter 实现移动端双平台
- Whisper.cpp 和 llama.cpp 成熟稳定

### 负面

- Flutter 桌面端不如移动端成熟
- Rust 学习曲线较陡
- 移动端需要维护 Dart + Rust 两套代码

---

*决策日期: 2026-03-09*
*关联文档: [系统概览](../architecture/system-overview.md)*