use bevy::prelude::Resource;

#[derive(Default, Resource)]
pub struct HUDTextResource {
    pub text: String,
}
