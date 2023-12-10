use std::f32::consts::PI;

use bevy::{prelude::*, window::PrimaryWindow};
use bevy_rapier3d::prelude::*;
use leafwing_input_manager::action_state::ActionState;
use rand::{distributions::uniform::SampleRange, Rng};

use crate::{coord::CoordDistance, state::InteractionState, Coord};

use super::{GameElement, LevelConfig, PlayerAction};

const PRESENT_COLORS: [Color; 5] = [
    Color::rgb(0.878, 0.106, 0.141),
    Color::rgb(0.20, 0.82, 0.478),
    Color::rgb(0.208, 0.518, 0.894),
    Color::rgb(0.965, 0.827, 0.176),
    Color::rgb(0.569, 0.255, 0.675),
];

#[derive(Component, Default)]
pub struct Present {}

#[derive(Component, Default)]
pub struct CollectPresent {}

#[derive(Resource)]
pub struct SelectedMaterial {
    pub mat: Handle<StandardMaterial>,
}

pub fn init(mut commands: Commands, mut materials: ResMut<Assets<StandardMaterial>>) {
    commands.insert_resource(SelectedMaterial {
        mat: materials.add(StandardMaterial {
            base_color: Color::rgba_u8(255, 255, 255, 255),
            unlit: true,
            ..default()
        }),
    });
}

pub fn spawn(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    level_config: Res<LevelConfig>,
) {
    // Spawn a bunch of junk in 2 layers at different densities.
    // This would be nice if we drove this from some form of config.
    // Maybe some kinda level config will come.

    let mesh = meshes.add(Mesh::from(shape::Box::new(
        level_config.present_size,
        level_config.present_size,
        level_config.present_size,
    )));
    let mut present_materials = Vec::<Handle<StandardMaterial>>::new();
    for color in PRESENT_COLORS {
        let material = materials.add(StandardMaterial {
            base_color: color,
            unlit: true,
            ..default()
        });
        present_materials.push(material);
    }

    for _ in 0..level_config.low_orbit_presents {
        let coord = gen_coord(21.0..24.0);
        let material = present_materials[gen_index(present_materials.len())].clone();
        spawn_present(
            &mut commands,
            mesh.clone(),
            material,
            coord,
            level_config.present_hitbox_size,
        );
    }

    for _ in 0..level_config.high_orbit_presents {
        let coord = gen_coord(24.0..35.0);
        let material = present_materials[gen_index(present_materials.len())].clone();
        spawn_present(
            &mut commands,
            mesh.clone(),
            material,
            coord,
            level_config.present_hitbox_size,
        );
    }
}

fn spawn_present(
    commands: &mut Commands,
    mesh: Handle<Mesh>,
    material: Handle<StandardMaterial>,
    coord: Coord,
    hitbox_size: f32,
) {
    commands.spawn((
        Present::default(),
        coord,
        PbrBundle {
            mesh,
            material,
            transform: coord.to_transform(),
            ..default()
        },
        RigidBody::Fixed,
        Collider::cuboid(hitbox_size, hitbox_size, hitbox_size),
        GameElement,
    ));
}

fn gen_coord<R>(dist_range: R) -> Coord
where
    R: SampleRange<f32>,
{
    let mut r = rand::thread_rng();
    Coord {
        long: r.gen_range(-PI..PI),
        lat: r.gen_range(-PI..PI),
        dist: CoordDistance::Orbit(r.gen_range(dist_range)),
    }
}

fn gen_index(max: usize) -> usize {
    let mut r = rand::thread_rng();
    r.gen_range(0..max)
}

pub fn cast_ray(
    mut commands: Commands,
    windows: Query<&Window, With<PrimaryWindow>>,
    rapier_context: Res<RapierContext>,
    cameras: Query<(&Camera, &GlobalTransform)>,
    action_query: Query<&ActionState<PlayerAction>>,
    present_query: Query<(Entity, &Present)>,
    mut next_interaction_state: ResMut<NextState<InteractionState>>,
    interaction_state: Res<State<InteractionState>>,
    selected_material: Res<SelectedMaterial>,
) {
    let window = windows.single();

    let Some(cursor_position) = window.cursor_position() else {
        return;
    };

    let mut on_entity = false;

    for (camera, camera_transform) in &cameras {
        // Compute a ray from the mouse position.
        let Some(ray) = camera.viewport_to_world(camera_transform, cursor_position) else {
            return;
        };

        // Then cast the ray.
        let hit = rapier_context.cast_ray(
            ray.origin,
            ray.direction,
            f32::MAX,
            true,
            QueryFilter::only_fixed(),
        );

        if let Some((entity, _toi)) = hit {
            if *interaction_state == InteractionState::OnEntity
                && action_query.single().just_released(PlayerAction::CanMove)
            {
                if present_query.contains(entity) {
                    commands.entity(entity).insert(CollectPresent::default());

                    commands
                        .entity(entity)
                        .insert(selected_material.mat.clone());
                }
            }
            on_entity = true;
        }
    }

    if on_entity && action_query.single().just_pressed(PlayerAction::CanMove) {
        next_interaction_state.set(InteractionState::OnEntity);
    }

    if action_query.single().just_released(PlayerAction::CanMove) {
        next_interaction_state.set(InteractionState::Idle);
    }
}
