use crate::camera::CameraPlugin;
use crate::game::GamePlugin;
use crate::scene::ScenePlugin;
use crate::state::AppState;
use asset::LoadingAssets;
use bevy::asset::AssetMetaCheck;
use bevy::core_pipeline::tonemapping::Tonemapping;
use bevy::prelude::*;
use bevy::window::PresentMode;
use bevy_rapier3d::prelude::*;

#[cfg(feature = "editor")]
use bevy_editor_pls::EditorPlugin;

pub mod camera;
mod coord;
use camera::orbit::OrbitCamera;
pub use coord::Coord;
use game::{Player, PlayerAction};
use input::InputPlugin;
use leafwing_input_manager::action_state::ActionState;
pub mod asset;
pub mod game;
pub mod input;
pub mod scene;
pub mod state;
pub mod ui;

pub struct ApplicationPlugin;

impl Plugin for ApplicationPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(AssetMetaCheck::Never) // Required to stop failed meta lookups. See https://github.com/bevyengine/bevy/pull/10623
            .insert_resource(ClearColor(Color::BLACK))
            .insert_resource(AmbientLight {
                color: Color::WHITE,
                brightness: 0.05,
            })
            .insert_resource(Msaa::Sample4)
            .add_plugins((DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Santa F**ked Up".into(),
                    resolution: (1280., 800.).into(),
                    present_mode: PresentMode::AutoVsync,
                    fit_canvas_to_parent: true, // Tells wasm to resize the window according to the available canvas
                    ..default()
                }),
                ..default()
            }),))
            .add_state::<AppState>()
            .add_plugins((ScenePlugin, GamePlugin, CameraPlugin, InputPlugin))
            .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
            .add_plugins(bevy_framepace::FramepacePlugin)
            .add_systems(Startup, setup_camera)
            .add_plugins((
                ui::UiPlugin,
                ui::splash::SplashPlugin,
                ui::menu::MenuPlugin,
                ui::loading::LoadingPlugin,
                ui::diagnostics::DiagnosticsPlugin,
            ))
            .insert_resource(LoadingAssets::default())
            .add_systems(Update, handle_pause);

        #[cfg(debug_assertions)]
        app.add_plugins(RapierDebugRenderPlugin::default());

        #[cfg(feature = "editor")]
        app.add_plugins(EditorPlugin::default());
    }
}

pub fn setup_camera(mut commands: Commands) {
    let transform = Transform::from_translation(Vec3::new(0.0, 1.0, 5.0))
        * Transform::from_rotation(Quat::from_rotation_y(0.0_f32.to_radians()));

    commands.spawn((
        Camera3dBundle {
            camera: Camera {
                hdr: false,
                ..default()
            },
            transform,
            tonemapping: Tonemapping::None,
            ..default()
        },
        OrbitCamera::new(),
        Coord::from_dist(80.0),
        Player::default(),
    ));
}

pub fn handle_pause(
    mut game_state: ResMut<NextState<AppState>>,
    query: Query<&ActionState<PlayerAction>>,
) {
    for action in query.iter() {
        if action.just_pressed(PlayerAction::Pause) {
            game_state.set(AppState::StartMenu);
        }
    }
}

// Generic system that takes a component as a parameter, and will despawn all entities with that component
pub fn despawn<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in &to_despawn {
        commands.entity(entity).despawn_recursive();
    }
}
