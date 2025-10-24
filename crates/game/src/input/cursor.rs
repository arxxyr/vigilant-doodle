use crate::core::state::GameState;
use bevy::prelude::*;
use bevy::window::{CursorGrabMode, CursorOptions};

pub struct CursorPlugin;

impl Plugin for CursorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), lock_cursor)
            .add_systems(OnExit(GameState::Playing), unlock_cursor)
            .add_systems(
                Update,
                handle_escape_key.run_if(in_state(GameState::Playing)),
            );
    }
}

fn lock_cursor(mut cursor_options: Query<&mut CursorOptions, With<Window>>) {
    let Ok(mut cursor) = cursor_options.single_mut() else {
        return;
    };

    cursor.visible = false;
    cursor.grab_mode = CursorGrabMode::Locked;

    info!("[Cursor] Cursor locked and hidden");
}

fn unlock_cursor(mut cursor_options: Query<&mut CursorOptions, With<Window>>) {
    let Ok(mut cursor) = cursor_options.single_mut() else {
        return;
    };

    cursor.visible = true;
    cursor.grab_mode = CursorGrabMode::None;

    info!("[Cursor] Cursor unlocked and visible");
}

fn handle_escape_key(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    if keyboard.just_pressed(KeyCode::Escape) {
        info!("[Input] ESC pressed, returning to menu");
        game_state.set(GameState::MainMenu);
    }
}
