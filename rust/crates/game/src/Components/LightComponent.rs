use bevy_ecs::component::Component;
use game_api::Vec3Data;

#[derive(Component)]
pub struct LightComponent {
    pub intensity: f32,
    pub translation: Vec3Data,
}

impl Default for LightComponent {
    fn default() -> Self {
        Self {
            intensity: 2_000_000.0,
            translation: Vec3Data {
                x: 4.0,
                y: 8.0,
                z: 4.0,
            },
        }
    }
}
