use bevy::prelude::*;

use crate::{
    bevy_inspector_component::BevyInspectorComponent,
    bevy_inspector_resource::BevyInspectorResource,
    game_loop_system::{
        shell_apply_render_packet_update_system,
        shell_drive_game_update_system,
        shell_update_overlay_text_update_system,
    },
    input_system::{
        shell_bevy_inspector_ui_update_system,
        shell_capture_input_update_system,
        shell_toggle_bevy_inspector_update_system,
    },
    platform_system,
    scene_setup_system::shell_startup_system,
};

pub struct ShellPlugin;

impl Plugin for ShellPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<BevyInspectorComponent>()
            .register_type::<BevyInspectorResource>()
            .insert_resource(BevyInspectorResource::default())
            .add_systems(Startup, shell_startup_system)
            .add_systems(
                Update,
                (
                    shell_capture_input_update_system,
                    shell_toggle_bevy_inspector_update_system,
                    shell_bevy_inspector_ui_update_system,
                    shell_drive_game_update_system,
                    shell_apply_render_packet_update_system,
                    shell_update_overlay_text_update_system,
                    platform_system::track_window_position_update_system,
                    platform_system::persist_window_position_on_close_update_system,
                ),
            );
    }
}
