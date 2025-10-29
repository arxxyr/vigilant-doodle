//! 寻敌检测系统
//!
//! 负责敌人对玩家的检测逻辑，包括：
//! - 视野范围检测
//! - 距离计算
//! - 失去目标判定

use bevy::prelude::*;

/// 检测配置组件
///
/// 控制敌人的检测行为参数
#[derive(Component, Clone)]
pub struct DetectionConfig {
    /// 初始检测范围（敌人首次发现玩家的距离）
    pub detection_range: f32,

    /// 失去目标范围（一旦发现后，需要这个距离才会失去目标）
    /// 通常设置为 detection_range 的 1.5-2 倍，避免频繁切换状态
    pub lose_target_range: f32,

    /// 视野角度（度数，未来扩展用）
    /// 0-360，180 表示前方半圆
    #[allow(dead_code)]
    pub field_of_view: f32,
}

impl Default for DetectionConfig {
    fn default() -> Self {
        Self {
            detection_range: 35.0,
            lose_target_range: 50.0,  // 更大的范围避免反复切换
            field_of_view: 360.0,      // 全方位检测
        }
    }
}

/// 检测结果
pub enum DetectionResult {
    /// 未检测到目标
    NoTarget,
    /// 检测到新目标
    NewTarget,
    /// 保持当前目标
    KeepTarget,
    /// 失去目标
    LostTarget,
}

/// 检测系统
pub struct DetectionSystem;

impl DetectionSystem {
    /// 检查是否能检测到玩家（首次发现）
    pub fn can_detect_player(
        config: &DetectionConfig,
        enemy_pos: Vec3,
        player_pos: Vec3,
    ) -> bool {
        let distance = enemy_pos.distance(player_pos);
        distance <= config.detection_range
    }

    /// 检查是否应该失去目标（已追踪状态）
    pub fn should_lose_target(
        config: &DetectionConfig,
        enemy_pos: Vec3,
        player_pos: Vec3,
    ) -> bool {
        let distance = enemy_pos.distance(player_pos);
        distance > config.lose_target_range
    }

    /// 计算到玩家的方向向量（归一化）
    pub fn direction_to_player(enemy_pos: Vec3, player_pos: Vec3) -> Vec3 {
        (player_pos - enemy_pos).normalize_or_zero()
    }

    /// 计算到玩家的距离
    pub fn distance_to_player(enemy_pos: Vec3, player_pos: Vec3) -> f32 {
        enemy_pos.distance(player_pos)
    }

    /// 综合检测逻辑（考虑当前是否已有目标）
    pub fn detect(
        config: &DetectionConfig,
        enemy_pos: Vec3,
        player_pos: Vec3,
        has_target: bool,
    ) -> DetectionResult {
        let distance = Self::distance_to_player(enemy_pos, player_pos);

        if has_target {
            // 已有目标，检查是否失去
            if distance > config.lose_target_range {
                DetectionResult::LostTarget
            } else {
                DetectionResult::KeepTarget
            }
        } else {
            // 无目标，检查是否发现
            if distance <= config.detection_range {
                DetectionResult::NewTarget
            } else {
                DetectionResult::NoTarget
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detection_range() {
        let config = DetectionConfig::default();
        let enemy_pos = Vec3::ZERO;

        // 在检测范围内
        let player_pos_in = Vec3::new(30.0, 0.0, 0.0);
        assert!(DetectionSystem::can_detect_player(&config, enemy_pos, player_pos_in));

        // 在检测范围外
        let player_pos_out = Vec3::new(40.0, 0.0, 0.0);
        assert!(!DetectionSystem::can_detect_player(&config, enemy_pos, player_pos_out));
    }

    #[test]
    fn test_lose_target_range() {
        let config = DetectionConfig::default();
        let enemy_pos = Vec3::ZERO;

        // 在失去目标范围内
        let player_pos_in = Vec3::new(45.0, 0.0, 0.0);
        assert!(!DetectionSystem::should_lose_target(&config, enemy_pos, player_pos_in));

        // 超出失去目标范围
        let player_pos_out = Vec3::new(55.0, 0.0, 0.0);
        assert!(DetectionSystem::should_lose_target(&config, enemy_pos, player_pos_out));
    }

    #[test]
    fn test_direction_calculation() {
        let enemy_pos = Vec3::ZERO;
        let player_pos = Vec3::new(10.0, 0.0, 0.0);

        let direction = DetectionSystem::direction_to_player(enemy_pos, player_pos);
        assert_eq!(direction, Vec3::X);
    }
}
