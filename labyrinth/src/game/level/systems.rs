use std::{cell::RefCell, rc::Rc};

use bevy::prelude::*;

use super::super::components::*;
use super::level_generator::*;
use crate::utils::*;

pub fn spawn_level(mut commands: Commands, asset_server: Res<AssetServer>) {
    let walls = make_random_wall_tiles();

    commands
        .spawn((SpatialBundle::default(), Level {}))
        .with_children(|parent| {
            parent
                .spawn((SpatialBundle::default(), Walls {}))
                .with_children(|parent| {
                    for tile_position in walls {
                        put_wall(parent, &asset_server, tile_position)
                    }
                });
        });
}

pub fn despawn_level(mut commands: Commands, level_query: Query<Entity, With<Level>>) {
    for entity in level_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

fn put_wall(commands: &mut ChildBuilder, asset_server: &Res<AssetServer>, world_position: Vec3) {
    let viewport_position = world_to_viewport(world_position);

    commands.spawn((
        SpriteBundle {
            transform: Transform::from_translation(viewport_position),
            texture: asset_server.load(sprite("wall_64x64.png")),
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
