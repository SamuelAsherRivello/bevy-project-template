use std::{env, error::Error};

use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::prelude::*;
use bevy_inspector_egui::{DefaultInspectorConfigPlugin, bevy_egui::EguiPlugin};
#[cfg(not(target_arch = "wasm32"))]
use std::time::Instant;
#[cfg(target_arch = "wasm32")]
use web_time::Instant;

#[path = "Components/BevyInspectorComponent.rs"]
mod bevy_inspector_component;
#[path = "Resources/BevyInspectorResource.rs"]
mod bevy_inspector_resource;
#[path = "Systems/GameRuntimeSystem.rs"]
mod game_runtime_system;
#[path = "Systems/PlatformSystem.rs"]
mod platform_system;
#[path = "Resources/ShellContextResource.rs"]
mod shell_context_resource;
#[path = "Plugins/ShellPlugin.rs"]
mod shell_plugin;
#[path = "Resources/ShellRuntimeResource.rs"]
mod shell_runtime_resource;

use shell_context_resource::ShellContextResource;
use shell_plugin::ShellPlugin;
use shell_runtime_resource::ShellRuntimeResource;

fn main() {
    if let Err(error) = run() {
        eprintln!("failed to start game shell: {error}");
        std::process::exit(1);
    }
}

fn run() -> Result<(), Box<dyn Error + Send + Sync>> {
    let hot_reload_enabled = env::args().any(|arg| arg == "--hot-reload");
    let initial_window_position = platform_system::load_saved_window_position();

    platform_system::configure_runtime();
    platform_system::ensure_game_ready();

    let reload_count = 1;
    let mut context = ShellContextResource {
        reload_count,
        frame_global_count: 0,
        frame_local_count: 0,
    };

    #[cfg(not(target_arch = "wasm32"))]
    let game_source = platform_system::GameSourceWatcher::new()?;

    if hot_reload_enabled && platform_system::supports_hot_reload() {
        println!("hot reload enabled");
    } else if hot_reload_enabled {
        println!("browser live reload enabled via dev server");
    } else {
        println!("hot reload disabled");
    }

    let mut app = App::new();
    app.add_plugins((
        DefaultPlugins.set(WindowPlugin {
            primary_window: Some(platform_system::primary_window(initial_window_position)),
            ..Default::default()
        }),
        FrameTimeDiagnosticsPlugin::default(),
        EguiPlugin,
        DefaultInspectorConfigPlugin,
        ShellPlugin,
    ));

    app.finish();
    app.cleanup();

    let game =
        platform_system::GameRuntime::load(app.world_mut(), &mut context, true, reload_count)?;

    app.insert_non_send_resource(ShellRuntimeResource {
        context,
        game,
        reload_count,
        #[cfg(not(target_arch = "wasm32"))]
        game_source,
        #[cfg(not(target_arch = "wasm32"))]
        hot_reload_enabled,
        last_game_frame: Instant::now(),
        #[cfg(not(target_arch = "wasm32"))]
        last_window_position: initial_window_position,
    })
    .run();

    Ok(())
}
