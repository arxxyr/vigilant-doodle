use crate::camera::components::CameraTarget;
use crate::core::state::GameState;
use crate::gameplay::movement::CollisionRadius;
use bevy::color::palettes::css::YELLOW_GREEN;
use bevy::prelude::*;

#[derive(Component)]
pub struct Player {
    pub speed: f32,
}

impl Default for Player {
    fn default() -> Self {
        Self { speed: 10.0 }
    }
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::AssetLoading), spawn_player)
            .add_systems(Update, player_movement.run_if(in_state(GameState::Playing)));
    }
}

fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // 使用简单的胶囊体作为玩家（暂时没有模型文件）
    commands
        .spawn((
            Mesh3d(meshes.add(Capsule3d::new(0.4, 1.0))),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: Color::srgb(0.3, 0.5, 0.8),
                ..default()
            })),
            Transform::from_xyz(0.0, 0.5, 0.0),
            Player::default(),
            CameraTarget,              // 标记为相机跟随目标
            CollisionRadius::new(0.6), // 碰撞半径略大于胶囊体半径（0.4）以提供缓冲
            Name::new("Player"),
        ))
        .with_children(|parent| {
            // 玩家子实体：探照灯
            parent.spawn((
                SpotLight {
                    color: YELLOW_GREEN.into(),
                    range: 50.0,
                    intensity: 5000.0,
                    outer_angle: 0.8,
                    inner_angle: 0.4,
                    shadows_enabled: true,
                    ..default()
                },
                Transform::from_xyz(0.0, 2.0, 0.0).looking_at(Vec3::new(0.0, 0.0, 1.0), Vec3::Y),
                Name::new("PlayerFlashlight"),
            ));
        });

    info!("[Player] Player spawned");
}

fn player_movement(
    mut query: Query<(&mut Transform, &Player)>,
    actions: Res<crate::input::actions::InputActions>,
    camera_query: Query<&Transform, (With<Camera3d>, Without<Player>)>,
    time: Res<Time>,
) {
    let Ok((mut transform, player)) = query.single_mut() else {
        return;
    };

    let Ok(camera_transform) = camera_query.single() else {
        return;
    };

    if actions.movement.length() == 0.0 {
        return;
    };

    // 基于相机方向计算移动向量
    let forward = *camera_transform.forward();
    let right = *camera_transform.right();

    let move_direction = (forward.with_y(0.0).normalize() * actions.movement.y)
        + (right.with_y(0.0).normalize() * actions.movement.x);

    // 移动玩家
    transform.translation += move_direction * player.speed * time.delta_secs();

    // 朝向移动方向
    if move_direction.length() > 0.0 {
        transform.look_to(move_direction, Vec3::Y);
    }
}
