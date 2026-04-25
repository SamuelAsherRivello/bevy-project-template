use bevy::prelude::IntoSystemConfigs;
use bevy_app::{App, Plugin, Startup, Update};

use crate::{
    input_system::input_update_system,
    player_system::{player_startup_system, player_update_system},
};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, player_startup_system)
            .add_systems(Update, player_update_system.after(input_update_system));
    }
}
