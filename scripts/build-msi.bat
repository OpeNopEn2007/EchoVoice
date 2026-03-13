@echo off
REM EchoVoice Windows MSI 打包脚本
REM 此脚本需在 Windows PowerShell 或 CMD 中运行

set APP_NAME=EchoVoice
set APP_VERSION=0.1.0
set SCRIPT_DIR=%~dp0
set PROJECT_DIR=%SCRIPT_DIR%..

echo ========================================
echo EchoVoice Windows MSI 打包工具
echo 版本: %APP_VERSION%
echo ========================================
echo.

REM 检查 WiX Toolset
where candle.exe >nul 2>&1
if %errorlevel% neq 0 (
    echo 错误: 未找到 WiX Toolset (candle.exe)
    echo 请先安装 WiX Toolset: https://wixtoolset.org
    echo.
    echo 安装方式:
    echo   choco install wixtoolset
    exit /b 1
)

REM Release 构建
echo --- 步骤 1: Release 构建 ---
cd %PROJECT_DIR%
cargo build --release
if %errorlevel% neq 0 (
    echo 错误: 构建失败
    exit /b 1
)
echo [OK] 构建完成
echo.

REM 准备打包目录
echo --- 步骤 2: 准备打包文件 ---
set PACKAGE_DIR=target\windows-package
if exist %PACKAGE_DIR% rmdir /s /q %PACKAGE_DIR%
mkdir %PACKAGE_DIR%

REM 复制可执行文件
copy "target\release\echovoice.exe" "%PACKAGE_DIR%\"

REM 复制依赖 DLL
echo 复制依赖 DLL...
echo   (实际使用时需要 cargo-copy-deps 或类似工具)

REM 创建图标
if exist "src-tauri\icons\icon.ico" (
    copy "src-tauri\icons\icon.ico" "%PACKAGE_DIR%\"
)

echo [OK] 文件准备完成
echo.

REM 编译 WiX 文件
echo --- 步骤 3: 编译 WiX 文件 ---
cd %SCRIPT_DIR%
candle.exe -arch x64 -dSourceDir="%PROJECT_DIR%\%PACKAGE_DIR%" -dVersion="%APP_VERSION%" -out "%PROJECT_DIR%\%PACKAGE_DIR%\echovoice.wixobj" echovoice.wxs
if %errorlevel% neq 0 (
    echo 错误: WiX 编译失败
    exit /b 1
)

light.exe -ext WixUIExtension -ext WixUtilExtension -out "%PROJECT_DIR%\target\%APP_NAME%-%APP_VERSION%.msi" "%PROJECT_DIR%\%PACKAGE_DIR%\echovoice.wixobj"
if %errorlevel% neq 0 (
    echo 错误: MSI 创建失败
    exit /b 1
)

cd %PROJECT_DIR%
echo [OK] MSI 创建完成
echo.

REM 清理
echo --- 步骤 4: 清理 ---
if exist %PACKAGE_DIR% rmdir /s /q %PACKAGE_DIR%
echo [OK] 清理完成
echo.

echo ========================================
echo 打包完成！
echo ========================================
echo.
echo 输出文件: target\%APP_NAME%-%APP_VERSION%.msi
echo.
dir "target\%APP_NAME%-%APP_VERSION%.msi"

pause
