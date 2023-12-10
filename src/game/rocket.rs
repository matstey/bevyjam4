use bevy::prelude::*;

use super::present::{CollectPresent, Present};

#[derive(Component, Default)]
pub struct Rocket {
    pub target: Option<Entity>,
}

#[derive(Component, Default)]
pub struct RocketTarget {}

pub fn collect_presents(
    mut commands: Commands,
    mut rocket_query: Query<(Entity, &mut Rocket)>,
    present_query: Query<(Entity, &CollectPresent, &Present), Without<RocketTarget>>,
) {
    for (present, _, _) in present_query.iter() {
        for (_, mut rocket) in rocket_query.iter_mut() {
            if rocket.target.is_none() {
                rocket.target = Some(present);
                commands.entity(present).insert(RocketTarget::default());
                break;
            }
        }
    }
}

pub fn update_target(mut commands: Commands, mut rocket_query: Query<(Entity, &mut Rocket)>) {
    let mut despawn = Vec::<Entity>::new();
    for (_, mut rocket) in rocket_query.iter_mut() {
        if let Some(target) = rocket.target {
            despawn.push(target);
            rocket.target = None;
        }
    }

    for entity in despawn {
        commands.entity(entity).despawn_recursive();
    }
}
