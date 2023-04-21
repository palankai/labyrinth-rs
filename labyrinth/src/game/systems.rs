use bevy::prelude::*;

use super::events::*;
use crate::AppState;
use bevy::input::touch;

pub fn handle_key_esc(
    keyboard_input: Res<Input<KeyCode>>,
    mut next_app_state: ResMut<NextState<AppState>>,
) {
    if keyboard_input.just_released(KeyCode::Escape) {
        next_app_state.set(AppState::Menu);
    }
}

pub fn handle_movement_keys(
    keyboard_input: Res<Input<KeyCode>>,
    mut movement_event_writer: EventWriter<MoveRequested>,
) {
    let mut movement = Vec3::new(0.0, 0.0, 0.0);
    let mut any = false;
    if keyboard_input.pressed(KeyCode::Left) || keyboard_input.pressed(KeyCode::A) {
        movement += Vec3::new(-1.0, 0.0, 0.0);
        any = true;
    }
    if keyboard_input.pressed(KeyCode::Right) || keyboard_input.pressed(KeyCode::D) {
        movement += Vec3::new(1.0, 0.0, 0.0);
        any = true;
    }
    if keyboard_input.pressed(KeyCode::Up) || keyboard_input.pressed(KeyCode::W) {
        movement += Vec3::new(0.0, 1.0, 0.0);
        any = true;
    }
    if keyboard_input.pressed(KeyCode::Down) || keyboard_input.pressed(KeyCode::S) {
        movement += Vec3::new(0.0, -1.0, 0.0);
        any = true;
    }
    if any {
        movement_event_writer.send(MoveRequested {
            direction: movement,
        });
    }
}
