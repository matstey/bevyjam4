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
    Loading,
    InGame,
    PostGame,
}

#[derive(States, Debug, Copy, Clone, Hash, Eq, PartialEq, Default)]
pub enum GameState {
    #[default]
    Running,
    Paused,
}
