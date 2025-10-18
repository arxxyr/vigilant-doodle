# CLAUDE.md

本文件为 Claude Code 提供项目指引。

## 项目概述

**Vigilant Doodle** 是一个基于 Bevy 0.17.2 的 3D 俯视角动作游戏，采用 Cargo Workspace 架构。

### 核心特性
- **引擎**：Bevy 0.17.2
- **语言**：Rust Edition 2024
- **架构**：Workspace（launcher + game）
- **视角**：斜向俯视 45°
- **平台**：Windows / Linux / macOS / Web

---

## 架构设计（重要）

### Workspace 结构
```
vigilant-doodle/
├── crates/
│   ├── launcher/           # 启动器（可执行程序）
│   │   └── src/main.rs     # ~100 行，负责初始化
│   └── game/               # 游戏逻辑（动态库）
│       └── src/
│           ├── lib.rs      # 插件注册中心
│           ├── core/       # 状态机、设置
│           ├── assets/     # 资源加载
│           ├── camera/     # 斜向俯视相机
│           ├── gameplay/   # 玩家、敌人、移动
│           ├── world/      # 地形、生成
│           ├── ui/         # 菜单、HUD
│           ├── input/      # 输入、光标
│           └── audio/      # 音频
├── assets/                 # 游戏资源（共享）
└── docs/                   # 技术文档
```

### 设计原则
1. **职责分离**：启动器只负责初始化，游戏逻辑全在 game crate
2. **动态链接**：开发模式使用 dylib 加速编译
3. **模块化**：每个模块独立，通过插件注册

---

## 核心概念

### 状态机（GameState）
定义在 `crates/game/src/core/state.rs`：
```rust
pub enum GameState {
    AssetLoading,    // 资源加载（默认状态）
    MainMenu,        // 主菜单（游戏场景 + 模糊遮罩 + UI）
    Playing,         // 游戏进行
}
```

**流程**：
```
AssetLoading → MainMenu → Playing
                  ↑           ↓
                  └─── ESC ───┘
```

### 单相机架构
- 只有一个 `Camera3d`（斜向俯视 45°）
- 始终激活，不需要切换
- 平滑跟随玩家
- MainMenu 状态下也继续跟随（背景有动感）

### 实体生命周期
- **预生成**：所有实体在 `AssetLoading` 状态就创建
- **始终可见**：不使用 show/hide 系统
- **状态驱动**：通过 `.run_if(in_state())` 控制系统是否执行

### UI 叠加渲染
```
Z-Index 0:   3D 游戏场景（Camera3d）
Z-Index 100: 黑色半透明遮罩（60% 不透明）
Z-Index 200: 菜单 UI（屏幕居中）
```

---

## 常用命令

### 开发
```bash
# 运行游戏（动态链接，快速编译）
cargo run --features dev

# 运行特定 crate
cargo run -p vigilant-doodle-launcher --features dev

# 检查编译
cargo check --workspace
```

### 构建
```bash
# 发布构建（静态链接，完全优化）
cargo build --release

# 仅构建游戏库
cargo build -p vigilant-doodle-game
```

### 代码质量
```bash
cargo fmt --all
cargo clippy --workspace -- -D warnings
```

---

## 技术文档索引

### 必读文档（按顺序）
1. **[WORKSPACE_ARCHITECTURE.md](./docs/WORKSPACE_ARCHITECTURE.md)**
   Workspace 架构设计，启动器与游戏库的分离策略

2. **[ARCHITECTURE_BEST_PRACTICE.md](./docs/ARCHITECTURE_BEST_PRACTICE.md)**
   游戏逻辑架构，模块划分，完整代码示例

3. **[IMPLEMENTATION_GUIDE.md](./docs/IMPLEMENTATION_GUIDE.md)**
   实施步骤，文件清单，进度追踪

4. **[docs/README.md](./docs/README.md)**
   快速参考，设计决策表格，开发规范

---

## Bevy 0.17.2 API 变更

### 关键变更
```rust
// 状态管理
app.init_state::<GameState>()  // 而非 add_state()

// 颜色
Color::srgb(0.2, 0.2, 0.3)  // 而非 Color::rgb()
bevy::color::palettes::css::GRAY  // 而非 Color::GRAY

// 文本（新的元组组件方式）
(
    Text::new("Hello"),
    TextFont { font, font_size: 32.0, ..default() },
    TextColor(Color::WHITE),
    Node::default(),
)

// UI 图像
ImageNode::new(handle)  // 而非 UiImage

// 应用退出
AppExit::Success  // 而非 AppExit

// 实体销毁
commands.entity(e).despawn()  // 自动递归，无需 despawn_recursive
```

### 导入变更
```rust
use bevy_state::prelude::*;  // 用于 OnEnter, OnExit, in_state
```

---

## 编码规范

### 命名约定
- **类型/组件**：`UpperCamelCase`（例：`IsometricCamera`）
- **函数/变量**：`snake_case`（例：`spawn_player`）
- **常量**：`UPPER_SNAKE_CASE`（例：`FLOOR_LENGTH`）

### 日志格式
```rust
info!("[模块名] 事件描述");
warn!("[模块名] 警告信息");
error!("[模块名] 错误信息: {}", detail);
```

### 系统注册模式
```rust
app.add_systems(
    Update,
    my_system
        .in_set(GameplaySet::Logic)
        .run_if(in_state(GameState::Playing))
        .after(other_system)
);
```

### 错误处理
```rust
// ✅ 推荐
let Ok(player) = player_query.get_single() else {
    warn!("[Player] Player not found");
    return;
};

// ❌ 避免
let player = player_query.get_single().unwrap();
let player = player_query.get_single().expect("Player not found");
```

---

## 资源管理

### 资源预加载
所有资源在 `AssetLoading` 状态通过 `bevy_asset_loader` 加载：
```rust
#[derive(AssetCollection, Resource)]
pub struct GameAssets {
    #[asset(path = "fonts/FiraSans-Bold.ttf")]
    pub font: Handle<Font>,

    #[asset(path = "model/player.glb#Scene0")]
    pub player_model: Handle<Scene>,

    // ...
}
```

### 资源路径
所有路径相对于 workspace 根目录的 `assets/`：
```rust
assets.load("model/player.glb#Scene0")  // ✅ 正确
assets.load("../assets/model/player.glb")  // ❌ 错误
```

---

## 常见任务

### 添加新的游戏系统
1. 在 `crates/game/src/` 创建新模块
2. 定义组件和系统
3. 创建插件并实现 `Plugin` trait
4. 在 `crates/game/src/lib.rs` 注册插件

### 修改状态机
编辑 `crates/game/src/core/state.rs`：
- 添加新状态到 `GameState` 枚举
- 在 `StatePlugin` 中添加状态转换逻辑

### 调整相机参数
编辑 `crates/game/src/camera/isometric.rs`：
```rust
pub struct IsometricCamera {
    pub offset: Vec3,          // 相机偏移（默认: 0, 12, 10）
    pub follow_speed: f32,     // 跟随速度（默认: 5.0）
    pub pitch: f32,            // 俯仰角（默认: -45°）
}
```

### 添加 UI 元素
在 `crates/game/src/ui/` 对应模块：
- `menu/` - 菜单相关
- `hud/` - 游戏内 HUD

---

## 性能优化

### 编译优化
- 开发模式自动使用 `dylib`（通过 `--features dev`）
- 发布模式自动静态链接 + LTO
- 链接器已配置（lld/zld/rust-lld）

### 运行时优化
- 使用 `.in_set()` 组织相关系统
- 使用 `.after()` / `.before()` 明确依赖
- 查询使用 `With<T>` / `Without<T>` 过滤

---

## 调试工具

### Inspector（可选）
```bash
cargo run --features dev,inspector
```

### 日志级别
```bash
# 设置日志级别
RUST_LOG=debug cargo run --features dev
RUST_LOG=info,wgpu=error cargo run --features dev
```

---

## 发布流程

### Git 标签触发自动发布
```bash
git tag v0.2.0
git push origin v0.2.0
```

GitHub Actions 将构建：
- Windows (.exe)
- Linux (AppImage)
- macOS (.dmg)
- Web (Wasm)

---

## 依赖版本

| 库 | 版本 | 说明 |
|----|----|------|
| bevy | 0.17.2 | 游戏引擎 |
| bevy_asset_loader | 0.24.0-rc.1 | 资源预加载 |
| bevy_kira_audio | 0.24.0 | 音频播放 |
| rand | 0.9.2 | 随机数生成 |
| bevy-inspector-egui | 0.34.0 | 调试工具（dev） |

---

## 注意事项

### Workspace 相关
- 所有 `cargo` 命令默认在 workspace 根目录执行
- 使用 `-p <package>` 指定特定 crate
- 使用 `--workspace` 操作所有 crate

### 动态链接限制
- Windows Release 模式不支持 `dylib`
- 仅在开发模式使用 `--features dev`

### 资源加载
- 确保所有资源在 `AssetLoading` 状态加载完成
- 使用 `AssetCollection` 组织资源

### 状态驱动
- 所有游戏逻辑系统应添加 `.run_if(in_state(GameState::Playing))`
- 避免手动检查状态，使用 Bevy 的状态过滤

---

## 快速参考

### 查找信息
- **架构问题** → `docs/WORKSPACE_ARCHITECTURE.md`
- **游戏逻辑** → `docs/ARCHITECTURE_BEST_PRACTICE.md`
- **实施步骤** → `docs/IMPLEMENTATION_GUIDE.md`
- **快速查询** → `docs/README.md`

### 社区资源
- [Bevy Discord](https://discord.gg/bevy) - #help 频道
- [Bevy Examples](https://github.com/bevyengine/bevy/tree/main/examples)
- [Bevy Cheat Book](https://bevy-cheatbook.github.io/)

---

## 代码提交规范

遵循 Conventional Commits：
```
feat(camera): 实现斜向俯视相机系统
fix(player): 修复边界碰撞检测
refactor(ui): 重构菜单系统为组件化结构
docs: 更新架构文档
```

---

最后更新：2025-10-19
Bevy 版本：0.17.2
架构版本：v2.0 (Workspace)
