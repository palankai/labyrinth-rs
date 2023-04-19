use bevy::prelude::*;

use super::super::components::*;
use super::level_generator::*;
use crate::utils::*;

pub fn spawn_level(mut commands: Commands, asset_server: Res<AssetServer>) {
    let level_map = LevelMap::new(11).unwrap();

    commands
        .spawn((SpatialBundle::default(), Level {}))
        .with_children(|parent| {
            spawn_walls(parent, &asset_server, &level_map);
            spawn_exit(parent, &asset_server, &level_map);
        });
}

pub fn despawn_level(mut commands: Commands, level_query: Query<Entity, With<Level>>) {
    for entity in level_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

fn spawn_walls(parent: &mut ChildBuilder, asset_server: &Res<AssetServer>, level_map: &LevelMap) {
    parent
        .spawn((SpatialBundle::default(), Walls {}))
        .with_children(|parent| {
            for tile_position in level_map.get_coordinates_for(Element::Wall) {
                let viewport_position = world_to_viewport(tile_position);
                parent.spawn((
                    SpriteBundle {
                        transform: Transform::from_translation(viewport_position),
                        texture: asset_server.load(sprite("wall_64x64.png")),
                        ..default()
                    },
                    Wall {},
                    WallCollider {},
                ));
            }
        });
}

fn spawn_exit(parent: &mut ChildBuilder, asset_server: &Res<AssetServer>, level_map: &LevelMap) {
    for tile_position in level_map.get_coordinates_for(Element::Exit) {
        let viewport_position = world_to_viewport(tile_position);
        parent.spawn((
            SpriteBundle {
                transform: Transform::from_translation(viewport_position),
                texture: asset_server.load(sprite("door_64x64.png")),
                ..default()
            },
            ExitDoor {},
            ExitDoorCollider {},
        ));
    }
}

#[allow(dead_code)]
fn spawn_player(parent: &mut ChildBuilder, asset_server: &Res<AssetServer>, level_map: &LevelMap) {
    for tile_position in level_map.get_coordinates_for(Element::Exit) {
        let viewport_position = world_to_viewport(tile_position);
        parent.spawn((
            SpriteBundle {
                transform: Transform::from_translation(viewport_position),
                texture: asset_server.load(sprite("player_64x64.png")),
                ..default()
            },
            Wall {},
            WallCollider {},
        ));
    }
}
