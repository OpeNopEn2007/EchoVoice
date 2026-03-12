# EchoVoice 开发文档

> 轻量级 AI 语音输入法 - 设计与开发文档中心

---

## 🚀 开发计划

### 阶段一：桌面端开发（当前阶段）✅ 优先开发

**技术栈**: Tauri 2.x + Rust + Whisper.cpp + llama.cpp

**核心功能**:
- 全局热键触发（F9 / CapsLock）
- 按住说话，释放停止
- **悬浮胶囊**（录音波条动画）
- ASR 语音识别（Whisper）
- LLM 文本润色（SmolLM2）
- 模拟键盘输入 / 剪贴板输出
- 系统托盘后台运行
- **设置面板**（必须）

**设置面板功能**:
- 热键配置（快捷键自定义）
- 提示音开关（启动/录音/完成）
- 模型选择（ASR / LLM）
- 主题/透明度
- 开机自启动
- 录音灵敏度
- 语言设置

### 阶段二：移动端开发（待开发）

**技术栈**: Flutter + Whisper Flutter binding + llama.cpp Flutter binding

**核心功能**:
- 悬浮球触发（长按说话）
- 按住说话，释放停止
- 直接上屏输出

---

## 文档导航

### 🏗️ [架构设计](./architecture/)
系统整体架构、数据流、多端策略

- [系统概览](./architecture/system-overview.md) - 一句话描述系统
- [数据流设计](./architecture/data-flow.md) - 从录音到上屏的完整流程
- [多端策略](./architecture/multi-platform.md) - 桌面端 vs 移动端

### 📦 [模块设计](./modules/)
每个模块的详细设计、接口定义

| 模块 | 职责 | 文档 |
|------|------|------|
| [audio](./modules/audio/) | 音频录制 | [设计](./modules/audio/design.md) · [接口](./modules/audio/interface.md) |
| [asr](./modules/asr/) | 语音识别 | [设计](./modules/asr/design.md) · [接口](./modules/asr/interface.md) |
| [llm](./modules/llm/) | 文本润色 | [设计](./modules/llm/design.md) · [提示词工程](./modules/llm/prompt-engineering.md) · [接口](./modules/llm/interface.md) |
| [config](./modules/config/) | 配置管理 | [设计](./modules/config/design.md) · [接口](./modules/config/interface.md) |
| [hotkey](./modules/hotkey/) | 全局快捷键 | [设计](./modules/hotkey/design.md) · [接口](./modules/hotkey/interface.md) |
| [tray](./modules/tray/) | 系统托盘 | [设计](./modules/tray/design.md) · [接口](./modules/tray/interface.md) |
| [floating](./modules/floating/) | 悬浮胶囊 | [设计](./modules/floating/design.md) · [接口](./modules/floating/interface.md) |
| [settings](./modules/settings/) | 设置面板 | [设计](./modules/settings/design.md) · [接口](./modules/settings/interface.md) |

### 📋 [规范定义](./specs/)
API规范、错误码、命名约定

- [API参考](./specs/API.md) - 模块接口定义

### 📚 [开发规范](./docs/)
开发流程、编码规范

- [Git工作流](./docs/GIT-WORKFLOW.md) - 分支策略、提交规范、PR流程

### 💡 [决策记录](./decisions/)
关键设计决策及其原因

- [ADR-001: 技术栈选择](./decisions/adr-001-tech-stack.md)
- [ADR-002: 模型选择](./decisions/adr-002-model-selection.md)
- [ADR-003: 分端策略](./decisions/adr-003-multi-platform.md)

---

## 快速开始

1. **了解系统**: 阅读[系统概览](./architecture/system-overview.md)
2. **理解数据流**: 查看[数据流设计](./architecture/data-flow.md)
3. **开发模块**: 按[模块列表](./modules/)逐个实现
4. **查阅接口**: 参考[API参考](./specs/API.md)
5. **遵循规范**: 按照[Git工作流](./docs/GIT-WORKFLOW.md)开发

---

## 文档原则

- **文档先行**: 每段代码必须有对应的设计文档
- **链接成网**: 文档之间通过Markdown链接关联
- **版本控制**: 文档与代码同步更新
- **可追溯**: 每个设计决策都有ADR记录

---

## 文档统计

| 目录 | 文档数 |
|------|--------|
| architecture/ | 3 |
| modules/ | 23 |
| specs/ | 1 |
| docs/ | 2 |
| decisions/ | 3 |
| **总计** | **32** |

---

*文档版本: 0.2.0*
*最后更新: 2026-03-12*