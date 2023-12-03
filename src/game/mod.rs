use bevy::prelude::*;

mod action;
mod junk;
mod player;

pub use player::Player;

pub use action::Action;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Startup, (player::spawn, junk::spawn));
    }
}
