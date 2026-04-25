use bevy::{prelude::*, window::WindowPosition};
use bevy_simple_subsecond_system as hot_reload;
use hot_reload::prelude::SimpleSubsecondPlugin;
use shared::{
    bevy_inspector_plugin, context_plugin, custom_window_plugin, custom_window_resource,
    custom_window_system, input_plugin,
};

#[cfg(test)]
#[path = "../Tests/PlayerTests.rs"]
mod player_tests;

// Modules: game-owned components, resources, and systems.
#[path = "Components/PlayerComponent.rs"]
pub(crate) mod player_component;
#[path = "Plugins/PlayerPlugin.rs"]
pub(crate) mod player_plugin;
#[path = "Systems/PlayerSystem.rs"]
pub(crate) mod player_system;
#[path = "Resources/HUDResource.rs"]
pub(crate) mod hud_resource;
#[path = "Plugins/HUDPlugin.rs"]
pub(crate) mod hud_plugin;
#[path = "Systems/HUDSystem.rs"]
pub(crate) mod hud_system;
#[path = "Components/HUDTextComponent.rs"]
pub(crate) mod hud_text_component;
#[path = "Plugins/WorldPlugin.rs"]
pub(crate) mod world_plugin;
#[path = "Systems/WorldSystem.rs"]
pub(crate) mod world_system;
fn main() {
    #[cfg(not(target_arch = "wasm32"))]
    {
        if std::env::var_os("WGPU_BACKEND").is_none() {
            unsafe {
                std::env::set_var("WGPU_BACKEND", "dx12");
            }
        }
    }

    let mut app = App::new();
    let initial_primary_window_position =
        custom_window_system::load_custom_window_position().map(WindowPosition::At);

    // Bevy engine defaults.
    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: "Bevy Project Template".to_owned(),
            resolution: (
                custom_window_resource::TARGET_RESOLUTION.x,
                custom_window_resource::TARGET_RESOLUTION.y,
            )
                .into(),
            position: initial_primary_window_position.unwrap_or_default(),
            #[cfg(target_arch = "wasm32")]
            fit_canvas_to_parent: true,
            ..Default::default()
        }),
        ..Default::default()
    })
    .set(AssetPlugin {
        file_path: "Bevy/Crates/Game/Assets".to_owned(),
        ..Default::default()
    }));

    // Subsecond hot reload.
    app.add_plugins(SimpleSubsecondPlugin::default());

    // Context Plugin: Contains frame and hot-reload state.
    app.add_plugins(context_plugin::ContextPlugin);

    // HUD Plugin: Contains on-screen status text.
    app.add_plugins(hud_plugin::HUDPlugin);

    // World Plugin: Contains camera, lights, floor, and world setup.
    app.add_plugins(world_plugin::WorldPlugin);

    // Custom Window Plugin: Contains primary window position persistence.
    app.add_plugins(custom_window_plugin::CustomWindowPlugin);

    // Input Plugin: Contains keyboard input state and updates.
    app.add_plugins(input_plugin::InputPlugin);

    // Player Plugin: Contains player spawn and movement updates.
    app.add_plugins(player_plugin::PlayerPlugin);

    // Bevy Inspector Plugin: Contains toggleable world inspection tools.
    app.add_plugins(bevy_inspector_plugin::BevyInspectorPlugin);

    app.run();
}
