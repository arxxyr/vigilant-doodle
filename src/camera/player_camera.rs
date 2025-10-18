use bevy::prelude::*;
use bevy::input::mouse::MouseMotion;

use crate::player::player::Player;
use crate::{GameState, MovementSystem};

pub struct PlayerCameraPlugin;

#[derive(Component)]
struct PlayerCamera {
    /// 相机距离玩家的距离
    distance: f32,
    /// 相机的偏航角（水平旋转）
    yaw: f32,
    /// 相机的俯仰角（垂直旋转）
    pitch: f32,
    /// 鼠标灵敏度
    sensitivity: f32,
    /// 是否锁定光标
    cursor_locked: bool,
}

impl Default for PlayerCamera {
    fn default() -> Self {
        Self {
            distance: 10.0,
            yaw: 0.0,
            pitch: -30.0_f32.to_radians(),
            sensitivity: 0.002,
            cursor_locked: false,
        }
    }
}

impl Plugin for PlayerCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Loading), spawn_camera)
            .add_systems(OnEnter(GameState::Playing), bind_player)
            .add_systems(
                Update,
                (update_camera_controls, update_camera_transform)
                    .chain()
                    .in_set(MovementSystem::Input)
                    .run_if(in_state(GameState::Playing)),
            )
            .add_systems(OnExit(GameState::Playing), reset_cursor);
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        Camera3d::default(),
        Camera {
            is_active: false,  // 初始禁用，进入 Playing 状态时启用
            ..default()
        },
        Transform::from_xyz(0.0, 8.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        PlayerCamera::default(),
        Name::new("ThirdPersonCamera"),
    ));
}

fn update_camera_controls(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut mouse_motion: MessageReader<MouseMotion>,
    mut game_state: ResMut<NextState<GameState>>,
    mut camera_query: Query<&mut PlayerCamera>,
) {
    let Ok(mut camera) = camera_query.single_mut() else {
        return;
    };

    // Tab 键切换菜单并解锁光标
    if keyboard_input.just_pressed(KeyCode::Tab) {
        camera.cursor_locked = !camera.cursor_locked;
        game_state.set(GameState::Menu);
        return;
    }

    // 只有在光标锁定时才处理鼠标移动
    if camera.cursor_locked {
        for motion in mouse_motion.read() {
            // 更新偏航角和俯仰角
            camera.yaw -= motion.delta.x * camera.sensitivity;
            camera.pitch -= motion.delta.y * camera.sensitivity;

            // 限制俯仰角范围（防止翻转）
            camera.pitch = camera.pitch.clamp(-89.0_f32.to_radians(), 89.0_f32.to_radians());
        }
    }
}

fn update_camera_transform(
    player_query: Query<&Transform, With<Player>>,
    mut camera_query: Query<(&mut Transform, &PlayerCamera), (With<Camera3d>, Without<Player>)>,
) {
    let Ok(player_transform) = player_query.single() else {
        return;
    };

    let Ok((mut camera_transform, camera_settings)) = camera_query.single_mut() else {
        return;
    };

    // 计算相机位置
    let yaw_rotation = Quat::from_rotation_y(camera_settings.yaw);
    let pitch_rotation = Quat::from_rotation_x(camera_settings.pitch);
    let combined_rotation = yaw_rotation * pitch_rotation;

    let offset = combined_rotation * Vec3::new(0.0, 0.0, camera_settings.distance);
    let target_position = player_transform.translation + offset;

    // 设置相机位置和朝向
    camera_transform.translation = target_position;
    camera_transform.look_at(player_transform.translation, Vec3::Y);
}

fn bind_player(
    mut camera_query: Query<(&mut PlayerCamera, &mut Camera), With<Camera3d>>,
    mut cursor_options_query: Query<&mut bevy::window::CursorOptions>,
) {
    let Ok((mut player_camera, mut camera)) = camera_query.single_mut() else {
        return;
    };

    player_camera.cursor_locked = true;
    camera.is_active = true;  // 启用 3D 相机

    // 锁定并隐藏光标
    if let Ok(mut cursor_options) = cursor_options_query.single_mut() {
        cursor_options.visible = false;
        cursor_options.grab_mode = bevy::window::CursorGrabMode::Locked;
    }
}

fn reset_cursor(
    mut camera_query: Query<(&mut PlayerCamera, &mut Camera), With<Camera3d>>,
    mut cursor_options_query: Query<&mut bevy::window::CursorOptions>,
    mut windows: Query<&mut Window>,
) {
    let Ok((mut player_camera, mut camera)) = camera_query.single_mut() else {
        return;
    };

    player_camera.cursor_locked = false;
    camera.is_active = false;  // 禁用 3D 相机

    // 恢复光标显示并解锁
    if let Ok(mut cursor_options) = cursor_options_query.single_mut() {
        cursor_options.visible = true;
        cursor_options.grab_mode = bevy::window::CursorGrabMode::None;
    }

    // 将光标移动到窗口中心
    if let Ok(mut window) = windows.single_mut() {
        let width = window.resolution.width();
        let height = window.resolution.height();
        let _ = window.set_cursor_position(Some(Vec2::new(width / 2.0, height / 2.0)));
    }
}
