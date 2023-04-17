mod consts;
mod game;
mod menu;
mod model;
mod utils;
mod viewport;

use bevy::prelude::*;

use game::*;
use menu::*;
use viewport::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Csaba's labyrinth game".into(),
                resolution: (984., 984.).into(),
                resizable: false,
                position: WindowPosition::Automatic,
                ..default()
            }),
            ..default()
        }))
        .add_state::<AppState>()
        .add_plugin(ViewportPlugin)
        .add_plugin(MenuPlugin)
        .add_plugin(GamePlugin)
        .run();
}

#[derive(States, Default, Debug, Hash, Clone, Copy, Eq, PartialEq)]
pub enum AppState {
    #[default]
    Menu,
    Game,
    GameOver,
}
