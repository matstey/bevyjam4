use bevy::prelude::*;

/// Component to tag an entity as only needed in some of the states
#[derive(Component, Debug)]
pub struct ForState<T> {
    pub states: Vec<T>,
}

#[derive(States, Debug, Copy, Clone, Hash, Eq, PartialEq, Default)]
pub enum AppState {
    #[default]
    Splash,
    StartMenu,
    GameLoading,
    GameRunning,
    GamePaused,
    GameOver,
}

impl AppState {
    pub const ANY_GAME_STATE: [AppState; 4] = [
        AppState::GameLoading,
        AppState::GameRunning,
        AppState::GamePaused,
        AppState::GameOver,
    ];
    pub fn is_any_game_state(&self) -> bool {
        AppState::ANY_GAME_STATE.contains(self)
    }
}
