use crate::camera::CameraPlugin;
use crate::game::GamePlugin;
use crate::scene::ScenePlugin;
use crate::state::AppState;
use asset::LoadingAssets;
use bevy::asset::LoadState;
use bevy::core_pipeline::tonemapping::Tonemapping;
use bevy::prelude::*;
use bevy::render::render_resource::{TextureViewDescriptor, TextureViewDimension};
use bevy::window::PresentMode;
use bevy::{asset::AssetMetaCheck, core_pipeline::Skybox};
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
use state::{GameState, InteractionState};
pub mod asset;
pub mod game;
pub mod input;
pub mod scene;
pub mod state;
pub mod ui;

#[derive(Resource)]
struct Cubemap {
    is_loaded: bool,
    image_handle: Handle<Image>,
}

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
                    resolution: (1200., 800.).into(),
                    present_mode: PresentMode::AutoVsync,
                    fit_canvas_to_parent: true, // Tells wasm to resize the window according to the available canvas
                    ..default()
                }),
                ..default()
            }),))
            .add_state::<AppState>()
            .add_state::<GameState>()
            .add_state::<InteractionState>()
            .add_plugins((ScenePlugin, GamePlugin, CameraPlugin, InputPlugin))
            .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
            .add_plugins(bevy_framepace::FramepacePlugin)
            .add_systems(Startup, setup_camera)
            .add_plugins((
                ui::UiPlugin,
                ui::splash::SplashPlugin,
                ui::start::StartMenuPlugin,
                ui::paused::PausedMenuPlugin,
                ui::loading::LoadingPlugin,
                ui::game::GamePlugin,
                ui::post::PostGamePlugin,
                //ui::diagnostics::DiagnosticsPlugin,
            ))
            .insert_resource(LoadingAssets::default())
            .add_systems(Update, (handle_pause, cubemap_loaded));

        //#[cfg(debug_assertions)]
        #[cfg(feature = "editor")]
        app.add_plugins(RapierDebugRenderPlugin::default());

        #[cfg(feature = "editor")]
        app.add_plugins(EditorPlugin::default());
    }
}

pub fn setup_camera(mut commands: Commands, asset_server: Res<AssetServer>) {
    let transform = Transform::from_translation(Vec3::new(0.0, 1.0, 5.0))
        * Transform::from_rotation(Quat::from_rotation_y(0.0_f32.to_radians()));

    let skybox = asset_server.load("sky.png");

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
        Skybox(skybox.clone()),
    ));

    commands.insert_resource(Cubemap {
        is_loaded: false,
        image_handle: skybox,
    });
}

fn cubemap_loaded(
    asset_server: Res<AssetServer>,
    mut images: ResMut<Assets<Image>>,
    mut cubemap: ResMut<Cubemap>,
    mut skyboxes: Query<&mut Skybox>,
) {
    if !cubemap.is_loaded && asset_server.load_state(&cubemap.image_handle) == LoadState::Loaded {
        let image = images.get_mut(&cubemap.image_handle).unwrap();
        // NOTE: PNGs do not have any metadata that could indicate they contain a cubemap texture,
        // so they appear as one texture. The following code reconfigures the texture as necessary.
        if image.texture_descriptor.array_layer_count() == 1 {
            image.reinterpret_stacked_2d_as_array(image.height() / image.width());
            image.texture_view_descriptor = Some(TextureViewDescriptor {
                dimension: Some(TextureViewDimension::Cube),
                ..default()
            });
        }

        for mut skybox in &mut skyboxes {
            skybox.0 = cubemap.image_handle.clone();
        }

        cubemap.is_loaded = true;
    }
}

pub fn handle_pause(
    mut next_game_state: ResMut<NextState<GameState>>,
    game_state: Res<State<GameState>>,
    query: Query<&ActionState<PlayerAction>>,
) {
    for action in query.iter() {
        if action.just_pressed(PlayerAction::Pause) {
            match game_state.get() {
                GameState::Running => next_game_state.set(GameState::Paused),
                GameState::Paused => next_game_state.set(GameState::Running),
            }
        }
    }
}

// Generic system that takes a component as a parameter, and will despawn all entities with that component
pub fn despawn<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in &to_despawn {
        commands.entity(entity).despawn_recursive();
    }
}
