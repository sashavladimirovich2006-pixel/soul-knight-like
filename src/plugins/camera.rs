//! Owns the single 2D gameplay camera.

use bevy::prelude::*;

/// Marker for the main gameplay camera, so other systems can find it without
/// assuming there is exactly one camera in the world.
#[derive(Component, Debug)]
pub struct MainCamera;

/// Spawns the gameplay camera.
pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera);
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn((Camera2d, MainCamera, Name::new("MainCamera")));
}
