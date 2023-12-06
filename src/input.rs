use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

use crate::{camera::CameraMovement, game::Player, state::AppState};

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugins(InputManagerPlugin::<CameraMovement>::default())
            .add_systems(
                Update,
                add_player_input.run_if(in_state(AppState::GameRunning)),
            );
    }
}

pub fn add_player_input(mut commands: Commands, query: Query<Entity, Added<Player>>) {
    for player in query.iter() {
        info!("Adding input to new player {:#?}", player);

        commands
            .entity(player)
            .insert(InputManagerBundle::<CameraMovement> {
                action_state: ActionState::default(),
                input_map: create_map(),
            });
    }
}

pub fn create_map() -> InputMap<CameraMovement> {
    InputMap::default()
        .insert(MouseButton::Left, CameraMovement::CanMove)
        .insert(DualAxis::mouse_motion(), CameraMovement::Move)
        .insert(SingleAxis::mouse_wheel_y(), CameraMovement::Zoom)
        .build()
}
