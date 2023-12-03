use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

#[derive(PartialEq, Eq, Clone, Copy, Hash, Debug, Reflect, Actionlike)]
pub enum Action {
    Move,
    Zoom,
}
