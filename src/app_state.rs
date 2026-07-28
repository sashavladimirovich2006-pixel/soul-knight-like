//! Global application state machine and the gameplay timestep.

use bevy::prelude::*;

/// Gameplay logic ticks per second.
///
/// All simulation runs in `FixedUpdate` at this rate so that a run is
/// reproducible for a given seed, independently of the render frame rate.
pub const FIXED_HZ: f64 = 60.0;

/// High level application state.
///
/// Transitions are requested through `NextState`; per-state setup and teardown
/// belong in `OnEnter` / `OnExit` systems of the owning feature plugin.
#[derive(States, Debug, Clone, Copy, Default, Eq, PartialEq, Hash)]
pub enum AppState {
    /// Waiting for assets to finish loading.
    #[default]
    AssetLoading,
    /// Title screen.
    MainMenu,
    /// Hub between runs: shop and character selection.
    Lobby,
    /// An active run inside the dungeon.
    Run,
    /// Run is suspended, simulation is frozen.
    Paused,
    /// The run ended in death or victory.
    GameOver,
}

/// Registers [`AppState`] and the fixed timestep used by gameplay systems.
pub struct AppStatePlugin;

impl Plugin for AppStatePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<AppState>()
            .insert_resource(Time::<Fixed>::from_hz(FIXED_HZ));
    }
}

#[cfg(test)]
mod tests {
    use super::AppState;

    #[test]
    fn default_state_is_asset_loading() {
        assert_eq!(AppState::default(), AppState::AssetLoading);
    }
}
