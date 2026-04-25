use bevy::prelude::*;

use crate::{
    bevy_inspector_component::BevyInspectorComponent,
    bevy_inspector_resource::BevyInspectorResource,
    game_runtime_system::shell_drive_game_update_system, platform_system,
};

pub struct ShellPlugin;

impl Plugin for ShellPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<BevyInspectorComponent>()
            .register_type::<BevyInspectorResource>()
            .insert_resource(BevyInspectorResource::default())
            .add_systems(
                Update,
                (
                    shell_drive_game_update_system,
                    platform_system::track_window_position_update_system,
                    platform_system::persist_window_position_on_close_update_system,
                ),
            );
    }
}
