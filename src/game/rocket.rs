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

pub fn update_target(
    mut commands: Commands,
    time: Res<Time>,
    mut rocket_query: Query<(Entity, &mut Rocket, &mut Transform), Without<RocketTarget>>,
    present_query: Query<(Entity, &Present, &Transform), With<RocketTarget>>,
) {
    for (_, mut rocket, mut rocket_trans) in rocket_query.iter_mut() {
        if let Some(target) = rocket.target {
            let (_, _, present_trans) = present_query.get(target).unwrap();

            let dir = (present_trans.translation - rocket_trans.translation).normalize();
            let pos = rocket_trans.translation + dir * time.delta_seconds() * 10.0;

            rocket_trans.translation = pos;
            let t = rocket_trans.looking_at(present_trans.translation, Vec3::Y);
            rocket_trans.rotation = rocket_trans
                .rotation
                .slerp(t.rotation, time.delta_seconds() * 5.0);

            // Once we are close to the present despawn it
            let distance = present_trans.translation.distance(rocket_trans.translation);
            if distance < 0.1 {
                commands.entity(target).despawn_recursive();
                rocket.target = None;
            }
        }
    }
}
