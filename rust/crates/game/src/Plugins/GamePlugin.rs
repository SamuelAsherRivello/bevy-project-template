use bevy_app::{App, Plugin, Startup, Update};

use crate::{
    context_resource::ContextResource,
    context_system::context_update_system,
    input_plugin::InputPlugin,
    player_plugin::PlayerPlugin,
    ui::UiTextResource,
    ui_system::{ui_startup_system, ui_update_system},
    world_plugin::WorldPlugin,
};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ContextResource>()
            .init_resource::<UiTextResource>()
            .add_systems(Startup, ui_startup_system)
            .add_systems(Update, (context_update_system, ui_update_system))
            .add_plugins((WorldPlugin, InputPlugin, PlayerPlugin));
    }
}
