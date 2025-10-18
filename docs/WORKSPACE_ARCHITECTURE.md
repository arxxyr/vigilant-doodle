# Vigilant Doodle - Workspace 架构设计

## 架构概述

采用 **Cargo Workspace + 动态库** 架构，将启动器与游戏逻辑完全分离。

### 核心理念
1. **启动器**：轻量级可执行程序，负责初始化引擎和加载游戏库
2. **游戏逻辑**：独立的动态库，包含所有游戏代码
3. **热重载**：开发模式下支持游戏逻辑热重载（可选）
4. **模块化**：清晰的职责分离，易于维护和扩展

---

## 一、目录结构

```
vigilant-doodle/                    # Workspace 根目录
├── Cargo.toml                      # Workspace 配置
├── Cargo.lock
├── .cargo/
│   └── config.toml                 # 统一的构建配置
│
├── crates/                         # 所有 crate 放在这里
│   ├── launcher/                   # 启动器 crate（可执行程序）
│   │   ├── Cargo.toml
│   │   └── src/
│   │       └── main.rs             # 启动入口（100 行以内）
│   │
│   └── game/                       # 游戏 crate（动态库）
│       ├── Cargo.toml
│       └── src/
│           ├── lib.rs              # 游戏插件注册
│           ├── core/               # 核心系统
│           │   ├── mod.rs
│           │   ├── state.rs
│           │   └── settings.rs
│           ├── assets/             # 资源管理
│           ├── camera/             # 相机系统
│           ├── gameplay/           # 游戏玩法
│           ├── world/              # 世界管理
│           ├── ui/                 # UI 系统
│           ├── input/              # 输入管理
│           └── audio/              # 音频系统
│
├── assets/                         # 游戏资源（所有 crate 共享）
│   ├── model/
│   ├── audio/
│   ├── fonts/
│   └── textures/
│
├── docs/                           # 文档
│   ├── README.md
│   ├── ARCHITECTURE_BEST_PRACTICE.md
│   └── WORKSPACE_ARCHITECTURE.md   # 本文档
│
├── build/                          # 构建产物（图标等）
├── .github/                        # CI/CD 配置
└── README.md                       # 项目说明
```

---

## 二、Workspace 配置

### 根 Cargo.toml

```toml
[workspace]
resolver = "2"
members = [
    "crates/launcher",
    "crates/game",
]

# Workspace 级别的依赖配置（所有成员共享）
[workspace.dependencies]
bevy = { version = "0.17.2", default-features = false }
bevy_kira_audio = { version = "0.24.0" }
bevy_asset_loader = { version = "0.24.0-rc.1" }
rand = { version = "0.9.2" }

# Workspace 级别的优化配置
[workspace.package]
edition = "2024"
authors = ["ffqi <ffqi@outlook.com>"]
license = "MIT"
repository = "https://github.com/your-org/vigilant-doodle"

# 开发模式：第三方库优化，自身代码快速编译
[profile.dev.package."*"]
opt-level = 3

[profile.dev]
opt-level = 1

# 发布模式：完全优化
[profile.release]
lto = true
codegen-units = 1
opt-level = 3
strip = true  # 移除调试符号，减小体积

# 分发模式（cargo dist）
[profile.dist]
inherits = "release"
lto = "thin"
```

---

## 三、启动器 Crate（launcher）

### crates/launcher/Cargo.toml

```toml
[package]
name = "vigilant-doodle-launcher"
version = "0.2.0"
edition.workspace = true
authors.workspace = true
publish = false

# 可执行程序
[lib]
path = "src/lib.rs"

[[bin]]
name = "vigilant-doodle"
path = "src/main.rs"

[dependencies]
# 使用 workspace 依赖
bevy = { workspace = true, features = [
    "bevy_winit",
    "bevy_render",
    "bevy_core_pipeline",
    "png",
    "x11",
    "wayland",
] }

# 游戏库（动态链接）
vigilant-doodle-game = { path = "../game" }

# 窗口图标支持
winit = { version = "0.30.12", default-features = false }
image = { version = "0.25.8", default-features = false, features = ["png"] }

[features]
# 开发模式：启用动态链接加速编译
dev = ["bevy/dynamic_linking", "vigilant-doodle-game/dev"]
```

### crates/launcher/src/main.rs

```rust
//! Vigilant Doodle - 游戏启动器
//!
//! 职责：
//! - 初始化 Bevy App
//! - 配置窗口
//! - 加载游戏插件
//! - 设置窗口图标

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use bevy::prelude::*;
use bevy::window::{PrimaryWindow, WindowLevel, WindowTheme};
use bevy::winit::WinitWindows;
use std::io::Cursor;
use winit::window::Icon;

fn main() {
    // 初始化日志（可选：使用 tracing-subscriber 自定义）
    #[cfg(debug_assertions)]
    {
        std::env::set_var("RUST_LOG", "info,wgpu=error,naga=warn");
    }

    App::new()
        // Bevy 默认插件
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Vigilant Doodle".to_string(),
                resolution: (1280., 720.).into(),
                position: WindowPosition::Centered(MonitorSelection::Primary),
                window_level: WindowLevel::Normal,
                window_theme: Some(WindowTheme::Dark),
                canvas: Some("#bevy".to_owned()),  // Web 支持
                prevent_default_event_handling: false,
                ..default()
            }),
            ..default()
        }))
        // 加载游戏插件（动态库）
        .add_plugins(vigilant_doodle_game::GamePlugin)
        // 启动器专属系统
        .add_systems(Startup, set_window_icon)
        .run();
}

/// 设置窗口图标（仅 Windows 和 Linux X11）
fn set_window_icon(
    windows: Option<NonSend<WinitWindows>>,
    primary_window: Query<Entity, With<PrimaryWindow>>,
) {
    let Some(windows) = windows else { return };
    let Ok(primary_entity) = primary_window.get_single() else { return };
    let Some(primary) = windows.get_window(primary_entity) else { return };

    let icon_buf = Cursor::new(include_bytes!(
        "../../../build/macos/AppIcon.iconset/icon_256x256.png"
    ));

    if let Ok(image) = image::load(icon_buf, image::ImageFormat::Png) {
        let image = image.into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        let icon = Icon::from_rgba(rgba, width, height).unwrap();
        primary.set_window_icon(Some(icon));
    }
}
```

---

## 四、游戏 Crate（game）

### crates/game/Cargo.toml

```toml
[package]
name = "vigilant-doodle-game"
version = "0.2.0"
edition.workspace = true
authors.workspace = true
publish = false

# 库（支持动态链接）
[lib]
crate-type = ["rlib", "dylib"]  # 同时支持静态和动态链接

[dependencies]
# 使用 workspace 依赖
bevy = { workspace = true, features = [
    "animation",
    "bevy_asset",
    "bevy_scene",
    "bevy_winit",
    "bevy_core_pipeline",
    "bevy_pbr",
    "bevy_gltf",
    "bevy_render",
    "bevy_sprite",
    "bevy_text",
    "bevy_ui",
    "bevy_state",
    "serialize",
    "reflect_auto_register",
    "png",
    "hdr",
    "zstd_rust",
    "ktx2",
    "file_watcher",
    "tonemapping_luts",
] }

bevy_kira_audio = { workspace = true }
bevy_asset_loader = { workspace = true }
rand = { workspace = true }

# 开发工具（仅 debug 模式）
[dev-dependencies]
bevy-inspector-egui = "0.34.0"

[features]
# 开发模式特性
dev = ["bevy/dynamic_linking"]

# 调试工具（可选启用）
inspector = ["dep:bevy-inspector-egui"]
```

### crates/game/src/lib.rs

```rust
//! Vigilant Doodle - 游戏核心库
//!
//! 这是游戏的主入口，负责注册所有游戏插件。

#![allow(clippy::type_complexity)]

// 模块声明
mod core;
mod assets;
mod camera;
mod gameplay;
mod world;
mod ui;
mod input;
mod audio;

// 导入
use bevy::prelude::*;
use core::state::StatePlugin;
use assets::loader::AssetLoaderPlugin;
use camera::isometric::IsometricCameraPlugin;
use gameplay::{player::PlayerPlugin, enemy::EnemyPlugin, movement::MovementPlugin};
use world::spawning::SpawningPlugin;
use ui::menu::{OverlayPlugin, MainMenuPlugin};
use input::{actions::InputPlugin, cursor::CursorPlugin};
use audio::manager::AudioPlugin;

/// 游戏主插件
///
/// 这是唯一对外暴露的接口，启动器通过它加载所有游戏逻辑。
pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        info!("[Game] Loading game plugin...");

        app
            // 1. 核心系统（状态机）
            .add_plugins(StatePlugin)

            // 2. 资源加载
            .add_plugins(AssetLoaderPlugin)

            // 3. 世界生成（实体生成）
            .add_plugins(SpawningPlugin)

            // 4. 相机系统
            .add_plugins(IsometricCameraPlugin)

            // 5. 输入管理
            .add_plugins((InputPlugin, CursorPlugin))

            // 6. 游戏玩法
            .add_plugins((PlayerPlugin, EnemyPlugin, MovementPlugin))

            // 7. UI 系统
            .add_plugins((OverlayPlugin, MainMenuPlugin))

            // 8. 音频系统
            .add_plugins(AudioPlugin);

        // 调试工具（仅 debug 模式）
        #[cfg(debug_assertions)]
        {
            app.add_plugins((
                bevy::diagnostic::FrameTimeDiagnosticsPlugin::default(),
                bevy::diagnostic::LogDiagnosticsPlugin::default(),
            ));
        }

        // Inspector（可选，通过 feature 启用）
        #[cfg(feature = "inspector")]
        {
            app.add_plugins(bevy_inspector_egui::quick::WorldInspectorPlugin::new());
        }

        info!("[Game] Game plugin loaded successfully");
    }
}

// 导出公共类型（如果需要被启动器或其他模块使用）
pub use core::state::{GameState, MenuState};
```

---

## 五、模块骨架示例

### crates/game/src/core/mod.rs

```rust
pub mod state;
pub mod settings;
```

### crates/game/src/core/state.rs

```rust
use bevy::prelude::*;

#[derive(States, Default, Clone, Eq, PartialEq, Debug, Hash)]
pub enum GameState {
    #[default]
    AssetLoading,
    MainMenu,
    Playing,
}

#[derive(States, Default, Clone, Eq, PartialEq, Debug, Hash)]
pub enum MenuState {
    #[default]
    Disabled,
    Main,
    Settings,
    SettingsDisplay,
    SettingsSound,
}

pub struct StatePlugin;

impl Plugin for StatePlugin {
    fn build(&self, app: &mut App) {
        app
            .init_state::<GameState>()
            .init_state::<MenuState>()
            .add_systems(OnEnter(GameState::AssetLoading), log_enter_loading)
            .add_systems(OnEnter(GameState::MainMenu), log_enter_menu)
            .add_systems(OnEnter(GameState::Playing), log_enter_playing);
    }
}

fn log_enter_loading() {
    info!("[State] → AssetLoading");
}

fn log_enter_menu() {
    info!("[State] → MainMenu");
}

fn log_enter_playing() {
    info!("[State] → Playing");
}
```

### 其他模块占位符

```rust
// crates/game/src/assets/mod.rs
pub mod loader;

// crates/game/src/camera/mod.rs
pub mod isometric;
pub mod components;

// crates/game/src/gameplay/mod.rs
pub mod player;
pub mod enemy;
pub mod movement;

// crates/game/src/world/mod.rs
pub mod spawning;
pub mod terrain;

// crates/game/src/ui/mod.rs
pub mod menu;
pub mod hud;
pub mod styles;

// crates/game/src/input/mod.rs
pub mod actions;
pub mod cursor;

// crates/game/src/audio/mod.rs
pub mod manager;
```

---

## 六、构建与运行

### 开发模式（动态链接，快速编译）

```bash
# 运行启动器（自动编译游戏库）
cargo run -p vigilant-doodle-launcher --features dev

# 或者使用 workspace 级别命令
cargo run --features dev
```

### 发布模式（静态链接，完全优化）

```bash
cargo build --release -p vigilant-doodle-launcher
```

### 单独编译游戏库

```bash
# 仅编译游戏库（用于测试）
cargo build -p vigilant-doodle-game

# 检查游戏库代码
cargo clippy -p vigilant-doodle-game
```

### Workspace 命令

```bash
# 编译所有 crate
cargo build --workspace

# 测试所有 crate
cargo test --workspace

# 格式化所有代码
cargo fmt --all

# Clippy 检查
cargo clippy --workspace -- -D warnings
```

---

## 七、优势分析

### 1. 职责清晰
| Crate | 职责 | 代码量 |
|-------|------|--------|
| `launcher` | 初始化引擎、窗口配置 | ~100 行 |
| `game` | 所有游戏逻辑 | 主要代码 |

### 2. 编译效率
- **增量编译**：修改游戏代码时，启动器无需重新编译
- **并行编译**：多个 crate 可以并行编译
- **动态链接**：开发模式下使用 `dylib`，编译速度提升 3-5 倍

### 3. 可扩展性
```
未来可以添加：
crates/
├── launcher/        # 主启动器
├── game/            # 游戏 A
├── game-editor/     # 关卡编辑器
├── game-server/     # 多人服务器
└── shared/          # 共享代码
```

### 4. 热重载支持（高级）
使用 `bevy_dylib` 或自定义方案，开发时可以：
1. 启动游戏
2. 修改游戏代码
3. 重新编译游戏库
4. 游戏自动重新加载（不重启）

### 5. 分发优化
- **Release 构建**：启动器和游戏库静态链接为单个可执行文件
- **模块化更新**：理论上可以单独更新游戏库（需要额外的加载器设计）

---

## 八、注意事项

### 1. 资源路径
所有 crate 共享 `assets/` 目录，相对路径从 workspace 根目录开始：

```rust
// ✅ 正确
assets.load("model/player.glb#Scene0")

// ❌ 错误
assets.load("../../../assets/model/player.glb#Scene0")
```

### 2. 依赖版本
使用 `workspace = true` 确保所有 crate 使用相同的依赖版本：

```toml
[dependencies]
bevy = { workspace = true, features = [...] }
```

### 3. Feature 传递
启动器的 `dev` feature 需要传递给游戏库：

```toml
# crates/launcher/Cargo.toml
[features]
dev = ["bevy/dynamic_linking", "vigilant-doodle-game/dev"]
```

### 4. 动态库限制
- Windows Release 模式下 `dylib` 可能有问题，仅用于开发
- Release 构建使用 `rlib`（静态链接）

---

## 九、迁移步骤

### 步骤 1：创建 Workspace 结构
```bash
# 创建 crates 目录
mkdir crates

# 创建启动器
cargo new crates/launcher --name vigilant-doodle-launcher

# 创建游戏库
cargo new crates/game --lib --name vigilant-doodle-game
```

### 步骤 2：修改根 Cargo.toml
将现有的 `Cargo.toml` 改为 workspace 配置（参考第二章）

### 步骤 3：迁移代码
```bash
# 将现有 src/ 移动到游戏库
mv src crates/game/

# 清理启动器代码
# 将 main.rs 简化为启动入口
```

### 步骤 4：更新配置
- 修改 `crates/launcher/Cargo.toml`
- 修改 `crates/game/Cargo.toml`
- 更新 `.cargo/config.toml`

### 步骤 5：测试编译
```bash
cargo build --workspace
cargo run --features dev
```

---

## 十、最佳实践

### 1. 依赖管理
- 所有共享依赖在 `[workspace.dependencies]` 中定义
- 特定 crate 的依赖写在各自的 `Cargo.toml` 中

### 2. 版本管理
- Workspace 级别统一版本号（`workspace.package.version`）
- 所有 crate 使用 `version.workspace = true`

### 3. 代码组织
- 启动器尽量保持简洁（< 200 行）
- 游戏库按模块划分（参考 ARCHITECTURE_BEST_PRACTICE.md）

### 4. 构建配置
- 开发模式：`dev` feature 启用动态链接
- 发布模式：默认静态链接

---

## 十一、CI/CD 配置

### GitHub Actions 示例

```yaml
# .github/workflows/ci.yml
name: CI

on: [push, pull_request]

jobs:
  build:
    runs-on: ${{ matrix.os }}
    strategy:
      matrix:
        os: [ubuntu-latest, windows-latest, macos-latest]

    steps:
      - uses: actions/checkout@v3
      - uses: dtolnay/rust-toolchain@stable

      - name: Cache Cargo registry
        uses: actions/cache@v3
        with:
          path: ~/.cargo/registry
          key: ${{ runner.os }}-cargo-registry

      - name: Build (dev mode)
        run: cargo build --workspace --features dev

      - name: Run tests
        run: cargo test --workspace

      - name: Clippy
        run: cargo clippy --workspace -- -D warnings

      - name: Build (release mode)
        run: cargo build --release -p vigilant-doodle-launcher
```

---

## 总结

这个 Workspace 架构具有以下优势：

1. ✅ **职责分离**：启动器只负责初始化，游戏逻辑独立
2. ✅ **编译效率**：增量编译、动态链接、并行构建
3. ✅ **可扩展性**：易于添加新的 crate（编辑器、服务器等）
4. ✅ **模块化**：清晰的依赖关系
5. ✅ **专业性**：符合商业项目结构

这是 Rust 大型项目的标准做法，完全适合商业开发！
