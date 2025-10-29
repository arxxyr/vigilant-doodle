//! 敌人行为执行系统
//!
//! 负责根据 AI 状态执行具体的行为逻辑

use bevy::prelude::*;

use super::detection::{DetectionConfig, DetectionResult, DetectionSystem};
use super::enemy_ai::{AIEvent, EnemyAIConfig, EnemyAIState, EnemyTarget};
use vigilant_doodle_gameplay::Player;

/// 行为系统集合
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub enum BehaviorSystemSet {
    /// 检测阶段：检测玩家
    Detection,
    /// 决策阶段：状态转换
    Decision,
    /// 执行阶段：执行行为
    Execution,
}

/// AI 行为插件（内部使用）
pub struct BehaviorSystemPlugin;

impl Plugin for BehaviorSystemPlugin {
    fn build(&self, app: &mut App) {
        use vigilant_doodle_core::state::GameState;

        app.configure_sets(
            Update,
            (
                BehaviorSystemSet::Detection,
                BehaviorSystemSet::Decision,
                BehaviorSystemSet::Execution,
            )
                .chain()
                .run_if(in_state(GameState::Playing)),
        )
        .add_systems(
            Update,
            (
                detect_player_system.in_set(BehaviorSystemSet::Detection),
                update_ai_state_system.in_set(BehaviorSystemSet::Decision),
                (
                    chase_behavior_system,
                    idle_behavior_system,
                )
                    .in_set(BehaviorSystemSet::Execution),
            ),
        );
    }
}

/// 检测玩家系统
///
/// 检测玩家并更新目标信息
fn detect_player_system(
    player_query: Query<(Entity, &Transform), With<Player>>,
    mut enemy_query: Query<(
        &Transform,
        &DetectionConfig,
        &mut EnemyTarget,
        &EnemyAIState,
    )>,
) {
    let Ok((player_entity, player_transform)) = player_query.single() else {
        return;
    };

    for (enemy_transform, detection_config, mut target, state) in enemy_query.iter_mut() {
        let enemy_pos = enemy_transform.translation;
        let player_pos = player_transform.translation;

        // 执行检测
        let detection_result = DetectionSystem::detect(
            detection_config,
            enemy_pos,
            player_pos,
            state.has_target(),
        );

        match detection_result {
            DetectionResult::NewTarget => {
                // 发现新目标
                target.set_target(player_entity, player_pos);
                debug!("[AI] 敌人发现玩家！");
            }
            DetectionResult::KeepTarget => {
                // 保持追踪，更新位置
                target.update_position(player_pos);
            }
            DetectionResult::LostTarget => {
                // 失去目标
                target.clear_target();
                debug!("[AI] 敌人失去玩家！");
            }
            DetectionResult::NoTarget => {
                // 无目标，不做操作
            }
        }
    }
}

/// 更新 AI 状态系统
///
/// 根据检测结果更新 AI 状态
fn update_ai_state_system(
    mut enemy_query: Query<(&EnemyTarget, &mut EnemyAIState)>,
) {
    for (target, mut state) in enemy_query.iter_mut() {
        let old_state = *state;

        // 根据目标状态生成事件
        let event = if target.has_target() {
            AIEvent::PlayerDetected
        } else if old_state.has_target() {
            AIEvent::PlayerLost
        } else {
            continue; // 无状态变化
        };

        // 状态转换
        *state = state.transition(event);

        // 日志输出状态变化
        if *state != old_state {
            debug!(
                "[AI] 状态转换: {} -> {}",
                old_state.display_name(),
                state.display_name()
            );
        }
    }
}

/// 追击行为系统
///
/// 执行追击玩家的行为
fn chase_behavior_system(
    mut enemy_query: Query<(
        &mut Transform,
        &EnemyAIConfig,
        &EnemyTarget,
        &EnemyAIState,
    )>,
    time: Res<Time>,
) {
    for (mut transform, config, target, state) in enemy_query.iter_mut() {
        // 只在追击状态执行
        if *state != EnemyAIState::Chasing {
            continue;
        }

        // 获取目标位置
        let Some(target_pos) = target.last_known_position else {
            continue;
        };

        let enemy_pos = transform.translation;
        let to_target = target_pos - enemy_pos;
        let distance = to_target.length();

        // 如果距离足够近，停止移动
        if distance <= config.stop_distance {
            continue;
        }

        let direction = to_target.normalize();
        let movement = direction * config.move_speed * time.delta_secs();

        // 只在水平面移动（Y 轴保持不变）
        transform.translation.x += movement.x;
        transform.translation.z += movement.z;

        // 朝向目标
        let look_direction = Vec3::new(direction.x, 0.0, direction.z);
        if look_direction.length() > 0.001 {
            transform.look_to(look_direction, Vec3::Y);
        }
    }
}

/// 闲置行为系统
///
/// 执行闲置状态的行为（当前为空，未来可以添加动画等）
fn idle_behavior_system(
    _enemy_query: Query<&EnemyAIState>,
) {
    // 未来可以添加：
    // - 播放闲置动画
    // - 随机转向
    // - 等待玩家接近
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_behavior_system_sets() {
        // 验证系统集合的定义
        let detection = BehaviorSystemSet::Detection;
        let decision = BehaviorSystemSet::Decision;
        let execution = BehaviorSystemSet::Execution;

        assert_ne!(detection, decision);
        assert_ne!(decision, execution);
    }
}
