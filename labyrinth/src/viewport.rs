use bevy::prelude::*;

pub struct ViewportPlugin;
use systems::*;

impl Plugin for ViewportPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_camera);
    }
}

mod systems {
    use bevy::window::PrimaryWindow;
    use bevy::{prelude::*, render::camera::ScalingMode};

    pub fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
        let window = window_query.get_single().unwrap();

        commands.spawn(Camera2dBundle {
            projection: OrthographicProjection {
                scaling_mode: ScalingMode::WindowSize(2.0),
                ..default()
            },
            transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
            ..default()
        });
    }
}
