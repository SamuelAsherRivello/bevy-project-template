use bevy_ecs::prelude::Resource;

#[derive(Resource, Default)]
pub struct FrameInputResource {
    pub delta_seconds: f32,
    pub elapsed_seconds: f32,
    pub turn_input: f32,
    pub left_pressed: bool,
    pub right_pressed: bool,
}

