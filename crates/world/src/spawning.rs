use vigilant_doodle_core::state::GameState;
use crate::terrain::{FLOOR_LENGTH, FLOOR_WIDTH};
use bevy::prelude::*;

pub struct SpawningPlugin;

impl Plugin for SpawningPlugin {
    fn build(&self, app: &mut App) {
        app
            // 在资源加载完成后立即生成所有实体
            .add_systems(
                OnEnter(GameState::AssetLoading),
                (spawn_terrain, spawn_lights).chain(),
            );
    }
}

fn spawn_terrain(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Mesh3d(
            meshes.add(
                Plane3d::default()
                    .mesh()
                    .size(FLOOR_LENGTH, FLOOR_WIDTH)
                    .build(),
            ),
        ),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.3, 0.3, 0.35),
            perceptual_roughness: 0.8,
            ..default()
        })),
        Transform::from_xyz(0.0, -0.5, 0.0),
        Name::new("Floor"),
    ));

    info!("[World] Terrain spawned ({}x{})", FLOOR_LENGTH, FLOOR_WIDTH);
}

fn spawn_lights(mut commands: Commands) {
    // 环境光
    commands.insert_resource(AmbientLight {
        color: Color::srgb(0.8, 0.8, 1.0),
        brightness: 200.0,
        affects_lightmapped_meshes: true,
    });

    // 主方向光（太阳光）
    commands.spawn((
        DirectionalLight {
            color: Color::srgb(1.0, 0.95, 0.9),
            illuminance: 5000.0,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_rotation(Quat::from_euler(
            EulerRot::XYZ,
            -std::f32::consts::FRAC_PI_4, // -45 度俯角
            std::f32::consts::FRAC_PI_4,  // 45 度侧向
            0.0,
        )),
        Name::new("SunLight"),
    ));

    info!("[World] Lights spawned (ambient + directional)");
}
