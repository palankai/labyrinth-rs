use bevy::prelude::*;

use crate::model::logic::*;

#[derive(Resource)]
pub struct LabyrinthMap {
    pub map: [[Element; 41]; 41],
}

impl Default for LabyrinthMap {
    fn default() -> Self {
        LabyrinthMap {
            map: [[Element::default(); 41]; 41],
        }
    }
}
