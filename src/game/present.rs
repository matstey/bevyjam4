use std::f32::consts::PI;

use bevy::prelude::*;
use rand::{distributions::uniform::SampleRange, Rng};

use crate::{coord::CoordDistance, Coord};

const PRESENT_COLORS: [Color; 5] = [
    Color::rgb(0.878, 0.106, 0.141),
    Color::rgb(0.20, 0.82, 0.478),
    Color::rgb(0.208, 0.518, 0.894),
    Color::rgb(0.965, 0.827, 0.176),
    Color::rgb(0.569, 0.255, 0.675),
];

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
    let mut present_materials = Vec::<Handle<StandardMaterial>>::new();
    for color in PRESENT_COLORS {
        let material = materials.add(StandardMaterial {
            base_color: color,
            metallic: 1.0,
            perceptual_roughness: 0.0,
            ..default()
        });
        present_materials.push(material);
    }

    for _ in 0..5000 {
        let coord = gen_coord(21.0..24.0);
        let material = present_materials[gen_index(present_materials.len())].clone();
        spawn_present(&mut commands, mesh.clone(), material, coord);
    }

    for _ in 0..1000 {
        let coord = gen_coord(24.0..35.0);
        let material = present_materials[gen_index(present_materials.len())].clone();
        spawn_present(&mut commands, mesh.clone(), material, coord);
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

fn gen_index(max: usize) -> usize {
    let mut r = rand::thread_rng();
    r.gen_range(0..max)
}
