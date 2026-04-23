use bevy_ecs::component::Component;

#[derive(Component, Default)]
pub struct InputComponent {
    pub delta_seconds: f32,
    pub elapsed_seconds: f32,
    pub turn_input: f32,
    pub left_pressed: bool,
    pub right_pressed: bool,
}

