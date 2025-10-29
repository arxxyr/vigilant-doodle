//! Vigilant Doodle World - 世界生成系统
//!
//! 提供世界生成、地形管理和实体生成系统。

#![allow(clippy::type_complexity)]

pub mod spawning;
pub mod terrain;

// 重新导出常用类型
pub use spawning::SpawningPlugin;
