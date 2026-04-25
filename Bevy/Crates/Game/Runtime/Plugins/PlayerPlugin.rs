use bevy::prelude::{App, IntoScheduleConfigs, Plugin, Startup, Update};
use shared::input_system::input_update_system;

use crate::player_system::{player_startup_system, player_update_system};

/// Wires player spawn and per-frame player behavior into the app.
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        // Spawn after core Bevy startup begins.
        app.add_systems(Startup, player_startup_system)
            // Read input only after the input system has refreshed it this frame.
            .add_systems(Update, player_update_system.after(input_update_system));
    }
}
