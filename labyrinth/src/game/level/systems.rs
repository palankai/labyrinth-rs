use std::f32::consts::{FRAC_PI_2, PI};

use bevy::prelude::*;

use super::super::components::*;
use super::simple_level_generator::SimpleLevel;
use super::Element;
use crate::utils::*;

pub fn spawn_level(mut commands: Commands, asset_server: Res<AssetServer>) {
    let main = commands.spawn((SpatialBundle::default(), Level {})).id();
    let walls = commands.spawn((SpatialBundle::default(), Walls {})).id();
    commands.entity(main).add_child(walls);

    let level = SimpleLevel::generate(4, 3);

    for tile in level.tiles().iter() {
        for element in tile.elements.iter() {
            let viewport_position = world_to_viewport(Vec3 {
                x: tile.x as f32,
                y: tile.y as f32,
                z: 0.0,
            });
            match element {
                Element::Path => {}
                Element::Exit => {
                    let exit = commands
                        .spawn((
                            SpriteBundle {
                                transform: Transform::from_translation(viewport_position),
                                texture: asset_server.load(sprite("door_64x64.png")),
                                ..default()
                            },
                            ExitDoor {},
                            ExitDoorCollider {},
                        ))
                        .id();
                    commands.entity(main).add_child(exit);
                }
                Element::Player => {
                    let player = spawn_player(&mut commands, &asset_server, viewport_position);
                    commands.entity(main).add_child(player);
                }
                Element::Wall => {
                    let r = pick(vec![0.0, FRAC_PI_2, PI, -FRAC_PI_2]).unwrap();
                    let wall = commands
                        .spawn((
                            SpriteBundle {
                                transform: Transform {
                                    translation: viewport_position,
                                    rotation: Quat::from_rotation_z(r),
                                    ..default()
                                },
                                texture: asset_server.load(sprite("wall_64x64.png")),

                                ..default()
                            },
                            Wall {},
                            WallCollider {},
                        ))
                        .id();
                    commands.entity(walls).add_child(wall);
                }
            }
        }
    }
}

pub fn despawn_level(mut commands: Commands, level_query: Query<Entity, With<Level>>) {
    for entity in level_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

fn spawn_player(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    position: Vec3,
) -> Entity {
    commands
        .spawn((
            SpatialBundle {
                transform: Transform::from_translation(position),
                ..default()
            },
            Player {},
        ))
        .with_children(|parent| {
            parent.spawn(SpriteBundle {
                transform: Transform {
                    translation: Vec3 {
                        x: 0.0,
                        y: 6.0,
                        z: 0.0,
                    },
                    scale: Vec3 {
                        x: 64.0 / 96.0,
                        y: 64.0 / 96.0,
                        z: 0.0,
                    },
                    ..default()
                },

                texture: asset_server.load(sprite("player_64x64.png")),
                ..default()
            });
        })
        .id()
}
