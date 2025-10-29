#!/usr/bin/env bash

# ============================================================================
# 快速构建脚本（开发模式）
# ============================================================================
# 功能：快速构建并运行游戏（开发模式，启用动态链接）
# 用途：日常开发测试

set -euo pipefail

# Shell 检测
detect_shell() {
  if [ -n "${BASH_VERSION:-}" ]; then echo bash; return; fi
  if [ -n "${ZSH_VERSION:-}" ]; then echo zsh; return; fi
  echo sh
}

shell=$(detect_shell)
case "$shell" in
  bash) : ;;
  zsh)  emulate -L sh; setopt SH_WORD_SPLIT ;;
  *)    echo "错误: 请在 bash 或 zsh 中运行此脚本" >&2; exit 1 ;;
esac

# 配置
WORKSPACE_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "${WORKSPACE_ROOT}"

echo "========================================"
echo "  快速构建（开发模式）"
echo "========================================"

# 解析参数
RUN_GAME=true
BUILD_FEATURES=""

while [[ $# -gt 0 ]]; do
    case $1 in
        --no-run)
            RUN_GAME=false
            shift
            ;;
        --inspector)
            BUILD_FEATURES="--features dev,inspector"
            shift
            ;;
        *)
            echo "未知参数: $1"
            echo "用法: $0 [--no-run] [--inspector]"
            exit 1
            ;;
    esac
done

# 默认特性
if [ -z "$BUILD_FEATURES" ]; then
    BUILD_FEATURES="--features dev"
fi

echo "构建配置: $BUILD_FEATURES"
echo ""

# 构建
echo "[1/2] 编译中..."
if cargo build $BUILD_FEATURES; then
    echo "✓ 构建成功"
else
    echo "✗ 构建失败"
    exit 1
fi

# 运行
if [ "$RUN_GAME" = true ]; then
    echo ""
    echo "[2/2] 启动游戏..."
    echo "========================================"
    cargo run $BUILD_FEATURES
else
    echo ""
    echo "========================================"
    echo "构建完成（未运行）"
    echo "手动运行: cargo run $BUILD_FEATURES"
fi
