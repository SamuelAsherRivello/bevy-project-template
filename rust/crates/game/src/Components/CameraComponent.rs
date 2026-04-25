use bevy_ecs::component::Component;
use game_api::Vec3Data;

#[derive(Component)]
pub struct CameraComponent {
    pub translation: Vec3Data,
    pub look_at: Vec3Data,
}

impl Default for CameraComponent {
    fn default() -> Self {
        Self {
            translation: Vec3Data {
                x: -5.0,
                y: 4.5,
                z: 9.0,
            },
            look_at: Vec3Data {
                x: 0.0,
                y: 1.0,
                z: 0.0,
            },
        }
    }
}
