# Audio 模块接口定义

> API 规范

---

## AudioRecorder

### `new() -> Result<Self>`

**描述**: 创建新的音频录制器实例

**返回**:
- `Ok(AudioRecorder)` - 成功
- `Err` - 设备不可用或其他错误

**示例**:
```rust
let recorder = AudioRecorder::new()?;
```

---

### `start(&mut self) -> Result<()>`

**描述**: 开始录音

**前置条件**: 未在录音中

**后置条件**: 
- 音频流已启动
- 数据写入内部缓冲区

**错误**:
- `AlreadyRecording` - 已在录音中
- `DeviceUnavailable` - 设备不可用
- `PermissionDenied` - 无权限

**示例**:
```rust
recorder.start()?;
```

---

### `stop(&mut self) -> Result<Vec<f32>>`

**描述**: 停止录音并返回音频数据

**前置条件**: 正在录音中

**返回**: 
- `Ok(Vec<f32>)` - 音频样本数组（f32格式）

**错误**:
- `NotRecording` - 未在录音中

**示例**:
```rust
let audio_data = recorder.stop()?;
// audio_data: Vec<f32>, 范围 [-1.0, 1.0]
```

---

### `sample_rate(&self) -> u32`

**描述**: 获取当前采样率

**返回**: 采样率（Hz），通常为 16000

**示例**:
```rust
let rate = recorder.sample_rate();
assert_eq!(rate, 16000);
```

---

## 错误类型

```rust
pub enum AudioError {
    AlreadyRecording,
    NotRecording,
    DeviceUnavailable(String),
    PermissionDenied,
    InvalidFormat(String),
}
```

---

## 使用示例

```rust
use echovoice::audio::AudioRecorder;

fn main() -> anyhow::Result<()> {
    // 创建录制器
    let mut recorder = AudioRecorder::new()?;
    
    // 开始录音
    recorder.start()?;
    
    // ... 等待一段时间 ...
    std::thread::sleep(std::time::Duration::from_secs(5));
    
    // 停止并获取数据
    let audio_data = recorder.stop()?;
    
    println!("录制了 {} 个样本", audio_data.len());
    println!("时长: {:.2} 秒", audio_data.len() as f32 / 16000.0);
    
    Ok(())
}
```

---

*关联: [详细设计](./design.md)*
