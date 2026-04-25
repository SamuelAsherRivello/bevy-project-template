use bevy_ecs::prelude::Resource;

#[derive(Default, Resource)]
pub struct UiTextResource {
    pub text: String,
}
