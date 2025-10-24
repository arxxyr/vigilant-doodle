# Vigilant Doodle

一个基于 Bevy 0.17.2 的 3D 俯视角动作游戏，采用现代 Rust 工程架构。

## 🎮 游戏特性

- **斜向俯视视角**：45° 角度，经典 ARPG 风格
- **玩家角色**：带探照灯的角色，WASD 移动
- **敌人 AI**：智能追逐玩家
- **菜单系统**：游戏场景叠加显示的菜单
- **多语言支持**：🌐 中文/English 双语，一键切换
- **跨平台**：支持 Windows / Linux / macOS / Web

## 🏗️ 项目架构

采用 **Cargo Workspace** 架构，启动器与游戏逻辑分离：

```
vigilant-doodle/
├── crates/
│   ├── launcher/     # 启动器（可执行程序）
│   └── game/         # 游戏逻辑（动态库）
├── assets/           # 游戏资源
└── docs/             # 技术文档
```

**优势**：
- 编译速度快（开发模式动态链接）
- 模块化清晰
- 易于扩展

## 🚀 快速开始

### 环境要求
- Rust 2024 Edition（推荐 nightly）
- Bevy 0.17.2

### 开发模式（快速编译）
```bash
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
| 菜单 | ESC |
| 退出 | ESC（菜单中点击 Quit）|

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
cargo run --features dev,inspector
```

## 📦 依赖库

| 库名 | 版本 | 用途 |
|------|------|------|
| bevy | 0.17.2 | 游戏引擎 |
| bevy_asset_loader | 0.24.0-rc.1 | 资源预加载 |
| bevy_kira_audio | 0.24.0 | 音频播放 |
| rand | 0.9.2 | 随机数生成 |

## 🎨 资源结构

```
assets/
├── fonts/                        # 字体文件
│   └── SarasaTermSCNerd/        # 等宽字体（支持中文）
├── localization/                 # 多语言翻译文件
│   ├── zh_CN.json               # 中文
│   └── en_US.json               # 英文
├── model/                        # 3D 模型（GLTF，待添加）
├── audio/                        # 音频文件（待添加）
└── textures/                     # UI 图标（待添加）
```

## 🔧 构建配置

### 编译优化
- **开发模式**：第三方库 O3，自身代码 O1
- **发布模式**：LTO + 单编译单元 + 完全优化
- **链接器**：Linux(lld) / macOS(zld) / Windows(rust-lld)

### Feature Flags
- `dev` - 启用动态链接（加速编译）
- `inspector` - 启用实体查看器

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

- Web 构建在 Firefox 中可能有音频性能问题
- Windows 发布模式不支持动态链接（dev feature）

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

### v0.2.0（2025-10-25）
- 🏗️ 重构为 Workspace 架构
- 📐 采用单相机俯视设计
- 🎨 菜单叠加渲染
- ⚡ 优化编译速度（dylib 模式）
- 🌐 完整的本地化系统（中文/English）
- 🎯 语言切换按钮（右上角）
- 🎮 完善的 UI 系统（主菜单 + 设置菜单）
- 🔧 输入系统（WASD 移动 + 光标管理）

### v0.1.0
- 🎮 基础游戏玩法
- 🎯 玩家移动
- 👹 敌人追逐
- 🎵 音频系统框架

---

由 [Claude Code](https://claude.com/claude-code) 驱动构建
