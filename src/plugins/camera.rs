//! Owns the single 2D gameplay camera.

use bevy::camera::{OrthographicProjection, Projection, ScalingMode};
use bevy::prelude::*;

/// Height of the camera's view, in world units.
///
/// One world unit is one texture pixel, so this is the vertical resolution the
/// art is authored against. The width follows the window aspect ratio, which
/// means wider displays see more of the room instead of a cropped or stretched
/// image.
pub const VIEWPORT_HEIGHT: f32 = 360.0;

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
    commands.spawn((
        Camera2d,
        Projection::Orthographic(OrthographicProjection {
            scaling_mode: ScalingMode::FixedVertical {
                viewport_height: VIEWPORT_HEIGHT,
            },
            ..OrthographicProjection::default_2d()
        }),
        MainCamera,
        Name::new("MainCamera"),
    ));
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The projection is what keeps sprite sizes predictable, so a missing or
    /// wrong scaling mode should fail here instead of being noticed by eye.
    #[test]
    fn camera_uses_the_fixed_pixel_art_viewport() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins).add_plugins(CameraPlugin);
        app.update();

        let mut query = app
            .world_mut()
            .query_filtered::<&Projection, With<MainCamera>>();
        let projections: Vec<&Projection> = query.iter(app.world()).collect();

        assert_eq!(
            projections.len(),
            1,
            "exactly one gameplay camera is expected"
        );

        match projections[0] {
            Projection::Orthographic(orthographic) => match orthographic.scaling_mode {
                ScalingMode::FixedVertical { viewport_height } => {
                    assert_eq!(viewport_height, VIEWPORT_HEIGHT);
                }
                _ => panic!("the camera should use a fixed vertical viewport"),
            },
            _ => panic!("a 2D camera should use an orthographic projection"),
        }
    }
}
