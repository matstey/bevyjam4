use bevy::prelude::*;
use leafwing_input_manager::Actionlike;

pub mod player;
pub mod present;
pub use player::Player;
pub mod ground_station;
pub use ground_station::GroundStation;
pub mod rocket;

use crate::{
    asset::LoadingAssets,
    despawn,
    state::{AppState, GameState},
};

use self::present::Present;

#[derive(Resource)]
pub struct LevelConfig {
    pub low_orbit_presents: i32,
    pub high_orbit_presents: i32,
    pub time: i32,
    pub present_size: f32,
    pub present_hitbox_size: f32,
}

#[derive(Resource, Default)]
pub struct GameData {
    pub presents_collected: i32,
    pub won: bool,
}

#[derive(PartialEq, Eq, Clone, Copy, Hash, Debug, Reflect, Actionlike)]
pub enum PlayerAction {
    CanMove,
    Move,
    Zoom,
    Pause,
}

#[derive(Component)]
struct GameElement;

#[derive(Resource, Deref, DerefMut)]
pub struct GameTimer(Timer);

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Startup, present::init)
            .add_systems(OnEnter(AppState::Loading), ground_station::load_resources)
            .add_systems(
                OnEnter(AppState::InGame),
                (init, player::spawn, present::spawn, ground_station::spawn)
                    .run_if(in_state(AppState::InGame)),
            )
            .add_systems(
                Update,
                (
                    present::cast_ray.run_if(in_state(AppState::InGame)),
                    rocket::collect_presents.run_if(in_state(AppState::InGame)),
                    rocket::update_target.run_if(in_state(AppState::InGame)),
                    update_present_count.run_if(in_state(AppState::InGame)),
                ),
            )
            .add_systems(
                Update,
                check_assets_loaded.run_if(in_state(AppState::Loading)),
            )
            .add_systems(OnExit(AppState::InGame), despawn::<GameElement>)
            .add_systems(Update, countdown.run_if(in_state(AppState::InGame)))
            .add_systems(
                OnEnter(GameState::Paused),
                pause.run_if(in_state(AppState::InGame)),
            )
            .add_systems(
                OnExit(GameState::Paused),
                unpause.run_if(in_state(AppState::InGame)),
            )
            .insert_resource(LevelConfig {
                low_orbit_presents: 140,
                high_orbit_presents: 30,
                time: 160,
                present_size: 0.4,
                present_hitbox_size: 0.5,
            })
            .insert_resource(GameData::default());
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

fn init(mut commands: Commands, level_config: Res<LevelConfig>, mut game_data: ResMut<GameData>) {
    commands.insert_resource(GameTimer(Timer::from_seconds(
        level_config.time as f32,
        TimerMode::Once,
    )));

    game_data.presents_collected = 0;
    game_data.won = false;
}

fn countdown(
    mut app_state: ResMut<NextState<AppState>>,
    time: Res<Time>,
    mut timer: ResMut<GameTimer>,
) {
    if timer.tick(time.delta()).finished() {
        app_state.set(AppState::PostGame);
    }
}

fn pause(mut timer: ResMut<GameTimer>) {
    timer.pause();
}

fn unpause(mut timer: ResMut<GameTimer>) {
    timer.unpause();
}

fn update_present_count(
    present_query: Query<&Present>,
    level_config: Res<LevelConfig>,
    mut game_data: ResMut<GameData>,
    mut app_state: ResMut<NextState<AppState>>,
) {
    let target = level_config.high_orbit_presents + level_config.low_orbit_presents;
    game_data.presents_collected = (target - present_query.iter().len() as i32).clamp(0, target);

    // Did we win??
    if game_data.presents_collected >= target {
        app_state.set(AppState::PostGame);
        game_data.won = true;
    }
}
