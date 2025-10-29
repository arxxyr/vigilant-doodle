# Vigilant Doodle

一个基于 Bevy 0.17.2 的 3D 俯视角动作游戏，采用现代 Rust 工程架构。

## 🎮 游戏特性

- **斜向俯视视角**：45° 角度，经典 ARPG 风格
- **玩家角色**：带探照灯的角色，WASD 移动，空格跳跃
- **敌人 AI 系统**：智能追踪、碰撞检测、动态行为
- **存档系统**：加密存档（AES-256-GCM），自动保存/加载
- **菜单系统**：游戏场景叠加显示的菜单 + 暂停功能
- **多语言支持**：🌐 中文/English 双语，一键切换，异步加载
- **模块化架构**：11 个独立 crate，清晰的职责分离
- **跨平台**：支持 Windows / Linux / macOS / Web

## 🏗️ 项目架构

采用 **Cargo Workspace** 架构，完全模块化设计：

```
vigilant-doodle/
├── crates/
│   ├── launcher/     # 启动器（可执行程序，~100 行）
│   ├── game/         # 游戏逻辑（主 crate，插件注册）
│   ├── core/         # 核心系统（状态机、设置、存档）
│   ├── assets/       # 资源加载与管理
│   ├── camera/       # 斜向俯视相机系统
│   ├── world/        # 世界生成与地形
│   ├── input/        # 输入处理与光标管理
│   ├── gameplay/     # 玩家控制与游戏逻辑
│   ├── ai/           # 敌人 AI 行为系统
│   ├── ui/           # UI 菜单与 HUD
│   └── audio/        # 音频系统（预留）
├── assets/           # 游戏资源
├── scripts/          # 构建与打包脚本
└── docs/             # 技术文档
```

**优势**：
- 编译速度快（Linux/macOS 支持动态链接，Windows 使用快速配置）
- 职责分离清晰，易于维护
- 模块可独立测试和重用
- 依赖关系明确（单向依赖）

## 🚀 快速开始

### 环境要求
- Rust 2024 Edition（推荐 nightly）
- Bevy 0.17.2

### 开发模式（快速编译）

**Windows 用户**：
```bash
# ✅ 推荐：标准开发模式（稳定）
cargo run

# ✅ 或使用快速发布模式（2-3分钟，性能好）
cargo run --profile release-fast

# ❌ 不要使用（会崩溃）
# cargo run --features dev
```

**Linux/macOS 用户**：
```bash
# ✅ 快速编译（动态链接）
cargo run --features dev
```

### 发布模式（完全优化）
```bash
cargo build --release
```

### Web 构建
```bash
# 安装工具
cargo install --locked trunk
rustup target add wasm32-unknown-unknown

# 启动开发服务器
trunk serve
```

## 🎯 控制说明

| 操作 | 按键 |
|------|------|
| 移动 | WASD |
| 跳跃 | 空格 |
| 暂停 | ESC |
| 继续 | ESC（暂停菜单中）|
| 退出 | 主菜单 → Quit |

## 📚 技术文档

完整的技术文档位于 [`docs/`](./docs/) 目录：

### 必读文档
1. **[WORKSPACE_ARCHITECTURE.md](./docs/WORKSPACE_ARCHITECTURE.md)** - Workspace 架构设计
2. **[ARCHITECTURE_BEST_PRACTICE.md](./docs/ARCHITECTURE_BEST_PRACTICE.md)** - 游戏逻辑架构
3. **[IMPLEMENTATION_GUIDE.md](./docs/IMPLEMENTATION_GUIDE.md)** - 实施指南

### 快速参考
- [docs/README.md](./docs/README.md) - 文档索引与快速查询

## 🛠️ 开发工具

### 代码检查
```bash
cargo fmt --all              # 格式化
cargo clippy --workspace     # Lint 检查
cargo check --workspace      # 快速检查
```

### 调试模式
```bash
# 启用 inspector（实体查看器）
# Windows: 仅使用 inspector
cargo run --features inspector

# Linux/macOS: 可以同时使用 dev 和 inspector
cargo run --features dev,inspector
```

## 📦 依赖库

| 库名 | 版本 | 用途 |
|------|------|------|
| bevy | 0.17.2 | 游戏引擎 |
| bevy_asset_loader | 0.24.0-rc.1 | 资源预加载 |
| bevy_egui | 0.38.0 | Inspector UI |
| rand | 0.9.2 | 随机数生成 |
| bincode | 2.0.1 | 二进制序列化（存档） |
| aes-gcm | 0.10 | 加密存档 |
| crc32fast | 1.4 | 校验和计算 |
| fluent | 0.17.0 | Fluent i18n 框架 |
| fluent-bundle | 0.16.0 | 翻译资源管理 |
| serde/serde_json | 1.0 | JSON 序列化 |
| dirs | 6.0.0 | 系统目录获取 |

## 🎨 资源结构

```
assets/
├── fonts/                        # 字体文件
│   └── SarasaTermSCNerd/        # 等宽字体（支持中文）
├── i18n/                         # 多语言翻译文件（Fluent 格式）
│   ├── zh-Hans/                 # 简体中文
│   │   ├── main.ftl
│   │   ├── menu.ftl
│   │   └── settings.ftl
│   └── en/                      # 英文
│       ├── main.ftl
│       ├── menu.ftl
│       └── settings.ftl
├── model/                        # 3D 模型（GLTF，待添加）
├── audio/                        # 音频文件（待添加）
└── textures/                     # UI 图标（待添加）
```

## 🔧 构建配置

### 编译优化
- **开发模式**：第三方库 O3，自身代码 O1
- **发布模式**：LTO + 单编译单元 + 完全优化 + strip
- **链接器**：Linux(lld) / macOS(zld) / Windows(rust-lld)

### Feature Flags
- `dev` - 启用动态链接（**仅 Linux/macOS**，Windows 会崩溃）
- `inspector` - 启用实体查看器

### 打包脚本
提供完整的打包工具，详见 [scripts/README.md](./scripts/README.md)：

```bash
# Linux/macOS 快速构建
./scripts/quick-build.sh

# Linux 打包
./scripts/package-linux.sh [版本号]

# Windows 打包
.\scripts\package-windows.ps1 [-Version 版本号]
```

**输出目录**：所有构建产物统一输出到 `bin/` 目录

## 🚢 发布流程

### 自动发布
推送 `v*.*.*` 格式的 Git 标签：
```bash
git tag v0.2.0
git push origin v0.2.0
```

GitHub Actions 将自动构建：
- Windows (exe)
- Linux (AppImage)
- macOS (dmg)
- Web (Wasm)

### 手动发布
```bash
# 构建所有平台
cargo build --release --workspace

# 仅构建启动器
cargo build --release -p vigilant-doodle-launcher
```

## 🧪 测试

```bash
# 运行测试
cargo test --workspace

# 运行基准测试（如果有）
cargo bench
```

## 🐛 已知问题

### Windows 开发环境崩溃（已解决）
- **问题**：运行 `cargo run --features dev` 导致访问违例崩溃
- **原因**：Windows MSVC 工具链与 Bevy 0.17.2 动态链接不兼容
- **解决**：使用 `cargo run` 或 `cargo run --profile release-fast`
- **影响**：仅 Windows 平台，Linux/macOS 不受影响

### 其他已知问题
- Web 构建在 Firefox 中可能有音频性能问题

## 💡 平台特定说明

### Windows
- **发布版本**：不显示控制台窗口，像普通游戏一样启动
- **开发版本**：显示控制台窗口，方便查看日志输出
- 通过 `#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]` 自动控制

## 📄 许可证

本项目基于 MIT 许可证（详见 LICENSE 文件）。

资源文件（`assets/` 和 `build/` 中的图标）可能有不同的许可证，请查看 [credits/CREDITS.md](./credits/CREDITS.md)。

## 🤝 贡献

欢迎提交 Issue 和 Pull Request！

开发前请阅读：
- [技术文档](./docs/)
- [代码风格指南](./docs/README.md#开发规范)

## 🔗 相关资源

- [Bevy 官网](https://bevyengine.org/)
- [Bevy 文档](https://docs.rs/bevy/)
- [Bevy Cheat Book](https://bevy-cheatbook.github.io/)
- [Bevy Discord](https://discord.gg/bevy)

## 📝 更新日志

### v0.3.0（当前版本 - 2025-10-30）
- 🏗️ **架构升级**：完整拆分为 11 个独立 crate 模块
  - 启动器：`launcher`（可执行程序）
  - 游戏逻辑：`game`（插件注册中心）
  - 核心系统：`core`（状态机、设置、存档加密）
  - 其他模块：`assets`, `camera`, `world`, `input`, `gameplay`, `ai`, `ui`, `audio`
- 💾 **存档系统**：完整的加密存档实现
  - AES-256-GCM 加密
  - bincode 2.0 二进制序列化
  - CRC32 校验和验证
  - 跨平台存档路径管理
- 🤖 **敌人 AI 系统**：完整的敌人行为实现
  - 智能追踪玩家
  - 碰撞检测与响应
  - 状态机驱动的动态行为
- ⏸️ **暂停菜单**：完整的游戏暂停功能
  - ESC 键暂停/继续
  - 暂停菜单 UI
  - 状态保持与恢复
- 🌍 **Fluent i18n 系统**：现代化多语言支持
  - 使用 Mozilla Fluent 框架（行业标准）
  - 支持参数化翻译、复数形式、性别变化
  - 异步加载 .ftl 翻译文件
  - 无需打包或加密，易于翻译协作
- 🚀 **打包脚本**：跨平台自动化构建
  - Linux 打包脚本（AppImage）
  - Windows 打包脚本（PowerShell）
  - 快速构建脚本（开发用）
- ⚡ **性能优化**：关键问题修复
  - 修复人物移动抖动问题
  - 修复转弯抖动问题
  - 优化资源加载性能
- 🌐 **本地化改进**：完善多语言支持
  - 迁移到 Fluent i18n 系统
  - 支持高级语言特性（参数化、复数）
  - 更快的语言切换和热重载
- 📦 **依赖升级**：现代化技术栈
  - bincode 2.0（更好的序列化）
  - 完整的 3D 模型资源系统
  - 优化的资源管理流程

### v0.2.0（2025-10-25） - 原型版本
- 🏗️ 重构为 Workspace 架构
- 📐 采用单相机俯视设计
- 🎨 菜单叠加渲染
- ⚡ 优化编译速度（dylib 模式）
- 🌐 完整的本地化系统（中文/English）
- 🎯 语言切换按钮（右上角）
- 🎮 完善的 UI 系统（主菜单 + 设置菜单）
- 🔧 输入系统（WASD 移动 + 光标管理）

### v0.1.0（2025-10-20） - 初始版本
- 🎮 基础游戏玩法
- 🎯 玩家移动
- 👹 敌人追逐
- 🎵 音频系统框架

---

由 [Claude Code](https://claude.com/claude-code) 驱动构建
