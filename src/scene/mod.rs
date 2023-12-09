use bevy::prelude::*;

use crate::{asset::LoadingAssets, despawn, state::AppState, Coord};

pub struct ScenePlugin;

impl Plugin for ScenePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(OnEnter(AppState::Loading), load_resources)
            .add_systems(OnExit(AppState::Loading), setup)
            .add_systems(Update, update_coords)
            .add_systems(OnExit(AppState::InGame), despawn::<SceneElement>);
    }
}

#[derive(Component)]
struct SceneElement;

#[derive(Resource)]
pub struct SceneResources {
    pub earth: Handle<Scene>,
}

pub fn load_resources(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut loading: ResMut<LoadingAssets>,
) {
    let earth = asset_server.load("earth.glb#Scene0");
    loading.0.push(earth.id().into());
    commands.insert_resource(SceneResources { earth });
}

fn setup(
    mut commands: Commands,
    mut _meshes: ResMut<Assets<Mesh>>,
    mut _materials: ResMut<Assets<StandardMaterial>>,
    scene_res: Res<SceneResources>,
) {
    let scene = SceneBundle {
        scene: scene_res.earth.clone_weak(),
        ..default()
    };
    commands.spawn((scene, SceneElement));

    commands.spawn((
        DirectionalLightBundle {
            directional_light: DirectionalLight {
                shadows_enabled: false,
                ..default()
            },
            transform: Transform {
                translation: Vec3::ZERO,
                rotation: Quat::from_rotation_y(90f32.to_radians())
                    * Quat::from_rotation_x(180f32.to_radians()),
                ..default()
            },
            ..default()
        },
        SceneElement,
    ));

    info!("Setting up game");
}

pub fn update_coords(mut query: Query<(&mut Transform, &Coord)>) {
    for (mut transform, coord) in query.iter_mut() {
        coord.apply(&mut transform);
    }
}
