//! 输入模块
//!
//! 处理玩家输入，包括键盘、鼠标控制和光标管理。

pub mod actions;
pub mod cursor;

pub use actions::InputPlugin;
pub use cursor::CursorPlugin;
