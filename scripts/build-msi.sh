#!/bin/bash

# EchoVoice Windows MSI 打包脚本
# 此脚本需在 Windows 环境（Git Bash 或 WSL）中运行

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_DIR="$(dirname "$SCRIPT_DIR")"
APP_NAME="EchoVoice"
APP_VERSION="${1:-0.1.0}"

cd "$PROJECT_DIR"

echo "========================================"
echo "EchoVoice Windows MSI 打包工具"
echo "版本: $APP_VERSION"
echo "========================================"
echo ""

# 检查是否在 Windows 环境
if ! command -v candle.exe &> /dev/null; then
    echo "错误: 未找到 WiX Toolset (candle.exe)"
    echo "请先安装 WiX Toolset: https://wixtoolset.org"
    echo ""
    echo "安装方式:"
    echo "  choco install wixtoolset"
    exit 1
fi

# Release 构建
echo "--- 步骤 1: Release 构建 ---"
cargo build --release
echo "✓ 构建完成"
echo ""

# 准备打包目录
echo "--- 步骤 2: 准备打包文件 ---"
PACKAGE_DIR="target/windows-package"
rm -rf "$PACKAGE_DIR"
mkdir -p "$PACKAGE_DIR"

# 复制可执行文件
cp "target/release/echovoice.exe" "$PACKAGE_DIR/"

# 复制依赖 DLL（使用 cargo-copy-deps 或手动复制）
echo "复制依赖 DLL..."
# 注意：实际使用时需要 cargo-copy-deps 或类似工具
echo "  (手动复制所需 DLL 到 $PACKAGE_DIR)"
echo ""

# 创建图标（如果有）
if [ -f "src-tauri/icons/icon.ico" ]; then
    cp "src-tauri/icons/icon.ico" "$PACKAGE_DIR/"
fi

echo "✓ 文件准备完成"
echo ""

# 编译 WiX 文件
echo "--- 步骤 3: 编译 WiX 文件 ---"
cd "$SCRIPT_DIR"

candle.exe -arch x64 -dSourceDir="$PACKAGE_DIR" -dVersion="$APP_VERSION" -out "$PACKAGE_DIR/echovoice.wixobj" echovoice.wxs
light.exe -ext WixUIExtension -ext WixUtilExtension -out "$PROJECT_DIR/target/${APP_NAME}-${APP_VERSION}.msi" "$PACKAGE_DIR/echovoice.wixobj"

cd "$PROJECT_DIR"
echo "✓ MSI 创建完成"
echo ""

# 清理
echo "--- 步骤 4: 清理 ---"
rm -rf "$PACKAGE_DIR"
echo "✓ 清理完成"
echo ""

echo "========================================"
echo "打包完成！"
echo "========================================"
echo ""
echo "输出文件: target/${APP_NAME}-${APP_VERSION}.msi"
echo ""
ls -lh "target/${APP_NAME}-${APP_VERSION}.msi"
