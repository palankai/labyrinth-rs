use bevy::{prelude::*, utils::Instant};

#[derive(Component)]
pub struct Level {}

#[derive(Component)]
pub struct Player {}

#[derive(Component, Debug)]
pub struct MovingPlayer {
    pub started: Instant,
}

#[derive(Component)]
pub struct Walls {
    // This components holds all of the walls
}

#[derive(Component)]
pub struct Wall {}

#[derive(Component)]
pub struct ExitDoor {}

#[derive(Component)]
pub struct WallCollider {}

#[derive(Component)]
pub struct ExitDoorCollider {}
