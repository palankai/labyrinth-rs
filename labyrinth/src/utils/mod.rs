pub mod collections;

use super::consts::SPRITE_SIZE;

use bevy::prelude::Vec3;

pub fn path_join(segments: Vec<&str>) -> String {
    segments.join(std::path::MAIN_SEPARATOR_STR)
}

pub fn sprite(s: &str) -> String {
    path_join(vec!["sprites", s])
}

pub fn font(s: &str) -> String {
    path_join(vec!["fonts", s])
}

pub fn world_to_viewport(p: Vec3) -> Vec3 {
    p * SPRITE_SIZE
}
