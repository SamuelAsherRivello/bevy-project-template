use bevy_ecs::component::Component;

#[derive(Component, Default)]
pub struct InputComponent {
    pub is_left_arrow_pressed: bool,
    pub is_right_arrow_pressed: bool,
}
