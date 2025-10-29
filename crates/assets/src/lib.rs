//! Vigilant Doodle Assets - 资源加载系统
//!
//! 提供游戏资源的预加载和管理功能。

#![allow(clippy::type_complexity)]

pub mod loader;

// 重新导出常用类型
pub use loader::{AssetLoaderPlugin, GameAssets};
