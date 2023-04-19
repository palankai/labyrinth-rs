use bevy::prelude::*;

use super::components::*;
use crate::model::logic::*;
use crate::utils::*;

use super::resources::*;

pub fn remove_labyrinth_map_resource(mut commands: Commands) {
    commands.remove_resource::<LabyrinthMap>();
}

pub fn spawn_walls(mut commands: Commands, asset_server: Res<AssetServer>) {
    let map = make_map();
    let labyrinth = LabyrinthMap { map: map.clone() };
    commands.insert_resource(labyrinth);
    for y in 0..41 {
        for x in 0..41 {
            match map[y][x] {
                Element::Path => {}
                Element::Exit => {
                    put_exit(&mut commands, x as f32, y as f32);
                }
                Element::Wall => {
                    put_wall(&mut commands, &asset_server, x as f32, y as f32);
                }
            }
        }
    }
}

pub fn despawn_walls(
    mut commands: Commands,
    wall_query: Query<Entity, With<Wall>>,
    exit_door_query: Query<Entity, With<ExitDoor>>,
) {
    for entity in wall_query.iter() {
        commands.entity(entity).despawn();
    }
    for entity in exit_door_query.iter() {
        commands.entity(entity).despawn();
    }
}

fn put_wall(commands: &mut Commands, asset_server: &Res<AssetServer>, world_x: f32, world_y: f32) {
    let x: f32 = world_x * 24.0 + 12.0;
    let y: f32 = world_y * 24.0 + 12.0;

    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(x, y, 0.0),
            texture: asset_server.load(sprite("wall_24x24.png")),
            ..default()
        },
        Wall {},
        WallCollider {},
    ));
}

fn put_exit(commands: &mut Commands, world_x: f32, world_y: f32) {
    let x: f32 = world_x * 24.0 + 12.0;
    let y: f32 = world_y * 24.0 + 12.0;

    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(x, y, 0.0),
            ..default()
        },
        ExitDoor {},
        ExitDoorCollider {},
    ));
}