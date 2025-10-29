//! Vigilant Doodle Input - 输入系统
//!
//! 处理玩家输入，包括键盘、鼠标控制和光标管理。

#![allow(clippy::type_complexity)]

pub mod actions;
pub mod cursor;

// 重新导出常用类型
pub use actions::InputPlugin;
pub use cursor::CursorPlugin;
