#!/bin/bash

# EchoVoice 构建脚本
# 支持 macOS 和 Linux 构建

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_DIR="$(dirname "$SCRIPT_DIR")"

cd "$PROJECT_DIR"

echo "========================================"
echo "EchoVoice 构建工具"
echo "========================================"
echo ""

# 检查 Rust
if ! command -v rustc &> /dev/null; then
    echo "错误: 未找到 Rust，请先安装: https://rustup.rs"
    exit 1
fi

echo "Rust 版本: $(rustc --version)"
echo ""

# 检查模型
if [ ! -d "$PROJECT_DIR/models" ] || [ -z "$(ls -A "$PROJECT_DIR/models" 2>/dev/null)" ]; then
    echo "⚠ 警告: models/ 目录为空"
    echo "是否下载模型? (y/n)"
    read -r response
    if [[ "$response" =~ ^([yY][eE][sS]|[yY])$ ]]; then
        bash "$SCRIPT_DIR/download-models.sh"
    fi
fi

# 构建选项
BUILD_TYPE="${1:-release}"

echo "--- 构建配置 ---"
echo "类型: $BUILD_TYPE"
echo ""

if [ "$BUILD_TYPE" == "release" ]; then
    echo "开始 Release 构建..."
    cargo build --release
    echo ""
    echo "✓ 构建完成"
    echo "可执行文件: ./target/release/echovoice"
    echo ""
    ls -lh ./target/release/echovoice
else
    echo "开始 Debug 构建..."
    cargo build
    echo ""
    echo "✓ 构建完成"
    echo "可执行文件: ./target/debug/echovoice"
fi

echo ""
echo "运行: ./target/$BUILD_TYPE/echovoice"
