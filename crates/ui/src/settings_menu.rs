//! 设置菜单系统
//!
//! 提供完整的设置界面，包括外观、图形、声音、控制、游戏性设置。

use vigilant_doodle_assets::GameAssets;
use vigilant_doodle_core::localization::{CurrentLanguage, Language, LocalizedText, TranslationResources};
use vigilant_doodle_core::state::GameState;
use bevy::prelude::*;

// ============================================================================
// 常量定义
// ============================================================================

const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::srgb(0.35, 0.75, 0.35);

// ============================================================================
// 组件定义
// ============================================================================

/// 标记：设置菜单根节点
#[derive(Component)]
struct SettingsRoot;

/// 设置菜单状态
#[derive(States, Default, Clone, Eq, PartialEq, Debug, Hash)]
pub enum SettingsMenuState {
    #[default]
    Hidden, // 隐藏（在主菜单或游戏中）
    Main,       // 主设置菜单（显示分类）
    Appearance, // 外观设置
    Graphics,   // 图形设置
    Audio,      // 声音设置
    Controls,   // 控制设置
    Gameplay,   // 游戏性设置
}

/// 按钮动作
#[derive(Component)]
enum SettingsButtonAction {
    // 主菜单按钮
    ToAppearance,
    ToGraphics,
    ToAudio,
    ToControls,
    ToGameplay,
    Back, // 返回主菜单

    // 语言选择
    LanguageChinese,
    LanguageEnglish,
}

// ============================================================================
// 插件定义
// ============================================================================

pub struct SettingsMenuPlugin;

impl Plugin for SettingsMenuPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<SettingsMenuState>()
            // 进入主菜单状态时，确保设置菜单隐藏
            .add_systems(OnEnter(GameState::MainMenu), hide_settings)
            // 主设置菜单
            .add_systems(OnEnter(SettingsMenuState::Main), setup_main_settings_menu)
            .add_systems(OnExit(SettingsMenuState::Main), cleanup_settings_menu)
            // 外观设置
            .add_systems(
                OnEnter(SettingsMenuState::Appearance),
                setup_appearance_settings,
            )
            .add_systems(OnExit(SettingsMenuState::Appearance), cleanup_settings_menu)
            // 按钮交互
            .add_systems(
                Update,
                settings_button_system.run_if(not(in_state(SettingsMenuState::Hidden))),
            );

        info!("[SettingsMenu] 设置菜单插件已加载");
    }
}

// ============================================================================
// 系统实现
// ============================================================================

/// 隐藏设置菜单
fn hide_settings(mut next_state: ResMut<NextState<SettingsMenuState>>) {
    next_state.set(SettingsMenuState::Hidden);
}

/// 显示设置菜单（从主菜单的设置按钮触发）
pub fn show_settings(mut next_state: ResMut<NextState<SettingsMenuState>>) {
    info!("[SettingsMenu] 显示设置菜单");
    next_state.set(SettingsMenuState::Main);
}

/// 清理设置菜单UI
fn cleanup_settings_menu(
    mut commands: Commands,
    settings_query: Query<Entity, With<SettingsRoot>>,
) {
    for entity in settings_query.iter() {
        commands.entity(entity).despawn();
    }
    info!("[SettingsMenu] 设置菜单UI已清理");
}

/// 生成主设置菜单（显示分类）
fn setup_main_settings_menu(
    mut commands: Commands,
    assets: Res<GameAssets>,
    current_language: Res<CurrentLanguage>,
    translation_resources: Res<TranslationResources>,
) {
    info!("[SettingsMenu] ========== 生成主设置菜单 ==========");

    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            BackgroundColor(Color::srgb(0.1, 0.1, 0.1)),
            SettingsRoot,
            Name::new("SettingsRoot"),
        ))
        .with_children(|parent| {
            // 标题
            parent.spawn((
                Text::new(translation_resources.get(current_language.language, "settings.title")),
                TextFont {
                    font: assets.font.clone(),
                    font_size: 60.0,
                    ..default()
                },
                TextColor(Color::srgb(0.9, 0.9, 0.9)),
                LocalizedText::new("settings.title"),
            ));

            // 分类按钮容器
            parent
                .spawn(Node {
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    row_gap: Val::Px(15.0),
                    margin: UiRect::top(Val::Px(40.0)),
                    ..default()
                })
                .with_children(|button_parent| {
                    // 外观按钮
                    button_parent
                        .spawn((
                            Button,
                            Node {
                                width: Val::Px(300.0),
                                height: Val::Px(60.0),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            BackgroundColor(NORMAL_BUTTON),
                            SettingsButtonAction::ToAppearance,
                        ))
                        .with_child((
                            Text::new(
                                translation_resources
                                    .get(current_language.language, "settings.appearance"),
                            ),
                            TextFont {
                                font: assets.font.clone(),
                                font_size: 28.0,
                                ..default()
                            },
                            TextColor(Color::srgb(0.9, 0.9, 0.9)),
                            LocalizedText::new("settings.appearance"),
                        ));

                    // 图形按钮
                    button_parent
                        .spawn((
                            Button,
                            Node {
                                width: Val::Px(300.0),
                                height: Val::Px(60.0),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            BackgroundColor(NORMAL_BUTTON),
                            SettingsButtonAction::ToGraphics,
                        ))
                        .with_child((
                            Text::new(
                                translation_resources
                                    .get(current_language.language, "settings.graphics"),
                            ),
                            TextFont {
                                font: assets.font.clone(),
                                font_size: 28.0,
                                ..default()
                            },
                            TextColor(Color::srgb(0.9, 0.9, 0.9)),
                            LocalizedText::new("settings.graphics"),
                        ));

                    // 声音按钮
                    button_parent
                        .spawn((
                            Button,
                            Node {
                                width: Val::Px(300.0),
                                height: Val::Px(60.0),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            BackgroundColor(NORMAL_BUTTON),
                            SettingsButtonAction::ToAudio,
                        ))
                        .with_child((
                            Text::new(
                                translation_resources
                                    .get(current_language.language, "settings.audio"),
                            ),
                            TextFont {
                                font: assets.font.clone(),
                                font_size: 28.0,
                                ..default()
                            },
                            TextColor(Color::srgb(0.9, 0.9, 0.9)),
                            LocalizedText::new("settings.audio"),
                        ));

                    // 控制按钮
                    button_parent
                        .spawn((
                            Button,
                            Node {
                                width: Val::Px(300.0),
                                height: Val::Px(60.0),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            BackgroundColor(NORMAL_BUTTON),
                            SettingsButtonAction::ToControls,
                        ))
                        .with_child((
                            Text::new(
                                translation_resources
                                    .get(current_language.language, "settings.controls"),
                            ),
                            TextFont {
                                font: assets.font.clone(),
                                font_size: 28.0,
                                ..default()
                            },
                            TextColor(Color::srgb(0.9, 0.9, 0.9)),
                            LocalizedText::new("settings.controls"),
                        ));

                    // 游戏性按钮
                    button_parent
                        .spawn((
                            Button,
                            Node {
                                width: Val::Px(300.0),
                                height: Val::Px(60.0),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            BackgroundColor(NORMAL_BUTTON),
                            SettingsButtonAction::ToGameplay,
                        ))
                        .with_child((
                            Text::new(
                                translation_resources
                                    .get(current_language.language, "settings.gameplay"),
                            ),
                            TextFont {
                                font: assets.font.clone(),
                                font_size: 28.0,
                                ..default()
                            },
                            TextColor(Color::srgb(0.9, 0.9, 0.9)),
                            LocalizedText::new("settings.gameplay"),
                        ));

                    // 返回按钮
                    button_parent
                        .spawn((
                            Button,
                            Node {
                                width: Val::Px(300.0),
                                height: Val::Px(60.0),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            BackgroundColor(NORMAL_BUTTON),
                            SettingsButtonAction::Back,
                        ))
                        .with_child((
                            Text::new(
                                translation_resources
                                    .get(current_language.language, "settings.back"),
                            ),
                            TextFont {
                                font: assets.font.clone(),
                                font_size: 28.0,
                                ..default()
                            },
                            TextColor(Color::srgb(0.9, 0.9, 0.9)),
                            LocalizedText::new("settings.back"),
                        ));
                });
        });

    info!("[SettingsMenu] 主设置菜单生成完成");
}

/// 生成外观设置菜单
fn setup_appearance_settings(
    mut commands: Commands,
    assets: Res<GameAssets>,
    current_language: Res<CurrentLanguage>,
    translation_resources: Res<TranslationResources>,
) {
    info!("[SettingsMenu] ========== 生成外观设置 ==========");

    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            BackgroundColor(Color::srgb(0.1, 0.1, 0.1)),
            SettingsRoot,
            Name::new("AppearanceSettings"),
        ))
        .with_children(|parent| {
            // 标题
            parent.spawn((
                Text::new(
                    translation_resources.get(current_language.language, "settings.appearance"),
                ),
                TextFont {
                    font: assets.font.clone(),
                    font_size: 60.0,
                    ..default()
                },
                TextColor(Color::srgb(0.9, 0.9, 0.9)),
                LocalizedText::new("settings.appearance"),
            ));

            // 选项容器
            parent
                .spawn(Node {
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    row_gap: Val::Px(15.0),
                    margin: UiRect::top(Val::Px(40.0)),
                    ..default()
                })
                .with_children(|options_parent| {
                    // 语言选择标题
                    options_parent.spawn((
                        Text::new(
                            translation_resources
                                .get(current_language.language, "settings.language"),
                        ),
                        TextFont {
                            font: assets.font.clone(),
                            font_size: 32.0,
                            ..default()
                        },
                        TextColor(Color::srgb(0.9, 0.9, 0.9)),
                        LocalizedText::new("settings.language"),
                        Node {
                            margin: UiRect::top(Val::Px(20.0)),
                            ..default()
                        },
                    ));

                    // 语言按钮容器（水平排列）
                    options_parent
                        .spawn(Node {
                            flex_direction: FlexDirection::Row,
                            column_gap: Val::Px(20.0),
                            ..default()
                        })
                        .with_children(|lang_parent| {
                            // 中文按钮
                            lang_parent
                                .spawn((
                                    Button,
                                    Node {
                                        width: Val::Px(150.0),
                                        height: Val::Px(50.0),
                                        justify_content: JustifyContent::Center,
                                        align_items: AlignItems::Center,
                                        ..default()
                                    },
                                    BackgroundColor(
                                        if current_language.language == Language::Chinese {
                                            PRESSED_BUTTON
                                        } else {
                                            NORMAL_BUTTON
                                        },
                                    ),
                                    SettingsButtonAction::LanguageChinese,
                                ))
                                .with_child((
                                    Text::new(translation_resources.get(
                                        current_language.language,
                                        "settings.language_chinese",
                                    )),
                                    TextFont {
                                        font: assets.font.clone(),
                                        font_size: 24.0,
                                        ..default()
                                    },
                                    TextColor(Color::srgb(0.9, 0.9, 0.9)),
                                    LocalizedText::new("settings.language_chinese"),
                                ));

                            // 英文按钮
                            lang_parent
                                .spawn((
                                    Button,
                                    Node {
                                        width: Val::Px(150.0),
                                        height: Val::Px(50.0),
                                        justify_content: JustifyContent::Center,
                                        align_items: AlignItems::Center,
                                        ..default()
                                    },
                                    BackgroundColor(
                                        if current_language.language == Language::English {
                                            PRESSED_BUTTON
                                        } else {
                                            NORMAL_BUTTON
                                        },
                                    ),
                                    SettingsButtonAction::LanguageEnglish,
                                ))
                                .with_child((
                                    Text::new(translation_resources.get(
                                        current_language.language,
                                        "settings.language_english",
                                    )),
                                    TextFont {
                                        font: assets.font.clone(),
                                        font_size: 24.0,
                                        ..default()
                                    },
                                    TextColor(Color::srgb(0.9, 0.9, 0.9)),
                                    LocalizedText::new("settings.language_english"),
                                ));
                        });

                    // 返回按钮
                    options_parent
                        .spawn((
                            Button,
                            Node {
                                width: Val::Px(300.0),
                                height: Val::Px(60.0),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                margin: UiRect::top(Val::Px(30.0)),
                                ..default()
                            },
                            BackgroundColor(NORMAL_BUTTON),
                            SettingsButtonAction::Back,
                        ))
                        .with_child((
                            Text::new(
                                translation_resources
                                    .get(current_language.language, "settings.back"),
                            ),
                            TextFont {
                                font: assets.font.clone(),
                                font_size: 28.0,
                                ..default()
                            },
                            TextColor(Color::srgb(0.9, 0.9, 0.9)),
                            LocalizedText::new("settings.back"),
                        ));
                });
        });

    info!("[SettingsMenu] 外观设置生成完成");
}

/// 按钮交互系统
fn settings_button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &SettingsButtonAction),
        (Changed<Interaction>, With<Button>),
    >,
    mut settings_state: ResMut<NextState<SettingsMenuState>>,
    mut current_language: ResMut<CurrentLanguage>,
    current_menu_state: Res<State<SettingsMenuState>>,
) {
    for (interaction, mut color, action) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *color = PRESSED_BUTTON.into();
                match action {
                    SettingsButtonAction::ToAppearance => {
                        info!("[SettingsMenu] 进入外观设置");
                        settings_state.set(SettingsMenuState::Appearance);
                    }
                    SettingsButtonAction::ToGraphics => {
                        info!("[SettingsMenu] 进入图形设置");
                        settings_state.set(SettingsMenuState::Graphics);
                    }
                    SettingsButtonAction::ToAudio => {
                        info!("[SettingsMenu] 进入声音设置");
                        settings_state.set(SettingsMenuState::Audio);
                    }
                    SettingsButtonAction::ToControls => {
                        info!("[SettingsMenu] 进入控制设置");
                        settings_state.set(SettingsMenuState::Controls);
                    }
                    SettingsButtonAction::ToGameplay => {
                        info!("[SettingsMenu] 进入游戏性设置");
                        settings_state.set(SettingsMenuState::Gameplay);
                    }
                    SettingsButtonAction::Back => match **current_menu_state {
                        SettingsMenuState::Main => {
                            info!("[SettingsMenu] 返回主菜单");
                            settings_state.set(SettingsMenuState::Hidden);
                        }
                        _ => {
                            info!("[SettingsMenu] 返回设置主菜单");
                            settings_state.set(SettingsMenuState::Main);
                        }
                    },
                    SettingsButtonAction::LanguageChinese => {
                        info!("[SettingsMenu] 切换到中文");
                        current_language.language = Language::Chinese;
                    }
                    SettingsButtonAction::LanguageEnglish => {
                        info!("[SettingsMenu] 切换到英文");
                        current_language.language = Language::English;
                    }
                }
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
            }
        }
    }
}
