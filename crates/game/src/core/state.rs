use bevy::prelude::*;

/// 游戏主状态
#[derive(States, Default, Clone, Eq, PartialEq, Debug, Hash)]
pub enum GameState {
    #[default]
    AssetLoading,    // 资源加载
    MainMenu,        // 主菜单（游戏场景 + 模糊 + 菜单 UI）
    Playing,         // 游戏进行
}

/// 菜单子状态（仅在 MainMenu 状态下激活）
#[derive(States, Default, Clone, Eq, PartialEq, Debug, Hash)]
pub enum MenuState {
    #[default]
    Disabled,        // 菜单禁用（Playing 状态）
    Main,            // 主菜单界面
    Settings,        // 设置界面
    SettingsDisplay, // 显示设置
    SettingsSound,   // 声音设置
}

/// 状态机插件
pub struct StatePlugin;

impl Plugin for StatePlugin {
    fn build(&self, app: &mut App) {
        app
            .init_state::<GameState>()
            .init_state::<MenuState>()
            // 添加状态转换事件监听（用于日志）
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
