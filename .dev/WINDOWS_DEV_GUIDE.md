# EchoVoice Windows 端开发指南

> 本指南用于在 Windows 机器上运行 Claude Code，完成 EchoVoice 的 Windows 端开发。

---

## 环境准备

### 1. 安装基础工具

以管理员身份打开 PowerShell，执行：

```powershell
# 安装 Chocolatey（包管理器）
Set-ExecutionPolicy Bypass -Scope Process -Force
[System.Net.ServicePointManager]::SecurityProtocol = [System.Net.ServicePointManager]::SecurityProtocol -bor 3072
iex ((New-Object System.Net.WebClient).DownloadString('https://community.chocolatey.org/install.ps1'))

# 安装开发工具
choco install -y git
choco install -y rust
choco install -y llvm
choco install -y cmake
choco install -y vscode  # 可选

# 重启 PowerShell
```

### 2. 验证安装

```powershell
git --version
rustc --version
cargo --version
clang --version
```

### 3. 安装 Claude Code

```powershell
# 安装 Node.js（如未安装）
choco install -y nodejs

# 安装 Claude Code
npm install -g @anthropic-ai/claude-code

# 验证
claude --version
```

---

## 项目设置

### 1. 克隆仓库

```powershell
# 选择工作目录
cd C:\Users\$env:USERNAME\Projects

# 克隆仓库
git clone https://github.com/OpeNopEn2007/EchoVoice.git
cd EchoVoice
```

### 2. 下载模型

```powershell
# 使用提供的脚本
.\scripts\download-models.ps1

# 或手动下载
mkdir -p models
cd models

# Whisper Base (148MB)
Invoke-WebRequest -Uri "https://huggingface.co/ggerganov/whisper.cpp/resolve/main/ggml-base.bin" -OutFile "ggml-base.bin"

# SmolLM2 360M (380MB)
Invoke-WebRequest -Uri "https://huggingface.co/HuggingFaceTB/SmolLM2-360M-Instruct-GGUF/resolve/main/smollm2-360m-instruct-q8_0.gguf" -OutFile "smollm2-360m-instruct-q8_0.gguf"

cd ..
```

### 3. 首次构建

```powershell
# Debug 构建（测试用）
cargo build

# Release 构建（发布用）
cargo build --release
```

---

## 开发任务

### 当前状态

macOS 端已完成：
- ✅ 核心流程（录音 → ASR → LLM → 剪贴板）
- ✅ 悬浮胶囊（NSVisualEffectView + Core Animation）
- ✅ 设置面板（Tauri Webview）
- ✅ 构建脚本

Windows 端已实现但需验证：
- 🔄 Windows 悬浮胶囊（WS_EX_LAYERED + Direct2D）
- 🔄 代码已写，需测试编译和运行

### 待验证/修复任务

#### 任务 1：验证 Windows 悬浮胶囊编译

```powershell
# 构建 echovoice-floating crate
cargo build -p echovoice-floating --release

# 检查是否有编译错误
```

**预期问题**：
- Direct2D 依赖可能需调整
- Windows API 调用可能需要特定 feature flag

**参考文件**：`crates/floating/src/windows.rs`

#### 任务 2：测试悬浮胶囊显示

```powershell
# 运行完整程序
cargo run --release

# 按下 F9 测试胶囊是否显示
```

**预期行为**：
1. 按住 F9 → 胶囊显示在屏幕底部中央
2. 显示"正在听" + 三个白点动画
3. 松开 F9 → 显示"思考中"
4. 完成后显示"✓ 已复制"

#### 任务 3：修复 Direct2D 渲染问题（如有）

如遇到 Direct2D 错误：

1. 检查 Windows 版本（需 Windows 10 1809+）
2. 安装 Windows SDK：
   ```powershell
   choco install -y windows-sdk-10.1
   ```
3. 更新 Cargo.toml 中的 Windows feature：
   ```toml
   [target.'cfg(target_os = "windows")'.dependencies]
   windows = { version = "0.52", features = [
       "Win32_Foundation",
       "Win32_Graphics_Direct2D",
       "Win32_Graphics_Direct2D_Common",
       "Win32_Graphics_Dxgi_Common",
       "Win32_Graphics_Gdi",
       "Win32_UI_WindowsAndMessaging",
       "Win32_System_LibraryLoader",
       "Win32_System_Performance",
   ]}
   ```

#### 任务 4：添加缺失的 Windows 剪贴板支持

当前 `src-tauri/src/main.rs` 中的 `copy_to_clipboard` 只有 macOS 实现：

```rust
#[tauri::command]
fn copy_to_clipboard(text: String) {
    #[cfg(target_os = "macos")]
    {
        use std::process::Command;
        let _ = Command::new("pbcopy")
            .arg(&text)
            .spawn();
    }

    // TODO: 添加 Windows 实现
    #[cfg(target_os = "windows")]
    {
        // 使用 clipboard-win crate 或 Win32 API
    }
}
```

**解决方案**：
1. 添加 `clipboard-win` 依赖到 `src-tauri/Cargo.toml`
2. 实现 Windows 剪贴板复制

---

## 与 macOS 端协作流程

### 1. Git 工作流

```powershell
# 创建 Windows 特性分支
git checkout -b windows-fix

# 完成修改后
git add -A
git commit -m "fix(windows): 修复 Direct2D 渲染问题"
git push origin windows-fix
```

### 2. 提交 PR

1. 访问 https://github.com/OpeNopEn2007/EchoVoice
2. 创建 Pull Request 从 `windows-fix` 到 `main`
3. 描述修改内容
4. macOS 端审查并合并

### 3. 同步更新

```powershell
# 拉取最新代码
git checkout main
git pull origin main
```

---

## 常见问题

### Q1: 编译时找不到 `stdio.h`

**原因**：缺少 Windows SDK 或 Clang 配置错误

**解决**：
```powershell
choco install -y visualstudio2022buildtools --package-parameters "--add Microsoft.VisualStudio.Workload.VCTools"
```

### Q2: whisper-rs 或 llama-cpp 编译失败

**原因**：C++ 编译器或 CMake 问题

**解决**：
```powershell
# 确保 cmake 在 PATH 中
[Environment]::SetEnvironmentVariable("Path", $env:Path + ";C:\Program Files\CMake\bin", "User")

# 重新构建
cargo clean
cargo build --release
```

### Q3: 悬浮胶囊不显示

**排查步骤**：
1. 检查窗口位置计算是否正确
2. 检查 Direct2D 设备是否创建成功
3. 添加调试输出查看错误

### Q4: 运行时崩溃

**排查**：
```powershell
# 使用 Debug 构建查看详细错误
cargo run

# 检查 panic 信息
$env:RUST_BACKTRACE=1
cargo run
```

---

## 关键文件位置

| 功能 | 文件路径 |
|------|----------|
| Windows 胶囊实现 | `crates/floating/src/windows.rs` |
| 主程序入口 | `src/main.rs` |
| Tauri 命令 | `src-tauri/src/main.rs` |
| 配置文件 | `crates/config/src/lib.rs` |
| 音频录制 | `crates/audio/src/lib.rs` |

---

## 快速检查清单

每次开发前确认：
- [ ] Rust 环境正常 (`rustc --version`)
- [ ] 模型文件已下载 (`ls models/`)
- [ ] 代码已同步 (`git pull origin main`)

每次提交前确认：
- [ ] 代码能编译 (`cargo build --release`)
- [ ] 功能正常（录音 → 识别 → 复制）
- [ ] 悬浮胶囊显示正常

---

## 联系 macOS 端

如遇到无法解决的问题：
1. 记录详细错误信息
2. 创建 GitHub Issue 并 @macOS 开发者
3. 附上完整的构建日志

---

*指南版本: 1.0*
*创建时间: 2026-03-13*
*适用平台: Windows 10/11*
