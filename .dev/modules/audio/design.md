# Audio 模块设计文档

## 模块职责

负责音频的录制和播放，是 EchoVoice 的输入/输出基础。

## 功能需求

1. **录音**
   - 支持系统默认输入设备
   - 采样率：16kHz（Whisper 要求）
   - 格式：f32 PCM
   - 支持按住录音（Push-to-talk）

2. **播放**
   - 支持系统默认输出设备
   - 播放提示音（开始录音、结束录音、错误）

## 技术选型

- **库**: `cpal`（跨平台音频 I/O）
- **后端**: CoreAudio (macOS)、ALSA (Linux)、WASAPI (Windows)

## 接口设计

```rust
pub struct AudioRecorder {
    stream: Option<Stream>,
    buffer: Arc<Mutex<Vec<f32>>>,
}

impl AudioRecorder {
    pub fn new() -> Result<Self>;
    pub fn start(&mut self) -> Result<()>;
    pub fn stop(&mut self) -> Result<Vec<f32>>;
    pub fn is_recording(&self) -> bool;
}

pub struct AudioPlayer {
    stream: Option<Stream>,
}

impl AudioPlayer {
    pub fn new() -> Result<Self>;
    pub fn play(&mut self, samples: &[f32]) -> Result<()>;
    pub fn play_beep(&mut self) -> Result<()>;  // 提示音
}
```

## 错误处理

- 设备未找到
- 权限被拒绝
- 采样率不支持

## 依赖

```toml
[dependencies]
cpal = "0.15"
```

## 测试策略

1. 单元测试：模拟音频流
2. 集成测试：实际录音/播放

## 下一步

1. 实现 AudioRecorder
2. 实现 AudioPlayer
3. 集成到主流程