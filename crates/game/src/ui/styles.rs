use bevy::prelude::*;

/// UI 样式常量
pub struct MenuStyle;

impl MenuStyle {
    // 颜色
    pub const BACKGROUND_COLOR: Color = Color::srgba(0.0, 0.0, 0.0, 0.7);
    pub const BUTTON_NORMAL: Color = Color::srgba(0.15, 0.15, 0.15, 0.9);
    pub const BUTTON_HOVERED: Color = Color::srgba(0.25, 0.25, 0.25, 0.9);
    pub const BUTTON_PRESSED: Color = Color::srgba(0.35, 0.35, 0.35, 0.9);
    pub const TEXT_COLOR: Color = Color::srgb(0.9, 0.9, 0.9);
    pub const TEXT_HOVERED: Color = Color::srgb(1.0, 1.0, 1.0);

    // 尺寸
    pub const BUTTON_WIDTH: Val = Val::Px(250.0);
    pub const BUTTON_HEIGHT: Val = Val::Px(60.0);
    pub const BUTTON_MARGIN: Val = Val::Px(15.0);
    pub const FONT_SIZE: f32 = 32.0;
    pub const TITLE_SIZE: f32 = 64.0;
}

/// 创建按钮样式
pub fn button_style() -> Node {
    Node {
        width: MenuStyle::BUTTON_WIDTH,
        height: MenuStyle::BUTTON_HEIGHT,
        margin: UiRect::all(MenuStyle::BUTTON_MARGIN),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    }
}

/// 创建按钮文本样式
pub fn button_text_style(font: Handle<Font>) -> (Text, TextFont, TextColor) {
    (
        Text::new(""),
        TextFont {
            font,
            font_size: MenuStyle::FONT_SIZE,
            ..default()
        },
        TextColor(MenuStyle::TEXT_COLOR),
    )
}

/// 创建标题文本样式
pub fn title_text_style(font: Handle<Font>) -> (Text, TextFont, TextColor) {
    (
        Text::new(""),
        TextFont {
            font,
            font_size: MenuStyle::TITLE_SIZE,
            ..default()
        },
        TextColor(MenuStyle::TEXT_COLOR),
    )
}

/// 创建菜单容器样式
pub fn menu_container_style() -> Node {
    Node {
        width: Val::Percent(100.0),
        height: Val::Percent(100.0),
        flex_direction: FlexDirection::Column,
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        row_gap: Val::Px(20.0),
        ..default()
    }
}
