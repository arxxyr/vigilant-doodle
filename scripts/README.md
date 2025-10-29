# 构建与打包脚本

本目录包含游戏的构建和打包脚本。

## 脚本列表

| 脚本 | 平台 | 用途 |
|------|------|------|
| `quick-build.sh` | Linux/macOS | 快速构建（开发模式） |
| `package-linux.sh` | Linux | 打包 Linux 发行版 |
| `package-windows.ps1` | Windows | 打包 Windows 发行版 |

---

## 1. 快速构建（开发）

**用途**：日常开发，快速编译和测试。

### Linux/macOS

```bash
# 构建并运行（动态链接，快速编译）
./scripts/quick-build.sh

# 仅构建，不运行
./scripts/quick-build.sh --no-run

# 启用 Inspector 调试工具
./scripts/quick-build.sh --inspector
```

**特性**：
- 使用 `--features dev` 启用动态链接（dylib）
- 编译速度快，适合迭代开发
- 自动运行游戏

---

## 2. 打包 Linux 发行版

**用途**：构建和打包 Linux 平台的完整发行版。

### 前置要求

```bash
# 安装编译依赖
sudo apt-get update
sudo apt-get install pkg-config libx11-dev libasound2-dev libudev-dev
```

### 使用方法

```bash
# 打包开发版本
./scripts/package-linux.sh

# 打包指定版本
./scripts/package-linux.sh v0.3.0
```

### 输出

所有文件输出到项目根目录的 `bin/` 目录：

```
bin/
├── vigilant-doodle              # 可执行文件（已 strip）
├── run.sh                       # 启动脚本
├── assets/                      # 游戏资源
├── credits/                     # 致谢文件
├── README.txt                   # 使用说明
└── vigilant-doodle_VERSION_linux.tar.gz  # 发行包
```

### 测试发行包

```bash
# 本地直接运行
cd bin
./run.sh

# 或解压压缩包测试
cd /tmp
tar -xzf ~/repo/rust/vigilant-doodle/bin/vigilant-doodle_v0.3.0_linux.tar.gz
./run.sh
```

---

## 3. 打包 Windows 发行版

**用途**：构建和打包 Windows 平台的完整发行版（在 Windows 系统上运行）。

### 前置要求

- Windows 10/11
- PowerShell 5.1+
- Rust 工具链（rustup）

### 使用方法

```powershell
# 打包开发版本
.\scripts\package-windows.ps1

# 打包指定版本
.\scripts\package-windows.ps1 -Version v0.3.0
```

### 输出

所有文件输出到项目根目录的 `bin/` 目录：

```
bin/
├── vigilant-doodle.exe          # 可执行文件
├── run.bat                      # 启动脚本
├── assets/                      # 游戏资源
├── credits/                     # 致谢文件
├── README.txt                   # 使用说明
└── vigilant-doodle_VERSION_windows.zip  # 发行包
```

### 测试发行包

```powershell
# 本地直接运行
cd bin
.\run.bat

# 或解压压缩包测试
cd C:\Temp
Expand-Archive -Path "~\repo\rust\vigilant-doodle\bin\vigilant-doodle_v0.3.0_windows.zip"
cd vigilant-doodle_v0.3.0_windows
.\run.bat
```

---

## 输出目录

**所有平台的打包脚本统一输出到项目根目录的 `bin/` 目录**：

```
vigilant-doodle/
├── bin/                         # 发行包输出目录（已添加到 .gitignore）
│   ├── vigilant-doodle          # Linux 可执行文件
│   ├── vigilant-doodle.exe      # Windows 可执行文件
│   ├── run.sh / run.bat         # 启动脚本
│   ├── assets/                  # 游戏资源
│   ├── credits/                 # 致谢文件
│   ├── README.txt               # 使用说明
│   ├── *.tar.gz                 # Linux 发行包
│   └── *.zip                    # Windows 发行包
└── scripts/                     # 打包脚本
```

**优点**：
- ✅ 统一的输出位置，便于查找
- ✅ 可以直接运行测试，无需解压
- ✅ 发行包也在同一目录，便于分发
- ✅ 已添加到 `.gitignore`，不会提交到版本控制

---

## 构建配置说明

### 开发模式 vs 发行模式

| 配置项 | 开发模式 | 发行模式 |
|--------|----------|----------|
| 链接方式 | 动态链接（dylib） | 静态链接 |
| 优化级别 | 基础优化 | 完全优化（LTO） |
| 编译速度 | 快 | 慢 |
| 二进制大小 | 大 | 小（strip） |
| 调试信息 | 包含 | 移除 |
| 输出目录 | `target/debug` | `bin/` |
| 适用场景 | 日常开发 | 发布/分发 |

### Cargo 特性标志

- `dev`：启用动态链接，加速编译
- `inspector`：启用 bevy-inspector-egui 调试工具

---

## CI/CD 自动发布

项目已配置 GitHub Actions 自动发布流程（`.github/workflows/release.yaml`）。

### 触发方式

```bash
# 创建版本标签
git tag v0.3.0
git push origin v0.3.0
```

### 自动构建平台

- ✅ Linux (x86_64) - `.tar.gz`
- ✅ Windows (x64) - `.zip`
- ✅ macOS (Universal Binary) - `.dmg`
- ✅ Web (Wasm) - `.zip`
- ✅ iOS - `.ipa` (unsigned)
- ✅ Android - `.apk`

---

## 常见问题

### Linux: 缺少依赖错误

**问题**：编译时提示找不到 `alsa` 或 `udev`。

**解决**：
```bash
sudo apt-get install pkg-config libx11-dev libasound2-dev libudev-dev
```

### Windows: 链接器错误

**问题**：发行模式构建失败，提示链接器错误。

**解决**：确保使用静态链接（发行模式自动启用，不要添加 `--features dev`）。

### 二进制文件过大

**问题**：打包后的文件体积超过 100MB。

**检查**：
1. 确认使用 `--release` 构建
2. 确认执行了 `strip` 命令（脚本自动处理）
3. 检查是否误包含了 `target/` 目录

### 权限错误

**问题**：脚本无法执行。

**解决**：
```bash
chmod +x scripts/*.sh
```

---

## 自定义构建

### 修改游戏名称

编辑脚本顶部的配置：

```bash
# Linux 脚本
GAME_NAME="vigilant-doodle"  # 修改为你的游戏名称（必须与 Cargo.toml 中的 [[bin]] name 一致）
```

```powershell
# Windows 脚本
$GAME_NAME = "vigilant-doodle"  # 修改为你的游戏名称（必须与 Cargo.toml 中的 [[bin]] name 一致）
```

### 添加额外文件

在 `prepare_package()` 函数中添加复制逻辑：

```bash
# 示例：复制许可证文件
if [ -f "${WORKSPACE_ROOT}/LICENSE" ]; then
    cp "${WORKSPACE_ROOT}/LICENSE" "${BUILD_DIR}/"
    echo "  - 已复制: LICENSE"
fi
```

---

## 相关文档

- [Cargo Workspace 架构](../docs/WORKSPACE_ARCHITECTURE.md)
- [项目架构最佳实践](../docs/ARCHITECTURE_BEST_PRACTICE.md)
- [Bevy 发布指南](https://github.com/bevyengine/bevy/blob/main/docs/releasing.md)

---

**最后更新**：2025-10-29
**适用版本**：Bevy 0.17.2, Rust Edition 2024
