//! 敌人 AI 组件设置
//!
//! 为敌人实体添加 AI 组件，使用平衡配置

use bevy::prelude::*;
use vigilant_doodle_ai::{DetectionConfig, EnemyAIConfig, EnemyAIState, EnemyTarget};
use vigilant_doodle_core::{BalanceConfig, GameState};
use vigilant_doodle_gameplay::Enemy;

pub struct EnemySetupPlugin;

impl Plugin for EnemySetupPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            setup_enemy_ai.run_if(in_state(GameState::AssetLoading)),
        );
    }
}

/// 为新生成的敌人添加 AI 组件
fn setup_enemy_ai(
    mut commands: Commands,
    enemy_query: Query<Entity, (With<Enemy>, Without<EnemyAIState>)>,
    balance: Res<BalanceConfig>,
) {
    for entity in enemy_query.iter() {
        // 从配置文件创建 AI 组件
        let ai_config = EnemyAIConfig {
            move_speed: balance.enemy.ai.move_speed,
            stop_distance: balance.enemy.ai.stop_distance,
            rotation_speed: balance.enemy.ai.rotation_speed,
        };

        let detection_config = DetectionConfig {
            detection_range: balance.enemy.detection.detection_range,
            lose_target_range: balance.enemy.detection.lose_target_range,
            field_of_view: balance.enemy.detection.field_of_view,
        };

        // 保存值用于日志输出
        let move_speed = ai_config.move_speed;
        let detection_range = detection_config.detection_range;

        commands.entity(entity).insert((
            EnemyAIState::default(),
            ai_config,
            detection_config,
            EnemyTarget::default(),
        ));

        debug!(
            "[EnemySetup] 敌人 AI 配置: 速度={}, 检测范围={}",
            move_speed, detection_range
        );
    }
}
