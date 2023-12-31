use bevy::prelude::*;

use crate::{asset::LoadingAssets, Coord};

use super::{rocket::Rocket, GameElement};

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

#[derive(Resource)]
pub struct StationResources {
    pub pad: Handle<Scene>,
    pub rocket: Handle<Scene>,
}

pub fn load_resources(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut loading: ResMut<LoadingAssets>,
) {
    let pad = asset_server.load("launch_pad.glb#Scene0");
    let rocket = asset_server.load("rocket.glb#Scene0");
    loading.0.push(pad.id().into());
    loading.0.push(rocket.id().into());
    commands.insert_resource(StationResources { pad, rocket });
}

pub fn spawn(mut commands: Commands, scene_res: Res<StationResources>) {
    for station in STATIONS {
        spawn_station(&mut commands, station, &scene_res);
    }
}

fn spawn_station(commands: &mut Commands, station: Info, scene_res: &Res<StationResources>) {
    let coord = Coord::from_degrees(station.coord);

    let rocket_scene = SceneBundle {
        scene: scene_res.rocket.clone_weak(),
        transform: Transform::from_scale(Vec3::new(0.2, 0.2, 0.2)),
        ..default()
    };
    let rocket_entity = commands.spawn(rocket_scene).id();
    commands
        .spawn((
            Rocket::default(),
            TransformBundle {
                local: coord.to_transform()
                    * Transform::from_rotation(Quat::from_rotation_x(180f32.to_radians()))
                    * Transform::from_translation(Vec3::new(0.0, 0.0, -0.5)),
                global: GlobalTransform::IDENTITY,
            },
            Visibility::Visible,
            InheritedVisibility::VISIBLE,
            GameElement,
        ))
        .push_children(&[rocket_entity]);

    let pad_scene = SceneBundle {
        scene: scene_res.pad.clone_weak(),
        transform: Transform::from_rotation(Quat::from_rotation_x(90f32.to_radians()))
            .with_scale(Vec3::new(0.2, 0.2, 0.2)),
        ..default()
    };
    let pad_entity = commands.spawn((pad_scene, GameElement)).id();

    commands
        .spawn((
            GroundStation::default(),
            coord,
            TransformBundle {
                local: coord.to_transform(),
                global: GlobalTransform::IDENTITY,
            },
            Visibility::Visible,
            InheritedVisibility::VISIBLE,
            GameElement,
        ))
        .push_children(&[pad_entity]);
}
