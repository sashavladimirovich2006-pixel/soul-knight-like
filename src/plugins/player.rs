//! The player entity: spawning, keyboard movement and mouse aiming.

use bevy::prelude::*;

use crate::logic::movement::movement_direction;

/// Top speed in world units per second. One world unit is one texture pixel.
pub const PLAYER_SPEED: f32 = 120.0;

/// Side length of the placeholder sprite, in world units.
const PLAYER_SIZE: f32 = 12.0;

/// Marks the entity the player controls.
#[derive(Component, Debug)]
pub struct Player;

/// Unit vector pointing from the player towards the cursor.
///
/// Weapons read this instead of looking at the cursor themselves, so firing
/// logic stays independent of the windowing system.
#[derive(Component, Debug, Default)]
pub struct Aim(pub Vec2);

/// Everything the player entity needs.
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        // Movement is simulation and belongs on the fixed timestep. Aiming
        // follows the cursor, which is presentation-rate input.
        app.add_systems(Startup, spawn_player)
            .add_systems(FixedUpdate, move_player)
            .add_systems(Update, update_aim);
    }
}

fn spawn_player(mut commands: Commands) {
    commands.spawn((
        Sprite::from_color(Color::srgb(0.92, 0.94, 1.0), Vec2::splat(PLAYER_SIZE)),
        Transform::from_xyz(0.0, 0.0, 1.0),
        Player,
        Aim::default(),
        Name::new("Player"),
    ));
}

fn move_player(
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut players: Query<&mut Transform, With<Player>>,
) {
    let (x, y) = movement_direction(
        keys.pressed(KeyCode::KeyW),
        keys.pressed(KeyCode::KeyS),
        keys.pressed(KeyCode::KeyA),
        keys.pressed(KeyCode::KeyD),
    );

    let step = Vec2::new(x, y) * PLAYER_SPEED * time.delta_secs();
    if step == Vec2::ZERO {
        return;
    }

    for mut transform in &mut players {
        transform.translation += step.extend(0.0);
    }
}

fn update_aim(
    camera_query: Single<(&Camera, &GlobalTransform)>,
    window: Single<&Window>,
    mut players: Query<(&Transform, &mut Aim), With<Player>>,
) {
    let (camera, camera_transform) = *camera_query;

    // No cursor over the window, or a viewport that cannot be projected yet:
    // keep the previous aim instead of snapping the player somewhere random.
    let Some(cursor) = window.cursor_position() else {
        return;
    };
    let Ok(cursor_world) = camera.viewport_to_world_2d(camera_transform, cursor) else {
        return;
    };

    for (transform, mut aim) in &mut players {
        let to_cursor = cursor_world - transform.translation.truncate();
        if let Some(direction) = to_cursor.try_normalize() {
            aim.0 = direction;
        }
    }
}
