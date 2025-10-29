//! Vigilant Doodle Gameplay - 游戏玩法系统
//!
//! 包含玩家、敌人和移动系统。

#![allow(clippy::type_complexity)]

mod enemy;
mod movement;
mod player;

pub use enemy::{Enemy, EnemyPlugin};
pub use movement::MovementPlugin;
pub use player::{Player, PlayerPlugin};
