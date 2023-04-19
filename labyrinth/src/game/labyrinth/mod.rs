pub mod components;
pub mod resources;
mod systems;

use bevy::prelude::*;

use crate::AppState;

use systems::*;

pub struct LabyrinthPlugin;

impl Plugin for LabyrinthPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(spawn_walls.in_schedule(OnEnter(AppState::Game)))
            .add_system(remove_labyrinth_map_resource.in_schedule(OnExit(AppState::Game)))
            .add_system(despawn_walls.in_schedule(OnExit(AppState::Game)));
    }
}
