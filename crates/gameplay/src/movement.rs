use vigilant_doodle_core::state::GameState;
use crate::{Enemy, Player};
use vigilant_doodle_world::terrain::{FLOOR_HALF_LENGTH, FLOOR_HALF_WIDTH};
use bevy::prelude::*;

/// 碰撞体积组件（圆形碰撞）
#[derive(Component)]
pub struct CollisionRadius {
    pub radius: f32,
}

impl CollisionRadius {
    pub fn new(radius: f32) -> Self {
        Self { radius }
    }
}

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                clamp_to_terrain,
                separate_entities, // 分离重叠的实体
            )
                .chain()
                .run_if(in_state(GameState::Playing)),
        );
    }
}

fn clamp_to_terrain(mut query: Query<&mut Transform, Or<(With<Player>, With<Enemy>)>>) {
    for mut transform in query.iter_mut() {
        transform.translation.x = transform
            .translation
            .x
            .clamp(-FLOOR_HALF_LENGTH, FLOOR_HALF_LENGTH);
        transform.translation.z = transform
            .translation
            .z
            .clamp(-FLOOR_HALF_WIDTH, FLOOR_HALF_WIDTH);
    }
}

/// 分离重叠的实体（防止敌人和玩家重叠）
fn separate_entities(
    mut query: Query<(Entity, &mut Transform, &CollisionRadius), Or<(With<Player>, With<Enemy>)>>,
) {
    let mut combinations = query.iter_combinations_mut();

    while let Some(
        [
            (entity_a, mut transform_a, radius_a),
            (entity_b, mut transform_b, radius_b),
        ],
    ) = combinations.fetch_next()
    {
        let delta = transform_a.translation - transform_b.translation;
        let distance = delta.length();
        let min_distance = radius_a.radius + radius_b.radius;

        // 如果重叠，则推开
        if distance < min_distance && distance > 0.001 {
            let push_direction = delta.normalize();
            let push_amount = (min_distance - distance) * 0.5; // 每个实体各推开一半

            // 只在水平面推开（保持 Y 轴不变）
            let push = Vec3::new(
                push_direction.x * push_amount,
                0.0,
                push_direction.z * push_amount,
            );

            transform_a.translation += push;
            transform_b.translation -= push;
        }
    }
}
