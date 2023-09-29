#![allow(clippy::type_complexity)]

use bevy_third_person_camera::ThirdPersonCameraPlugin;

mod actions;
mod audio;
mod camera;
mod enemies;
mod loading;
mod menu;
mod player;
mod world;

use crate::actions::ActionsPlugin;
use crate::audio::InternalAudioPlugin;
use crate::loading::LoadingPlugin;
use crate::menu::MenuPlugin;
use crate::player::player::PlayerPlugin;
// 从 `world` 模块中引入 `WorldPlugin`
use crate::camera::player_camera::PlayerCameraPlugin;
use crate::enemies::enemies::EnemiesPlugin;
use crate::world::world::WorldPlugin;

use bevy::app::App;
use bevy::prelude::*;

#[cfg(debug_assertions)]
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
#[cfg(debug_assertions)]
use bevy_inspector_egui::quick::WorldInspectorPlugin;

// This example game uses States to separate logic
// See https://bevy-cheatbook.github.io/programming/states.html
// Or https://github.com/bevyengine/bevy/blob/main/examples/ecs/state.rs
#[derive(States, Default, Clone, Eq, PartialEq, Debug, Hash)]
pub enum GameState {
    // During the loading State the LoadingPlugin will load our assets
    #[default]
    Loading,
    // During this State the actual game logic is executed
    Playing,
    // Here the menu is drawn and waiting for player interaction
    Menu,
}

/// A group of related system sets, used for controlling the order of systems. Systems can be
/// added to any number of sets.
#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
enum MovementSystem {
    /// everything that handles input
    Input,
    /// everything that updates player state
    Player,
    Enemy,
    /// everything that moves things (works with transforms)
    PlayerMovement,
    EnemyMovement,
    /// systems that update the world map
    Map,
}

// One of the two settings that can be set through the menu. It will be a resource in the app
#[derive(Resource, Debug, Component, PartialEq, Eq, Clone, Copy)]
enum DisplayQuality {
    Low,
    Medium,
    High,
}

// One of the two settings that can be set through the menu. It will be a resource in the app
#[derive(Resource, Debug, Component, PartialEq, Eq, Clone, Copy)]
struct Volume(u32);

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<GameState>()
            .configure_sets(
                Update,
                (
                    MovementSystem::Input,
                    MovementSystem::Player,
                    MovementSystem::Enemy,
                    MovementSystem::PlayerMovement,
                    MovementSystem::EnemyMovement,
                    MovementSystem::Map,
                )
                    .chain(),
            )
            .add_plugins((
                LoadingPlugin,
                MenuPlugin,
                ActionsPlugin,
                WorldPlugin,
                PlayerCameraPlugin,
                InternalAudioPlugin,
                PlayerPlugin,
                EnemiesPlugin,
                ThirdPersonCameraPlugin,
                #[cfg(debug_assertions)]
                WorldInspectorPlugin::new(),
            ));

        #[cfg(debug_assertions)]
        {
            app.add_plugins((FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin::default()));
        }
    }
}
