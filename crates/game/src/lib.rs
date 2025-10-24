//! Vigilant Doodle - 游戏核心库
//!
//! 基于 Bevy 0.17.2 的游戏引擎，采用 Workspace 架构。
//! 当前仅包含资源加载和主菜单系统。

#![allow(clippy::type_complexity)]

// ============================================================================
// 模块声明
// ============================================================================

mod assets;
mod camera;
mod core;
mod ui;
mod world;

// 游戏玩法模块
mod gameplay;
// 音频模块（当前为空）
#[allow(unused)]
mod audio;
// 输入模块
mod input;

// ============================================================================
// 导入
// ============================================================================

use assets::loader::AssetLoaderPlugin;
use bevy::prelude::*;
use camera::IsometricCameraPlugin;
use core::localization::LocalizationPlugin;
use core::state::StatePlugin;
use gameplay::{EnemyPlugin, MovementPlugin, PlayerPlugin};
use input::{CursorPlugin, InputPlugin};
use ui::settings_menu::SettingsMenuPlugin;
use ui::simple_menu::SimpleMenuPlugin;
use world::SpawningPlugin;

// ============================================================================
// 游戏主插件
// ============================================================================

/// 游戏主插件
///
/// 这是唯一对外暴露的接口，启动器通过它加载所有游戏逻辑。
pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        info!("[Game] 加载游戏插件...");

        app
            // 1. 核心系统（状态机、本地化）
            .add_plugins((StatePlugin, LocalizationPlugin))
            // 2. 资源加载
            .add_plugins(AssetLoaderPlugin)
            // 3. 相机系统（斜向俯视）
            .add_plugins(IsometricCameraPlugin)
            // 4. 世界生成（地形、光照）
            .add_plugins(SpawningPlugin)
            // 5. 游戏玩法（玩家、敌人、移动）
            .add_plugins((PlayerPlugin, EnemyPlugin, MovementPlugin))
            // 6. 输入系统（键盘、鼠标、光标）
            .add_plugins((InputPlugin, CursorPlugin))
            // 7. UI系统（主菜单、设置菜单）
            .add_plugins((SimpleMenuPlugin, SettingsMenuPlugin));

        // Inspector 工具（可选启用）
        #[cfg(feature = "inspector")]
        {
            app.add_plugins(bevy_inspector_egui::DefaultInspectorConfigPlugin);
            info!("[Game] Inspector 已启用");
        }

        // 调试工具（仅 debug 模式）
        #[cfg(debug_assertions)]
        {
            app.add_plugins((
                bevy::diagnostic::FrameTimeDiagnosticsPlugin::default(),
                bevy::diagnostic::LogDiagnosticsPlugin::default(),
            ));
        }

        info!("[Game] 游戏插件加载完成");
    }
}

// ============================================================================
// 导出公共类型
// ============================================================================

pub use core::state::GameState;
