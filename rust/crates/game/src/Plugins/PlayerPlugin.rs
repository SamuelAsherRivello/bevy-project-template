use bevy_app::{App, Plugin, Startup, Update};

use crate::player_system::{player_startup_system, player_update_system};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, player_startup_system)
            .add_systems(Update, player_update_system);
    }
}

