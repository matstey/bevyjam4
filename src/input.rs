use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

use crate::{game::Player, game::PlayerAction, state::AppState};

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugins(InputManagerPlugin::<PlayerAction>::default())
            .add_systems(
                OnEnter(AppState::GameRunning),
                add_player_input.run_if(in_state(AppState::GameRunning)),
            )
            .add_systems(OnExit(AppState::GameRunning), remove_player_input);
    }
}

pub fn add_player_input(mut commands: Commands, query: Query<Entity, With<Player>>) {
    for player in query.iter() {
        info!("Adding input to player {:#?}", player);

        commands
            .entity(player)
            .insert(InputManagerBundle::<PlayerAction> {
                action_state: ActionState::default(),
                input_map: create_map(),
            });
    }
}

pub fn remove_player_input(mut commands: Commands, query: Query<Entity, With<Player>>) {
    for player in query.iter() {
        info!("Removing input from player {:#?}", player);

        commands
            .entity(player)
            .remove::<ActionState<PlayerAction>>()
            .remove::<InputMap<PlayerAction>>();
    }
}

pub fn create_map() -> InputMap<PlayerAction> {
    InputMap::default()
        .insert(MouseButton::Left, PlayerAction::CanMove)
        .insert(DualAxis::mouse_motion(), PlayerAction::Move)
        .insert(SingleAxis::mouse_wheel_y(), PlayerAction::Zoom)
        .insert(KeyCode::Escape, PlayerAction::Pause)
        .build()
}
