#!/usr/bin/env bash

# ============================================================================
# Linux 打包脚本
# ============================================================================
# 功能：构建并打包游戏的 Linux 发行版本
# 输出：vigilant_doodle_VERSION_linux.tar.gz

set -euo pipefail

# ----------------------------------------------------------------------------
# Shell 检测
# ----------------------------------------------------------------------------
detect_shell() {
  if [ -n "${BASH_VERSION:-}" ]; then echo bash; return; fi
  if [ -n "${ZSH_VERSION:-}" ]; then echo zsh; return; fi
  s=$(ps -p $$ -o comm= 2>/dev/null || echo "")
  case "$s" in
    bash) echo bash ;;
    zsh)  echo zsh  ;;
    *)    echo sh   ;;
  esac
}

shell=$(detect_shell)
case "$shell" in
  bash) : ;;
  zsh)  emulate -L sh; setopt SH_WORD_SPLIT ;;
  *)    echo "错误: 请在 bash 或 zsh 中运行此脚本" >&2; exit 1 ;;
esac

# ----------------------------------------------------------------------------
# 配置
# ----------------------------------------------------------------------------
GAME_NAME="vigilant-doodle"  # 二进制文件名（与 Cargo.toml 中的 bin name 一致）
WORKSPACE_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
BUILD_DIR="${WORKSPACE_ROOT}/bin"
VERSION="${1:-dev}"

echo "========================================"
echo "  Linux 打包脚本"
echo "========================================"
echo "游戏名称: ${GAME_NAME}"
echo "版本:     ${VERSION}"
echo "工作目录: ${WORKSPACE_ROOT}"
echo "构建目录: ${BUILD_DIR}"
echo ""

# ----------------------------------------------------------------------------
# 检查依赖
# ----------------------------------------------------------------------------
check_dependencies() {
    echo "[1/5] 检查依赖..."

    local missing_deps=()

    # 检查 Rust 工具链
    if ! command -v cargo >/dev/null 2>&1; then
        missing_deps+=("cargo (Rust 工具链)")
    fi

    # 检查系统依赖（编译需要）
    local sys_deps=(
        "pkg-config"
        "libx11-dev"
        "libasound2-dev"
        "libudev-dev"
    )

    for dep in "${sys_deps[@]}"; do
        # 使用 dpkg-query 检查包是否已安装（更可靠）
        if ! dpkg-query -W "${dep}" >/dev/null 2>&1; then
            missing_deps+=("${dep}")
        fi
    done

    if [ ${#missing_deps[@]} -gt 0 ]; then
        echo "错误: 缺少以下依赖:"
        for dep in "${missing_deps[@]}"; do
            echo "  - ${dep}"
        done
        echo ""
        echo "安装命令:"
        echo "  sudo apt-get update"
        echo "  sudo apt-get install pkg-config libx11-dev libasound2-dev libudev-dev"
        exit 1
    fi

    echo "✓ 所有依赖已满足"
}

# ----------------------------------------------------------------------------
# 清理旧构建
# ----------------------------------------------------------------------------
clean_build() {
    echo ""
    echo "[2/5] 清理旧构建..."

    if [ -d "${BUILD_DIR}" ]; then
        rm -rf "${BUILD_DIR}"
        echo "✓ 已清理: ${BUILD_DIR}"
    fi

    mkdir -p "${BUILD_DIR}"
}

# ----------------------------------------------------------------------------
# 构建 Release 版本
# ----------------------------------------------------------------------------
build_release() {
    echo ""
    echo "[3/5] 构建 Release 版本..."
    echo "这可能需要几分钟..."
    echo ""

    cd "${WORKSPACE_ROOT}"

    # 使用 nightly 工具链 + 完全优化
    cargo build --release

    echo ""
    echo "✓ 构建完成"
}

# ----------------------------------------------------------------------------
# 准备发行包
# ----------------------------------------------------------------------------
prepare_package() {
    echo ""
    echo "[4/5] 准备发行包..."

    local release_bin="${WORKSPACE_ROOT}/target/release/${GAME_NAME}"

    # 检查二进制文件是否存在
    if [ ! -f "${release_bin}" ]; then
        echo "错误: 找不到构建产物: ${release_bin}" >&2
        exit 1
    fi

    # Strip 符号表（减小体积）
    echo "  - 优化二进制文件（strip）..."
    strip "${release_bin}"

    # 设置可执行权限
    chmod +x "${release_bin}"

    # 复制文件到构建目录
    echo "  - 复制游戏文件..."
    cp "${release_bin}" "${BUILD_DIR}/"

    # 复制资源文件
    if [ -d "${WORKSPACE_ROOT}/assets" ]; then
        cp -r "${WORKSPACE_ROOT}/assets" "${BUILD_DIR}/"
        echo "  - 已复制: assets/"
    fi

    if [ -d "${WORKSPACE_ROOT}/credits" ]; then
        cp -r "${WORKSPACE_ROOT}/credits" "${BUILD_DIR}/"
        echo "  - 已复制: credits/"
    fi

    # 创建启动脚本
    cat > "${BUILD_DIR}/run.sh" << 'EOF'
#!/usr/bin/env bash
cd "$(dirname "${BASH_SOURCE[0]}")"
exec ./vigilant-doodle "$@"
EOF
    chmod +x "${BUILD_DIR}/run.sh"
    echo "  - 已创建: run.sh"

    # 创建 README
    cat > "${BUILD_DIR}/README.txt" << EOF
Vigilant Doodle - Linux 发行版
================================

版本: ${VERSION}
构建日期: $(date '+%Y-%m-%d %H:%M:%S')

运行方法
--------
1. 解压此压缩包
2. 进入目录，运行:
   ./run.sh

或直接运行可执行文件:
   ./vigilant-doodle

系统要求
--------
- Linux (x86_64)
- OpenGL 3.3+
- ALSA 音频支持

许可证
------
MIT License
详见: LICENSE 文件

问题反馈
--------
https://github.com/arxxyr/vigilant-doodle/issues
EOF
    echo "  - 已创建: README.txt"

    echo "✓ 发行包准备完成"
}

# ----------------------------------------------------------------------------
# 打包
# ----------------------------------------------------------------------------
create_archive() {
    echo ""
    echo "[5/5] 创建压缩包..."

    local archive_name="${GAME_NAME}_${VERSION}_linux.tar.gz"
    local archive_path="${BUILD_DIR}/${archive_name}"

    cd "${BUILD_DIR}"

    # 打包所有文件（不包含 bin 目录本身）
    tar -czf "${archive_name}" \
        "${GAME_NAME}" \
        run.sh \
        README.txt \
        assets \
        credits

    local size=$(du -h "${archive_path}" | cut -f1)

    echo ""
    echo "========================================"
    echo "  打包完成！"
    echo "========================================"
    echo "输出文件: ${archive_path}"
    echo "文件大小: ${size}"
    echo ""
    echo "测试运行:"
    echo "  cd bin"
    echo "  tar -xzf ${archive_name}"
    echo "  ./run.sh"
    echo "========================================"
}

# ----------------------------------------------------------------------------
# 主流程
# ----------------------------------------------------------------------------
main() {
    check_dependencies
    clean_build
    build_release
    prepare_package
    create_archive
}

main "$@"
