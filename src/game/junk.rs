use std::f32::consts::PI;

use bevy::prelude::*;
use rand::{distributions::uniform::SampleRange, Rng};

use crate::Coord;

#[derive(Component, Default)]
pub struct Junk {}

pub fn spawn(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Spawn a bunch of junk in 2 layers at different densities.
    // This would be nice if we drove this from some form of config.
    // Maybe some kinda level config will come.

    for _ in 0..5000 {
        let coord = gen_coord(21.0..24.0);
        spawn_junk(&mut commands, &mut meshes, &mut materials, coord);
    }

    for _ in 0..1000 {
        let coord = gen_coord(24.0..35.0);
        spawn_junk(&mut commands, &mut meshes, &mut materials, coord);
    }
}

fn spawn_junk(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    coord: Coord,
) {
    commands.spawn((
        Junk::default(),
        coord.clone(),
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Box::new(0.1, 0.1, 0.1))),
            material: materials.add(StandardMaterial {
                base_color: Color::GRAY,
                ..default()
            }),
            transform: coord.to_transform(),
            ..default()
        },
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
        dist: r.gen_range(dist_range),
    }
}
