use bevy::prelude::*;

use crate::Coord;

#[derive(Clone, Default)]
pub struct Info {
    pub coord: Vec2,
    pub name: &'static str,
    pub country: Country,
}

#[derive(Clone, Default)]
pub enum Country {
    Australia,
    Canada,
    China,
    FrenchGuiana,
    Japan,
    Usa,
    Uk,
    Iran,
    Russia,
    Norway,
    #[default]
    Unknown,
}

// Pulled from https://en.wikipedia.org/wiki/List_of_rocket_launch_sites
// South and West are negative
#[allow(clippy::excessive_precision)]
const STATIONS: [Info; 10] = [
    Info {
        coord: Vec2::new(-30.95875, 136.50366),
        name: "Woomera Test Range",
        country: Country::Australia,
    },
    Info {
        coord: Vec2::new(28.607856, -80.604208),
        name: "Cape Canaveral",
        country: Country::Usa,
    },
    Info {
        coord: Vec2::new(51.20706, 59.85003),
        name: "Yasny Cosmodrome",
        country: Country::Russia,
    },
    Info {
        coord: Vec2::new(57.33000, -7.33000),
        name: "South Uist",
        country: Country::Uk,
    },
    Info {
        coord: Vec2::new(45.30688, -60.98767),
        name: "Maritime Spaceport",
        country: Country::Canada,
    },
    Info {
        coord: Vec2::new(69.29430, 16.02070),
        name: "Andøya Space Center",
        country: Country::Norway,
    },
    Info {
        coord: Vec2::new(40.96056, 100.29833),
        name: "Jiuquan Satellite Launch Center",
        country: Country::China,
    },
    Info {
        coord: Vec2::new(5.23739, -52.76950),
        name: "Guiana Space Centre",
        country: Country::FrenchGuiana,
    },
    Info {
        coord: Vec2::new(31.25186, 131.07914),
        name: "Uchinoura Space Center",
        country: Country::Japan,
    },
    Info {
        coord: Vec2::new(35.234631, 53.920941),
        name: "Semnan spaceport",
        country: Country::Iran,
    },
];

#[derive(Component, Default)]
pub struct GroundStation {}

pub fn spawn(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for station in STATIONS {
        spawn_station(&mut commands, &mut meshes, &mut materials, station);
    }
}

fn spawn_station(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    station: Info,
) {
    let visual_entity = commands
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Capsule::default())),
            material: materials.add(StandardMaterial {
                base_color: Color::RED,
                ..default()
            }),
            transform: Transform::from_rotation(Quat::from_rotation_x(90f32.to_radians())),
            ..default()
        })
        .id();

    let coord = Coord::from_degrees(station.coord);
    commands
        .spawn((
            GroundStation::default(),
            coord,
            coord.to_transform(),
            Visibility::Visible,
            InheritedVisibility::VISIBLE,
            GlobalTransform::IDENTITY,
        ))
        .add_child(visual_entity);
}
