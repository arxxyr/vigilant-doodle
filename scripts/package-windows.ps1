# ============================================================================
# Windows 打包脚本
# ============================================================================
# 功能：构建并打包游戏的 Windows 发行版本
# 输出：vigilant_doodle_VERSION_windows.zip

param(
    [string]$Version = "dev"
)

# 错误处理
$ErrorActionPreference = "Stop"

# Shell 检测
if (-not $PSVersionTable) {
    Write-Error "请在 PowerShell 中运行"
    exit 1
}

# ----------------------------------------------------------------------------
# 配置
# ----------------------------------------------------------------------------
$GAME_NAME = "vigilant-doodle"  # 二进制文件名（与 Cargo.toml 中的 bin name 一致）
$WORKSPACE_ROOT = Split-Path -Parent $PSScriptRoot
$BUILD_DIR = Join-Path $WORKSPACE_ROOT "bin"
$ARCHIVE_NAME = "${GAME_NAME}_${Version}_windows.zip"

Write-Host "========================================"
Write-Host "  Windows 打包脚本"
Write-Host "========================================"
Write-Host "游戏名称: $GAME_NAME"
Write-Host "版本:     $Version"
Write-Host "工作目录: $WORKSPACE_ROOT"
Write-Host "构建目录: $BUILD_DIR"
Write-Host ""

# ----------------------------------------------------------------------------
# 检查依赖
# ----------------------------------------------------------------------------
function Check-Dependencies {
    Write-Host "[1/5] 检查依赖..."

    # 检查 Rust 工具链
    if (-not (Get-Command cargo -ErrorAction SilentlyContinue)) {
        Write-Error "错误: 未找到 cargo (Rust 工具链)`n请安装: https://rustup.rs/"
        exit 1
    }

    Write-Host "✓ 所有依赖已满足"
}

# ----------------------------------------------------------------------------
# 清理旧构建
# ----------------------------------------------------------------------------
function Clean-Build {
    Write-Host ""
    Write-Host "[2/5] 清理旧构建..."

    if (Test-Path $BUILD_DIR) {
        Remove-Item -Recurse -Force $BUILD_DIR
        Write-Host "✓ 已清理: $BUILD_DIR"
    }

    New-Item -ItemType Directory -Force -Path $BUILD_DIR | Out-Null
}

# ----------------------------------------------------------------------------
# 构建 Release 版本
# ----------------------------------------------------------------------------
function Build-Release {
    Write-Host ""
    Write-Host "[3/5] 构建 Release 版本..."
    Write-Host "这可能需要几分钟..."
    Write-Host ""

    Set-Location $WORKSPACE_ROOT

    # 使用 nightly 工具链 + 完全优化
    cargo build --release

    if ($LASTEXITCODE -ne 0) {
        Write-Error "构建失败"
        exit 1
    }

    Write-Host ""
    Write-Host "✓ 构建完成"
}

# ----------------------------------------------------------------------------
# 准备发行包
# ----------------------------------------------------------------------------
function Prepare-Package {
    Write-Host ""
    Write-Host "[4/5] 准备发行包..."

    $release_bin = Join-Path $WORKSPACE_ROOT "target\release\${GAME_NAME}.exe"

    # 检查二进制文件是否存在
    if (-not (Test-Path $release_bin)) {
        Write-Error "找不到构建产物: $release_bin"
        exit 1
    }

    # 复制文件到构建目录
    Write-Host "  - 复制游戏文件..."
    Copy-Item $release_bin $BUILD_DIR

    # 复制资源文件
    $assets_dir = Join-Path $WORKSPACE_ROOT "assets"
    if (Test-Path $assets_dir) {
        Copy-Item -Recurse $assets_dir $BUILD_DIR
        Write-Host "  - 已复制: assets\"
    }

    $credits_dir = Join-Path $WORKSPACE_ROOT "credits"
    if (Test-Path $credits_dir) {
        Copy-Item -Recurse $credits_dir $BUILD_DIR
        Write-Host "  - 已复制: credits\"
    }

    # 创建启动脚本
    $bat_content = @"
@echo off
cd /d "%~dp0"
start "" vigilant-doodle.exe %*
"@
    Set-Content -Path (Join-Path $BUILD_DIR "run.bat") -Value $bat_content -Encoding ASCII
    Write-Host "  - 已创建: run.bat"

    # 创建 README
    $readme_content = @"
Vigilant Doodle - Windows 发行版
=================================

版本: $Version
构建日期: $(Get-Date -Format 'yyyy-MM-dd HH:mm:ss')

运行方法
--------
双击运行:
  run.bat

或直接运行可执行文件:
  vigilant-doodle.exe

系统要求
--------
- Windows 10/11 (x64)
- DirectX 12
- 4GB RAM

许可证
------
MIT License

问题反馈
--------
https://github.com/arxxyr/vigilant-doodle/issues
"@
    Set-Content -Path (Join-Path $BUILD_DIR "README.txt") -Value $readme_content -Encoding UTF8
    Write-Host "  - 已创建: README.txt"

    Write-Host "✓ 发行包准备完成"
}

# ----------------------------------------------------------------------------
# 打包
# ----------------------------------------------------------------------------
function Create-Archive {
    Write-Host ""
    Write-Host "[5/5] 创建压缩包..."

    $archive_path = Join-Path $WORKSPACE_ROOT $ARCHIVE_NAME

    # 删除旧压缩包
    if (Test-Path $archive_path) {
        Remove-Item $archive_path
    }

    # 创建 ZIP 压缩包（仅打包必要文件，不包含 bin 目录本身）
    $files_to_pack = @(
        "${GAME_NAME}.exe",
        "run.bat",
        "README.txt",
        "assets",
        "credits"
    )

    $archive_path = Join-Path $BUILD_DIR $ARCHIVE_NAME

    # 删除旧压缩包
    if (Test-Path $archive_path) {
        Remove-Item $archive_path
    }

    # 压缩文件
    Push-Location $BUILD_DIR
    Compress-Archive -Path $files_to_pack -DestinationPath $ARCHIVE_NAME
    Pop-Location

    $size_mb = [math]::Round((Get-Item $archive_path).Length / 1MB, 2)

    Write-Host ""
    Write-Host "========================================"
    Write-Host "  打包完成！"
    Write-Host "========================================"
    Write-Host "输出文件: $archive_path"
    Write-Host "文件大小: ${size_mb} MB"
    Write-Host ""
    Write-Host "测试运行:"
    Write-Host "  1. cd bin"
    Write-Host "  2. 解压 $ARCHIVE_NAME"
    Write-Host "  3. 双击 run.bat"
    Write-Host "========================================"
}

# ----------------------------------------------------------------------------
# 主流程
# ----------------------------------------------------------------------------
function Main {
    Check-Dependencies
    Clean-Build
    Build-Release
    Prepare-Package
    Create-Archive
}

Main
