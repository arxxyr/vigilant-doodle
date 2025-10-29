//! 敌人 AI 状态机
//!
//! 定义敌人的 AI 状态和状态转换逻辑

use bevy::prelude::*;

/// 敌人 AI 状态
#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum EnemyAIState {
    /// 闲置：原地待命，等待玩家接近
    #[default]
    Idle,

    /// 巡逻：在区域内随机移动（未来扩展）
    #[allow(dead_code)]
    Patrol,

    /// 追击：持续追踪玩家
    /// 一旦进入此状态，即使玩家超出初始检测范围也会继续追踪
    /// 直到超出 lose_target_range 才会停止
    Chasing,

    /// 攻击：在攻击范围内（未来扩展）
    #[allow(dead_code)]
    Attacking,

    /// 搜索：失去目标后，朝向玩家最后已知位置移动（未来扩展）
    #[allow(dead_code)]
    Searching,

    /// 撤退：低血量时逃跑（未来扩展）
    #[allow(dead_code)]
    Retreating,
}

impl EnemyAIState {
    /// 状态转换逻辑
    pub fn transition(&self, event: AIEvent) -> Self {
        match (self, event) {
            // 闲置状态
            (Self::Idle, AIEvent::PlayerDetected) => Self::Chasing,

            // 追击状态
            (Self::Chasing, AIEvent::PlayerLost) => Self::Idle,
            (Self::Chasing, AIEvent::PlayerDetected) => Self::Chasing, // 保持追击

            // 其他状态保持不变
            (state, _) => *state,
        }
    }

    /// 获取状态的显示名称（用于调试）
    pub fn display_name(&self) -> &'static str {
        match self {
            Self::Idle => "闲置",
            Self::Patrol => "巡逻",
            Self::Chasing => "追击",
            Self::Attacking => "攻击",
            Self::Searching => "搜索",
            Self::Retreating => "撤退",
        }
    }

    /// 判断当前状态是否需要移动
    pub fn should_move(&self) -> bool {
        matches!(self, Self::Chasing | Self::Patrol | Self::Searching | Self::Retreating)
    }

    /// 判断当前状态是否有目标
    pub fn has_target(&self) -> bool {
        matches!(self, Self::Chasing | Self::Attacking | Self::Searching)
    }
}

/// AI 事件
///
/// 用于触发状态转换
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AIEvent {
    /// 检测到玩家
    PlayerDetected,
    /// 失去玩家
    PlayerLost,
    /// 到达目标位置（未来扩展）
    #[allow(dead_code)]
    ReachedDestination,
    /// 进入攻击范围（未来扩展）
    #[allow(dead_code)]
    InAttackRange,
    /// 受到伤害（未来扩展）
    #[allow(dead_code)]
    TookDamage,
}

/// 敌人目标信息
///
/// 存储敌人追踪的目标信息
#[derive(Component, Default)]
pub struct EnemyTarget {
    /// 目标实体（通常是玩家）
    pub entity: Option<Entity>,
    /// 目标最后已知位置
    pub last_known_position: Option<Vec3>,
    /// 失去目标的时间（未来用于搜索行为）
    #[allow(dead_code)]
    pub lost_time: Option<f32>,
}

impl EnemyTarget {
    /// 设置新目标
    pub fn set_target(&mut self, entity: Entity, position: Vec3) {
        self.entity = Some(entity);
        self.last_known_position = Some(position);
        self.lost_time = None;
    }

    /// 更新目标位置
    pub fn update_position(&mut self, position: Vec3) {
        self.last_known_position = Some(position);
    }

    /// 清除目标
    pub fn clear_target(&mut self) {
        self.entity = None;
        // 保留最后已知位置用于搜索行为
    }

    /// 是否有有效目标
    pub fn has_target(&self) -> bool {
        self.entity.is_some()
    }
}

/// AI 参数配置
#[derive(Component, Clone)]
pub struct EnemyAIConfig {
    /// 移动速度
    pub move_speed: f32,
    /// 停止距离（到目标多近时停止）
    pub stop_distance: f32,
    /// 旋转速度（朝向目标的速度）
    pub rotation_speed: f32,
}

impl Default for EnemyAIConfig {
    fn default() -> Self {
        Self {
            move_speed: 7.0,
            stop_distance: 1.5,
            rotation_speed: 10.0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_state_transitions() {
        let mut state = EnemyAIState::Idle;

        // 闲置 -> 检测到玩家 -> 追击
        state = state.transition(AIEvent::PlayerDetected);
        assert_eq!(state, EnemyAIState::Chasing);

        // 追击 -> 失去玩家 -> 闲置
        state = state.transition(AIEvent::PlayerLost);
        assert_eq!(state, EnemyAIState::Idle);
    }

    #[test]
    fn test_state_properties() {
        assert!(EnemyAIState::Chasing.should_move());
        assert!(EnemyAIState::Chasing.has_target());

        assert!(!EnemyAIState::Idle.should_move());
        assert!(!EnemyAIState::Idle.has_target());
    }

    #[test]
    fn test_target_management() {
        let mut target = EnemyTarget::default();
        assert!(!target.has_target());

        // 设置目标
        let entity = Entity::from_raw(123);
        target.set_target(entity, Vec3::ZERO);
        assert!(target.has_target());
        assert_eq!(target.entity, Some(entity));

        // 清除目标
        target.clear_target();
        assert!(!target.has_target());
        // 最后已知位置应保留
        assert!(target.last_known_position.is_some());
    }
}
