use bevy::prelude::*;

pub mod orbit;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Startup, orbit::init)
            .add_systems(Update, orbit::update_input);
    }
}
