use bevy::prelude::*;

pub struct ViewportPlugin;
use systems::*;

impl Plugin for ViewportPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_camera);
    }
}

mod systems {
    use bevy::{prelude::*, render::camera::ScalingMode};

    pub fn spawn_camera(mut commands: Commands) {
        commands.spawn(Camera2dBundle {
            projection: OrthographicProjection {
                scaling_mode: ScalingMode::WindowSize(2.0),
                ..default()
            },
            ..default()
        });
    }
}
