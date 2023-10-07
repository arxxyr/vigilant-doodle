use bevy::{
    core_pipeline::bloom::BloomSettings,
    // input::mouse::{MouseMotion, MouseScrollUnit, MouseWheel},
    prelude::*,
};
use bevy_third_person_camera::{ThirdPersonCamera, Zoom};

use crate::player::player::Player;
use crate::{GameState, MovementSystem};
pub struct PlayerCameraPlugin;

#[derive(Component)]
struct PlayerCamera;

// const CAEMRA_ZOOM_SPEED: f32 = 0.5;
// const CAEMRA_ZOOM_MIN: f32 = 5.0;
// const CAEMRA_ZOOM_MAX: f32 = 50.0;

impl Plugin for PlayerCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Loading), spawn_camera)
            .add_systems(OnEnter(GameState::Playing), bind_player)
            .add_systems(
                Update,
                (update_behaviour, update_camera)
                    .chain()
                    .in_set(MovementSystem::Input)
                    .run_if(in_state(GameState::Playing)),
            )
            // .add_systems(Update, update_camera)
            .add_systems(OnExit(GameState::Playing), reset_cursor);
    }
}

fn spawn_camera(mut commands: Commands) {
    let camera = (
        Camera3dBundle {
            transform: Transform::from_xyz(0.0, 8.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        BloomSettings {
            intensity: 0.25, // the default is 0.3
            ..default()
        },
        ThirdPersonCamera {
            zoom: Zoom::new(10.0, 25.0),
            // cursor_lock_key: KeyCode::Tab,
            cursor_lock_active: false,
            mouse_orbit_button_enabled: false,
            // zoom_sensitivity: 1.0,
            // mouse_sensitivity: 1.0,
            mouse_sensitivity: 0.0,
            ..Default::default()
        },
        PlayerCamera,
        Name::new("fov"),
    );
    commands.spawn(camera);
}

fn update_behaviour(
    keyboard_input: Res<Input<KeyCode>>,
    // mut scroll_evr: EventReader<MouseWheel>,
    mut game_state: ResMut<NextState<GameState>>,
    mut third_camera: Query<&mut ThirdPersonCamera>,
) {
    if keyboard_input.just_pressed(KeyCode::Tab) {
        third_camera.for_each_mut(|mut third| {
            third.cursor_lock_active = !third.cursor_lock_active;
        });
        game_state.set(GameState::Menu);
    }
    // for ev in scroll_evr.iter() {
    //     match ev.unit {
    //         MouseScrollUnit::Line => {
    //             println!(
    //                 "Scroll (line units): vertical: {}, horizontal: {}",
    //                 ev.y, ev.x
    //             );
    //             // if ev.y > 0.0 {
    //             //     third_camera.for_each_mut(|mut third| {
    //             //         third.zoom.distance -= CAEMRA_ZOOM_SPEED;
    //             //         if third.zoom.distance < CAEMRA_ZOOM_MIN {
    //             //             third.zoom.distance = CAEMRA_ZOOM_MIN;
    //             //         }
    //             //     });
    //             // } else {
    //             //     third_camera.for_each_mut(|mut third| {
    //             //         third.zoom.distance += CAEMRA_ZOOM_SPEED;
    //             //         if third.zoom.distance > CAEMRA_ZOOM_MAX {
    //             //             third.zoom.distance = CAEMRA_ZOOM_MAX;
    //             //         }
    //             //     });
    //             // }
    //         }
    //         MouseScrollUnit::Pixel => {
    //             println!(
    //                 "Scroll (pixel units): vertical: {}, horizontal: {}",
    //                 ev.y, ev.x
    //             );
    //         }
    //     }
    // }
}

fn update_camera(
    player_query: Query<&Transform, With<Player>>,
    mut third_camera: Query<&mut ThirdPersonCamera, (With<PlayerCamera>, Without<Player>)>,
    // time: Res<Time>,
    // mouse_motion_events: EventReader<MouseMotion>,
    // mut query: Query<(&Transform, &mut GlobalTransform)>,
) {
    if let Some(player_transform) = player_query.iter().next() {
        for mut third in third_camera.iter_mut() {
            // third.mouse_sensitivity = 0.0;
            third.focus = player_transform.translation;

            // camera.translation.y = player_transform.translation;
            // third.
            // camera_transform.translation = player_transform.translation + Vec3::new(0.0, 5.0, 5.0);
            // camera_transform.look_at(player_transform.translation, Vec3::Y);
        }
    }
}

fn bind_player(mut third_camera: Query<&mut ThirdPersonCamera>) {
    third_camera.for_each_mut(|mut third| {
        third.cursor_lock_active = true;
    });
}

fn reset_cursor(mut third_camera: Query<&mut ThirdPersonCamera>, mut windows: Query<&mut Window>) {
    third_camera.for_each_mut(|mut third| {
        third.cursor_lock_active = false;
    });
    let mut www = windows.single_mut();
    let width = www.width();
    let height = www.height();
    www.set_cursor_position(Some(Vec2::new(width / 2., height / 2.)));
}
