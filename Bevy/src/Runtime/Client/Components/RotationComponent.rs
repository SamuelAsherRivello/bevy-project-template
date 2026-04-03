use bevy::prelude::*;

/// Per-entity angular speed in radians per second.
#[derive(Component, Reflect, Debug, Clone, Copy, PartialEq)]
#[reflect(Component)]
pub struct Rotation {
    pub radians_per_second: f32,
}

impl Default for Rotation {
    fn default() -> Self {
        Self {
            radians_per_second: 0.1,
        }
    }
}