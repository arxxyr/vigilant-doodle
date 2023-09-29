use bevy::prelude::*;
use rand::prelude::*;

use crate::world::world::{FLOOR_LENGTH, FLOOR_WIDTH};
use crate::GameState;

pub struct EnemiesPlugin;

#[derive(Component)]
pub struct Enemy;

impl Plugin for EnemiesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Loading), genarate_enemies)
            .add_systems(OnEnter(GameState::Playing), show_enemies)
            // .add_systems(Update, update_camera.run_if(in_state(GameState::Playing)))
            // .add_systems(Update, update_camera)
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
    );
    commands.spawn(enemy);
}

fn show_enemies(mut commands: Commands, mut query: Query<Entity, With<Enemy>>) {
    for entity in &mut query {
        commands.entity(entity).insert(Visibility::Visible);
    }
}

fn hide_enemies(mut commands: Commands, mut query: Query<Entity, With<Enemy>>) {
    for entity in &mut query {
        commands.entity(entity).insert(Visibility::Hidden);
    }
}
