use crate::loading::FontAssets;
use crate::DisplayQuality;
use crate::GameState;
use crate::Volume;
use bevy::{app::AppExit, prelude::*};

pub struct MenuPlugin;

/// This plugin is responsible for the game menu (containing only one button...)
/// The menu is only drawn during the State `GameState::Menu` and is removed when that state is exited
impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<MenuState>()
            // .init_resource::<ButtonColors>()
            .insert_resource(DisplayQuality::Medium)
            .insert_resource(Volume(7))
            .add_systems(OnEnter(GameState::Menu), setup_menu)
            // Systems to handle the main menu screen
            .add_systems(OnEnter(MenuState::Main), main_menu_setup)
            .add_systems(OnExit(MenuState::Main), despawn_screen::<OnMainMenuScreen>)
            // Systems to handle the settings menu screen
            .add_systems(OnEnter(MenuState::Settings), settings_menu_setup)
            .add_systems(
                OnExit(MenuState::Settings),
                despawn_screen::<OnSettingsMenuScreen>,
            )
            // Systems to handle the display settings screen
            .add_systems(
                OnEnter(MenuState::SettingsDisplay),
                display_settings_menu_setup,
            )
            .add_systems(
                Update,
                (setting_button::<DisplayQuality>.run_if(in_state(MenuState::SettingsDisplay)),),
            )
            .add_systems(
                OnExit(MenuState::SettingsDisplay),
                despawn_screen::<OnDisplaySettingsMenuScreen>,
            )
            // Systems to handle the sound settings screen
            .add_systems(OnEnter(MenuState::SettingsSound), sound_settings_menu_setup)
            .add_systems(
                Update,
                setting_button::<Volume>.run_if(in_state(MenuState::SettingsSound)),
            )
            .add_systems(
                OnExit(MenuState::SettingsSound),
                despawn_screen::<OnSoundSettingsMenuScreen>,
            )
            // Common systems to all screens that handles buttons behavior
            .add_systems(
                Update,
                (menu_action, button_system).run_if(in_state(GameState::Menu)),
            )
            // .add_systems(OnExit(GameState::Menu), cleanup_menu)
            ;
    }
    // fn build(&self, app: &mut App) {
    //     app.add_state::<MenuState>()
    //         .init_resource::<ButtonColors>()
    //         .add_systems(OnEnter(GameState::Menu), setup_menu)
    //         .add_systems(Update, menu_action.run_if(in_state(GameState::Menu)))
    //         .add_systems(OnExit(GameState::Menu), despawn_screen::<OnMainMenuScreen>);
    // }
}

// #[derive(Resource)]
// struct ButtonColors {
//     normal: Color,
//     hovered: Color,
// }

// impl Default for ButtonColors {
//     fn default() -> Self {
//         ButtonColors {
//             normal: Color::srgb(0.15, 0.15, 0.15),
//             hovered: Color::srgb(0.25, 0.25, 0.25),
//         }
//     }
// }

// State used for the current menu screen
#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
enum MenuState {
    Main,
    Settings,
    SettingsDisplay,
    SettingsSound,
    #[default]
    Disabled,
}

// Tag component used to tag entities added on the main menu screen
#[derive(Component)]
struct OnMainMenuScreen;

// Tag component used to tag entities added on the settings menu screen
#[derive(Component)]
struct OnSettingsMenuScreen;

// Tag component used to tag entities added on the display settings menu screen
#[derive(Component)]
struct OnDisplaySettingsMenuScreen;

// Tag component used to tag entities added on the sound settings menu screen
#[derive(Component)]
struct OnSoundSettingsMenuScreen;

const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);
const HOVERED_PRESSED_BUTTON: Color = Color::srgb(0.25, 0.65, 0.25);
const PRESSED_BUTTON: Color = Color::srgb(0.35, 0.75, 0.35);

// Tag component used to mark which setting is currently selected
#[derive(Component)]
struct SelectedOption;

// All actions that can be triggered from a button click
#[derive(Component)]
enum MenuButtonAction {
    Play,
    Settings,
    SettingsDisplay,
    SettingsSound,
    BackToMainMenu,
    BackToSettings,
    Quit,
}

// This system handles changing all buttons color based on mouse interaction
fn button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, Option<&SelectedOption>),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut color, selected) in &mut interaction_query {
        *color = match (*interaction, selected) {
            (Interaction::Pressed, _) | (Interaction::None, Some(_)) => PRESSED_BUTTON.into(),
            (Interaction::Hovered, Some(_)) => HOVERED_PRESSED_BUTTON.into(),
            (Interaction::Hovered, None) => HOVERED_BUTTON.into(),
            (Interaction::None, None) => NORMAL_BUTTON.into(),
        }
    }
}

// fn setup_menu(
//     mut commands: Commands,
//     font_assets: Res<FontAssets>,
//     button_colors: Res<ButtonColors>,
// ) {
//     commands.spawn(Camera2dBundle::default());
//     commands
//         .spawn((
//             NodeBundle {
//                 style: Style {
//                     width: Val::Percent(100.0),
//                     align_items: AlignItems::Center,
//                     justify_content: JustifyContent::Center,
//                     ..default()
//                 },
//                 ..default()
//             },
//             OnMainMenuScreen,
//         ))
//         .with_children(|parent| {
//             parent
//                 .spawn(ButtonBundle {
//                     style: Style {
//                         width: Val::Px(120.0),
//                         height: Val::Px(50.0),
//                         margin: UiRect::all(Val::Auto),
//                         justify_content: JustifyContent::Center,
//                         align_items: AlignItems::Center,
//                         ..Default::default()
//                     },
// background_color: button_colors.normal.into(),
//                     ..Default::default()
//                 })
//                 .with_children(|parent| {
//                     parent.spawn((
//                         TextBundle::from_section(
//                             "Play",
//                             TextStyle {
//                                 font: font_assets.fira_sans.clone(),
//                                 font_size: 40.0,
//                                 color: Color::srgb(0.9, 0.9, 0.9),
//                             },
//                         ),
//                         MenuButtonAction::Play,
//                     ));
//                     parent.spawn((
//                         TextBundle::from_section(
//                             "Quit",
//                             TextStyle {
//                                 font: font_assets.fira_sans.clone(),
//                                 font_size: 40.0,
//                                 color: Color::srgb(0.9, 0.9, 0.9),
//                             },
//                         ),
//                         MenuButtonAction::Quit,
//                     ));
//                 });
//         });
// }

fn menu_action(
    interaction_query: Query<
        (&Interaction, &MenuButtonAction),
        (Changed<Interaction>, With<Button>),
    >,
    mut app_exit_events: EventWriter<AppExit>,
    mut menu_state: ResMut<NextState<MenuState>>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    for (interaction, menu_button_action) in &interaction_query {
        if *interaction == Interaction::Pressed {
            match menu_button_action {
                MenuButtonAction::Quit => {
                    // println!("Quitting game");
                    app_exit_events.send(AppExit::Success)
                }
                MenuButtonAction::Play => {
                    // println!("Starting game");
                    game_state.set(GameState::Playing);
                    menu_state.set(MenuState::Disabled);
                }
                MenuButtonAction::Settings => menu_state.set(MenuState::Settings),
                MenuButtonAction::SettingsDisplay => {
                    menu_state.set(MenuState::SettingsDisplay);
                }
                MenuButtonAction::SettingsSound => {
                    menu_state.set(MenuState::SettingsSound);
                }
                MenuButtonAction::BackToMainMenu => menu_state.set(MenuState::Main),
                MenuButtonAction::BackToSettings => {
                    menu_state.set(MenuState::Settings);
                }
            }
        }
    }
}

fn setup_menu(mut menu_state: ResMut<NextState<MenuState>>) {
    menu_state.set(MenuState::Main);
    // commands.spawn(Camera2dBundle::default());
}
// This system updates the settings when a new value for a setting is selected, and marks
// the button as the one currently selected
fn setting_button<T: Resource + Component + PartialEq + Copy>(
    interaction_query: Query<(&Interaction, &T, Entity), (Changed<Interaction>, With<Button>)>,
    mut selected_query: Query<(Entity, &mut BackgroundColor), With<SelectedOption>>,
    mut commands: Commands,
    mut setting: ResMut<T>,
) {
    for (interaction, button_setting, entity) in &interaction_query {
        if *interaction == Interaction::Pressed && *setting != *button_setting {
            let Ok((previous_button, mut previous_color)) = selected_query.single_mut() else {
                return;
            };
            *previous_color = NORMAL_BUTTON.into();
            commands.entity(previous_button).remove::<SelectedOption>();
            commands.entity(entity).insert(SelectedOption);
            *setting = *button_setting;
        }
    }
}

fn main_menu_setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    font_assets: Res<FontAssets>,
) {
    // Common style for all buttons on the screen
    let button_style = Style {
        width: Val::Px(400.0),
        height: Val::Px(50.0),
        margin: UiRect::all(Val::Auto),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..Default::default()
    };

    let button_font = TextFont {
        font: font_assets.fira_sans.clone(),
        font_size: 40.0,
        ..default()
    };
    let button_text_color = TextColor(Color::srgb(0.9, 0.9, 0.9));

    let button_icon_style = Style {
        width: Val::Px(30.0),
        // This takes the icons out of the flexbox flow, to be positioned exactly
        position_type: PositionType::Absolute,
        // The icon will be close to the left border of the button
        left: Val::Px(10.0),
        right: Val::Auto,
        ..default()
    };

    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                ..default()
            },
            OnMainMenuScreen,
        ))
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        display: Display::Flex,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    // background_color: Color::CRIMSON.into(),
                    ..default()
                })
                .with_children(|parent| {
                    // Display the game name
                    parent.spawn((
                        Text::new("vigilant-doodle"),
                        TextFont {
                            font: font_assets.fira_sans.clone(),
                            font_size: 60.0,
                            ..default()
                        },
                        TextColor(Color::srgb(0.9, 0.9, 0.9)),
                        Node {
                            margin: UiRect::all(Val::Px(30.0)),
                            ..default()
                        },
                    ));

                    // Display three buttons for each action available from the main menu:
                    // - new game
                    // - settings
                    // - quit
                    parent
                        .spawn((
                            ButtonBundle {
                                style: button_style.clone(),
                                // background_color: NORMAL_BUTTON.into(),
                                ..default()
                            },
                            MenuButtonAction::Play,
                        ))
                        .with_children(|parent| {
                            let icon = asset_server.load("textures/right.png");
                            parent.spawn(ImageBundle {
                                style: button_icon_style.clone(),
                                image: ImageNode::new(icon),
                                ..default()
                            });
                            parent.spawn((
                                Text::new("New Game"),
                                button_font.clone(),
                                button_text_color,
                            ));
                        });
                    parent
                        .spawn((
                            ButtonBundle {
                                style: button_style.clone(),
                                // background_color: NORMAL_BUTTON.into(),
                                ..default()
                            },
                            MenuButtonAction::Settings,
                        ))
                        .with_children(|parent| {
                            let icon = asset_server.load("textures/wrench.png");
                            parent.spawn(ImageBundle {
                                style: button_icon_style.clone(),
                                image: ImageNode::new(icon),
                                ..default()
                            });
                            parent.spawn((
                                Text::new("Settings"),
                                button_font.clone(),
                                button_text_color,
                            ));
                        });
                    parent
                        .spawn((
                            ButtonBundle {
                                style: button_style,
                                // background_color: NORMAL_BUTTON.into(),
                                ..default()
                            },
                            MenuButtonAction::Quit,
                        ))
                        .with_children(|parent| {
                            let icon = asset_server.load("textures/exitRight.png");
                            parent.spawn(ImageBundle {
                                style: button_icon_style.clone(),
                                image: ImageNode::new(icon),
                                ..default()
                            });
                            parent.spawn((
                                Text::new("Quit"),
                                button_font.clone(),
                                button_text_color,
                            ));
                        });
                });
        });
}

fn settings_menu_setup(mut commands: Commands, font_assets: Res<FontAssets>) {
    let button_style = Style {
        width: Val::Px(120.0),
        height: Val::Px(50.0),
        margin: UiRect::all(Val::Auto),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..Default::default()
    };

    let button_font = TextFont {
        font: font_assets.fira_sans.clone(),
        font_size: 40.0,
        ..default()
    };
    let button_text_color = TextColor(Color::srgb(0.9, 0.9, 0.9));

    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                ..default()
            },
            OnSettingsMenuScreen,
        ))
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    // background_color: Color::CRIMSON.into(),
                    ..default()
                })
                .with_children(|parent| {
                    for (action, text) in [
                        (MenuButtonAction::SettingsDisplay, "Display"),
                        (MenuButtonAction::SettingsSound, "Sound"),
                        (MenuButtonAction::BackToMainMenu, "Back"),
                    ] {
                        parent
                            .spawn((
                                ButtonBundle {
                                    style: button_style.clone(),
                                    // background_color: NORMAL_BUTTON.into(),
                                    ..default()
                                },
                                action,
                            ))
                            .with_children(|parent| {
                                parent.spawn((
                                    Text::new(text),
                                    button_font.clone(),
                                    button_text_color,
                                ));
                            });
                    }
                });
        });
}

fn display_settings_menu_setup(
    mut commands: Commands,
    display_quality: Res<DisplayQuality>,
    font_assets: Res<FontAssets>,
) {
    let button_style = Style {
        width: Val::Px(120.0),
        height: Val::Px(50.0),
        margin: UiRect::all(Val::Auto),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..Default::default()
    };

    let button_font = TextFont {
        font: font_assets.fira_sans.clone(),
        font_size: 40.0,
        ..default()
    };
    let button_text_color = TextColor(Color::srgb(0.9, 0.9, 0.9));

    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                ..default()
            },
            OnDisplaySettingsMenuScreen,
        ))
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    // background_color: Color::CRIMSON.into(),
                    ..default()
                })
                .with_children(|parent| {
                    // Create a new `NodeBundle`, this time not setting its `flex_direction`. It will
                    // use the default value, `FlexDirection::Row`, from left to right.
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            // background_color: Color::CRIMSON.into(),
                            ..default()
                        })
                        .with_children(|parent| {
                            // Display a label for the current setting
                            parent.spawn((
                                Text::new("Display Quality"),
                                button_font.clone(),
                                button_text_color,
                            ));
                            // Display a button for each possible value
                            for quality_setting in [
                                DisplayQuality::Low,
                                DisplayQuality::Medium,
                                DisplayQuality::High,
                            ] {
                                let mut entity = parent.spawn(ButtonBundle {
                                    style: Style {
                                        width: Val::Px(150.0),
                                        height: Val::Px(65.0),
                                        ..button_style.clone()
                                    },
                                    // background_color: NORMAL_BUTTON.into(),
                                    ..default()
                                });
                                entity.insert(quality_setting).with_children(|parent| {
                                    parent.spawn((
                                        Text::new(format!("{quality_setting:?}")),
                                        button_font.clone(),
                                        button_text_color,
                                    ));
                                });
                                if *display_quality == quality_setting {
                                    entity.insert(SelectedOption);
                                }
                            }
                        });
                    // Display the back button to return to the settings screen
                    parent
                        .spawn((
                            ButtonBundle {
                                style: button_style,
                                // background_color: NORMAL_BUTTON.into(),
                                ..default()
                            },
                            MenuButtonAction::BackToSettings,
                        ))
                        .with_children(|parent| {
                            parent.spawn((
                                Text::new("Back"),
                                button_font.clone(),
                                button_text_color,
                            ));
                        });
                });
        });
}

fn sound_settings_menu_setup(
    mut commands: Commands,
    volume: Res<Volume>,
    font_assets: Res<FontAssets>,
) {
    let button_style = Style {
        width: Val::Px(120.0),
        height: Val::Px(50.0),
        margin: UiRect::all(Val::Auto),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..Default::default()
    };

    let button_font = TextFont {
        font: font_assets.fira_sans.clone(),
        font_size: 40.0,
        ..default()
    };
    let button_text_color = TextColor(Color::srgb(0.9, 0.9, 0.9));

    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                ..default()
            },
            OnSoundSettingsMenuScreen,
        ))
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    // background_color: Color::CRIMSON.into(),
                    ..default()
                })
                .with_children(|parent| {
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            // background_color: Color::CRIMSON.into(),
                            ..default()
                        })
                        .with_children(|parent| {
                            parent.spawn((
                                Text::new("Volume"),
                                button_font.clone(),
                                button_text_color,
                            ));
                            for volume_setting in [0, 1, 2, 3, 4, 5, 6, 7, 8, 9] {
                                let mut entity = parent.spawn(ButtonBundle {
                                    style: Style {
                                        width: Val::Px(30.0),
                                        height: Val::Px(65.0),
                                        ..button_style.clone()
                                    },
                                    // background_color: NORMAL_BUTTON.into(),
                                    ..default()
                                });
                                entity.insert(Volume(volume_setting));
                                if *volume == Volume(volume_setting) {
                                    entity.insert(SelectedOption);
                                }
                            }
                        });
                    parent
                        .spawn((
                            ButtonBundle {
                                style: button_style,
                                // background_color: NORMAL_BUTTON.into(),
                                ..default()
                            },
                            MenuButtonAction::BackToSettings,
                        ))
                        .with_children(|parent| {
                            parent.spawn((
                                Text::new("Back"),
                                button_font.clone(),
                                button_text_color,
                            ));
                        });
                });
        });
}

// fn menu_action(
//     interaction_query: Query<
//         (&Interaction, &MenuButtonAction),
//         (Changed<Interaction>, With<Button>),
//     >,
//     mut app_exit_events: EventWriter<AppExit>,
//     mut menu_state: ResMut<NextState<MenuState>>,
//     mut game_state: ResMut<NextState<GameState>>,
// ) {
//     for (interaction, menu_button_action) in &interaction_query {
//         if *interaction == Interaction::Pressed {
//             match menu_button_action {
//                 MenuButtonAction::Quit => app_exit_events.send(AppExit),
//                 MenuButtonAction::Play => {
//                     game_state.set(GameState::Playing);
//                     menu_state.set(MenuState::Disabled);
//                 }
//                 MenuButtonAction::Settings => menu_state.set(MenuState::Settings),
//                 MenuButtonAction::SettingsDisplay => {
//                     menu_state.set(MenuState::SettingsDisplay);
//                 }
//                 MenuButtonAction::SettingsSound => {
//                     menu_state.set(MenuState::SettingsSound);
//                 }
//                 MenuButtonAction::BackToMainMenu => menu_state.set(MenuState::Main),
//                 MenuButtonAction::BackToSettings => {
//                     menu_state.set(MenuState::Settings);
//                 }
//             }
//         }
//     }
// }

// Generic system that takes a component as a parameter, and will despawn all entities with that component
fn despawn_screen<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in &to_despawn {
        commands.entity(entity).despawn();
    }
}

// fn cleanup_menu(
//     mut commands: Commands,
//     menu_camera: Query<Entity, With<Camera2d>>,
//     // notice_text: Query<Entity, With<TextBundle>>,
// ) {
//     // commands.entity(button.single()).despawn_recursive();
//     // commands.entity(menu_camera.single()).despawn_recursive();
//     // commands.entity(notice_text.single()).despawn_recursive();
// }
