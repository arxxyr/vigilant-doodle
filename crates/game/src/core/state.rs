use bevy::prelude::*;

/// 游戏主状态
#[derive(States, Default, Clone, Eq, PartialEq, Debug, Hash)]
pub enum GameState {
    #[default]
    AssetLoading, // 资源加载（默认状态）
    MainMenu, // 主菜单（游戏场景 + 模糊遮罩 + UI）
    Playing,  // 游戏进行
    Paused,   // 游戏暂停（显示暂停菜单）
}

/// 游戏进度追踪（用于区分首次启动和游戏中暂停）
#[derive(Resource, Default)]
pub struct GameProgress {
    /// 是否有游戏正在进行（从 MainMenu 进入过 Playing）
    pub has_active_game: bool,
}

/// 状态机插件
pub struct StatePlugin;

impl Plugin for StatePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>()
            .init_resource::<GameProgress>()
            .add_systems(OnEnter(GameState::AssetLoading), log_enter_loading)
            .add_systems(OnEnter(GameState::MainMenu), log_enter_menu)
            .add_systems(
                OnEnter(GameState::Playing),
                (log_enter_playing, mark_game_active),
            )
            .add_systems(OnEnter(GameState::Paused), log_enter_paused);
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

fn log_enter_paused() {
    info!("[State] → Paused");
}

/// 标记游戏为活跃状态（进入 Playing 时）
fn mark_game_active(mut progress: ResMut<GameProgress>) {
    progress.has_active_game = true;
    info!("[State] Game marked as active");
}
