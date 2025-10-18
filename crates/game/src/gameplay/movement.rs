use bevy::prelude::*;
use crate::core::state::GameState;
use crate::gameplay::{player::Player, enemy::Enemy};
use crate::world::terrain::{FLOOR_HALF_LENGTH, FLOOR_HALF_WIDTH};

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            clamp_to_terrain
                .run_if(in_state(GameState::Playing))
        );
    }
}

fn clamp_to_terrain(
    mut query: Query<&mut Transform, Or<(With<Player>, With<Enemy>)>>,
) {
    for mut transform in query.iter_mut() {
        transform.translation.x = transform.translation.x.clamp(
            -FLOOR_HALF_LENGTH,
            FLOOR_HALF_LENGTH
        );
        transform.translation.z = transform.translation.z.clamp(
            -FLOOR_HALF_WIDTH,
            FLOOR_HALF_WIDTH
        );
    }
}
