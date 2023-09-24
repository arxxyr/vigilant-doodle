use crate::actions::Actions;
use crate::loading::TextureAssets;
use crate::GameState;
use bevy::prelude::*;
use bevy_third_person_camera::ThirdPersonCameraTarget;

pub struct PlayerPlugin;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
struct Speed(f32);

/// This plugin handles player related stuff like movement
/// Player logic is only active during the State `GameState::Playing`
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), spawn_player)
            .add_systems(Update, move_player.run_if(in_state(GameState::Playing)));
    }
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
            transform: Transform::from_xyz(0.0, 0.5, 0.0),
            ..default()
        },
        Player,
        Speed(2.5),
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
    if actions.player_movement.is_none() {
        return;
    }
    
    for (mut player_transform,player_speed )in &mut player_query {
        
        let movement = Vec3::new(
            actions.player_movement.unwrap().x * player_speed.0 * time.delta_seconds(),
            actions.player_movement.unwrap().y * player_speed.0 * time.delta_seconds(),
            0.,
        );
        player_transform.translation += movement;
        if movement.length_squared() > 0.0 {
            player_transform.look_to(movement, Vec3::Y);
        }
    }
}
