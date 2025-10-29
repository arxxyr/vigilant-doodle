//! Vigilant Doodle UI - 用户界面系统
//!
//! 提供游戏的用户界面，包括主菜单、设置菜单等。

#![allow(clippy::type_complexity)]

// 菜单系统
pub mod menu;
pub mod settings_menu;

// 重新导出常用类型
pub use menu::MenuPlugin;
pub use settings_menu::SettingsMenuPlugin;
