//! Entry point for the game binary.
//!
//! This file stays thin on purpose: it only wires plugins together. Feature
//! logic lives in `src/plugins/*`, and engine-independent logic lives in its
//! own modules so it can be unit tested without a renderer.

mod app_state;
mod logic;
mod plugins;

use bevy::prelude::*;

use crate::app_state::AppStatePlugin;
use crate::plugins::camera::CameraPlugin;
use crate::plugins::player::PlayerPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "soul-knight-like".to_owned(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins((AppStatePlugin, CameraPlugin, PlayerPlugin))
        .run();
}
