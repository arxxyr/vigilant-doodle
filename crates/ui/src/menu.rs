use bevy::prelude::*;
use crate::core::state::{GameState, MenuState};
use crate::core::localization::{CurrentLanguage, LocalizedText};
use crate::assets::loader::GameAssets;
use super::components::*;
use super::styles::*;

/// 菜单覆盖层插件（半透明背景）
pub struct MenuOverlayPlugin;

impl Plugin for MenuOverlayPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(GameState::MainMenu), spawn_overlay)
            .add_systems(OnExit(GameState::MainMenu), despawn_overlay);
    }
}

fn spawn_overlay(mut commands: Commands) {
    // 全屏半透明黑色背景
    commands.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            position_type: PositionType::Absolute,
            ..default()
        },
        BackgroundColor(MenuStyle::BACKGROUND_COLOR),
        ZIndex(5),
        MenuOverlay,
        Name::new("MenuOverlay"),
    ));

    // 测试：在屏幕中央添加一个明显的红色方块
    commands.spawn((
        Node {
            width: Val::Px(200.0),
            height: Val::Px(200.0),
            position_type: PositionType::Absolute,
            left: Val::Percent(50.0),
            top: Val::Percent(50.0),
            ..default()
        },
        BackgroundColor(Color::srgb(1.0, 0.0, 0.0)),  // 纯红色
        ZIndex(100),  // 最高层级
        Name::new("TestRedBox"),
    ));

    info!("[UI] Menu overlay spawned (with test red box)");
}

fn despawn_overlay(
    mut commands: Commands,
    query: Query<Entity, With<MenuOverlay>>,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }

    info!("[UI] Menu overlay despawned");
}

/// 主菜单插件
pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(MenuState::Main), spawn_main_menu)
            .add_systems(OnExit(MenuState::Main), despawn_main_menu)
            .add_systems(
                Update,
                (
                    button_interaction_system,
                    button_action_system,
                    update_language_button_text,
                )
                    .run_if(in_state(GameState::MainMenu))
                    .run_if(in_state(MenuState::Main))
            );
    }
}

fn spawn_main_menu(
    mut commands: Commands,
    assets: Res<GameAssets>,
    current_language: Res<CurrentLanguage>,
) {
    let font = assets.font.clone();

    commands
        .spawn((
            menu_container_style(),
            ZIndex(10),
            MainMenuRoot,
            Name::new("MainMenu"),
        ))
        .with_children(|parent| {
            // 游戏标题
            let (text, text_font, text_color) = title_text_style(font.clone());
            parent.spawn((
                text,
                text_font,
                text_color,
                LocalizedText::new("menu.title"),
            ));

            // 垂直间隔
            parent.spawn(Node {
                height: Val::Px(40.0),
                ..default()
            });

            // 开始游戏按钮
            spawn_button(
                parent,
                "menu.play",
                ButtonAction::StartGame,
                font.clone(),
            );

            // 设置按钮
            spawn_button(
                parent,
                "menu.settings",
                ButtonAction::Settings,
                font.clone(),
            );

            // 退出按钮
            spawn_button(
                parent,
                "menu.quit",
                ButtonAction::Quit,
                font.clone(),
            );
        });

    // 语言切换按钮（右上角，独立于主容器）
    commands
        .spawn((
            Button,
            language_button_style(),
            BackgroundColor(MenuStyle::BUTTON_NORMAL),
            BorderColor::all(MenuStyle::BUTTON_BORDER),
            ZIndex(20),
            ButtonAction::ToggleLanguage,
            ButtonState::default(),
            MainMenuRoot,  // 标记为主菜单的一部分，方便一起清理
            Name::new("LanguageButton"),
        ))
        .with_children(|button| {
            let (text, text_font, text_color) = language_button_text_style(font.clone());
            button.spawn((
                text,
                text_font,
                text_color,
                LanguageButtonText,  // 特殊标记，用于更新语言显示
                Name::new("LanguageButtonText"),
            ));
        });

    info!("[UI] Main menu spawned with language: {:?}", current_language.language);
}

fn despawn_main_menu(
    mut commands: Commands,
    query: Query<Entity, With<MainMenuRoot>>,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }

    info!("[UI] Main menu despawned");
}

/// 创建按钮的辅助函数
fn spawn_button(
    parent: &mut ChildSpawnerCommands,
    text_key: &str,
    action: ButtonAction,
    font: Handle<Font>,
) {
    parent
        .spawn((
            Button,
            button_style(),
            BackgroundColor(MenuStyle::BUTTON_NORMAL),
            BorderColor::all(MenuStyle::BUTTON_BORDER),
            ZIndex(10),
            action,
            ButtonState::default(),
            Name::new(format!("Button_{:?}", action)),
        ))
        .with_children(|button: &mut ChildSpawnerCommands| {
            let (button_text, text_font, text_color) = button_text_style(font);
            button.spawn((
                button_text,
                text_font,
                text_color,
                LocalizedText::new(text_key),
            ));
        });
}

/// 按钮交互系统（处理悬停和点击视觉效果）
fn button_interaction_system(
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &mut ButtonState,
            &Children,
        ),
        (Changed<Interaction>, With<Button>),
    >,
    mut text_query: Query<&mut TextColor>,
) {
    for (interaction, mut bg_color, mut button_state, children) in interaction_query.iter_mut() {
        // 更新按钮背景颜色
        *bg_color = match *interaction {
            Interaction::Pressed => MenuStyle::BUTTON_PRESSED.into(),
            Interaction::Hovered => MenuStyle::BUTTON_HOVERED.into(),
            Interaction::None => MenuStyle::BUTTON_NORMAL.into(),
        };

        // 更新按钮状态
        button_state.is_hovered = *interaction == Interaction::Hovered;

        // 更新文本颜色
        for child in children.iter() {
            if let Ok(mut text_color) = text_query.get_mut(child) {
                text_color.0 = if button_state.is_hovered {
                    MenuStyle::TEXT_HOVERED
                } else {
                    MenuStyle::TEXT_COLOR
                };
            }
        }
    }
}

/// 按钮动作系统（处理按钮点击事件）
fn button_action_system(
    interaction_query: Query<(&Interaction, &ButtonAction), (Changed<Interaction>, With<Button>)>,
    mut game_state: ResMut<NextState<GameState>>,
    mut menu_state: ResMut<NextState<MenuState>>,
    mut app_exit: MessageWriter<AppExit>,
    mut current_language: ResMut<CurrentLanguage>,
) {
    for (interaction, action) in interaction_query.iter() {
        if *interaction == Interaction::Pressed {
            match action {
                ButtonAction::StartGame => {
                    info!("[UI] Start game button pressed");
                    game_state.set(GameState::Playing);
                    menu_state.set(MenuState::Disabled);
                }
                ButtonAction::Settings => {
                    info!("[UI] Settings button pressed");
                    menu_state.set(MenuState::Settings);
                }
                ButtonAction::Quit => {
                    info!("[UI] Quit button pressed");
                    app_exit.write(AppExit::Success);
                }
                ButtonAction::BackToMenu => {
                    info!("[UI] Back to menu button pressed");
                    menu_state.set(MenuState::Main);
                }
                ButtonAction::ToggleLanguage => {
                    current_language.language = current_language.language.toggle();
                    info!("[UI] Language switched to: {:?}", current_language.language);
                }
            }
        }
    }
}

/// 设置菜单插件
pub struct SettingsMenuPlugin;

impl Plugin for SettingsMenuPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(MenuState::Settings), spawn_settings_menu)
            .add_systems(OnExit(MenuState::Settings), despawn_settings_menu)
            .add_systems(
                Update,
                (button_interaction_system, button_action_system)
                    .run_if(in_state(GameState::MainMenu))
                    .run_if(in_state(MenuState::Settings))
            );
    }
}

fn spawn_settings_menu(
    mut commands: Commands,
    assets: Res<GameAssets>,
) {
    let font = assets.font.clone();

    commands
        .spawn((
            menu_container_style(),
            ZIndex(10),
            SettingsMenuRoot,
            Name::new("SettingsMenu"),
        ))
        .with_children(|parent| {
            // 设置标题
            let (mut text, text_font, text_color) = title_text_style(font.clone());
            text.0 = "设置".to_string();
            parent.spawn((text, text_font, text_color));

            // 垂直间隔
            parent.spawn(Node {
                height: Val::Px(40.0),
                ..default()
            });

            // 占位文本（后续可添加具体设置项）
            let (mut hint_text, hint_font, hint_color) = button_text_style(font.clone());
            hint_text.0 = "（设置选项待实现）".to_string();
            parent.spawn((hint_text, hint_font, hint_color));

            // 垂直间隔
            parent.spawn(Node {
                height: Val::Px(40.0),
                ..default()
            });

            // 返回按钮
            spawn_button(
                parent,
                "返回主菜单",
                ButtonAction::BackToMenu,
                font.clone(),
            );
        });

    info!("[UI] Settings menu spawned");
}

fn despawn_settings_menu(
    mut commands: Commands,
    query: Query<Entity, With<SettingsMenuRoot>>,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }

    info!("[UI] Settings menu despawned");
}

/// 更新语言按钮文本系统
fn update_language_button_text(
    current_language: Res<CurrentLanguage>,
    mut text_query: Query<&mut Text, With<LanguageButtonText>>,
) {
    if !current_language.is_changed() {
        return;
    }

    for mut text in text_query.iter_mut() {
        **text = current_language.language.display_name().to_string();
    }
}
