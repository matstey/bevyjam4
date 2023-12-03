use bevy::prelude::*;
use leafwing_input_manager::Actionlike;

pub mod orbit;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Startup, orbit::init).add_systems(
            Update,
            (
                orbit::update_input,
                orbit::update_movement.after(orbit::update_input),
            ),
        );
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Hash, Debug, Reflect, Actionlike)]
pub enum CameraMovement {
    CanMove,
    Move,
    Zoom,
}
