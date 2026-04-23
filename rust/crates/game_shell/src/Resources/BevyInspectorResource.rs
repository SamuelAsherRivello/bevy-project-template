use bevy::prelude::*;

#[derive(Resource, Reflect)]
#[reflect(Resource)]
pub struct BevyInspectorResource {
    pub enabled: bool,
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl Default for BevyInspectorResource {
    fn default() -> Self {
        Self {
            enabled: false,
            x: 20.0,
            y: 150.0,
            width: 150.0,
            height: 700.0,
        }
    }
}
