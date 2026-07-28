//! Engine independent game logic.
//!
//! Nothing in here may depend on Bevy. That keeps the rules of the game
//! testable without a window, a renderer or a running app.

pub mod movement;
