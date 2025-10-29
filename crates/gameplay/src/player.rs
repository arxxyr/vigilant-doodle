use vigilant_doodle_assets::GameAssets;
use vigilant_doodle_camera::components::CameraTarget;
use vigilant_doodle_core::state::GameState;
use crate::movement::CollisionRadius;
use bevy::color::palettes::css::YELLOW_GREEN;
use bevy::prelude::*;

#[derive(Component)]
pub struct Player {
    pub speed: f32,
    pub rotation_speed: f32, // 旋转速度（弧度/秒）
    pub jump_force: f32,      // 跳跃力度
    pub gravity: f32,         // 重力加速度
    pub vertical_velocity: f32, // 垂直速度（Y 轴）
    pub is_grounded: bool,    // 是否在地面
    pub ground_level: f32,    // 地面高度
}

impl Default for Player {
    fn default() -> Self {
        Self {
            speed: 10.0,
            rotation_speed: 10.0,
            jump_force: 7.5,       // 跳跃初速度（降低高度）
            gravity: -30.0,        // 重力（负值向下）
            vertical_velocity: 0.0,
            is_grounded: true,
            ground_level: 0.0,     // 默认地面高度
        }
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
    mut player_query: Query<(&mut Transform, &mut Player)>,
    enemy_query: Query<&Transform, (With<crate::Enemy>, Without<Player>)>,
    actions: Res<vigilant_doodle_input::actions::InputActions>,
    camera_query: Query<&Transform, (With<Camera3d>, Without<Player>, Without<crate::Enemy>)>,
    time: Res<Time>,
) {
    let Ok((mut transform, mut player)) = player_query.single_mut() else {
        return;
    };

    let Ok(camera_transform) = camera_query.single() else {
        return;
    };

    // 基于相机方向计算移动向量
    let forward = *camera_transform.forward();
    let right = *camera_transform.right();

    // 安全地归一化（防止零向量除零）
    let forward_2d = forward.with_y(0.0);
    let right_2d = right.with_y(0.0);

    let forward_normalized = if forward_2d.length_squared() > 0.001 {
        forward_2d.normalize()
    } else {
        Vec3::Z // 默认向前
    };

    let right_normalized = if right_2d.length_squared() > 0.001 {
        right_2d.normalize()
    } else {
        Vec3::X // 默认向右
    };

    let move_direction = (forward_normalized * actions.movement.y)
        + (right_normalized * actions.movement.x);

    // 移动玩家（如果有移动输入）
    if move_direction.length_squared() > 0.001 {
        let move_direction = move_direction.normalize();
        transform.translation += move_direction * player.speed * time.delta_secs();
    }

    // === 跳跃和重力系统 ===

    // 地面检测（简单检测：Y 轴是否接近地面高度）
    const GROUND_THRESHOLD: f32 = 0.01; // 地面检测阈值
    player.is_grounded = transform.translation.y <= player.ground_level + GROUND_THRESHOLD;

    // 如果在地面且按下跳跃键，施加跳跃力
    if player.is_grounded && actions.jump {
        player.vertical_velocity = player.jump_force;
        info!("[Player] 跳跃！");
    }

    // 应用重力（只要不在地面）
    if !player.is_grounded || player.vertical_velocity > 0.0 {
        player.vertical_velocity += player.gravity * time.delta_secs();
    } else {
        // 在地面时重置垂直速度
        player.vertical_velocity = 0.0;
    }

    // 更新垂直位置
    transform.translation.y += player.vertical_velocity * time.delta_secs();

    // 防止穿透地面
    if transform.translation.y < player.ground_level {
        transform.translation.y = player.ground_level;
        player.vertical_velocity = 0.0;
    }

    // 确定玩家朝向方向
    const DETECTION_RANGE: f32 = 20.0; // 20码检测范围

    // 查找20码范围内最近的敌人
    let nearest_enemy = enemy_query
        .iter()
        .map(|enemy_transform| {
            let distance = transform.translation.distance(enemy_transform.translation);
            (distance, enemy_transform.translation)
        })
        .filter(|(distance, _)| *distance <= DETECTION_RANGE)
        .min_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

    // 计算目标朝向
    let target_forward = if let Some((_, enemy_pos)) = nearest_enemy {
        // 有敌人：朝向最近的敌人
        let direction = enemy_pos - transform.translation;
        let direction_2d = Vec3::new(direction.x, 0.0, direction.z);

        // 安全检查：如果敌人太近（几乎在同一位置），保持当前朝向
        if direction_2d.length_squared() < 0.001 {
            return;
        }

        direction_2d.normalize()
    } else if move_direction.length_squared() > 0.001 {
        // 无敌人且有移动：朝向移动方向
        move_direction.normalize()
    } else {
        // 无敌人且无移动：保持当前朝向
        return;
    };

    // 计算目标旋转（模型默认朝向 -Z）
    let target_rotation = Quat::from_rotation_arc(Vec3::NEG_Z, target_forward);

    // 使用球面线性插值（slerp）平滑旋转
    // rotation_factor 限制在 [0, 1] 范围内
    let rotation_factor = (player.rotation_speed * time.delta_secs()).clamp(0.0, 1.0);
    transform.rotation = transform.rotation.slerp(target_rotation, rotation_factor);
}
