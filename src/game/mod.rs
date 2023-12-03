use bevy::prelude::*;

mod action;
mod player;
mod present;
pub use player::Player;
mod ground_station;
pub use ground_station::GroundStation;

pub use action::Action;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(PreStartup, ground_station::load_resources)
            .add_systems(
                Startup,
                (player::spawn, present::spawn, ground_station::spawn),
            );
    }
}
