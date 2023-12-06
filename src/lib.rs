use crate::camera::CameraPlugin;
use crate::game::GamePlugin;
use crate::scene::ScenePlugin;
use crate::state::AppState;
use asset::LoadingAssets;
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
use game::Player;
use input::InputPlugin;
pub mod asset;
pub mod game;
pub mod input;
pub mod scene;
pub mod state;
pub mod ui;

pub struct ApplicationPlugin;

impl Plugin for ApplicationPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ClearColor(Color::BLACK))
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
                    // Tells wasm to resize the window according to the available canvas
                    //fit_canvas_to_parent: true,
                    // Bind to canvas included in `index.html`
                    //canvas: Some("#bevy".to_owned()),
                    // Tells wasm not to override default event handling, like F5, Ctrl+R etc.
                    //prevent_default_event_handling: false,
                    ..default()
                }),
                ..default()
            }),))
            .add_state::<AppState>()
            .add_plugins((ScenePlugin, GamePlugin, CameraPlugin, InputPlugin))
            .add_plugins((
                RapierPhysicsPlugin::<NoUserData>::default(),
                RapierDebugRenderPlugin::default(),
            ))
            .add_systems(Startup, setup_camera)
            .add_plugins((
                ui::UiPlugin,
                ui::splash::SplashPlugin,
                ui::menu::MenuPlugin,
                ui::loading::LoadingPlugin,
                ui::diagnostics::DiagnosticsPlugin,
            ))
            .insert_resource(LoadingAssets::default());

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
