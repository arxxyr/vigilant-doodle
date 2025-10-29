use vigilant_doodle_assets::GameAssets;
use vigilant_doodle_camera::components::CameraTarget;
use vigilant_doodle_core::state::GameState;
use crate::movement::CollisionRadius;
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
        app.add_systems(
                OnEnter(GameState::AssetLoading),
                spawn_player.after(vigilant_doodle_assets::load_assets),
            )
            .add_systems(Update, player_movement.run_if(in_state(GameState::Playing)));
    }
}

fn spawn_player(
    mut commands: Commands,
    assets: Res<GameAssets>,
) {
    // 使用 glb 模型作为玩家
    commands
        .spawn((
            SceneRoot(assets.player_model.clone()),
            Transform::from_xyz(0.0, 0.0, 0.0),
            Player::default(),
            CameraTarget,              // 标记为相机跟随目标
            CollisionRadius::new(0.6), // 碰撞半径
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

    info!("[Player] Player spawned with model");
}

fn player_movement(
    mut query: Query<(&mut Transform, &Player)>,
    actions: Res<vigilant_doodle_input::actions::InputActions>,
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
