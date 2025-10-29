# CLAUDE.md

本文件为 Claude Code 提供项目指引。

## 项目概述

**Vigilant Doodle** 是一个基于 Bevy 0.17.2 的 3D 俯视角动作游戏，采用 Cargo Workspace 架构。

### 核心特性
- **引擎**：Bevy 0.17.2
- **语言**：Rust Edition 2024
- **架构**：Workspace（11 个独立 crate 模块）
- **视角**：斜向俯视 45°
- **平台**：Windows / Linux / macOS / Web
- **存档**：加密存档（AES-256-GCM + bincode 2.0）
- **本地化**：异步加载翻译文件，支持加密二进制格式

---

## 架构设计（重要）

### Workspace 结构
```
vigilant-doodle/
├── crates/
│   ├── launcher/           # 启动器（可执行程序）
│   │   └── src/main.rs     # ~100 行，负责初始化
│   ├── game/               # 游戏主 crate（插件注册中心）
│   │   └── src/
│   │       ├── lib.rs      # 插件注册
│   │       ├── save.rs     # 存档系统（顶层）
│   │       ├── enemy_setup.rs  # 敌人生成
│   │       └── bin/
│   │           └── pack_translations.rs  # 翻译打包工具
│   ├── core/               # 核心系统
│   │   └── src/
│   │       ├── state.rs    # 状态机
│   │       ├── settings.rs # 游戏设置
│   │       └── save/       # 存档加密模块
│   ├── assets/             # 资源加载（bevy_asset_loader）
│   ├── camera/             # 斜向俯视相机系统
│   ├── world/              # 世界生成与地形
│   ├── input/              # 输入处理与光标
│   ├── gameplay/           # 玩家控制逻辑
│   ├── ai/                 # 敌人 AI 行为
│   ├── ui/                 # UI 菜单与 HUD
│   └── audio/              # 音频系统（预留）
├── assets/                 # 游戏资源（共享）
├── scripts/                # 构建与打包脚本
│   ├── quick-build.sh      # 快速构建（Linux/macOS）
│   ├── package-linux.sh    # Linux 打包
│   └── package-windows.ps1 # Windows 打包
└── docs/                   # 技术文档
```

### 设计原则
1. **职责分离**：启动器只负责初始化，游戏逻辑拆分为 11 个独立 crate
2. **单向依赖**：所有 crate 依赖 core，避免循环依赖
3. **动态链接**：开发模式使用 dylib 加速编译（仅 game crate）
4. **插件化**：每个 crate 提供独立插件，由 game crate 统一注册
5. **安全存档**：使用 AES-256-GCM 加密，bincode 2.0 序列化，CRC32 校验

---

## 核心概念

### 状态机（GameState）
定义在 `crates/core/src/state.rs`：
```rust
pub enum GameState {
    AssetLoading,    // 资源加载（默认状态）
    MainMenu,        // 主菜单（游戏场景 + 模糊遮罩 + UI）
    Playing,         // 游戏进行
    Paused,          // 暂停（显示暂停菜单）
}
```

**流程**：
```
AssetLoading → MainMenu → Playing ⇄ Paused
                  ↑           ↓
                  └─── Quit ──┘
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

# 使用打包脚本（推荐）
./scripts/quick-build.sh                    # 快速构建（Linux/macOS）
./scripts/package-linux.sh [版本号]         # Linux 打包
.\scripts\package-windows.ps1 [-Version]    # Windows 打包
```

### 代码质量
```bash
cargo fmt --all
cargo clippy --workspace -- -D warnings
```

### 翻译打包工具
```bash
# 将 JSON 翻译文件转为加密二进制格式
cargo run --bin pack_translations

# 输入：assets/localization/*.json
# 输出：assets/localization/*.dat
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

## 存档与加密系统

### 存档系统架构
定义在 `crates/core/src/save/` 和 `crates/game/src/save.rs`：

```rust
// 核心加密模块（core crate）
pub struct SaveManager {
    save_path: PathBuf,
    key: [u8; 32],  // AES-256 密钥
}

// 游戏存档数据（game crate）
pub struct SaveData {
    pub version: u32,
    pub timestamp: u64,
    pub player: PlayerSaveData,
    pub enemies: Vec<EnemySaveData>,
    pub has_active_game: bool,
}
```

### 加密流程
1. **序列化**：使用 bincode 2.0 将数据结构转为二进制
2. **加密**：使用 AES-256-GCM 加密二进制数据
3. **校验**：计算 CRC32 校验和
4. **存储**：写入 `save.dat` 文件

### 翻译打包工具
定义在 `crates/game/src/bin/pack_translations.rs`：

```bash
# 运行工具
cargo run --bin pack_translations

# 工作流程：
# 1. 读取 assets/localization/*.json
# 2. 使用 AES-256-GCM 加密
# 3. 输出 assets/localization/*.dat
```

**加密原因**：
- 防止用户直接修改翻译文件
- 减小文件体积（二进制格式）
- 加快加载速度（无需 JSON 解析）

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
1. 在对应的 `crates/` 目录创建或修改模块
2. 定义组件和系统
3. 创建插件并实现 `Plugin` trait
4. 在 `crates/game/src/lib.rs` 注册插件

### 修改状态机
编辑 `crates/core/src/state.rs`：
- 添加新状态到 `GameState` 枚举
- 在 `StatePlugin` 中添加状态转换逻辑

### 调整相机参数
编辑 `crates/camera/src/isometric.rs`：
```rust
pub struct IsometricCamera {
    pub offset: Vec3,          // 相机偏移（默认: 0, 12, 10）
    pub follow_speed: f32,     // 跟随速度（默认: 5.0）
    pub pitch: f32,            // 俯仰角（默认: -45°）
}
```

### 添加 UI 元素
在 `crates/ui/src/` 对应模块：
- `menu/` - 菜单相关
- `hud/` - 游戏内 HUD
- `pause/` - 暂停菜单

### 存档系统使用
```rust
use vigilant_doodle_core::save::SaveManager;

// 保存游戏
fn save_game(save_manager: Res<SaveManager>, /* 其他查询 */) {
    let save_data = SaveData { /* ... */ };
    if let Err(e) = save_manager.save(&save_data) {
        error!("[Save] 保存失败: {}", e);
    }
}

// 加载游戏
fn load_game(save_manager: Res<SaveManager>) -> Option<SaveData> {
    save_manager.load().ok()
}
```

存档位置：
- Windows: `%APPDATA%/vigilant-doodle/save.dat`
- Linux: `~/.local/share/vigilant-doodle/save.dat`
- macOS: `~/Library/Application Support/vigilant-doodle/save.dat`

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
| bevy_egui | 0.38.0 | Inspector UI |
| bevy-inspector-egui | 0.35.0 | 调试工具（dev） |
| rand | 0.9.2 | 随机数生成 |
| bincode | 2.0.1 | 二进制序列化 |
| aes-gcm | 0.10 | 加密存档 |
| crc32fast | 1.4 | 校验和计算 |
| serde / serde_json | 1.0 | JSON 序列化 |
| dirs | 6.0.0 | 系统目录获取 |

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

遵循 Conventional Commits（**不使用 scope 括号**）：
```
feat: 实现斜向俯视相机系统
fix: 修复边界碰撞检测
refactor: 重构菜单系统为组件化结构
docs: 更新架构文档
```

**类型说明**：
- `feat` - 新功能
- `fix` - 修复 Bug
- `refactor` - 重构代码
- `docs` - 文档更新
- `test` - 测试相关
- `chore` - 构建/工具相关
- `style` - 代码格式
- `perf` - 性能优化
- `build` - 构建系统
- `ci` - CI/CD 相关
- `revert` - 回滚提交

**重要**：根据项目配置，commit message 中**不包含** Claude Code 的署名信息。

---

---

**最后更新**：2025-10-30
**项目版本**：v0.3.0
**Bevy 版本**：0.17.2
**架构版本**：v3.0 (11-Crate Workspace + 完整存档系统 + AI 系统)

**项目状态**：✅ 生产就绪
- 核心功能完整（玩家移动、敌人 AI、暂停菜单、存档系统）
- 跨平台构建脚本完成（Linux/Windows）
- 多语言支持完整（中文/English）
- 性能优化完成（移动抖动修复、异步加载）
