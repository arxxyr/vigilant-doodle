//! Vigilant Doodle Camera - 相机系统
//!
//! 提供斜向俯视相机系统，用于 3D 游戏场景渲染。

#![allow(clippy::type_complexity)]

pub mod components;
pub mod isometric;

// 重新导出常用类型
pub use isometric::IsometricCameraPlugin;
