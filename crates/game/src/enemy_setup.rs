//! 敌人 AI 组件设置
//!
//! 为敌人实体添加 AI 组件

use bevy::prelude::*;
use vigilant_doodle_ai::{DetectionConfig, EnemyAIConfig, EnemyAIState, EnemyTarget};
use vigilant_doodle_gameplay::Enemy;
use vigilant_doodle_core::GameState;

pub struct EnemySetupPlugin;

impl Plugin for EnemySetupPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            setup_enemy_ai
                .run_if(in_state(GameState::AssetLoading))
        );
    }
}

/// 为新生成的敌人添加 AI 组件
fn setup_enemy_ai(
    mut commands: Commands,
    enemy_query: Query<Entity, (With<Enemy>, Without<EnemyAIState>)>,
) {
    for entity in enemy_query.iter() {
        commands.entity(entity).insert((
            EnemyAIState::default(),
            EnemyAIConfig::default(),
            DetectionConfig::default(),
            EnemyTarget::default(),
        ));
    }
}
