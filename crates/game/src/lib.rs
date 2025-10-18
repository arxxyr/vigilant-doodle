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
// use ui::menu::{OverlayPlugin, MainMenuPlugin};
use input::{actions::InputPlugin, cursor::CursorPlugin};
// use audio::manager::AudioPlugin;

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

            // 7. UI 系统（待实现）
            // .add_plugins((OverlayPlugin, MainMenuPlugin))

            // 8. 音频系统（待实现）
            // .add_plugins(AudioPlugin)
            ;

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
