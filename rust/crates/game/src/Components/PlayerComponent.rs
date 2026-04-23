use bevy_ecs::component::Component;

#[derive(Component)]
pub struct PlayerComponent {
    pub angle_degrees: f32,
    pub speed_degrees_per_second: f32,
    pub scale: f32,
    pub pulse_speed: f32,
    pub pulse_amount: f32,
}

impl Default for PlayerComponent {
    fn default() -> Self {
        Self {
            angle_degrees: 0.0,
            speed_degrees_per_second: 180.0,
            scale: 1.0,
            pulse_speed: 1.7,
            pulse_amount: 0.35,
        }
    }
}
