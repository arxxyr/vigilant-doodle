use vigilant_doodle_core::state::GameState;
use crate::{Enemy, Player};
use vigilant_doodle_world::terrain::{FLOOR_HALF_LENGTH, FLOOR_HALF_WIDTH};
use bevy::prelude::*;

/// 移动系统集合（定义执行顺序）
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub enum MovementSystemSet {
    /// 碰撞分离（先执行）
    Separation,
    /// 地形限制（后执行）
    TerrainClamping,
}

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
        app
            // 配置系统集合的执行顺序
            .configure_sets(
                Update,
                (
                    MovementSystemSet::Separation,
                    MovementSystemSet::TerrainClamping,
                )
                    .chain() // 先碰撞分离，再地形限制
                    .run_if(in_state(GameState::Playing)),
            )
            // 添加系统到对应的集合
            .add_systems(
                Update,
                separate_entities.in_set(MovementSystemSet::Separation),
            )
            .add_systems(
                Update,
                clamp_to_terrain.in_set(MovementSystemSet::TerrainClamping),
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
    // 碰撞分离参数
    const SEPARATION_DAMPING: f32 = 0.3; // 阻尼系数（降低推力强度）
    const MIN_OVERLAP_THRESHOLD: f32 = 0.05; // 最小重叠阈值（小于此值不推开）

    let mut combinations = query.iter_combinations_mut();

    while let Some(
        [
            (_entity_a, mut transform_a, radius_a),
            (_entity_b, mut transform_b, radius_b),
        ],
    ) = combinations.fetch_next()
    {
        let delta = transform_a.translation - transform_b.translation;
        let distance = delta.length();
        let min_distance = radius_a.radius + radius_b.radius;
        let overlap = min_distance - distance;

        // 如果重叠超过阈值，则推开
        if overlap > MIN_OVERLAP_THRESHOLD && distance > 0.001 {
            let push_direction = delta.normalize();
            // 应用阻尼系数，使推力更平滑
            let push_amount = overlap * 0.5 * SEPARATION_DAMPING;

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
