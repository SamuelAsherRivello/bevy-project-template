use bevy_app::{App, Plugin, Startup, Update};

use crate::{
    input_plugin::InputPlugin,
    player_plugin::PlayerPlugin,
    ui::UiTextResource,
    ui_system::{ui_startup_system, ui_update_system},
    world_system::world_startup_system,
};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<UiTextResource>()
            .add_systems(Startup, world_startup_system)
            .add_systems(Startup, ui_startup_system)
            .add_systems(Update, ui_update_system)
            .add_plugins((InputPlugin, PlayerPlugin));
    }
}
