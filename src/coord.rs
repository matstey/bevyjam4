use bevy::prelude::*;

#[derive(Default, Copy, Clone)]
pub enum CoordDistance {
    #[default]
    Ground,
    Orbit(f32),
}

#[derive(Component, Default, Copy, Clone)]
pub struct Coord {
    pub long: f32,           // Rotation in radians around the global Y axis (longitudinal)
    pub lat: f32,            // Rotation in radians around the local X axis (latitudinal)
    pub dist: CoordDistance, // Distance from the center of the earth
}

impl Coord {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    pub fn from_dist(dist: f32) -> Self {
        Self {
            dist: CoordDistance::Orbit(dist),
            ..Default::default()
        }
    }

    pub fn from_degrees(pos: Vec2) -> Self {
        Self {
            lat: pos.x.to_radians(),
            long: pos.y.to_radians(),
            ..Default::default()
        }
    }

    pub fn apply(&self, transform: &mut Transform) {
        transform.rotation = self.get_rotation();
        transform.translation = self.get_translation(transform.rotation);
    }

    pub fn get_translation(&self, rotation: Quat) -> Vec3 {
        let focus = Vec3::ZERO; // Fuck it let just hardcode the location of the center of the earth
        focus + rotation * Vec3::new(0.0, 0.0, self.get_distance())
    }

    pub fn get_rotation(&self) -> Quat {
        Quat::from_rotation_y(self.long) * Quat::from_rotation_x(-self.lat)
    }

    pub fn to_transform(&self) -> Transform {
        let rotation = self.get_rotation();
        Transform::from_translation(self.get_translation(rotation)).with_rotation(rotation)
    }

    pub fn get_distance(&self) -> f32 {
        match self.dist {
            CoordDistance::Ground => 20.0,
            CoordDistance::Orbit(dist) => dist,
        }
    }
}
