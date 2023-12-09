use bevy::prelude::*;
use leafwing_input_manager::action_state::ActionState;

use crate::state::GameState;
use crate::{coord::CoordDistance, Coord};

use crate::game::PlayerAction;

// For some reason mouse zoom is very different on web
#[cfg(target_arch = "wasm32")]
const ZOOM_SCALER: f32 = 1.0;
#[cfg(not(target_arch = "wasm32"))]
const ZOOM_SCALER: f32 = 100.0;

#[derive(Component, Default)]
pub struct OrbitCamera {}

impl OrbitCamera {
    pub fn new() -> Self {
        Default::default()
    }
}

pub fn init(mut _commands: Commands) {}

pub fn update_input(
    time: Res<Time>,
    mut camera_query: Query<(&mut Coord, &ActionState<PlayerAction>)>,
    game_state: Res<State<GameState>>,
) {
    // If game is paused dont update
    if *game_state.get() == GameState::Paused {
        return;
    }

    for (mut coord, action) in camera_query.iter_mut() {
        if action.pressed(PlayerAction::Zoom) {
            let zoom_delta = action.value(PlayerAction::Zoom) * time.delta_seconds() * ZOOM_SCALER;
            coord.dist =
                CoordDistance::Orbit((coord.get_distance() - zoom_delta).clamp(30.0, 100.0));
        }

        let move_delta = match action.axis_pair(PlayerAction::Move) {
            Some(axis) => axis.xy() * time.delta_seconds() * 0.1,
            None => Vec2::ZERO,
        };

        if action.pressed(PlayerAction::CanMove) {
            coord.long -= move_delta.x;
            coord.lat += move_delta.y;
        }
    }
}
