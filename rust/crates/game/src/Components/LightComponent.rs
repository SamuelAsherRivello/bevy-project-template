use bevy::prelude::*;

#[derive(Component)]
pub struct LightComponent {
    pub intensity: f32,
    pub translation: Vec3,
}

impl Default for LightComponent {
    fn default() -> Self {
        Self {
            intensity: 2_000_000.0,
            translation: Vec3::new(4.0, 8.0, 4.0),
        }
    }
}
