use bevy::prelude::*;
use leafwing_input_manager::action_state::ActionState;

use crate::Coord;

use super::CameraMovement;

#[derive(Component, Default)]
pub struct OrbitCamera {}

impl OrbitCamera {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
}

pub fn init(mut _commands: Commands) {}

pub fn update_movement(
    _time: Res<Time>,
    mut camera_query: Query<(&mut Transform, &Coord), With<OrbitCamera>>,
) {
    for (mut _camera_transform, _coord) in camera_query.iter_mut() {
        //coord.apply(&mut camera_transform);
    }
}

pub fn update_input(
    time: Res<Time>,
    mut camera_query: Query<(&mut Coord, &ActionState<CameraMovement>)>,
) {
    for (mut coord, action) in camera_query.iter_mut() {
        let zoom_delta = action.value(CameraMovement::Zoom);
        coord.dist = (coord.dist - zoom_delta).clamp(40.0, 100.0);

        let move_delta = match action.axis_pair(CameraMovement::Move) {
            Some(axis) => axis.xy() * time.delta_seconds() * 0.1,
            None => Vec2::ZERO,
        };

        if action.pressed(CameraMovement::CanMove) {
            coord.long = coord.long - move_delta.x;
            coord.lat = coord.lat + move_delta.y;
        }
    }
}
