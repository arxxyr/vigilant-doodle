//! 简单菜单系统（基于 Bevy 官方示例）
//!
//! 参考: https://github.com/bevyengine/bevy/blob/main/examples/games/game_menu.rs

use super::settings_menu::SettingsMenuState;
use vigilant_doodle_assets::GameAssets;
use vigilant_doodle_core::localization::{CurrentLanguage, LocalizedText, TranslationResources};
use vigilant_doodle_core::save::SaveManager;
use vigilant_doodle_core::state::{GameProgress, GameState};
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

/// 标记：菜单UI根节点（用于管理显示/隐藏）
#[derive(Component)]
struct MenuRoot;

/// 按钮动作
#[derive(Component)]
enum MenuButtonAction {
    NewGame,        // 新游戏
    Resume,         // 继续游戏（恢复）
    SaveGame,       // 存档
    Settings,       // 设置
    BackToMainMenu, // 返回主菜单（从暂停返回）
    Quit,           // 退出
    ToggleLanguage, // 切换语言
}

/// 标记：语言按钮的文本
#[derive(Component)]
struct LanguageButtonText;

// ============================================================================
// 插件定义
// ============================================================================

pub struct SimpleMenuPlugin;

impl Plugin for SimpleMenuPlugin {
    fn build(&self, app: &mut App) {
        app
            // 进入主菜单时生成UI
            .add_systems(OnEnter(GameState::MainMenu), setup_menu)
            // 进入暂停菜单时生成UI
            .add_systems(OnEnter(GameState::Paused), setup_menu)
            // 进入 Playing 状态时隐藏菜单
            .add_systems(OnEnter(GameState::Playing), hide_menu)
            // 退出主菜单和暂停菜单时清理UI
            .add_systems(OnExit(GameState::MainMenu), cleanup_menu)
            .add_systems(OnExit(GameState::Paused), cleanup_menu)
            // 更新按钮交互和语言按钮文本
            .add_systems(
                Update,
                (button_system, update_language_button_text)
                    .run_if(in_state(GameState::MainMenu).or(in_state(GameState::Paused))),
            );

        info!("[SimpleMenu] 菜单插件已加载");
    }
}

// ============================================================================
// 系统实现
// ============================================================================

/// 生成主菜单UI
fn setup_menu(
    mut commands: Commands,
    assets: Res<GameAssets>,
    current_language: Res<CurrentLanguage>,
    translation_resources: Res<TranslationResources>,
    game_progress: Res<GameProgress>,
    current_state: Res<State<GameState>>,
) {
    let state = current_state.get();
    let is_paused = matches!(state, GameState::Paused);
    let has_active_game = game_progress.has_active_game;

    info!("[SimpleMenu] ========== 开始生成菜单UI ==========");
    info!(
        "[SimpleMenu] 当前状态: {:?}, 有活跃游戏: {}",
        state, has_active_game
    );

    // 根容器（全屏，使用绝对定位布局，半透明黑色背景）
    let root_entity = commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                position_type: PositionType::Absolute,
                ..default()
            },
            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.6)), // 60% 不透明度的黑色遮罩
            MenuRoot,                                          // 标记为菜单根节点
            Name::new("MenuRoot"),
        ))
        .id();

    // 主菜单内容容器（居中）
    commands.entity(root_entity).with_children(|parent| {
        // 创建居中的菜单内容
        parent
            .spawn(Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                ..default()
            })
            .with_children(|parent| {
                info!("[SimpleMenu] 开始创建子元素...");

                // 标题
                parent.spawn((
                    Text::new(translation_resources.get(current_language.language, "menu.title")),
                    TextFont {
                        font: assets.font.clone(),
                        font_size: 80.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.9, 0.9, 0.9)),
                    LocalizedText::new("menu.title"),
                ));

                info!("[SimpleMenu] 标题已创建");

                // 按钮容器
                parent
                    .spawn(Node {
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        row_gap: Val::Px(20.0),
                        margin: UiRect::top(Val::Px(50.0)),
                        ..default()
                    })
                    .with_children(|button_parent| {
                        info!("[SimpleMenu] 开始创建按钮...");

                        // 辅助宏：生成菜单按钮
                        macro_rules! add_button {
                            ($action:expr, $key:expr) => {
                                button_parent
                                    .spawn((
                                        Button,
                                        Node {
                                            width: Val::Px(300.0),
                                            height: Val::Px(65.0),
                                            justify_content: JustifyContent::Center,
                                            align_items: AlignItems::Center,
                                            ..default()
                                        },
                                        BackgroundColor(NORMAL_BUTTON),
                                        $action,
                                    ))
                                    .with_child((
                                        Text::new(
                                            translation_resources
                                                .get(current_language.language, $key),
                                        ),
                                        TextFont {
                                            font: assets.font.clone(),
                                            font_size: 32.0,
                                            ..default()
                                        },
                                        TextColor(Color::srgb(0.9, 0.9, 0.9)),
                                        LocalizedText::new($key),
                                    ));
                            };
                        }

                        // 根据状态决定显示哪些按钮
                        if is_paused || has_active_game {
                            add_button!(MenuButtonAction::Resume, "menu.resume");
                        }

                        add_button!(MenuButtonAction::NewGame, "menu.new_game");
                        add_button!(MenuButtonAction::SaveGame, "menu.save_game");
                        add_button!(MenuButtonAction::Settings, "menu.settings");

                        if is_paused {
                            add_button!(MenuButtonAction::BackToMainMenu, "menu.back_to_menu");
                        }

                        add_button!(MenuButtonAction::Quit, "menu.quit");

                        info!("[SimpleMenu] 按钮创建完成");
                    });
            });

        // 语言切换按钮（右上角，绝对定位）
        parent
            .spawn((
                Button,
                Node {
                    width: Val::Px(120.0),
                    height: Val::Px(50.0),
                    position_type: PositionType::Absolute,
                    top: Val::Px(20.0),
                    right: Val::Px(20.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    column_gap: Val::Px(6.0),
                    ..default()
                },
                BackgroundColor(NORMAL_BUTTON),
                MenuButtonAction::ToggleLanguage,
                Name::new("LanguageButton"),
            ))
            .with_children(|button| {
                // 地球图标
                button.spawn((
                    Text::new("🌐"),
                    TextFont {
                        font: assets.font.clone(),
                        font_size: 24.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.9, 0.9, 0.9)),
                ));

                // 语言文本（显示切换后的语言）
                let next_language = current_language.language.toggle();
                button.spawn((
                    Text::new(next_language.display_name()),
                    TextFont {
                        font: assets.font.clone(),
                        font_size: 20.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.9, 0.9, 0.9)),
                    LanguageButtonText,
                ));
            });

        info!("[SimpleMenu] 语言切换按钮已创建");
    });

    info!("[SimpleMenu] ========== 主菜单UI生成完成 ==========");
}

/// 隐藏菜单（进入游戏时）
fn hide_menu(mut menu_query: Query<&mut Visibility, With<MenuRoot>>) {
    if let Ok(mut visibility) = menu_query.single_mut() {
        *visibility = Visibility::Hidden;
        info!("[SimpleMenu] 菜单已隐藏");
    }
}

/// 清理菜单（退出菜单状态时）
fn cleanup_menu(mut commands: Commands, menu_query: Query<Entity, With<MenuRoot>>) {
    for entity in menu_query.iter() {
        commands.entity(entity).despawn();
        info!("[SimpleMenu] 菜单已清理");
    }
}

/// 按钮交互系统
fn button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &MenuButtonAction),
        (Changed<Interaction>, With<Button>),
    >,
    mut next_state: ResMut<NextState<GameState>>,
    mut app_exit_events: MessageWriter<AppExit>,
    mut settings_state: ResMut<NextState<SettingsMenuState>>,
    mut current_language: ResMut<CurrentLanguage>,
    mut game_progress: ResMut<GameProgress>,
    mut save_manager: ResMut<SaveManager>,
) {
    for (interaction, mut color, action) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *color = PRESSED_BUTTON.into();
                match action {
                    MenuButtonAction::NewGame => {
                        info!("[SimpleMenu] Clicked: NewGame - 开始新游戏");
                        // TODO: 清空存档，重置游戏状态
                        game_progress.has_active_game = false;
                        next_state.set(GameState::Playing);
                    }
                    MenuButtonAction::Resume => {
                        info!("[SimpleMenu] Clicked: Resume - 继续游戏");
                        next_state.set(GameState::Playing);
                    }
                    MenuButtonAction::SaveGame => {
                        info!("[SimpleMenu] Clicked: SaveGame - 请求保存游戏");
                        save_manager.request_save();
                        // 标记有活跃游戏
                        game_progress.has_active_game = true;
                    }
                    MenuButtonAction::Settings => {
                        info!("[SimpleMenu] Clicked: Settings - 进入设置菜单");
                        settings_state.set(SettingsMenuState::Main);
                    }
                    MenuButtonAction::BackToMainMenu => {
                        info!("[SimpleMenu] Clicked: BackToMainMenu - 返回主菜单");
                        next_state.set(GameState::MainMenu);
                    }
                    MenuButtonAction::Quit => {
                        info!("[SimpleMenu] Clicked: Quit");
                        app_exit_events.write(AppExit::Success);
                    }
                    MenuButtonAction::ToggleLanguage => {
                        current_language.language = current_language.language.toggle();
                        info!("[SimpleMenu] 语言已切换到: {:?}", current_language.language);
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

/// 更新语言按钮文本系统
/// 显示"下一个语言"（即点击后会切换到的语言）
fn update_language_button_text(
    current_language: Res<CurrentLanguage>,
    mut text_query: Query<&mut Text, With<LanguageButtonText>>,
) {
    if !current_language.is_changed() {
        return;
    }

    // 显示切换后的语言
    let next_language = current_language.language.toggle();
    for mut text in text_query.iter_mut() {
        **text = next_language.display_name().to_string();
    }

    info!(
        "[SimpleMenu] 语言按钮文本已更新，显示下一个语言: {} (当前: {})",
        next_language.display_name(),
        current_language.language.display_name()
    );
}
