use bevy::prelude::*;

use crate::model::components::Player;
use crate::model::logic::Element;
use crate::utils::sprite;
use crate::AppState;

use super::super::labyrinth::resources::LabyrinthMap;

pub fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    let x: f32 = 1.0 * 24.0 + 12.0;
    let y: f32 = 1.0 * 24.0 + 12.0;

    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(x, y, 0.0),
            texture: asset_server.load(sprite("player_24x24.png")),
            ..default()
        },
        Player {},
    ));
}

pub fn despawn_player(mut commands: Commands, query: Query<Entity, With<Player>>) {
    if let Ok(entity) = query.get_single() {
        commands.entity(entity).despawn();
    }
}

pub fn player_movement(
    mut player_query: Query<&mut Transform, With<Player>>,
    labyrinth: Res<LabyrinthMap>,
    keyboard_input: Res<Input<KeyCode>>,
    mut next_app_state: ResMut<NextState<AppState>>,
) {
    if let Ok(mut transform) = player_query.get_single_mut() {
        let mut position = transform.translation.clone();
        let mut step = Vec3::ZERO;

        if keyboard_input.just_released(KeyCode::Left) || keyboard_input.just_released(KeyCode::A) {
            step += Vec3::new(-1.0, 0.0, 0.0);
        }
        if keyboard_input.just_released(KeyCode::Right) || keyboard_input.just_released(KeyCode::D)
        {
            step += Vec3::new(1.0, 0.0, 0.0);
        }
        if keyboard_input.just_released(KeyCode::Up) || keyboard_input.just_released(KeyCode::W) {
            step += Vec3::new(0.0, 1.0, 0.0);
        }
        if keyboard_input.just_released(KeyCode::Down) || keyboard_input.just_released(KeyCode::S) {
            step += Vec3::new(0.0, -1.0, 0.0);
        }

        position += step * 24.0;
        let x = (position.x / 24.0) as usize;
        let y = (position.y / 24.0) as usize;

        if let Some(row) = labyrinth.map.get(y) {
            if let Some(place) = row.get(x) {
                if *place == Element::Path {
                    transform.translation = position;
                }
                if *place == Element::Exit {
                    transform.translation = position;
                    next_app_state.set(AppState::Menu);
                }
            }
        }
    }
}
