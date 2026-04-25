use bevy::prelude::{App, World};
use bevy_ecs::system::RunSystemOnce;
use game_api::Context;

// Modules: game-owned components, resources, and systems.
#[path = "Components/CameraComponent.rs"]
mod camera_component;
#[path = "Systems/CleanupSystem.rs"]
mod cleanup_system;
#[path = "Components/FloorComponent.rs"]
mod floor_component;
#[path = "Components/GameComponent.rs"]
mod game_component;
#[path = "Resources/GameFrameResource.rs"]
mod game_frame_resource;
#[path = "Plugins/GamePlugin.rs"]
mod game_plugin;
#[path = "Components/InputComponent.rs"]
mod input_component;
#[path = "Plugins/InputPlugin.rs"]
mod input_plugin;
#[path = "Systems/InputSystem.rs"]
mod input_system;
#[path = "Components/LightComponent.rs"]
mod light_component;
#[path = "Components/PlayerComponent.rs"]
mod player_component;
#[path = "Plugins/PlayerPlugin.rs"]
mod player_plugin;
#[path = "Systems/PlayerSystem.rs"]
mod player_system;
#[path = "Resources/UIResource.rs"]
mod ui;
#[path = "Systems/UISystem.rs"]
mod ui_system;
#[path = "Components/UITextComponent.rs"]
mod ui_text_component;
#[path = "Systems/WorldSystem.rs"]
mod world_system;

use cleanup_system::game_cleanup_system;
use game_frame_resource::GameFrameResource;
use input_system::{input_startup_system, input_update_system};
use player_system::{player_startup_system, player_update_system};
use ui_system::{ui_startup_system, ui_update_system};
use world_system::world_startup_system;

// Setup: create the conventional Bevy game app.
pub fn create_app() -> App {
    let mut app = App::new();
    app.add_plugins(game_plugin::GamePlugin);
    app
}

#[unsafe(no_mangle)]
#[allow(improper_ctypes_definitions, non_snake_case)]
pub extern "C" fn AppInitialize(context: &mut dyn Context) {
    app_initialize(context);
}

pub fn app_initialize(context: &mut dyn Context) {
    context.log("Game crate initialized");
}

// Reload: rebuild game-owned world entities.
#[unsafe(no_mangle)]
#[allow(improper_ctypes_definitions, non_snake_case)]
pub extern "C" fn AppHotReload(world: &mut World, context: &mut dyn Context, reload_count: u64) {
    app_hot_reload(world, context, reload_count);
}

pub fn app_hot_reload(world: &mut World, context: &mut dyn Context, reload_count: u64) {
    context.log(&format!("Game crate hot reloaded: {reload_count}"));
    update_game_frame_resource(world, context);
    world.run_system_once(world_startup_system).unwrap();
    world.run_system_once(input_startup_system).unwrap();
    world.run_system_once(player_startup_system).unwrap();
    world.run_system_once(ui_startup_system).unwrap();
    world.run_system_once(ui_update_system).unwrap();
}

// Frame: update game systems each tick.
#[unsafe(no_mangle)]
#[allow(improper_ctypes_definitions)]
pub extern "C" fn hot_frame(world: &mut World, context: &mut dyn Context) {
    hot_frame_rust(world, context);
}

pub fn hot_frame_rust(world: &mut World, context: &mut dyn Context) {
    update_game_frame_resource(world, context);
    world.run_system_once(input_update_system).unwrap();
    world.run_system_once(player_update_system).unwrap();
    world.run_system_once(ui_update_system).unwrap();
}

// Cleanup: remove game-owned world entities.
#[unsafe(no_mangle)]
#[allow(improper_ctypes_definitions, non_snake_case)]
pub extern "C" fn AppCleanup(world: &mut World) {
    app_cleanup(world);
}

pub fn app_cleanup(world: &mut World) {
    world.run_system_once(game_cleanup_system).unwrap();
}

// Frame: mirror shell counters into game resources.
fn update_game_frame_resource(world: &mut World, context: &dyn Context) {
    world.insert_resource(GameFrameResource {
        reload_count: context.reload_count(),
        frame_global_count: context.frame_global_count(),
        frame_local_count: context.frame_local_count(),
    });
}
