# Vigilant Doodle - 实施指南

## 📋 文档体系总览

本项目采用三层文档架构，从宏观到微观引导开发：

```
1. WORKSPACE_ARCHITECTURE.md    → 项目组织结构（宏观）
2. ARCHITECTURE_BEST_PRACTICE.md → 游戏逻辑架构（中观）
3. README.md                     → 快速参考手册（微观）
```

---

## 🚀 实施步骤总览

### 阶段 0：准备工作（已完成 ✅）
- [x] 设计 Workspace 架构
- [x] 设计游戏逻辑架构
- [x] 整理文档体系

### 阶段 1：创建 Workspace 结构（0.5 小时）
- [ ] 删除现有 `src/` 目录
- [ ] 创建 `crates/launcher` 和 `crates/game`
- [ ] 修改根 `Cargo.toml` 为 workspace 配置
- [ ] 验证编译通过

### 阶段 2：实现核心架构（2-3 小时）
- [ ] 实现 `core/state.rs`（状态机）
- [ ] 实现 `assets/loader.rs`（资源加载）
- [ ] 实现 `camera/isometric.rs`（斜向俯视相机）
- [ ] 实现 `world/spawning.rs`（实体生成）
- [ ] 验证：Loading → MainMenu 转换正常

### 阶段 3：实现游戏玩法（1-2 小时）
- [ ] 实现 `gameplay/player.rs`（玩家移动）
- [ ] 实现 `gameplay/enemy.rs`（敌人 AI）
- [ ] 实现 `gameplay/movement.rs`（碰撞检测）
- [ ] 实现 `input/actions.rs`（输入映射）
- [ ] 验证：玩家移动、敌人追逐正常

### 阶段 4：实现 UI 系统（1-2 小时）
- [ ] 实现 `ui/menu/overlay.rs`（模糊遮罩）
- [ ] 实现 `ui/menu/main_menu.rs`（主菜单）
- [ ] 实现 `ui/menu/settings_menu.rs`（设置菜单）
- [ ] 实现 `input/cursor.rs`（光标管理）
- [ ] 验证：菜单显示、按钮交互正常

### 阶段 5：完善功能（1 小时）
- [ ] 实现 `audio/manager.rs`（音频播放）
- [ ] 实现 `ui/hud/`（游戏内 HUD，可选）
- [ ] 性能测试与优化
- [ ] 代码审查与清理

### 阶段 6：文档与发布（0.5 小时）
- [ ] 更新 README.md
- [ ] 更新 CLAUDE.md
- [ ] 测试 CI/CD 流程
- [ ] 创建 Git tag

**总预估时间：6-9 小时**

---

## 📁 完整文件清单

### Workspace 根目录
```
vigilant-doodle/
├── Cargo.toml                      # ⚙️ Workspace 配置（需创建）
├── Cargo.lock
├── .cargo/
│   └── config.toml                 # 构建配置（已存在）
├── README.md                       # 项目说明（需更新）
└── CLAUDE.md                       # Claude 指令（需更新）
```

### 启动器 Crate
```
crates/launcher/
├── Cargo.toml                      # 依赖配置
└── src/
    └── main.rs                     # 启动入口（~100 行）
```

### 游戏 Crate
```
crates/game/
├── Cargo.toml                      # 依赖配置
└── src/
    ├── lib.rs                      # 插件注册（~100 行）
    │
    ├── core/                       # 核心系统
    │   ├── mod.rs
    │   ├── state.rs                # 状态机（~80 行）
    │   └── settings.rs             # 游戏设置（~50 行）
    │
    ├── assets/                     # 资源管理
    │   ├── mod.rs
    │   └── loader.rs               # 资源加载（~60 行）
    │
    ├── camera/                     # 相机系统
    │   ├── mod.rs
    │   ├── isometric.rs            # 斜向俯视相机（~120 行）
    │   └── components.rs           # 组件定义（~30 行）
    │
    ├── gameplay/                   # 游戏玩法
    │   ├── mod.rs
    │   ├── player.rs               # 玩家逻辑（~80 行）
    │   ├── enemy.rs                # 敌人 AI（~80 行）
    │   └── movement.rs             # 碰撞检测（~50 行）
    │
    ├── world/                      # 世界管理
    │   ├── mod.rs
    │   ├── spawning.rs             # 实体生成（~120 行）
    │   └── terrain.rs              # 地形生成（~50 行）
    │
    ├── ui/                         # UI 系统
    │   ├── mod.rs
    │   ├── menu/
    │   │   ├── mod.rs
    │   │   ├── overlay.rs          # 模糊遮罩（~50 行）
    │   │   ├── main_menu.rs        # 主菜单（~150 行）
    │   │   └── settings_menu.rs    # 设置菜单（~200 行）
    │   ├── hud/
    │   │   ├── mod.rs
    │   │   ├── health_bar.rs       # 血条（可选）
    │   │   └── score.rs            # 分数（可选）
    │   └── styles.rs               # UI 样式（~50 行）
    │
    ├── input/                      # 输入管理
    │   ├── mod.rs
    │   ├── actions.rs              # 输入映射（~80 行）
    │   └── cursor.rs               # 光标管理（~80 行）
    │
    └── audio/                      # 音频系统
        ├── mod.rs
        └── manager.rs              # 音频播放（~60 行）
```

**总代码量预估：~1600 行**（不含空行和注释）

---

## 🔧 关键命令速查

### 开发模式（动态链接，快速编译）
```bash
# 运行游戏
cargo run --features dev

# 或者指定 package
cargo run -p vigilant-doodle-launcher --features dev
```

### 发布模式（静态链接，完全优化）
```bash
# 构建发布版本
cargo build --release -p vigilant-doodle-launcher

# 构建所有 crate
cargo build --release --workspace
```

### 代码检查
```bash
# 格式化
cargo fmt --all

# Clippy 检查
cargo clippy --workspace -- -D warnings

# 检查编译（不生成二进制）
cargo check --workspace
```

### Workspace 操作
```bash
# 清理所有构建产物
cargo clean

# 仅编译游戏库
cargo build -p vigilant-doodle-game

# 测试所有 crate
cargo test --workspace
```

---

## 📊 实施进度追踪

### 检查点 1：Workspace 结构创建完成
**验证标准**：
- [ ] `cargo build --workspace` 编译通过
- [ ] `cargo run` 能启动窗口
- [ ] 控制台输出 `[Game] Loading game plugin...`

### 检查点 2：核心架构实现完成
**验证标准**：
- [ ] 窗口显示 Loading 状态（背景色）
- [ ] 1-2 秒后自动转换到 MainMenu 状态
- [ ] 控制台输出状态转换日志

### 检查点 3：游戏玩法实现完成
**验证标准**：
- [ ] 玩家可以通过 WASD 移动
- [ ] 相机平滑跟随玩家
- [ ] 敌人追逐玩家
- [ ] 碰撞检测生效（实体不穿越边界）

### 检查点 4：UI 系统实现完成
**验证标准**：
- [ ] MainMenu 状态显示模糊背景 + 菜单
- [ ] 按钮交互正常（悬停、点击）
- [ ] 点击 "New Game" 进入 Playing 状态
- [ ] 按 ESC 返回菜单

### 检查点 5：功能完善
**验证标准**：
- [ ] 音频播放正常
- [ ] 帧率稳定 60 FPS
- [ ] 无编译警告
- [ ] 所有文档更新完成

---

## ⚠️ 常见问题与解决方案

### 问题 1：编译时找不到 `vigilant-doodle-game`
**原因**：Workspace 配置错误或路径不对

**解决**：
```bash
# 检查 Workspace 成员
cargo metadata --format-version=1 | jq '.workspace_members'

# 确保根 Cargo.toml 包含：
[workspace]
members = ["crates/launcher", "crates/game"]
```

### 问题 2：动态链接失败（Windows）
**原因**：Windows 对 dylib 支持有限

**解决**：
- 开发模式使用 `--features dev`
- 发布模式不使用 dev feature（自动静态链接）

### 问题 3：资源路径找不到
**原因**：工作目录不在 workspace 根目录

**解决**：
```rust
// 确保从 workspace 根目录运行
// 或者在 Cargo.toml 中设置：
[[bin]]
path = "src/main.rs"
test = false
```

### 问题 4：多个 crate 版本冲突
**原因**：没有使用 workspace dependencies

**解决**：
```toml
# 根 Cargo.toml
[workspace.dependencies]
bevy = { version = "0.17.2", ... }

# 子 crate Cargo.toml
[dependencies]
bevy = { workspace = true, features = [...] }
```

---

## 📈 性能优化建议

### 编译速度优化
1. **启用 mold/lld 链接器**（.cargo/config.toml 已配置）
2. **使用 sccache**（可选）：
   ```bash
   cargo install sccache
   export RUSTC_WRAPPER=sccache
   ```
3. **增加并行编译**：
   ```bash
   export CARGO_BUILD_JOBS=8
   ```

### 运行时性能优化
1. **Release 模式启用 LTO**（已配置）
2. **使用 `--target-cpu=native`**（需要时）
3. **Profile 分析**：
   ```bash
   cargo install flamegraph
   cargo flamegraph --release
   ```

---

## 🎯 下一步行动

### 立即执行（5 分钟）
1. 阅读 `WORKSPACE_ARCHITECTURE.md` 第九章"迁移步骤"
2. 备份现有代码（创建 Git 分支）
3. 开始创建 Workspace 结构

### 推荐工作流
```bash
# 1. 创建开发分支
git checkout -b workspace-refactor

# 2. 按照文档创建结构
# （参考 WORKSPACE_ARCHITECTURE.md）

# 3. 每完成一个检查点就提交
git add .
git commit -m "feat(workspace): 完成检查点 1 - Workspace 结构创建"

# 4. 全部完成后合并
git checkout master
git merge workspace-refactor
git tag v0.2.0-workspace
```

---

## 📞 获取帮助

### 文档查找
- **项目结构问题** → WORKSPACE_ARCHITECTURE.md
- **游戏逻辑问题** → ARCHITECTURE_BEST_PRACTICE.md
- **快速查询** → docs/README.md

### 社区资源
- [Bevy Discord](https://discord.gg/bevy) - #help 频道
- [Bevy GitHub Discussions](https://github.com/bevyengine/bevy/discussions)
- [Rust Users Forum](https://users.rust-lang.org/)

---

## ✅ 总结

你现在拥有：
1. ✅ 完整的 Workspace 架构设计
2. ✅ 详细的游戏逻辑架构
3. ✅ 清晰的实施路线图
4. ✅ 完善的文档体系

**一切准备就绪，开始实施吧！🚀**

---

最后更新：2025-10-19
维护者：Claude Code
