//! Vigilant Doodle UI - 用户界面系统
//!
//! 提供游戏的用户界面，包括主菜单、设置菜单等。

#![allow(clippy::type_complexity)]

// 新的极简菜单系统
pub mod settings_menu;
pub mod simple_menu;

// 重新导出常用类型
pub use settings_menu::SettingsMenuPlugin;
pub use simple_menu::SimpleMenuPlugin;
