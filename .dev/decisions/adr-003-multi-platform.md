# ADR-003: 分端策略

> 决定桌面端和移动端的功能差异和实现策略

## 状态

已通过 ✅

## 背景

桌面端和移动端的：
- 交互方式不同（键盘+鼠标 vs 触控）
- 硬件能力不同（性能强 vs 受限）
- 用户期望不同（效率工具 vs 便捷工具）

需要设计合理的分端策略。

## 决策

### 触发方式

| 端 | 触发方式 | 理由 |
|---|----------|------|
| 桌面端 | **全局快捷键**（F9/Capslock）| 不需要切换应用 |
| 移动端 | **悬浮球长按** | 保留原输入法，不改变使用习惯 |

### 交互流程

```
桌面端：
  F9按下 → 录音 → F9释放 → 识别 → 润色 → 模拟输入/剪贴板

移动端：
  长按悬浮球 → 录音 → 释放 → 识别 → 润色 → 直接上屏
```

### 功能差异

| 功能 | 桌面端 | 移动端 |
|------|--------|--------|
| 快捷键自定义 | ✅ 支持 | ❌ 不适用 |
| 悬浮球透明度 | ❌ 不适用 | ✅ 可调 |
| 托盘图标 | ✅ 支持 | ❌ 不适用 |
| 系统通知 | ✅ 支持 | ✅ 支持 |
| 后台常驻 | ✅ 开机自启 | ✅ 前台服务 |

### 模型差异

| 模型 | 桌面端 | 移动端 |
|------|--------|--------|
| Whisper | Base (74MB) | Tiny (31MB) |
| SmolLM2 | 1.7B (~1GB) | 360M (~250MB) |
| 总内存 | ~2.5GB | ~600MB |

## 技术实现

### 代码共享

```
crates/                      # Rust 核心
├── echovoice-core/          # 核心逻辑
│   ├── audio/               # 音频处理
│   ├── asr/                 # 语音识别
│   └── llm/                 # 文本润色
├── echovoice-desktop/       # 桌面端入口
└── echovoice-mobile/        # 移动端入口 (Flutter)

flutter_app/                  # Flutter 移动端
├── lib/
│   ├── core/               # 调用 Rust 核心
│   ├── ui/                 # Flutter 界面
│   └── floating/           # 悬浮球实现
└── android/ + ios/
```

### 接口抽象

```rust
// 定义统一接口
trait InputTrigger {
    fn start_recording();
    fn stop_recording() -> Vec<f32>;
}

impl InputTrigger for HotkeyTrigger {}  // 桌面端
impl InputTrigger for FloatingBallTrigger {}  // 移动端
```

## 扩展性设计

### 模型热插拔

```yaml
# 配置文件
desktop:
  asr_model: "whisper-base.bin"
  llm_model: "smollm2-1.7b.gguf"

mobile:
  asr_model: "whisper-tiny.bin"
  llm_model: "smollm2-360m.gguf"
```

### 云端 API 预留

未来可接入付费云端服务：

```rust
enum ASRProvider {
    Local(WhisperASR),      // 本地
    Cloud(CloudASR),         // 云端（付费）
}

enum LLMProvider {
    Local(SmolLM2),         // 本地
    Cloud(CloudLLM),         // 云端（付费）
}
```

---

*决策日期: 2026-03-09*
*关联文档: [多端策略](../architecture/multi-platform.md)*