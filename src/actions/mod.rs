use bevy::math::Vec3Swizzles;
use bevy::prelude::*;

use crate::actions::game_control::{get_movement, GameControl};
use crate::player::Player;
use crate::player::Speed;
use crate::GameState;

mod game_control;

pub const FOLLOW_EPSILON: f32 = 5.;

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

    // if let Some(touch_position) = touch_input.first_pressed_position() {
    //     let cam = match camera.get_single(){
    //         Ok(c) =>c,
    //         Err(e) => panic!("Error getting camera: {}", e),
    //     };
    //     let mut direction = Vec3::ZERO;

    //     direction += player_movement.normalize_or_zero();

    //     if let Some(touch_position) = Camera2d.viewport_to_world_2d(camera_transform, touch_position)
    //     {
    //         let diff = touch_position - player.single().translation.xy();
    //         if diff.length() > FOLLOW_EPSILON {
    //             player_movement.x = diff.normalize().x;
    //             player_movement.y = diff.normalize().y;
    //             player_movement.z = 0.;
    //         }
    //     }
    // }

    if direction != Vec3::ZERO {
        // println!("direction: {:?}", direction);
        // let mut direction_unit = Some(direction.normalize());
        // if get_movement(GameControl::Jump, &keyboard_input) > 0. {
        //     direction_unit.y = 1.;
        // } else {
        //     direction_unit.y = 0.;
        // }
        actions.player_movement = Some(direction.normalize());
    } else {
        actions.player_movement = None;
    }
    // }
}
