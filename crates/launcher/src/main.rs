//! Vigilant Doodle - 启动器
//!
//! 负责初始化 Bevy 引擎并加载游戏库。

use bevy::prelude::*;
use vigilant_doodle_game::GamePlugin;

// ============================================================================
// 全局内存分配器
// ============================================================================

use mimalloc::MiMalloc;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

fn main() {
    info!("[Launcher] 正在启动 Vigilant Doodle...");

    // 获取 workspace 根目录的 assets 路径
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let assets_path = std::path::Path::new(manifest_dir)
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .join("assets");

    info!("[Launcher] Assets path: {:?}", assets_path);

    App::new()
        // 默认插件（窗口、渲染、输入等）
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Vigilant Doodle".to_string(),
                        resolution: (1280, 720).into(),
                        ..default()
                    }),
                    ..default()
                })
                .set(AssetPlugin {
                    file_path: assets_path.to_string_lossy().to_string(),
                    ..default()
                }),
        )
        // 游戏插件（包含所有游戏逻辑）
        .add_plugins(GamePlugin)
        .run();

    info!("[Launcher] 游戏已退出");
}
