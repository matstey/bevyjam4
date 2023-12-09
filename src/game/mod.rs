use bevy::prelude::*;
use leafwing_input_manager::Actionlike;

mod action;
mod player;
mod present;
pub use player::Player;
mod ground_station;
pub use ground_station::GroundStation;

pub use action::Action;

use crate::{asset::LoadingAssets, despawn, state::AppState};

#[derive(PartialEq, Eq, Clone, Copy, Hash, Debug, Reflect, Actionlike)]
pub enum PlayerAction {
    CanMove,
    Move,
    Zoom,
    Pause,
}

#[derive(Component)]
struct GameElement;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(OnEnter(AppState::Loading), ground_station::load_resources)
            .add_systems(
                OnEnter(AppState::InGame),
                (player::spawn, present::spawn, ground_station::spawn)
                    .run_if(in_state(AppState::InGame)),
            )
            .add_systems(
                Update,
                ground_station::cast_ray.run_if(in_state(AppState::InGame)),
            )
            .add_systems(
                Update,
                check_assets_loaded.run_if(in_state(AppState::Loading)),
            )
            .add_systems(OnExit(AppState::InGame), despawn::<GameElement>);
    }
}

fn check_assets_loaded(
    mut commands: Commands,
    mut game_state: ResMut<NextState<AppState>>,
    server: Res<AssetServer>,
    loading: Res<LoadingAssets>,
) {
    let mut loaded = 0;
    for id in loading.0.iter() {
        if server.is_loaded_with_dependencies(*id) {
            loaded += 1;
        }
    }

    if loaded == loading.0.len() {
        game_state.set(AppState::InGame);
        commands.remove_resource::<LoadingAssets>();
        info!("All {} assets loaded", loaded);
    }
}
