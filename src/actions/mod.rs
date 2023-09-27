use bevy::prelude::*;

use crate::actions::game_control::{get_movement, GameControl};
use crate::player::Player;
use crate::GameState;

mod game_control;

pub struct ActionsPlugin;

// This plugin listens for keyboard input and converts the input into Actions
// Actions can then be used as a resource in other systems to act on the player input.
impl Plugin for ActionsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Actions>().add_systems(
            Update,
            set_movement_actions.run_if(in_state(GameState::Playing)),
        );
    }
}

#[derive(Default, Resource)]
pub struct Actions {
    pub player_movement: Option<Vec3>,
}

pub fn set_movement_actions(
    mut actions: ResMut<Actions>,
    keyboard_input: Res<Input<KeyCode>>,
    // mut third_camera: Query<&mut ThirdPersonCamera, (With<ThirdPersonCamera>, Without<Player>)>,
    // mut game_state: ResMut<NextState<GameState>>,
    // touch_input: Res<Touches>,
    // player: Query<(&mut Transform, &Speed), With<Player>>,
    camera: Query<&Transform, (With<Camera3d>, Without<Player>)>,
) {
    // for (mut player_transform, player_speed) in player.iter_mut() {
    let cam = match camera.get_single() {
        Ok(c) => c,
        Err(e) => panic!("Error getting camera: {}", e),
    };

    let mut direction = Vec3::ZERO;

    if get_movement(GameControl::Forward, &keyboard_input) > 0. {
        direction += cam.forward();
    }
    if get_movement(GameControl::Back, &keyboard_input) > 0. {
        direction -= cam.forward();
    }
    if get_movement(GameControl::Left, &keyboard_input) > 0. {
        direction -= cam.right();
    }
    if get_movement(GameControl::Right, &keyboard_input) > 0. {
        direction += cam.right();
    }

    if get_movement(GameControl::Jump, &keyboard_input) > 0. {
        direction += cam.up();
    } else {
        direction.y = 0.;
    }

    if direction != Vec3::ZERO {
        actions.player_movement = Some(direction.normalize());
    } else {
        actions.player_movement = None;
    }
    // }
}
