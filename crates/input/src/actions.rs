use vigilant_doodle_core::state::GameState;
use bevy::prelude::*;

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
        app.init_resource::<InputActions>().add_systems(
            Update,
            (
                process_input.run_if(in_state(GameState::Playing)),
                handle_pause_toggle, // 处理暂停/恢复（在任何状态都监听）
            ),
        );
    }
}

fn process_input(keyboard: Res<ButtonInput<KeyCode>>, mut actions: ResMut<InputActions>) {
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

/// 处理 ESC 键的暂停/恢复切换
fn handle_pause_toggle(
    keyboard: Res<ButtonInput<KeyCode>>,
    current_state: Res<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if keyboard.just_pressed(KeyCode::Escape) {
        match current_state.get() {
            GameState::Playing => {
                // 游戏中按 ESC：进入暂停
                info!("[Input] ESC pressed: Playing → Paused");
                next_state.set(GameState::Paused);
            }
            GameState::Paused => {
                // 暂停中按 ESC：恢复游戏
                info!("[Input] ESC pressed: Paused → Playing");
                next_state.set(GameState::Playing);
            }
            _ => {
                // 其他状态不处理 ESC
            }
        }
    }
}
