use crate::actions::Actions;

use crate::GameState;
use bevy_third_person_camera::ThirdPersonCameraTarget;

use bevy::{core_pipeline::bloom::BloomSettings, prelude::*};
use bevy_third_person_camera::{ThirdPersonCamera, Zoom};

pub struct PlayerPlugin;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Speed(f32);

struct V_g(f32);

/// This plugin handles player related stuff like movement
/// Player logic is only active during the State `GameState::Playing`
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), (spawn_player, spawn_camera))
            .add_systems(Update, move_player.run_if(in_state(GameState::Playing)));
    }
}
fn spawn_camera(mut commands: Commands) {
    let camera = (
        Camera3dBundle {
            transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        BloomSettings {
            intensity: 0.25, // the default is 0.3
            ..default()
        },
        ThirdPersonCamera {
            zoom: Zoom::new(1.5, 5.0),
            cursor_lock_key: KeyCode::Tab,
            // zoom_sensitivity: 1.0,
            ..Default::default()
        },
        Name::new("FOV"),
    );
    commands.spawn(camera);
}

fn spawn_player(mut commands: Commands, assets: Res<AssetServer>) {
    let flashlight = (
        SpotLightBundle {
            spot_light: SpotLight {
                color: Color::rgba(1.0, 1.0, 0.47, 1.0),
                range: 10.0,
                intensity: 4000.0,
                outer_angle: 0.5,
                inner_angle: 0.4,
                shadows_enabled: true,
                ..default()
            },
            transform: Transform::from_xyz(0.083, -0.093, -0.52),
            ..default()
        },
        Name::new("FlashLight"),
    );
    let player = (
        SceneBundle {
            scene: assets.load("model/Player.gltf#Scene0"),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        },
        Player,
        Speed(15.0),
        ThirdPersonCameraTarget,
        Name::new("ffqi"),
    );

    commands.spawn(player).with_children(|parent| {
        parent.spawn(flashlight);
    });
    // commands
    //     .spawn(SpriteBundle {
    //         texture: textures.texture_bevy.clone(),
    //         transform: Transform::from_translation(Vec3::new(0., 0., 1.)),
    //         ..Default::default()
    //     })
    //     .insert(Player);
}

fn move_player(
    time: Res<Time>,
    actions: Res<Actions>,
    mut player_query: Query<(&mut Transform, &Speed), With<Player>>,
) {
    let mut player_transform = player_query.single_mut().0;
    // let mut y = player_transform.translation.y;
    // // acceleration is 9.81 m/s^2

    // if player_transform.translation.y > 0.0 {
    //     player_transform.translation.y -= time.delta_seconds() * time.delta_seconds() / 9.81;
    // }

    // if y < 0.0 {
    //     y = 0.0;
    // }
    // player_transform.translation.y = y;

    if actions.player_movement.is_none() {
        if player_transform.translation.y > 0.0 {
            // V_g = 9.81 * time.delta_seconds();
            player_transform.translation.y -= 9.81 * time.delta_seconds() * time.delta_seconds();
        }
        if player_transform.translation.y < 0.0 {
            player_transform.translation.y = 0.0;
        }

        return;
    }

    for (mut player_transform, player_speed) in &mut player_query {
        // if player_transform.translation.y > 0.0 {
        //     actions.player_movement.unwrap().y -= 9.81 * time.delta_seconds();
        // } else {
        //     0.0;
        // }
        // acoording gravity formula: v = v0 + a * t,caculate y
        // player_transform.translation.y += actions.player_movement.unwrap().y * time.delta_seconds()
        //     - time.delta_seconds() * time.delta_seconds() / 9.81;
        // player_transform.translation.y = 0.0;
        let movement = Vec3::new(
            actions.player_movement.unwrap().x * player_speed.0 * time.delta_seconds(),
            (actions.player_movement.unwrap().y * player_speed.0 - 9.81 * time.delta_seconds())
                * time.delta_seconds(),
            actions.player_movement.unwrap().z * player_speed.0 * time.delta_seconds(),
        );
        // println!("movement   : {:?}", movement);
        // println!("translation: {:?}", player_transform.translation);
        // println!("delta      : {:?}", time.delta_seconds());
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
