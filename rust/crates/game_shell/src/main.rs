use std::{env, error::Error};

use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::prelude::*;
use bevy_inspector_egui::{DefaultInspectorConfigPlugin, bevy_egui::EguiPlugin};
use game_api::RenderPacket;
#[cfg(not(target_arch = "wasm32"))]
use std::time::Instant;
#[cfg(target_arch = "wasm32")]
use web_time::Instant;

#[path = "Components/BevyInspectorComponent.rs"]
mod bevy_inspector_component;
#[path = "Components/DemoOverlayTextComponent.rs"]
mod demo_overlay_text_component;
#[path = "Components/DemoRenderItemComponent.rs"]
mod demo_render_item_component;
#[path = "Plugins/ShellPlugin.rs"]
mod shell_plugin;
#[path = "Resources/BevyInspectorResource.rs"]
mod bevy_inspector_resource;
#[path = "Resources/FrameResource.rs"]
mod frame_resource;
#[path = "Resources/HostStateResource.rs"]
mod host_state_resource;
#[path = "Resources/MetricsResource.rs"]
mod metrics_resource;
#[path = "Resources/ShellContextResource.rs"]
mod shell_context_resource;
#[path = "Systems/GameLoopSystem.rs"]
mod game_loop_system;
#[path = "Systems/InputSystem.rs"]
mod input_system;
#[path = "Systems/PlatformSystem.rs"]
mod platform_system;
#[path = "Systems/SceneSetupSystem.rs"]
mod scene_setup_system;

use frame_resource::FrameResource;
use host_state_resource::HostStateResource;
use metrics_resource::MetricsResource;
use shell_context_resource::ShellContextResource;
use shell_plugin::ShellPlugin;

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
    platform_system::debug_log("run(): runtime configured");
    platform_system::ensure_game_ready();
    platform_system::debug_log("run(): game ready check complete");

    let mut context = ShellContextResource {
        frame: 0,
        left_pressed: false,
        right_pressed: false,
        delta_seconds: 0.0,
        turn_input: 0.0,
        elapsed_seconds: 0.0,
    };
    platform_system::debug_log("run(): loading game runtime");
    let reload_count = 1;
    let game = platform_system::GameRuntime::load(&mut context, true, reload_count)?;
    platform_system::debug_log("run(): game runtime loaded");

    #[cfg(not(target_arch = "wasm32"))]
    let game_source = platform_system::GameSourceWatcher::new()?;

    if hot_reload_enabled && platform_system::supports_hot_reload() {
        println!("hot reload enabled");
    } else if hot_reload_enabled {
        println!("browser live reload enabled via dev server");
    } else {
        println!("hot reload disabled");
    }
    platform_system::debug_log("run(): bevy app setup starting");

    let host_state = HostStateResource {
        context,
        frame_resource: FrameResource::default(),
        metrics_resource: MetricsResource::default(),
        game,
        reload_count,
        #[cfg(not(target_arch = "wasm32"))]
        game_source,
        #[cfg(not(target_arch = "wasm32"))]
        hot_reload_enabled,
        last_game_frame: Instant::now(),
        latest_render_packet: RenderPacket::default(),
        #[cfg(not(target_arch = "wasm32"))]
        last_window_position: initial_window_position,
    };

    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(platform_system::primary_window(initial_window_position)),
                ..Default::default()
            }),
            FrameTimeDiagnosticsPlugin::default(),
            EguiPlugin,
            DefaultInspectorConfigPlugin,
            ShellPlugin,
        ))
        .insert_non_send_resource(host_state)
        .run();

    Ok(())
}
