mod simple_level_generator;

mod systems;

use bevy::prelude::*;

use crate::AppState;

use systems::*;

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(spawn_level.in_schedule(OnEnter(AppState::Game)))
            .add_system(despawn_level.in_schedule(OnExit(AppState::Game)));
    }
}

#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub enum Element {
    #[default]
    Path,
    Wall,
    Exit,
    Player,
}
