# EchoVoice 安装说明

## 系统要求

### macOS
- macOS 11.0 (Big Sur) 或更高版本
- 4GB 可用内存
- 500MB 磁盘空间
- Apple Silicon 或 Intel 处理器

### Windows
- Windows 10 版本 2004 或更高 / Windows 11
- 4GB 可用内存
- 500MB 磁盘空间

---

## 安装步骤

### macOS

#### 方式一：DMG 安装包（推荐）

1. 下载 `EchoVoice-x.x.x.dmg`
2. 双击打开 DMG 文件
3. 将 EchoVoice 拖入 Applications 文件夹
4. 首次运行：
   - 打开 **系统设置** → **隐私与安全性**
   - 点击"仍要打开"允许运行
   - 授予**麦克风**和**辅助功能**权限

#### 方式二：Homebrew

```bash
brew tap openopen/echovoice
brew install --cask echovoice
```

#### 方式三：手动构建

```bash
# 1. 克隆仓库
git clone https://github.com/OpeNopEn2007/EchoVoice.git
cd EchoVoice

# 2. 安装依赖
# macOS 需要 Xcode Command Line Tools
xcode-select --install

# 3. 下载模型
mkdir -p models
cd models
# 下载 Whisper 模型
curl -O https://huggingface.co/ggerganov/whisper.cpp/resolve/main/ggml-base.bin
# 下载 SmolLM2 模型
curl -O https://huggingface.co/HuggingFaceTB/SmolLM2-360M-Instruct-GGUF/resolve/main/smollm2-360m-instruct-q8_0.gguf
cd ..

# 4. 构建并运行
cargo build --release
./target/release/echovoice
```

---

### Windows

#### 方式一：MSI 安装包（推荐）

1. 下载 `EchoVoice-x.x.x.msi`
2. 双击运行安装程序
3. 按向导完成安装
4. 首次运行授予**麦克风**权限

#### 方式二：便携版

1. 下载 `EchoVoice-x.x.x-windows.zip`
2. 解压到任意目录
3. 运行 `echovoice.exe`

#### 方式三：手动构建

```powershell
# 1. 安装依赖
# 安装 Rust (https://rustup.rs)
# 安装 LLVM (https://releases.llvm.org)

# 2. 克隆仓库
git clone https://github.com/OpeNopEn2007/EchoVoice.git
cd EchoVoice

# 3. 下载模型（同 macOS）

# 4. 构建
cargo build --release
.\target\release\echovoice.exe
```

---

## 模型下载

程序需要以下模型文件放入 `models/` 目录：

| 模型 | 文件名 | 大小 | 下载地址 |
|------|--------|------|----------|
| Whisper Base | `ggml-base.bin` | 148MB | [下载](https://huggingface.co/ggerganov/whisper.cpp/resolve/main/ggml-base.bin) |
| SmolLM2 360M | `smollm2-360m-instruct-q8_0.gguf` | 380MB | [下载](https://huggingface.co/HuggingFaceTB/SmolLM2-360M-Instruct-GGUF/resolve/main/smollm2-360m-instruct-q8_0.gguf) |

### 自动下载脚本

**macOS/Linux:**
```bash
./scripts/download-models.sh
```

**Windows:**
```powershell
.\scripts\download-models.ps1
```

---

## 首次使用

1. **启动程序**：双击应用图标或在终端运行
2. **配置快捷键**：
   - 默认：按住 `F9` 录音，松开停止
   - 备选：`CapsLock`
3. **测试录音**：
   - 按住 F9 → 听到提示音 → 说话 → 松开 F9
   - 看到"已复制"提示后，文本已在剪贴板
4. **调整设置**：右键托盘图标 → 设置

---

## 常见问题

### macOS

**Q: 提示"无法打开，因为无法验证开发者"**
A: 系统设置 → 隐私与安全性 → 仍要打开

**Q: 无法录音**
A: 系统设置 → 隐私与安全性 → 麦克风 → 允许 EchoVoice

**Q: 无法自动粘贴**
A: 系统设置 → 隐私与安全性 → 辅助功能 → 允许 EchoVoice

### Windows

**Q: 提示缺少 DLL**
A: 安装 [Visual C++ Redistributable](https://aka.ms/vs/17/release/vc_redist.x64.exe)

**Q: 杀毒软件拦截**
A: 将 EchoVoice 添加到白名单

---

## 卸载

### macOS
```bash
rm -rf /Applications/EchoVoice.app
rm -rf ~/Library/Application\ Support/echovoice
```

### Windows
通过「设置」→「应用」→「EchoVoice」→「卸载"

---

## 更新

程序会自动检查更新。或手动下载新版本覆盖安装。

---

## 技术支持

- GitHub Issues: https://github.com/OpeNopEn2007/EchoVoice/issues
- 邮箱: support@echovoice.app

---

*版本: 0.1.0*
*更新日期: 2026-03-13*
