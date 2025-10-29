//! Vigilant Doodle AI - 敌人 AI 系统
//!
//! 提供敌人 AI 的完整实现，包括：
//! - 状态机（State Machine）
//! - 检测系统（Detection）
//! - 行为执行（Behavior）

#![allow(clippy::type_complexity)]

mod behavior;
mod detection;
mod enemy_ai;

// 公开导出
pub use behavior::BehaviorSystemPlugin;
pub use detection::{DetectionConfig, DetectionSystem};
pub use enemy_ai::{AIEvent, EnemyAIConfig, EnemyAIState, EnemyTarget};

use bevy::prelude::*;

/// 敌人 AI 插件
///
/// 整合所有 AI 相关系统
pub struct EnemyAIPlugin;

impl Plugin for EnemyAIPlugin {
    fn build(&self, app: &mut App) {
        info!("[AI] 加载敌人 AI 系统...");

        app.add_plugins(BehaviorSystemPlugin);

        info!("[AI] 敌人 AI 系统加载完成");
    }
}
