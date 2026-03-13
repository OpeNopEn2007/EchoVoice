#!/bin/bash

# EchoVoice 模型下载脚本
# 自动下载所需的 AI 模型

set -e

MODELS_DIR="${1:-./models}"
mkdir -p "$MODELS_DIR"

echo "========================================"
echo "EchoVoice 模型下载工具"
echo "========================================"
echo ""

# 颜色输出
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# 检查 curl 或 wget
if command -v curl &> /dev/null; then
    DOWNLOAD_CMD="curl -L -o"
elif command -v wget &> /dev/null; then
    DOWNLOAD_CMD="wget -O"
else
    echo -e "${RED}错误: 需要 curl 或 wget${NC}"
    exit 1
fi

# 下载函数
download_model() {
    local url=$1
    local output=$2
    local name=$3

    if [ -f "$output" ]; then
        echo -e "${GREEN}✓${NC} $name 已存在，跳过下载"
        return 0
    fi

    echo -e "${YELLOW}↓${NC} 正在下载 $name..."
    echo "   来源: $url"
    echo "   目标: $output"
    echo ""

    if $DOWNLOAD_CMD "$output" "$url"; then
        echo -e "${GREEN}✓${NC} $name 下载完成"
        return 0
    else
        echo -e "${RED}✗${NC} $name 下载失败"
        return 1
    fi
}

# 下载 Whisper 模型
echo "--- ASR 模型 (Whisper) ---"
download_model \
    "https://huggingface.co/ggerganov/whisper.cpp/resolve/main/ggml-base.bin" \
    "$MODELS_DIR/ggml-base.bin" \
    "Whisper Base"

echo ""

# 下载 LLM 模型
echo "--- LLM 模型 (SmolLM2) ---"
download_model \
    "https://huggingface.co/HuggingFaceTB/SmolLM2-360M-Instruct-GGUF/resolve/main/smollm2-360m-instruct-q8_0.gguf" \
    "$MODELS_DIR/smollm2-360m-instruct-q8_0.gguf" \
    "SmolLM2 360M"

echo ""
echo "========================================"
echo "模型下载完成！"
echo "========================================"
echo ""
echo "模型目录: $MODELS_DIR"
echo ""
ls -lh "$MODELS_DIR"
echo ""
echo "现在可以运行: cargo run --release"
