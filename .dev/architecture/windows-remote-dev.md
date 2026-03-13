# Windows 远程开发连接方案

## 方案概述

使用 **OpenSSH + Rust 工具链** 实现远程 Windows 开发环境接入。

---

## 方案一：OpenSSH 直连（推荐）

### 1. Windows 端配置（用户执行，约 5 分钟）

以管理员身份打开 PowerShell：

```powershell
# 安装 OpenSSH 服务器
Add-WindowsCapability -Online -Name OpenSSH.Server~~~~0.0.1.0

# 启动并设置开机自启
Start-Service sshd
Set-Service -Name sshd -StartupType 'Automatic'

# 配置防火墙
New-NetFirewallRule -Name 'OpenSSH-Server-In-TCP' -DisplayName 'OpenSSH Server (sshd)' -Enabled True -Direction Inbound -Protocol TCP -Action Allow -LocalPort 22
```

### 2. 创建开发用户

```powershell
# 创建专用开发账户（可选，也可使用现有账户）
$Password = Read-Host -AsSecureString -Prompt "输入密码"
New-LocalUser -Name "echodev" -Password $Password -FullName "EchoVoice Developer" -Description "Remote dev account"
Add-LocalGroupMember -Group "Administrators" -Member "echodev"
```

### 3. 公钥认证配置（更安全）

```powershell
# 在 Windows 上创建 .ssh 目录
$sshDir = "C:\Users\$env:USERNAME\.ssh"
New-Item -ItemType Directory -Force -Path $sshDir

# 用户将公钥内容粘贴到 authorized_keys 文件
# 公钥格式: ssh-ed25519 AAAAC3NzaC... comment
```

### 4. 提供连接信息给我

用户需要提供：
- Windows 机器的公网 IP 或域名
- SSH 端口（默认 22，如有 NAT 映射请提供外部端口）
- 用户名和密码（或私钥）

---

## 方案二：Tailscale/ZeroTier（内网穿透）

如果 Windows 机器没有公网 IP，使用 Tailscale 组建虚拟局域网：

### Windows 端安装

```powershell
# 下载并安装 Tailscale
winget install tailscale.tailscale

# 启动并登录
tailscale up
```

### 获取 Tailscale IP

```powershell
tailscale ip -4
# 输出类似: 100.x.x.x
```

将此 IP 提供给我即可通过 SSH 连接。

---

## 方案三：frp 内网穿透（用户有公网服务器）

如果用户有自己的公网服务器，可以用 frp 将 Windows 的 SSH 端口映射出去：

### frps.ini（服务端）
```ini
[common]
bind_port = 7000
token = your_secure_token
```

### frpc.ini（Windows 端）
```ini
[common]
server_addr = your.server.ip
server_port = 7000
token = your_secure_token

[ssh]
type = tcp
local_ip = 127.0.0.1
local_port = 22
remote_port = 6022
```

我通过 `ssh user@your.server.ip -p 6022` 连接。

---

## 开发环境初始化脚本

连接成功后，我在 Windows 上执行的初始化：

```powershell
# 安装 Chocolatey（包管理器）
Set-ExecutionPolicy Bypass -Scope Process -Force
[System.Net.ServicePointManager]::SecurityProtocol = [System.Net.ServicePointManager]::SecurityProtocol -bor 3072
iex ((New-Object System.Net.WebClient).DownloadString('https://community.chocolatey.org/install.ps1'))

# 安装开发工具
choco install -y git
choco install -y rust
choco install -y llvm          # bindgen 需要
choco install -y vscode        # 可选

# 重启 PowerShell 使环境变量生效
```

---

## 代码同步方案

### 方式 A：直接克隆（推荐）

```powershell
git clone https://github.com/OpeNopEn2007/EchoVoice.git
cd EchoVoice
```

### 方式 B：VS Code Remote

如果用户使用 VS Code，我可以指导配置 Remote-SSH 插件，获得完整的 IDE 体验。

---

## 安全建议

1. **使用密钥认证而非密码**
2. **限制 SSH 来源 IP**（如有固定 IP）
3. **使用非标准 SSH 端口**（如 2222 代替 22）
4. **开启 Windows 防火墙**
5. **开发完成后关闭 SSH 服务**

---

## 快速检查清单

用户准备就绪的标志：
- [ ] Windows 机器可以联网
- [ ] 已安装 OpenSSH 服务器
- [ ] 能提供 IP:端口 和登录凭证
- [ ] 机器有至少 4GB 空闲内存（Rust 编译需要）
- [ ] 磁盘剩余空间 > 5GB

---

## 备选：TeamViewer/向日葵远程桌面

如果 SSH 方案配置困难，也可以使用远程桌面软件：
- 用户开启远程桌面软件
- 我通过图形界面操作
- 在 Windows 上安装 VS Code + Remote WSL（如适用）

但效率较低，不推荐用于开发。

---

*方案版本: 1.0*
*创建时间: 2026-03-13*
