use bevy::prelude::*;

/// 菜单覆盖层标记（半透明背景）
#[derive(Component)]
pub struct MenuOverlay;

/// 主菜单根节点标记
#[derive(Component)]
pub struct MainMenuRoot;

/// 设置菜单根节点标记
#[derive(Component)]
pub struct SettingsMenuRoot;

/// 按钮动作类型
#[derive(Component, Clone, Copy, PartialEq, Eq, Debug)]
pub enum ButtonAction {
    StartGame,
    Settings,
    Quit,
    BackToMenu,
    ToggleLanguage,
}

/// 按钮状态组件
#[derive(Component)]
pub struct ButtonState {
    pub is_hovered: bool,
}

impl Default for ButtonState {
    fn default() -> Self {
        Self { is_hovered: false }
    }
}

/// 语言按钮文本标记
#[derive(Component)]
pub struct LanguageButtonText;
