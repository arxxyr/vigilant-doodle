use vigilant_doodle_core::state::GameState;
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
    /// 玩家模型
    pub player_model: Handle<Scene>,
    /// 敌人模型
    pub enemy_model: Handle<Scene>,
    /// 刀模型
    pub knife_model: Handle<Scene>,
}

/// 加载资源
pub fn load_assets(mut commands: Commands, asset_server: Res<AssetServer>) {
    info!("[Assets] 开始加载资源...");

    let assets = GameAssets {
        font: asset_server.load("fonts/SarasaTermSCNerd/SarasaTermSCNerd-Regular.ttf"),
        player_model: asset_server.load("model/player.glb#Scene0"),
        enemy_model: asset_server.load("model/enemy.glb#Scene0"),
        knife_model: asset_server.load("model/knife_sharp.glb#Scene0"),
    };

    commands.insert_resource(assets);

    info!("[Assets] 资源句柄已创建（字体 + 3个模型）");
}

/// 检查资源是否加载完成，完成后切换到主菜单
fn check_assets_ready(
    asset_server: Res<AssetServer>,
    assets: Res<GameAssets>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    // 检查所有资源是否加载完成
    let font_loaded = asset_server
        .get_load_state(&assets.font)
        .is_some_and(|s| s.is_loaded());
    let player_loaded = asset_server
        .get_load_state(&assets.player_model)
        .is_some_and(|s| s.is_loaded());
    let enemy_loaded = asset_server
        .get_load_state(&assets.enemy_model)
        .is_some_and(|s| s.is_loaded());
    let knife_loaded = asset_server
        .get_load_state(&assets.knife_model)
        .is_some_and(|s| s.is_loaded());

    if font_loaded && player_loaded && enemy_loaded && knife_loaded {
        info!("[Assets] 所有资源加载完成，切换到主菜单");
        next_state.set(GameState::MainMenu);
    }
}
