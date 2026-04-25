use bevy::prelude::*;

#[derive(Component)]
pub struct CameraComponent {
    pub translation: Vec3,
    pub look_at: Vec3,
}

impl Default for CameraComponent {
    fn default() -> Self {
        Self {
            translation: Vec3::new(-5.0, 4.5, 9.0),
            look_at: Vec3::new(0.0, 1.0, 0.0),
        }
    }
}
