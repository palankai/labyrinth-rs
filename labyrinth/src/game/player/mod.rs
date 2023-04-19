mod systems;

use bevy::prelude::*;

use crate::AppState;

use systems::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(spawn_player.in_schedule(OnEnter(AppState::Game)))
            .add_system(despawn_player.in_schedule(OnExit(AppState::Game)))
            .add_system(player_movement.in_set(OnUpdate(AppState::Game)));
    }
}
