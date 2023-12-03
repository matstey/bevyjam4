use std::f32::consts::PI;

use bevy::prelude::*;
use rand::{distributions::uniform::SampleRange, Rng};

use crate::{coord::CoordDistance, Coord};

#[derive(Component, Default)]
pub struct Present {}

pub fn spawn(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Spawn a bunch of junk in 2 layers at different densities.
    // This would be nice if we drove this from some form of config.
    // Maybe some kinda level config will come.

    let mesh = meshes.add(Mesh::from(shape::Box::new(0.1, 0.1, 0.1)));
    let material = materials.add(StandardMaterial {
        base_color: Color::GRAY,
        metallic: 1.0,
        perceptual_roughness: 0.0,
        ..default()
    });

    for _ in 0..5000 {
        let coord = gen_coord(21.0..24.0);
        spawn_present(&mut commands, mesh.clone(), material.clone(), coord);
    }

    for _ in 0..1000 {
        let coord = gen_coord(24.0..35.0);
        spawn_present(&mut commands, mesh.clone(), material.clone(), coord);
    }
}

fn spawn_present(
    commands: &mut Commands,
    mesh: Handle<Mesh>,
    material: Handle<StandardMaterial>,
    coord: Coord,
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
