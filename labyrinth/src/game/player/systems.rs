use bevy::prelude::*;
use bevy::sprite::collide_aabb::collide;

use crate::AppState;
use crate::{consts::*, game::events::MoveRequested};

use super::super::components::{ExitDoorCollider, Player, WallCollider};

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
    mut events: EventReader<MoveRequested>,
    //    keyboard_input: Res<Input<KeyCode>>,
    mut next_app_state: ResMut<NextState<AppState>>,
    time: Res<Time>,
) {
    let time_delta = time.delta_seconds();
    if let Ok(mut transform) = player_query.get_single_mut() {
        for event in events.iter() {
            let step_x = Vec3::new(event.direction.x, 0.0, 0.0);
            let step_y = Vec3::new(0.0, event.direction.y, 0.0);

            let step_x = step_x * PLAYER_SPEED * time_delta;
            let step_y = step_y * PLAYER_SPEED * time_delta;

            #[allow(clippy::clone_on_copy)]
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
            Vec2 { x: 58.0, y: 58.0 },
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
            Vec2 { x: 33.0, y: 33.0 },
            target_player_pos,
            Vec2 { x: 33.0, y: 33.0 },
        );
        if collision.is_some() {
            return true;
        }
    }
    false
}
