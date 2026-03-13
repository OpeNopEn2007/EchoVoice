# EchoVoice 模型下载脚本 (Windows PowerShell)
# 自动下载所需的 AI 模型

param(
    [string]$ModelsDir = "./models"
)

$ErrorActionPreference = "Stop"

# 创建模型目录
New-Item -ItemType Directory -Force -Path $ModelsDir | Out-Null

Write-Host "========================================" -ForegroundColor Cyan
Write-Host "EchoVoice 模型下载工具" -ForegroundColor Cyan
Write-Host "========================================" -ForegroundColor Cyan
Write-Host ""

# 下载函数
function Download-Model {
    param(
        [string]$Url,
        [string]$Output,
        [string]$Name
    )

    if (Test-Path $Output) {
        Write-Host "✓ $Name 已存在，跳过下载" -ForegroundColor Green
        return $true
    }

    Write-Host "↓ 正在下载 $Name..." -ForegroundColor Yellow
    Write-Host "   来源: $Url"
    Write-Host "   目标: $Output"
    Write-Host ""

    try {
        Invoke-WebRequest -Uri $Url -OutFile $Output -UseBasicParsing
        Write-Host "✓ $Name 下载完成" -ForegroundColor Green
        return $true
    }
    catch {
        Write-Host "✗ $Name 下载失败: $_" -ForegroundColor Red
        return $false
    }
}

# 下载 Whisper 模型
Write-Host "--- ASR 模型 (Whisper) ---" -ForegroundColor Gray
$whisperResult = Download-Model `
    -Url "https://huggingface.co/ggerganov/whisper.cpp/resolve/main/ggml-base.bin" `
    -Output "$ModelsDir\ggml-base.bin" `
    -Name "Whisper Base"

Write-Host ""

# 下载 LLM 模型
Write-Host "--- LLM 模型 (SmolLM2) ---" -ForegroundColor Gray
$llmResult = Download-Model `
    -Url "https://huggingface.co/HuggingFaceTB/SmolLM2-360M-Instruct-GGUF/resolve/main/smollm2-360m-instruct-q8_0.gguf" `
    -Output "$ModelsDir\smollm2-360m-instruct-q8_0.gguf" `
    -Name "SmolLM2 360M"

Write-Host ""
Write-Host "========================================" -ForegroundColor Cyan
Write-Host "模型下载完成！" -ForegroundColor Cyan
Write-Host "========================================" -ForegroundColor Cyan
Write-Host ""
Write-Host "模型目录: $ModelsDir"
Write-Host ""

# 显示文件列表
Get-ChildItem $ModelsDir | ForEach-Object {
    $size = "{0:N2} MB" -f ($_.Length / 1MB)
    Write-Host "$($_.Name) ($size)"
}

Write-Host ""
Write-Host "现在可以运行: cargo run --release"
