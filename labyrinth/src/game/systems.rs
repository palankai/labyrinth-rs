use bevy::prelude::*;

use crate::AppState;

pub fn handle_key_esc(
    keyboard_input: Res<Input<KeyCode>>,
    mut next_app_state: ResMut<NextState<AppState>>,
) {
    if keyboard_input.just_released(KeyCode::Escape) {
        next_app_state.set(AppState::Menu);
    }
}
