//! Entry point for the game binary.
//!
//! This file stays thin on purpose: it only wires plugins together. Feature
//! logic lives in `src/plugins/*`, and engine-independent logic lives in its
//! own modules so it can be unit tested without a renderer.

mod app_state;
mod plugins;

use bevy::image::ImagePlugin;
use bevy::prelude::*;

use crate::app_state::AppStatePlugin;
use crate::plugins::camera::CameraPlugin;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "soul-knight-like".to_owned(),
                        ..default()
                    }),
                    ..default()
                })
                // Pixel art must never be interpolated: linear filtering turns
                // scaled up sprites into a blurry mess.
                .set(ImagePlugin::default_nearest()),
        )
        .add_plugins((AppStatePlugin, CameraPlugin))
        .run();
}
