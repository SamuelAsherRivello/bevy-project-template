use bevy_app::{App, Plugin, Startup};

use crate::world_system::world_startup_system;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, world_startup_system);
    }
}
