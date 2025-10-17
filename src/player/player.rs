use crate::actions::Actions;

use crate::world::world::{FLOOR_LENGTH, FLOOR_WIDTH};
use crate::GameState;
use crate::MovementSystem;
// use bevy_third_person_camera::ThirdPersonCameraTarget;

use bevy::prelude::*;
use bevy::color::palettes::css::YELLOW_GREEN;
use bevy_state::prelude::*;
// use bevy_third_person_camera::{ThirdPersonCamera, Zoom};

pub struct PlayerPlugin;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Speed(f32);

/// This plugin handles player related stuff like movement
/// Player logic is only active during the State `GameState::Playing`
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Loading), spawn_player)
            .add_systems(OnEnter(GameState::Playing), show_player)
            .add_systems(OnExit(GameState::Playing), hide_player)
            .add_systems(
                Update,
                (move_player, player_collision_detection)
                    .chain()
                    .in_set(MovementSystem::PlayerMovement)
                    .run_if(in_state(GameState::Playing)),
            );
    }
}

fn spawn_player(mut commands: Commands, assets: Res<AssetServer>) {
    let flashlight = (
        SpotLightBundle {
            spot_light: SpotLight {
                color: YELLOW_GREEN.into(),
                range: 1000.0,
                intensity: 15000.0,
                outer_angle: 0.7,
                inner_angle: 0.3,
                shadows_enabled: true,
                ..default()
            },
            // transform: Transform::from_xyz(0.083, -0.093, -0.50),
            transform: Transform::from_xyz(0.0, 30.0, 0.0).looking_at(Vec3::ZERO, Vec3::Y),
            // transform: Transform::from_rotation(Quat::from_rotation_x(std::f32::consts::PI/2.0)),
            // transform: light_trans.look_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        Name::new("FlashLight"),
    );
    let player = (
        SceneBundle {
            scene: assets.load("model/player.glb#Scene0"),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            visibility: Visibility::Hidden,
            ..default()
        },
        Player,
        Speed(10.0),
        // ThirdPersonCameraTarget,
        Name::new("XiaoYu"),
    );

    commands.spawn(player).with_children(|parent| {
        parent.spawn(flashlight);
    });
}

fn show_player(mut commands: Commands, player: Query<Entity, With<Player>>) {
    for entity in player.iter() {
        commands.entity(entity).insert(Visibility::Visible);
    }
}

fn hide_player(mut commands: Commands, player: Query<Entity, With<Player>>) {
    for entity in player.iter() {
        commands.entity(entity).insert(Visibility::Hidden);
    }
}

fn move_player(
    time: Res<Time>,
    actions: Res<Actions>,
    mut player_query: Query<(&mut Transform, &Speed), With<Player>>,
) {
    if actions.player_movement.is_none() {
        return;
    }

    if let Ok((mut player_transform, player_speed)) = player_query.single_mut() {
        let movement = Vec3::new(
            actions.player_movement.unwrap().x * player_speed.0 * time.delta_secs(),
            (actions.player_movement.unwrap().y * player_speed.0 - 9.81 * time.delta_secs())
                * time.delta_secs(),
            actions.player_movement.unwrap().z * player_speed.0 * time.delta_secs(),
        );
        player_transform.translation.x += movement.x;
        player_transform.translation.z += movement.z;
        player_transform.translation.y += movement.y;
        if player_transform.translation.y < 0.0 {
            player_transform.translation.y = 0.0;
        }

        if movement.length_squared() > 0.0 {
            player_transform.look_to(
                Vec3 {
                    x: (movement.x),
                    y: (0.0),
                    z: (movement.z),
                },
                Vec3::Y,
            );
        }
    }
}

fn player_collision_detection(mut player_query: Query<&mut Transform, With<Player>>) {
    if let Ok(mut player_transform) = player_query.single_mut() {
        if player_transform.translation.x > (FLOOR_LENGTH - 2.0) / 2.0 {
            player_transform.translation.x = (FLOOR_LENGTH - 2.0) / 2.0 - 0.2;
        }
        if player_transform.translation.x < -(FLOOR_LENGTH - 2.0) / 2.0 {
            player_transform.translation.x = -(FLOOR_LENGTH - 2.0) / 2.0 + 0.2;
        }
        if player_transform.translation.z > (FLOOR_WIDTH - 2.0) / 2.0 {
            player_transform.translation.z = (FLOOR_WIDTH - 2.0) / 2.0 - 0.2;
        }
        if player_transform.translation.z < -(FLOOR_WIDTH - 2.0) / 2.0 {
            player_transform.translation.z = -(FLOOR_WIDTH - 2.0) / 2.0 + 0.2;
        }
    }
}
