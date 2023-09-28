use crate::actions::Actions;

use crate::world::world::FLOOR_SIZE;
use crate::GameState;
// use bevy_third_person_camera::ThirdPersonCameraTarget;

use bevy::prelude::*;
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
            .add_systems(Update, move_player.run_if(in_state(GameState::Playing)))
            .add_systems(
                Update,
                collision_detection
                    .run_if(in_state(GameState::Playing))
                    .after(move_player),
            );
    }
}

fn spawn_player(mut commands: Commands, assets: Res<AssetServer>) {
    let flashlight = (
        SpotLightBundle {
            spot_light: SpotLight {
                color: Color::rgba(1.0, 1.0, 0.47, 1.0),
                range: 100.0,
                intensity: 7000.0,
                outer_angle: 0.6,
                inner_angle: 0.4,
                shadows_enabled: true,
                ..default()
            },
            transform: Transform::from_xyz(0.083, -0.093, -0.50),
            ..default()
        },
        Name::new("FlashLight"),
    );
    let player = (
        SceneBundle {
            scene: assets.load("model/Player.gltf#Scene0"),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            visibility: Visibility::Hidden,
            ..default()
        },
        Player,
        Speed(15.0),
        // ThirdPersonCameraTarget,
        Name::new("XiaoYu"),
    );

    commands.spawn(player).with_children(|parent| {
        parent.spawn(flashlight);
    });
}

fn show_player(mut commands: Commands, mut player: Query<Entity, With<Player>>) {
    for entity in &mut player {
        commands.entity(entity).insert(Visibility::Visible);
    }
}

fn hide_player(mut commands: Commands, mut player: Query<Entity, With<Player>>) {
    for entity in &mut player {
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

    for (mut player_transform, player_speed) in &mut player_query {
        let movement = Vec3::new(
            actions.player_movement.unwrap().x * player_speed.0 * time.delta_seconds(),
            (actions.player_movement.unwrap().y * player_speed.0 - 9.81 * time.delta_seconds())
                * time.delta_seconds(),
            actions.player_movement.unwrap().z * player_speed.0 * time.delta_seconds(),
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

fn collision_detection(mut player_query: Query<&mut Transform, With<Player>>) {
    if let Ok(mut player_transform) = player_query.get_single_mut() {
        if player_transform.translation.x > (FLOOR_SIZE - 2.0) / 2.0 {
            player_transform.translation.x = (FLOOR_SIZE - 2.0) / 2.0 - 0.2;
        }
        if player_transform.translation.x < -(FLOOR_SIZE - 2.0) / 2.0 {
            player_transform.translation.x = -(FLOOR_SIZE - 2.0) / 2.0 + 0.2;
        }
        if player_transform.translation.z > (FLOOR_SIZE - 2.0) / 2.0 {
            player_transform.translation.z = (FLOOR_SIZE - 2.0) / 2.0 - 0.2;
        }
        if player_transform.translation.z < -(FLOOR_SIZE - 2.0) / 2.0 {
            player_transform.translation.z = -(FLOOR_SIZE - 2.0) / 2.0 + 0.2;
        }
    }
}
