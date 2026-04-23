use bevy_app::{App, Plugin, Startup};

use crate::input_system::input_startup_system;

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, input_startup_system);
    }
}

