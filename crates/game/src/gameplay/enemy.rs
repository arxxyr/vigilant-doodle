use crate::core::state::GameState;
use crate::gameplay::movement::CollisionRadius;
use crate::gameplay::player::Player;
use bevy::prelude::*;

#[derive(Component)]
pub struct Enemy {
    pub speed: f32,
    pub chase_distance: f32,
    pub stop_distance: f32,
}

impl Default for Enemy {
    fn default() -> Self {
        Self {
            speed: 7.0,
            chase_distance: 20.0,
            stop_distance: 1.5,
        }
    }
}

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::AssetLoading), spawn_enemies)
            .add_systems(
                Update,
                enemy_chase_player.run_if(in_state(GameState::Playing)),
            );
    }
}

fn spawn_enemies(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    use rand::Rng;
    let mut rng = rand::rng();

    // 生成 3 个敌人
    for i in 0..3 {
        let x = rng.random_range(-20.0..20.0);
        let z = rng.random_range(-10.0..10.0);

        commands.spawn((
            Mesh3d(meshes.add(Capsule3d::new(0.4, 1.0))),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: Color::srgb(0.8, 0.2, 0.2),
                ..default()
            })),
            Transform::from_xyz(x, 0.5, z),
            Enemy::default(),
            CollisionRadius::new(0.6), // 碰撞半径略大于胶囊体半径（0.4）以提供缓冲
            Name::new(format!("Enemy_{}", i)),
        ));
    }

    info!("[Enemies] 3 enemies spawned");
}

fn enemy_chase_player(
    player_query: Query<&Transform, With<Player>>,
    mut enemy_query: Query<(&mut Transform, &Enemy), Without<Player>>,
    time: Res<Time>,
) {
    let Ok(player_transform) = player_query.single() else {
        return;
    };

    for (mut enemy_transform, enemy) in enemy_query.iter_mut() {
        let to_player = player_transform.translation - enemy_transform.translation;
        let distance = to_player.length();

        // 在追逐距离内且未到达停止距离
        if distance < enemy.chase_distance && distance > enemy.stop_distance {
            let direction = to_player.normalize();
            let movement = direction * enemy.speed * time.delta_secs();

            // 只在水平面移动（Y 轴保持不变）
            enemy_transform.translation.x += movement.x;
            enemy_transform.translation.z += movement.z;

            // 朝向玩家
            let look_direction = Vec3::new(direction.x, 0.0, direction.z);
            if look_direction.length() > 0.001 {
                enemy_transform.look_to(look_direction, Vec3::Y);
            }
        }
    }
}
