use bevy::prelude::Component;

/// Runtime state and tuning values for the player entity.
#[derive(Component)]
pub struct PlayerComponent {
    /// Current yaw around the Y axis, stored in degrees for readable tuning.
    pub angle_degrees: f32,
    /// Turn speed applied when left or right input is held.
    pub speed_degrees_per_second: f32,
    /// Current visual pulse scale before the base mesh scale is applied.
    pub scale: f32,
    /// Speed of the idle pulse animation.
    pub pulse_speed: f32,
    /// Amount added to the base pulse scale.
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
