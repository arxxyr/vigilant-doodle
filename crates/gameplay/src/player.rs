use crate::movement::CollisionRadius;
use bevy::color::palettes::css::YELLOW_GREEN;
use bevy::prelude::*;
use vigilant_doodle_assets::GameAssets;
use vigilant_doodle_camera::components::CameraTarget;
use vigilant_doodle_core::{state::GameState, BalanceConfig};

#[derive(Component)]
pub struct Player {
    pub speed: f32,
    pub rotation_speed: f32,    // 旋转速度（弧度/秒）
    pub jump_force: f32,         // 跳跃力度
    pub gravity: f32,            // 重力加速度
    pub vertical_velocity: f32,  // 垂直速度（Y 轴）
    pub is_grounded: bool,       // 是否在地面
    pub ground_level: f32,       // 地面高度
    pub detection_range: f32,    // 敌人检测范围
    pub max_jump_count: u32,     // 最大跳跃次数（1=单跳，2=二段跳）
    pub jump_count: u32,         // 当前已跳跃次数
}

impl Player {
    /// 从平衡配置创建玩家
    pub fn from_balance(balance: &BalanceConfig) -> Self {
        Self {
            speed: balance.player.speed,
            rotation_speed: balance.player.rotation_speed,
            jump_force: balance.player.jump_force,
            gravity: balance.player.gravity,
            detection_range: balance.player.detection_range,
            max_jump_count: balance.player.max_jump_count,
            vertical_velocity: 0.0,
            is_grounded: true,
            ground_level: 0.0,
            jump_count: 0,
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
    balance: Res<BalanceConfig>,
) {
    // 从配置文件创建玩家
    let player = Player::from_balance(&balance);

    info!(
        "[Player] 玩家配置: 速度={}, 跳跃力={}, 最大跳跃次数={}",
        player.speed, player.jump_force, player.max_jump_count
    );

    // 使用 glb 模型作为玩家
    commands
        .spawn((
            SceneRoot(assets.player_model.clone()),
            Transform::from_xyz(0.0, 0.0, 0.0),
            player,
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

    // 在地面时重置跳跃次数
    if player.is_grounded {
        player.jump_count = 0;
    }

    // 按下跳跃键且还有跳跃次数（支持多段跳）
    if actions.jump && player.jump_count < player.max_jump_count {
        player.vertical_velocity = player.jump_force;
        player.jump_count += 1;
        info!(
            "[Player] 跳跃！({}/{})",
            player.jump_count, player.max_jump_count
        );
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

    // 确定玩家朝向方向（使用配置的检测范围）
    let detection_range = player.detection_range;

    // 查找检测范围内最近的敌人
    let nearest_enemy = enemy_query
        .iter()
        .map(|enemy_transform| {
            let distance = transform.translation.distance(enemy_transform.translation);
            (distance, enemy_transform.translation)
        })
        .filter(|(distance, _)| *distance <= detection_range)
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
