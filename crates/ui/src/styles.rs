use bevy::prelude::*;

/// UI 样式常量
pub struct MenuStyle;

impl MenuStyle {
    // 颜色 - 使用更明显的配色方案
    pub const BACKGROUND_COLOR: Color = Color::srgba(0.0, 0.0, 0.0, 0.8);
    pub const BUTTON_NORMAL: Color = Color::srgba(0.2, 0.3, 0.4, 0.95);  // 深蓝色
    pub const BUTTON_HOVERED: Color = Color::srgba(0.3, 0.5, 0.7, 0.95); // 亮蓝色
    pub const BUTTON_PRESSED: Color = Color::srgba(0.1, 0.4, 0.6, 0.95); // 中蓝色
    pub const BUTTON_BORDER: Color = Color::srgba(0.6, 0.7, 0.8, 1.0);   // 浅灰蓝色边框
    pub const TEXT_COLOR: Color = Color::srgb(0.95, 0.95, 0.95);         // 几乎白色
    pub const TEXT_HOVERED: Color = Color::srgb(1.0, 1.0, 1.0);          // 纯白色

    // 尺寸
    pub const BUTTON_WIDTH: Val = Val::Px(250.0);
    pub const BUTTON_HEIGHT: Val = Val::Px(60.0);
    pub const BUTTON_MARGIN: Val = Val::Px(15.0);
    pub const BUTTON_BORDER_WIDTH: Val = Val::Px(2.0);
    pub const FONT_SIZE: f32 = 32.0;
    pub const TITLE_SIZE: f32 = 64.0;
}

/// 创建按钮样式
pub fn button_style() -> Node {
    Node {
        width: MenuStyle::BUTTON_WIDTH,
        height: MenuStyle::BUTTON_HEIGHT,
        margin: UiRect::all(MenuStyle::BUTTON_MARGIN),
        border: UiRect::all(MenuStyle::BUTTON_BORDER_WIDTH),
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
        position_type: PositionType::Absolute,
        flex_direction: FlexDirection::Column,
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        row_gap: Val::Px(20.0),
        ..default()
    }
}

/// 创建语言切换按钮样式（圆形，右上角固定位置）
pub fn language_button_style() -> Node {
    Node {
        width: Val::Px(80.0),
        height: Val::Px(40.0),
        position_type: PositionType::Absolute,
        top: Val::Px(20.0),
        right: Val::Px(20.0),
        border: UiRect::all(MenuStyle::BUTTON_BORDER_WIDTH),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    }
}

/// 创建语言切换按钮文本样式
pub fn language_button_text_style(font: Handle<Font>) -> (Text, TextFont, TextColor) {
    (
        Text::new(""),
        TextFont {
            font,
            font_size: 20.0,
            ..default()
        },
        TextColor(MenuStyle::TEXT_COLOR),
    )
}
