use bevy::prelude::*;

use crate::GameState;
pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app
            // .add_systems(Startup, (spawn_floor,spawn_objects))
            .add_systems(OnEnter(GameState::Playing), (spawn_floor, spawn_objects));
    }
}

fn spawn_floor(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let floor = (
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Plane::from_size(15.0))),
            material: materials.add(Color::GRAY.into()),
            transform: Transform::from_xyz(0.0, -0.5, 0.0),
            ..default()
        },
        Name::new("Floor"),
    );

    commands.spawn(floor);
}

fn spawn_objects(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mut create_obj =
        |size: f32, color: Color, name: String, xyz: (f32, f32, f32)| -> (PbrBundle, Name) {
            (
                PbrBundle {
                    mesh: meshes.add(Mesh::from(shape::Cube::new(size))),
                    material: materials.add(color.into()),
                    transform: Transform::from_xyz(xyz.0, xyz.1, xyz.2),
                    ..default()
                },
                Name::new(name),
            )
        };

    commands.spawn(create_obj(
        3.0,
        Color::RED,
        "Red Cube".to_string(),
        (-4.5, 1.0, -4.5),
    ));
    commands.spawn(create_obj(
        2.0,
        Color::MIDNIGHT_BLUE,
        "Blue Cube".to_string(),
        (5.3, 0.5, 5.7),
    ));
}

