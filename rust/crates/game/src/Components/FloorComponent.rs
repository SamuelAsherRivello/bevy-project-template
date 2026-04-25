use bevy::prelude::*;

#[derive(Component)]
pub struct FloorComponent {
    pub color: Color,
    pub translation: Vec3,
    pub scale: Vec3,
}

impl Default for FloorComponent {
    fn default() -> Self {
        Self {
            color: Color::srgba(0.18, 0.22, 0.28, 1.0),
            translation: Vec3::new(0.0, -1.0, 0.0),
            scale: Vec3::new(4.5, 0.25, 4.5),
        }
    }
}
