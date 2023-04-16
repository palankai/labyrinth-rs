mod components;
mod styles;
mod systems;

use bevy::prelude::*;

use crate::AppState;
use systems::interactions::*;
use systems::layout::*;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(spawn_main_menu.in_schedule(OnEnter(AppState::Menu)))
            .add_system(despawn_main_menu.in_schedule(OnExit(AppState::Menu)))
            .add_systems(
                (
                    interact_with_play_button,
                    interact_with_quit_button,
                    handle_key_q,
                    handle_key_p,
                )
                    .in_set(OnUpdate(AppState::Menu)),
            );
    }
}
