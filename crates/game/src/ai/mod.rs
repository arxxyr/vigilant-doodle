//! AI 系统模块
//!
//! 提供敌人 AI 的完整实现，包括：
//! - 状态机（State Machine）
//! - 检测系统（Detection）
//! - 行为执行（Behavior）
//!
//! # 架构设计
//!
//! ```text
//! ┌─────────────────────────────────────────────────────────┐
//! │                      AI System                          │
//! ├─────────────────────────────────────────────────────────┤
//! │                                                         │
//! │  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐ │
//! │  │  Detection   │─→│   Decision   │─→│  Execution   │ │
//! │  │   检测玩家    │  │   状态转换    │  │   执行行为    │ │
//! │  └──────────────┘  └──────────────┘  └──────────────┘ │
//! │                                                         │
//! └─────────────────────────────────────────────────────────┘
//! ```
//!
//! # 状态机流转
//!
//! ```text
//!     ┌──────┐
//!     │ Idle │ ◄────────────┐
//!     └───┬──┘              │
//!         │                 │
//!         │ PlayerDetected  │ PlayerLost
//!         │                 │
//!         ▼                 │
//!     ┌─────────┐           │
//!     │ Chasing │───────────┘
//!     └─────────┘
//! ```
//!
//! # 使用示例
//!
//! ```rust,ignore
//! use bevy::prelude::*;
//! use crate::ai::EnemyAIPlugin;
//!
//! fn main() {
//!     App::new()
//!         .add_plugins(EnemyAIPlugin)
//!         .run();
//! }
//! ```

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_plugin_creation() {
        let plugin = EnemyAIPlugin;
        let mut app = App::new();
        app.add_plugins(plugin);
        // 验证插件能正常加载
    }
}
