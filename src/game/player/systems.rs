use bevy::prelude::*;
use bevy::sprite::collide_aabb::{collide, Collision};

use crate::consts::*;
use crate::utils::sprite;
use crate::AppState;

use super::super::labyrinth::components::{ExitDoorCollider, WallCollider};
use super::components::Player;

pub fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    let x: f32 = 1.0 * 24.0 + 12.0;
    let y: f32 = 1.0 * 24.0 + 14.0;

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
    wall_collider_query: Query<&Transform, (With<WallCollider>, Without<Player>)>,
    exit_collider_query: Query<&Transform, (With<ExitDoorCollider>, Without<Player>)>,
    keyboard_input: Res<Input<KeyCode>>,
    mut next_app_state: ResMut<NextState<AppState>>,
    time: Res<Time>,
) {
    let time_delta = time.delta_seconds();
    if let Ok(mut transform) = player_query.get_single_mut() {
        let mut step_up = Vec3::ZERO;
        let mut step_left = Vec3::ZERO;
        let mut step_down = Vec3::ZERO;
        let mut step_right = Vec3::ZERO;

        if keyboard_input.pressed(KeyCode::Left) || keyboard_input.pressed(KeyCode::A) {
            step_left = Vec3::new(-1.0, 0.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::Right) || keyboard_input.pressed(KeyCode::D) {
            step_right = Vec3::new(1.0, 0.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::Up) || keyboard_input.pressed(KeyCode::W) {
            step_up = Vec3::new(0.0, 1.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::Down) || keyboard_input.pressed(KeyCode::S) {
            step_down = Vec3::new(0.0, -1.0, 0.0);
        }
        let full_step = step_up + step_left + step_down + step_right;

        let new_position = transform.translation + full_step * PLAYER_SPEED * time_delta;

        let exit_collision = exit_collision_check(new_position, &exit_collider_query);
        if exit_collision.is_some() {
            next_app_state.set(AppState::Menu);
            return;
        }

        let collision = wall_collision_check(new_position, &wall_collider_query);
        if let Some(collision) = collision {
            let mut step = Vec3::ZERO;
            match collision {
                Collision::Top => step += step_down + step_left + step_right,
                Collision::Left => step += step_up + step_right + step_down,
                Collision::Bottom => step += step_left + step_up + step_right,
                Collision::Right => step += step_up + step_down + step_left,
                _ => {}
            };
            // TODO need to write a better collider
            //transform.translation += step * PLAYER_SPEED * time_delta;
        } else {
            transform.translation = new_position;
        }

        // if let Some(row) = labyrinth.map.get(y) {
        //     if let Some(place) = row.get(x) {
        //         if *place == Element::Path {
        //             transform.translation = position;
        //         }
        //         if *place == Element::Exit {
        //             transform.translation = position;
        //             next_app_state.set(AppState::Menu);
        //         }
        //     }
        // }
    }
}

fn wall_collision_check(
    target_player_pos: Vec3,
    wall_collider_query: &Query<&Transform, (With<WallCollider>, Without<Player>)>, // Maybe with With??
) -> Option<Collision> {
    for wall_transform in wall_collider_query.iter() {
        let collision = collide(
            wall_transform.translation,
            Vec2 { x: 24.0, y: 24.0 },
            target_player_pos,
            Vec2 { x: 16.0, y: 16.0 },
        );
        if collision.is_some() {
            return collision;
        }
    }
    return None;
}

fn exit_collision_check(
    target_player_pos: Vec3,
    exit_door_query: &Query<&Transform, (With<ExitDoorCollider>, Without<Player>)>, // Maybe with With??
) -> Option<Collision> {
    for transform in exit_door_query.iter() {
        let collision = collide(
            transform.translation,
            Vec2 { x: 24.0, y: 24.0 },
            target_player_pos,
            Vec2 { x: 16.0, y: 16.0 },
        );
        if collision.is_some() {
            return collision;
        }
    }
    return None;
}
