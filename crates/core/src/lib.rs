//! Vigilant Doodle Core - 核心系统库
//!
//! 提供游戏的核心功能：
//! - 状态机管理（GameState）
//! - 本地化系统（Localization）
//! - 存档系统（Save）
//! - 加密系统（Crypto）

#![allow(clippy::type_complexity)]

pub mod crypto;
pub mod localization;
pub mod save;
pub mod state;

// 重新导出常用类型
pub use state::GameState;
pub use localization::LocalizationPlugin;
pub use save::{SaveManager, SavePlugin};
pub use state::StatePlugin;
