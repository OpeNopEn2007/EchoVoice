#!/bin/bash

# EchoVoice macOS DMG 打包脚本

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_DIR="$(dirname "$SCRIPT_DIR")"
APP_NAME="EchoVoice"
APP_VERSION="${1:-0.1.0}"
BUNDLE_ID="com.echovoice.app"

cd "$PROJECT_DIR"

echo "========================================"
echo "EchoVoice macOS 打包工具"
echo "版本: $APP_VERSION"
echo "========================================"
echo ""

# 检查依赖
if ! command -v cargo &> /dev/null; then
    echo "错误: 未找到 cargo"
    exit 1
fi

# Release 构建
echo "--- 步骤 1: Release 构建 ---"
cargo build --release
echo "✓ 构建完成"
echo ""

# 创建应用目录结构
echo "--- 步骤 2: 创建应用包 ---"
APP_DIR="target/$APP_NAME.app"
rm -rf "$APP_DIR"
mkdir -p "$APP_DIR/Contents/MacOS"
mkdir -p "$APP_DIR/Contents/Resources"

# 复制可执行文件
cp target/release/echovoice "$APP_DIR/Contents/MacOS/"

# 创建 Info.plist
cat > "$APP_DIR/Contents/Info.plist" << EOF
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>CFBundleDevelopmentRegion</key>
    <string>en</string>
    <key>CFBundleExecutable</key>
    <string>echovoice</string>
    <key>CFBundleIconFile</key>
    <string>AppIcon</string>
    <key>CFBundleIdentifier</key>
    <string>$BUNDLE_ID</string>
    <key>CFBundleInfoDictionaryVersion</key>
    <string>6.0</string>
    <key>CFBundleName</key>
    <string>$APP_NAME</string>
    <key>CFBundlePackageType</key>
    <string>APPL</string>
    <key>CFBundleShortVersionString</key>
    <string>$APP_VERSION</string>
    <key>CFBundleVersion</key>
    <string>1</string>
    <key>LSMinimumSystemVersion</key>
    <string>11.0</string>
    <key>LSUIElement</key>
    <true/>
    <key>NSHighResolutionCapable</key>
    <true/>
    <key>NSMicrophoneUsageDescription</key>
    <string>EchoVoice 需要麦克风权限来录制您的语音输入</string>
</dict>
</plist>
EOF

# 复制图标（如果有）
if [ -f "src-tauri/icons/icon.icns" ]; then
    cp "src-tauri/icons/icon.icns" "$APP_DIR/Contents/Resources/AppIcon.icns"
fi

# 复制模型文件
if [ -d "models" ]; then
    echo "--- 复制模型文件 ---"
    mkdir -p "$APP_DIR/Contents/Resources/models"
    cp models/ggml-base.bin "$APP_DIR/Contents/Resources/models/" 2>/dev/null || echo "警告: 未找到 ggml-base.bin"
    cp models/smollm2-360m-q8.gguf "$APP_DIR/Contents/Resources/models/" 2>/dev/null || echo "警告: 未找到 smollm2-360m-q8.gguf"
    du -sh "$APP_DIR/Contents/Resources/models" 2>/dev/null || true
    echo "✓ 模型文件复制完成"
else
    echo "警告: 未找到 models 目录"
fi

echo "✓ 应用包创建完成"
echo ""

# 创建 DMG
echo "--- 步骤 3: 创建 DMG ---"
DMG_NAME="${APP_NAME}-${APP_VERSION}.dmg"
DMG_PATH="target/$DMG_NAME"

# 使用 create-dmg（如果已安装）
if command -v create-dmg &> /dev/null; then
    create-dmg \
        --volname "$APP_NAME $APP_VERSION" \
        --window-pos 200 120 \
        --window-size 600 400 \
        --icon-size 100 \
        --app-drop-link 450 185 \
        --icon "$APP_NAME.app" 150 185 \
        "$DMG_PATH" \
        "$APP_DIR"
else
    # 简单 DMG 创建（包含模型需要 600MB 空间）
    TEMP_DMG="target/temp.dmg"
    hdiutil create -srcfolder "$APP_DIR" -volname "$APP_NAME" -fs HFS+ \
        -format UDRW -size 600m "$TEMP_DMG"
    hdiutil convert "$TEMP_DMG" -format UDZO -o "$DMG_PATH"
    rm "$TEMP_DMG"
fi

echo "✓ DMG 创建完成"
echo ""

# 签名（可选）
if command -v codesign &> /dev/null && [ -n "$CODESIGN_IDENTITY" ]; then
    echo "--- 步骤 4: 代码签名 ---"
    codesign --force --deep --sign "$CODESIGN_IDENTITY" "$DMG_PATH"
    echo "✓ 签名完成"
    echo ""
fi

echo "========================================"
echo "打包完成！"
echo "========================================"
echo ""
echo "输出文件: $DMG_PATH"
echo "应用路径: $APP_DIR"
echo ""
ls -lh "$DMG_PATH"
