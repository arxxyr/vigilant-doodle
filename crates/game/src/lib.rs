//! Vigilant Doodle - 游戏集成库
//!
//! 基于 Bevy 0.17.2 的游戏引擎，采用 Workspace 架构。
//! 这是游戏的集成层，负责整合所有子 crates。

#![allow(clippy::type_complexity)]

// ============================================================================
// 本地模块
// ============================================================================

mod enemy_setup;
mod save;

// ============================================================================
// 导入所有子 crates
// ============================================================================

use bevy::prelude::*;

// 核心系统
use vigilant_doodle_core::{LocalizationPlugin, SavePlugin, StatePlugin};

// 本地模块插件
use enemy_setup::EnemySetupPlugin;
use save::GameSavePlugin;

// 资源加载
use vigilant_doodle_assets::AssetLoaderPlugin;

// 相机系统
use vigilant_doodle_camera::IsometricCameraPlugin;

// 世界生成
use vigilant_doodle_world::SpawningPlugin;

// 输入系统
use vigilant_doodle_input::{CursorPlugin, InputPlugin};

// 游戏玩法
use vigilant_doodle_gameplay::{EnemyPlugin, MovementPlugin, PlayerPlugin};

// AI 系统
use vigilant_doodle_ai::EnemyAIPlugin;

// UI 系统
use vigilant_doodle_ui::{SettingsMenuPlugin, SimpleMenuPlugin};

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
            // 1. 核心系统（状态机、本地化、存档管理器）
            .add_plugins((StatePlugin, LocalizationPlugin, SavePlugin, GameSavePlugin))
            // 2. 资源加载
            .add_plugins(AssetLoaderPlugin)
            // 3. 相机系统（斜向俯视）
            .add_plugins(IsometricCameraPlugin)
            // 4. 世界生成（地形、光照）
            .add_plugins(SpawningPlugin)
            // 5. 游戏玩法（玩家、敌人、移动）
            .add_plugins((PlayerPlugin, EnemyPlugin, MovementPlugin))
            // 6. 敌人 AI 设置（添加 AI 组件）
            .add_plugins(EnemySetupPlugin)
            // 7. AI 系统（敌人行为）
            .add_plugins(EnemyAIPlugin)
            // 8. 输入系统（键盘、鼠标、光标）
            .add_plugins((InputPlugin, CursorPlugin))
            // 9. UI系统（主菜单、设置菜单）
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

pub use vigilant_doodle_core::GameState;
