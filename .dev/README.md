# EchoVoice 开发文档

> 轻量级 AI 语音输入法 - 设计与开发文档中心

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
| [hotkey](./modules/hotkey/) | 全局快捷键 | [设计](./modules/hotkey/design.md) · [接口](./modules/hotkey/interface.md) |
| [tray](./modules/tray/) | 系统托盘 | [设计](./modules/tray/design.md) · [接口](./modules/tray/interface.md) |

### 📋 [规范定义](./specs/)
API规范、错误码、命名约定

- [API参考](./specs/api-reference.md)
- [错误码](./specs/error-codes.md)

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
4. **查阅接口**: 参考[API参考](./specs/api-reference.md)

---

## 文档原则

- **文档先行**: 每段代码必须有对应的设计文档
- **链接成网**: 文档之间通过Markdown链接关联
- **版本控制**: 文档与代码同步更新
- **可追溯**: 每个设计决策都有ADR记录

---

*文档版本: 0.1.0*
*最后更新: 2026-03-09*
