# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## 项目概述

这是一个基于 **Bevy 0.17.2** 引擎的 3D 游戏项目，使用 **Rust Edition 2024**，特点：
- 玩家角色（XiaoYu）带有探照灯，可在地板上移动
- 敌人（Enemy）会追逐玩家
- 使用 ECS（Entity-Component-System）架构
- 支持多平台：Linux、Windows、macOS、Web（Wasm）、Android、iOS

## 核心架构

### 状态机（GameState）
位于 `src/lib.rs:35-44`，定义三个状态：
- `Loading`：资源加载阶段（默认入口）
- `Playing`：游戏运行阶段
- `Menu`：菜单界面

### 系统执行顺序（MovementSystem）
位于 `src/lib.rs:48-60`，使用 `.chain()` 保证执行顺序：
```
Input → Player → Enemy → PlayerMovement → EnemyMovement → Map
```

### 插件架构
主插件 `GamePlugin` (src/lib.rs:76-110) 按顺序加载：
1. `LoadingPlugin` - 资源加载
2. `MenuPlugin` - 菜单系统
3. `ActionsPlugin` - 输入处理
4. `WorldPlugin` - 世界生成（地板）
5. `PlayerCameraPlugin` - 相机控制
6. `InternalAudioPlugin` - 音频系统
7. `PlayerPlugin` - 玩家逻辑
8. `EnemiesPlugin` - 敌人逻辑
9. `ThirdPersonCameraPlugin` - 第三人称相机
10. `WorldInspectorPlugin` - 调试专用（仅 debug 模式）

### 关键模块

#### Player 模块 (src/player/)
- `Player` 组件：玩家标记
- `Speed` 组件：移动速度（默认 10.0）
- 系统：`move_player`、`player_collision_detection`
- 玩家携带一个 SpotLight 子实体（探照灯）
- 边界检测：限制在 `(FLOOR_LENGTH-2.0)/2.0` 范围内

#### Enemy 模块 (src/enemies/)
- `Enemy` 组件：敌人标记
- `EnemySpeed` 组件：移动速度（默认 9.0）
- `EnemyTarget` 组件：追逐目标（玩家位置）
- 系统：`update_player`、`enemies_movement`、`enemy_collision_detection`
- 敌人会在距离玩家 1.0 米时停止移动

#### World 模块 (src/world/)
- `Floor` 组件：地板尺寸信息
- 常量：`FLOOR_LENGTH = 50.0`, `FLOOR_WIDTH = 25.0`
- 地板位于 Y=-0.5，灰色材质

## 构建与运行

### 开发环境
- **Rust 工具链**：nightly（推荐）
- **必需工具**：
  - 链接器：Linux 使用 `lld`，macOS 使用 `zld`，Windows 使用 `rust-lld.exe`
  - 配置见 `.cargo/config.toml`

### 常用命令

#### 本地开发（Native）
```bash
# 快速运行（开发模式，使用动态链接加速编译）
cargo run --features dev

# 发布构建（启用 LTO 优化）
cargo build --release

# 仅编译不运行
cargo build
```

#### Web 构建（Wasm）
```bash
# 安装依赖
cargo install --locked trunk
rustup target add wasm32-unknown-unknown

# 启动开发服务器（端口 8080，热重载）
trunk serve

# 构建 Web 版本
trunk build --release
```

#### 移动平台
```bash
# Android
cargo apk run -p mobile

# iOS（需要 macOS + Xcode）
cd mobile && make run
```

### VSCode 调试配置
`.vscode/launch.json` 提供三种配置：
- **Quick Launch**：快速启动（无调试器等待）
- **Debug**：完整调试（LLDB）
- **Debug unit tests**：单元测试调试

## 性能优化配置

### Cargo.toml 优化
- **开发模式依赖**：`opt-level = 3`（加速第三方库）
- **开发模式自身**：`opt-level = 1`
- **发布模式**：`lto = true`, `codegen-units = 1`, `opt-level = 3`

### 链接器配置（.cargo/config.toml）
- Linux：clang + lld + `-Zshare-generics=y`
- macOS：zld（需手动安装：`brew install michaeleisel/zld/zld`）
- Windows：rust-lld.exe + `-Zshare-generics=off`

## 代码约定

### 命名规范
- 组件/插件：`UpperCamelCase`（如 `PlayerPlugin`, `Enemy`）
- 系统函数：`snake_case`（如 `move_player`, `spawn_floor`）

### 系统注册模式
```rust
app.add_systems(
    OnEnter(GameState::Loading),  // 状态进入时执行
    spawn_player
)
.add_systems(
    Update,
    (system_a, system_b)
        .chain()  // 按顺序执行
        .in_set(MovementSystem::PlayerMovement)  // 归属系统集
        .run_if(in_state(GameState::Playing))  // 条件执行
);
```

### 实体生命周期
- **Spawn**：在 `OnEnter(GameState::Loading)` 创建实体
- **Show**：在 `OnEnter(GameState::Playing)` 设置 `Visibility::Visible`
- **Hide**：在 `OnExit(GameState::Playing)` 设置 `Visibility::Hidden`
- 避免频繁创建/销毁实体，使用可见性控制

### 资源加载
```rust
// 3D 模型（GLTF）
assets.load("model/player.glb#Scene0")

// 音频/纹理
assets.load("audio/flying.ogg")
```
资源目录：`assets/{audio, fonts, model, textures}`

## 调试工具（仅 Debug 模式）

### 运行时诊断
- `WorldInspectorPlugin`：实体查看器（egui）
- `FrameTimeDiagnosticsPlugin`：帧率监控
- `LogDiagnosticsPlugin`：日志输出

### 快捷键
- **ESC**：退出游戏（`bevy::window::close_on_esc` 系统）

## 常见修改场景

### 添加新实体类型
1. 在相应模块定义 `Component`（如 `src/player/weapon.rs`）
2. 创建 `Plugin` 并实现 `fn build()`
3. 在 `src/lib.rs` 的 `GamePlugin` 中注册插件
4. 遵循 `spawn → show → hide` 生命周期

### 修改地板尺寸
编辑 `src/world/world.rs:20-21`：
```rust
pub const FLOOR_LENGTH: f32 = 50.0;
pub const FLOOR_WIDTH: f32 = 25.0;
```
注意：玩家和敌人的碰撞检测会自动适配

### 调整游戏速度
- 玩家速度：`src/player/player.rs:64` → `Speed(10.0)`
- 敌人速度：`src/enemies/enemies.rs:49` → `EnemySpeed(9.0)`

### 添加新的 SystemSet
1. 在 `src/lib.rs:48-60` 的 `MovementSystem` 枚举中添加变体
2. 在 `src/lib.rs:79-89` 的 `.configure_sets()` 中添加到执行链

## 发布流程

### GitHub Actions
推送格式为 `v[0-9]+.[0-9]+.[0-9]+*` 的标签（如 `v0.2.0`）将触发自动构建：
- Windows（exe）
- Linux（AppImage）
- macOS（dmg）
- Web（Wasm，可部署到 GitHub Pages）

### 手动部署到 GitHub Pages
运行 workflow：`.github/workflows/deploy-github-page`

## Bevy 0.17.2 迁移要点

### Feature 名称变更
- `bevy/bevy_dylib` → `bevy/dynamic_linking`
- `filesystem_watcher` → `file_watcher`
- `zstd` → `zstd_rust`（或 `zstd_c` 用于 C 绑定）

### API 变更
- **时间 API**：`delta_seconds()` → `delta_secs()`
- **查询 API**：`get_single()` → `single()`, `get_single_mut()` → `single_mut()`
- **颜色 API**：
  - `Color::rgb()` → `Color::srgb()`
  - `Color::GRAY` → `bevy::color::palettes::css::GRAY`（需手动导入）
  - 颜色常量现在在 `bevy::color::palettes::css` 模块中
- **形状 API**：`shape::Plane` → `Plane3d::default().mesh().size(x, y).build()`
- **文本 API**：
  - `TextBundle::from_section()` 已移除
  - 新写法：`(Text::new("text"), TextFont{font, font_size, ..}, TextColor(color), Node{..})`
- **实体 API**：
  - `despawn_recursive()` → `despawn()`（现自动递归）
  - `UiImage` → `ImageNode`
- **状态 API**：
  - `add_state()` → `init_state()`
  - 需要导入：`use bevy_state::prelude::*;`（用于 OnEnter, OnExit, in_state 等）
- **应用退出**：`AppExit` → `AppExit::Success` 或 `AppExit::Error`

### 平台支持
- 默认启用 `x11` 和 `wayland`（Ubuntu 22.04 推荐）
- 如遇兼容性问题可切换显示服务器

## 注意事项

1. **动态链接限制**：`Cargo.toml:52` 的 `dynamic_linking` 特性已注释，Windows release 构建时会出现 bug
2. **音频问题**：部分浏览器的 Web 构建可能有音频性能问题（Firefox 已知问题）
3. **模型文件**：确保 GLTF 文件路径正确（`model/player.glb#Scene0` 中的 `Scene0` 是场景索引）
4. **边界碰撞**：玩家和敌人使用相同的边界检测逻辑（`FLOOR_*-2.0`），留有 0.2 的安全边距
5. **重力模拟**：玩家的 Y 轴移动包含简单的重力计算（`-9.81 * delta_secs`），但地面限制在 Y=0.0

## 依赖关键库
- `bevy` 0.17.2：核心引擎
- `bevy_kira_audio` 0.24.0：音频系统
- `bevy_asset_loader` 0.23.0：资源加载器
- `bevy_third_person_camera` 0.3.0：第三人称相机
- `bevy-inspector-egui` 0.34.0：调试 UI
- `rand` 0.9.2：随机数生成（敌人生成位置）
- 根据最新的文档改,不要简化功能,不要省略,不要待定,要完整的实现