//! Vigilant Doodle - 游戏启动器
//!
//! 职责：
//! - 初始化 Bevy App
//! - 配置窗口
//! - 加载游戏插件
//! - 设置窗口图标

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use bevy::prelude::*;
use bevy::window::{PrimaryWindow, WindowLevel, WindowTheme, WindowPosition, MonitorSelection};
use bevy::winit::WinitWindows;
use std::io::Cursor;
use winit::window::Icon;

fn main() {
    // 初始化日志
    #[cfg(debug_assertions)]
    {
        unsafe {
            std::env::set_var("RUST_LOG", "info,wgpu=error,naga=warn");
        }
    }

    info!("[Launcher] Starting Vigilant Doodle...");

    App::new()
        // Bevy 默认插件
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Vigilant Doodle".to_string(),
                resolution: (1280, 720).into(),
                position: WindowPosition::Centered(MonitorSelection::Primary),
                window_level: WindowLevel::Normal,
                window_theme: Some(WindowTheme::Dark),
                canvas: Some("#bevy".to_owned()),  // Web 支持
                prevent_default_event_handling: false,
                ..default()
            }),
            ..default()
        }))
        // 加载游戏插件（动态库）
        .add_plugins(vigilant_doodle_game::GamePlugin)
        // 启动器专属系统
        .add_systems(Startup, set_window_icon)
        .run();
}

/// 设置窗口图标（仅 Windows 和 Linux X11）
fn set_window_icon(
    windows: Option<NonSend<WinitWindows>>,
    primary_window: Query<Entity, With<PrimaryWindow>>,
) {
    let Some(windows) = windows else { return };
    let Ok(primary_entity) = primary_window.single() else { return };
    let Some(primary) = windows.get_window(primary_entity) else { return };

    let icon_buf = Cursor::new(include_bytes!(
        "../../../build/macos/AppIcon.iconset/icon_256x256.png"
    ));

    if let Ok(image) = image::load(icon_buf, image::ImageFormat::Png) {
        let image = image.into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        let icon = Icon::from_rgba(rgba, width, height).unwrap();
        primary.set_window_icon(Some(icon));
        info!("[Launcher] Window icon set successfully");
    }
}
