use bevy::prelude::*;
use bevy::sprite::collide_aabb::collide;

use crate::utils::sprite;
use crate::AppState;
use crate::{consts::*, utils::world_to_viewport};

use super::super::components::{ExitDoorCollider, Player, WallCollider};

pub fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    let player_position = world_to_viewport(Vec3 {
        x: 1.0,
        y: 1.0,
        z: 0.0,
    });
    commands
        .spawn((
            SpatialBundle {
                transform: Transform::from_translation(player_position),
                ..default()
            },
            Player {},
        ))
        .with_children(|parent| {
            parent.spawn(SpriteBundle {
                transform: Transform {
                    translation: Vec3 {
                        x: 0.0,
                        y: 10.0,
                        z: 0.0,
                    },
                    scale: Vec3 {
                        x: 80.0 / 96.0,
                        y: 80.0 / 96.0,
                        z: 0.0,
                    },
                    ..default()
                },

                texture: asset_server.load(sprite("player_64x64.png")),
                ..default()
            });
        });
}

pub fn despawn_player(mut commands: Commands, query: Query<Entity, With<Player>>) {
    if let Ok(entity) = query.get_single() {
        commands.entity(entity).despawn_recursive();
    }
}

pub fn player_movement(
    mut player_query: Query<&mut Transform, With<Player>>,
    mut camera_query: Query<&mut Transform, (With<Camera2d>, Without<Player>)>,
    wall_collider_query: Query<
        &Transform,
        (With<WallCollider>, Without<Player>, Without<Camera2d>),
    >,
    exit_collider_query: Query<
        &Transform,
        (With<ExitDoorCollider>, Without<Player>, Without<Camera2d>),
    >,
    keyboard_input: Res<Input<KeyCode>>,
    mut next_app_state: ResMut<NextState<AppState>>,
    time: Res<Time>,
) {
    let time_delta = time.delta_seconds();
    if let Ok(mut transform) = player_query.get_single_mut() {
        let mut step_x = Vec3::ZERO;
        let mut step_y = Vec3::ZERO;

        if keyboard_input.pressed(KeyCode::Left) || keyboard_input.pressed(KeyCode::A) {
            step_x += Vec3::new(-1.0, 0.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::Right) || keyboard_input.pressed(KeyCode::D) {
            step_x += Vec3::new(1.0, 0.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::Up) || keyboard_input.pressed(KeyCode::W) {
            step_y += Vec3::new(0.0, 1.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::Down) || keyboard_input.pressed(KeyCode::S) {
            step_y += Vec3::new(0.0, -1.0, 0.0);
        }

        let step_x = step_x * PLAYER_SPEED * time_delta;
        let step_y = step_y * PLAYER_SPEED * time_delta;

        let mut target = transform.translation.clone();

        if !wall_collision_check(target + step_x, &wall_collider_query) {
            target += step_x;
        }

        if !wall_collision_check(target + step_y, &wall_collider_query) {
            target += step_y;
        }

        if exit_collision_check(target, &exit_collider_query) {
            next_app_state.set(AppState::Menu);
        }

        transform.translation = target;
        if let Ok(mut camera_transform) = camera_query.get_single_mut() {
            camera_transform.translation = target;
        }
    }
}

fn wall_collision_check(
    target_player_pos: Vec3,
    wall_collider_query: &Query<
        &Transform,
        (With<WallCollider>, Without<Player>, Without<Camera2d>),
    >, // Maybe with With??
) -> bool {
    for wall_transform in wall_collider_query.iter() {
        let collision = collide(
            wall_transform.translation,
            Vec2 { x: 64.0, y: 64.0 },
            target_player_pos,
            Vec2 { x: 62.0, y: 62.0 },
        );
        if collision.is_some() {
            return true;
        }
    }
    false
}

fn exit_collision_check(
    target_player_pos: Vec3,
    exit_door_query: &Query<
        &Transform,
        (With<ExitDoorCollider>, Without<Player>, Without<Camera2d>),
    >, // Maybe with With??
) -> bool {
    for transform in exit_door_query.iter() {
        let collision = collide(
            transform.translation,
            Vec2 { x: 24.0, y: 24.0 },
            target_player_pos,
            Vec2 { x: 20.0, y: 20.0 },
        );
        if collision.is_some() {
            return true;
        }
    }
    false
}
