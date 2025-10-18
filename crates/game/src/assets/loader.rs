use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_kira_audio::AudioSource;
use crate::core::state::GameState;

pub struct AssetLoaderPlugin;

impl Plugin for AssetLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.add_loading_state(
            LoadingState::new(GameState::AssetLoading)
                .continue_to_state(GameState::MainMenu)
                .load_collection::<GameAssets>()
        );
    }
}

#[derive(AssetCollection, Resource)]
pub struct GameAssets {
    // 字体
    #[asset(path = "fonts/FiraSans-Bold.ttf")]
    pub font: Handle<Font>,

    // 3D 模型（暂时注释，等有模型文件后再启用）
    // #[asset(path = "model/player.glb#Scene0")]
    // pub player_model: Handle<Scene>,

    // #[asset(path = "model/enemy.glb#Scene0")]
    // pub enemy_model: Handle<Scene>,

    // 音频（暂时注释，音频文件格式有问题，待修复）
    // TODO: 修复音频文件格式后启用
    // #[asset(path = "audio/flying.ogg")]
    // pub bgm: Handle<AudioSource>,

    // UI 图标
    #[asset(path = "textures/right.png")]
    pub icon_right: Handle<Image>,

    #[asset(path = "textures/wrench.png")]
    pub icon_settings: Handle<Image>,

    #[asset(path = "textures/exitRight.png")]
    pub icon_exit: Handle<Image>,
}
