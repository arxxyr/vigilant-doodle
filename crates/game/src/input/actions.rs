use bevy::prelude::*;
use crate::core::state::GameState;

/// 输入动作资源
#[derive(Resource, Default)]
pub struct InputActions {
    pub movement: Vec2,
}

/// 游戏控制键位
pub enum GameControl {
    Forward,
    Backward,
    Left,
    Right,
}

impl GameControl {
    pub fn key(&self) -> KeyCode {
        match self {
            Self::Forward => KeyCode::KeyW,
            Self::Backward => KeyCode::KeyS,
            Self::Left => KeyCode::KeyA,
            Self::Right => KeyCode::KeyD,
        }
    }
}

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<InputActions>()
            .add_systems(
                Update,
                process_input
                    .run_if(in_state(GameState::Playing))
            );
    }
}

fn process_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut actions: ResMut<InputActions>,
) {

    let mut direction = Vec2::ZERO;

    // 基于相机方向的移动输入
    if keyboard.pressed(GameControl::Forward.key()) {
        direction.y += 1.0;
    }
    if keyboard.pressed(GameControl::Backward.key()) {
        direction.y -= 1.0;
    }
    if keyboard.pressed(GameControl::Left.key()) {
        direction.x -= 1.0;
    }
    if keyboard.pressed(GameControl::Right.key()) {
        direction.x += 1.0;
    }

    // 归一化（防止斜向移动过快）
    if direction.length() > 0.0 {
        direction = direction.normalize();
    }

    actions.movement = direction;
}
