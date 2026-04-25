use bevy_ecs::component::Component;
use game_api::{ColorRgba, Vec3Data};

#[derive(Component)]
pub struct FloorComponent {
    pub color: ColorRgba,
    pub translation: Vec3Data,
    pub scale: Vec3Data,
}

impl Default for FloorComponent {
    fn default() -> Self {
        Self {
            color: ColorRgba {
                red: 0.18,
                green: 0.22,
                blue: 0.28,
                alpha: 1.0,
            },
            translation: Vec3Data {
                x: 0.0,
                y: -1.0,
                z: 0.0,
            },
            scale: Vec3Data {
                x: 4.5,
                y: 0.25,
                z: 4.5,
            },
        }
    }
}
