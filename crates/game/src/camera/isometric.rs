use bevy::prelude::*;
use crate::camera::components::{IsometricCamera, CameraTarget};
use crate::core::state::GameState;

/// 相机插件
pub struct IsometricCameraPlugin;

impl Plugin for IsometricCameraPlugin {
    fn build(&self, app: &mut App) {
        app
            // 在 AssetLoading 阶段生成相机（始终存在）
            .add_systems(OnEnter(GameState::AssetLoading), spawn_camera)
            // 在 Playing 状态下更新相机跟随
            .add_systems(
                Update,
                update_camera_follow
                    .run_if(in_state(GameState::Playing))
            )
            // 在 MainMenu 状态下也更新相机跟随（让背景有动感）
            .add_systems(
                Update,
                update_camera_follow
                    .run_if(in_state(GameState::MainMenu))
            );
    }
}

fn spawn_camera(mut commands: Commands) {
    let camera_config = IsometricCamera::default();

    commands.spawn((
        Camera3d::default(),
        Transform::from_translation(camera_config.offset)
            .looking_at(Vec3::ZERO, Vec3::Y),
        IsometricCamera::default(),
        Name::new("IsometricCamera"),
    ));

    info!("[Camera] Isometric camera spawned at offset {:?}", camera_config.offset);
}

fn update_camera_follow(
    target_query: Query<&Transform, With<CameraTarget>>,
    mut camera_query: Query<
        (&mut Transform, &IsometricCamera),
        (With<Camera3d>, Without<CameraTarget>)
    >,
    time: Res<Time>,
) {
    let Ok(target_transform) = target_query.single() else {
        return;
    };

    let Ok((mut camera_transform, camera_config)) = camera_query.single_mut() else {
        return;
    };

    // 计算目标位置
    let target_position = target_transform.translation + camera_config.offset;

    // 平滑插值跟随
    camera_transform.translation = camera_transform.translation.lerp(
        target_position,
        camera_config.follow_speed * time.delta_secs()
    );

    // 始终看向目标
    camera_transform.look_at(target_transform.translation, Vec3::Y);
}
