use bevy::prelude::*;

use crate::state::AppState;

pub mod assets;
pub mod diagnostics;
pub mod loading;
pub mod menu;
pub mod splash;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, assets::setup);
    }
}
