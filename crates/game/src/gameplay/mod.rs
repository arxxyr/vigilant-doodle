//! 游戏玩法模块
//!
//! 包含玩家、敌人和移动系统。

mod enemy;
mod movement;
mod player;

pub use enemy::{Enemy, EnemyPlugin};
pub use movement::MovementPlugin;
pub use player::{Player, PlayerPlugin};
