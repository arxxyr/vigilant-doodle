use bevy::prelude::*;
use rand::prelude::*;

use crate::player::player::Player;
use crate::world::world::{FLOOR_LENGTH, FLOOR_WIDTH};
use crate::GameState;
use crate::MovementSystem;

pub struct EnemiesPlugin;

#[derive(Component)]
pub struct Enemy;

#[derive(Component)]
pub struct EnemySpeed(f32);

#[derive(Component)]
pub struct EnemyTarget(Vec3);

impl Plugin for EnemiesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Loading), genarate_enemies)
            .add_systems(OnEnter(GameState::Playing), show_enemies)
            .add_systems(
                Update,
                (update_player, enemies_movement, enemy_collision_detection)
                    .chain()
                    .in_set(MovementSystem::EnemyMovement)
                    .run_if(in_state(GameState::Playing)),
            )
            .add_systems(OnExit(GameState::Playing), hide_enemies);
    }
}

fn genarate_enemies(mut commands: Commands, assets: Res<AssetServer>) {
    // generate x,y in range -FLOOR_SIZE..FLOOR_SIZE
    let mut rng = rand::thread_rng();
    let random_x = rng.gen_range(-FLOOR_LENGTH..FLOOR_LENGTH);
    let random_z = rng.gen_range(-FLOOR_WIDTH..FLOOR_WIDTH);
    let enemy = (
        Name::new("Enemy"),
        Enemy,
        SceneBundle {
            scene: assets.load("model/enemy.glb#Scene0"),
            transform: Transform::from_xyz(random_x / 2.0, 0.0, random_z / 2.0),
            visibility: Visibility::Hidden,
            ..default()
        },
        EnemySpeed(9.0),
        EnemyTarget(Vec3::ZERO),
    );
    commands.spawn(enemy);
}

fn show_enemies(mut commands: Commands, query: Query<Entity, With<Enemy>>) {
    for entity in query.iter() {
        commands.entity(entity).insert(Visibility::Visible);
    }
}

fn hide_enemies(mut commands: Commands, query: Query<Entity, With<Enemy>>) {
    for entity in query.iter() {
        commands.entity(entity).insert(Visibility::Hidden);
    }
}

fn update_player(
    query_player: Query<&Transform, With<Player>>,
    mut query_enemies: Query<&mut EnemyTarget, With<Enemy>>,
) {
    let player_transform = match query_player.get_single() {
        Ok(p) => p,
        Err(e) => panic!("Error getting player transform: {}", e),
    };

    query_enemies.for_each_mut(|mut enemy_target| {
        enemy_target.0 = player_transform.translation;
    });
}

fn enemies_movement(
    time: Res<Time>,
    mut query_enemies: Query<(&mut Transform, &EnemySpeed, &EnemyTarget), With<Enemy>>,
) {
    query_enemies.for_each_mut(|(mut transform, enemy_speed, enemy_target)| {
        let mut direction = Vec3::ZERO;
        // 距离player 1.0米时停止移动
        if (enemy_target.0 - transform.translation).length() < 1.0 {
            return;
        }
        direction += enemy_target.0 - transform.translation;
        direction.y = 0.;
        direction = direction.normalize();
        transform.translation += direction * enemy_speed.0 * time.delta_seconds();
        transform.look_at(enemy_target.0, Vec3::Y);
    });
}

fn enemy_collision_detection(mut player_query: Query<&mut Transform, With<Enemy>>) {
    if let Ok(mut player_transform) = player_query.get_single_mut() {
        if player_transform.translation.x > (FLOOR_LENGTH - 2.0) / 2.0 {
            player_transform.translation.x = (FLOOR_LENGTH - 2.0) / 2.0 - 0.2;
        }
        if player_transform.translation.x < -(FLOOR_LENGTH - 2.0) / 2.0 {
            player_transform.translation.x = -(FLOOR_LENGTH - 2.0) / 2.0 + 0.2;
        }
        if player_transform.translation.z > (FLOOR_WIDTH - 2.0) / 2.0 {
            player_transform.translation.z = (FLOOR_WIDTH - 2.0) / 2.0 - 0.2;
        }
        if player_transform.translation.z < -(FLOOR_WIDTH - 2.0) / 2.0 {
            player_transform.translation.z = -(FLOOR_WIDTH - 2.0) / 2.0 + 0.2;
        }
    }
}
