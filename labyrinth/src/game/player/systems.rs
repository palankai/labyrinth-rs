use bevy::prelude::*;
use bevy::sprite::collide_aabb::collide;

use super::super::components::*;
use super::PlayerState;
use crate::game::events::MoveRequested;
use crate::AppState;

pub fn player_is_on_the_move(current_player_state: Res<State<PlayerState>>) {
    if current_player_state.0 == PlayerState::Moving {
        println!("Moving")
    }
}

pub fn player_is_moving(
    mut commands: Commands,
    mut player_query: Query<(Entity, &mut Transform, &mut MovingPlayer), With<MovingPlayer>>,
    mut camera_query: Query<&mut Transform, (With<Camera2d>, Without<MovingPlayer>)>,
    time: Res<Time>,
) {
    let mut camera_transform = camera_query.get_single_mut().unwrap();

    for (entity, mut player_transform, mut movement) in player_query.iter_mut() {
        movement.timer.tick(time.delta());
        if movement.timer.just_finished() {
            move_player(
                &mut player_transform,
                &mut camera_transform,
                movement.target,
            );
            commands.entity(entity).remove::<MovingPlayer>();
        } else {
            let position =
                movement.starging_position + movement.step * movement.timer.percent() * 0.85;
            move_player(&mut player_transform, &mut camera_transform, position);
        }
    }
}

pub fn move_player(
    player_transform: &mut Transform,
    camera_transform: &mut Transform,
    position: Vec3,
) {
    player_transform.translation = position;
    camera_transform.translation = position;
}

pub fn player_movement(
    mut commands: Commands,
    player_query: Query<&Transform, (With<Player>, Without<MovingPlayer>)>,
    player_entity: Query<Entity, With<Player>>,
    wall_collider_query: Query<
        &Transform,
        (With<WallCollider>, Without<Player>, Without<Camera2d>),
    >,
    exit_collider_query: Query<
        &Transform,
        (With<ExitDoorCollider>, Without<Player>, Without<Camera2d>),
    >,
    mut events: EventReader<MoveRequested>,
    mut next_app_state: ResMut<NextState<AppState>>,
) {
    if let Ok(transform) = player_query.get_single() {
        for event in events.iter() {
            let step_x = Vec3::new(event.direction.x, 0.0, 0.0);
            let step_y = Vec3::new(0.0, event.direction.y, 0.0);

            let step_x = step_x * 64.0; //PLAYER_SPEED * time_delta;
            let step_y = step_y * 64.0; //PLAYER_SPEED * time_delta;
            #[allow(clippy::clone_on_copy)]
            let mut step = Vec3::ZERO;

            if !wall_collision_check(transform.translation + step + step_x, &wall_collider_query) {
                step += step_x;
            }

            if !wall_collision_check(transform.translation + step + step_y, &wall_collider_query) {
                step += step_y;
            }

            if exit_collision_check(transform.translation + step, &exit_collider_query) {
                next_app_state.set(AppState::Menu);
            }

            if let Ok(player_entity) = player_entity.get_single() {
                commands.entity(player_entity).insert(MovingPlayer {
                    timer: Timer::from_seconds(0.15, TimerMode::Once),
                    starging_position: transform.translation,
                    step,
                    target: transform.translation + step,
                });
            }

            //transform.translation = target;
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
