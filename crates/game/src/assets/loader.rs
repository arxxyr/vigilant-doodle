use crate::core::state::GameState;
use bevy::prelude::*;

/// 资源加载插件
pub struct AssetLoaderPlugin;

impl Plugin for AssetLoaderPlugin {
    fn build(&self, app: &mut App) {
        app
            // 进入 AssetLoading 状态时开始加载资源
            .add_systems(OnEnter(GameState::AssetLoading), load_assets)
            // 检查资源加载状态
            .add_systems(
                Update,
                check_assets_ready.run_if(in_state(GameState::AssetLoading)),
            );
    }
}

/// 游戏资源集合
#[derive(Resource)]
pub struct GameAssets {
    /// 主字体
    pub font: Handle<Font>,
}

/// 加载资源
fn load_assets(mut commands: Commands, asset_server: Res<AssetServer>) {
    info!("[Assets] 开始加载资源...");

    let assets = GameAssets {
        font: asset_server.load("fonts/SarasaTermSCNerd/SarasaTermSCNerd-Regular.ttf"),
    };

    commands.insert_resource(assets);

    info!("[Assets] 资源句柄已创建");
}

/// 检查资源是否加载完成，完成后切换到主菜单
fn check_assets_ready(
    asset_server: Res<AssetServer>,
    assets: Res<GameAssets>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    // 检查字体是否加载完成
    if let Some(state) = asset_server.get_load_state(&assets.font) {
        if state.is_loaded() {
            info!("[Assets] 所有资源加载完成，切换到主菜单");
            next_state.set(GameState::MainMenu);
        }
    }
}
