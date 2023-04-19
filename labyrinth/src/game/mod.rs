pub mod components;
mod level;
mod player;
mod systems;

use bevy::prelude::*;

use crate::AppState;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(level::LevelPlugin)
            .add_plugin(player::PlayerPlugin)
            .add_system(systems::handle_key_esc.in_set(OnUpdate(AppState::Game)));
    }
}
