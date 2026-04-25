use bevy::prelude::*;

// Modules: game-owned components, resources, and systems.
#[path = "Components/CameraComponent.rs"]
mod camera_component;
#[path = "Resources/ContextResource.rs"]
mod context_resource;
#[path = "Systems/ContextSystem.rs"]
mod context_system;
#[path = "Components/FloorComponent.rs"]
mod floor_component;
#[path = "Components/GameComponent.rs"]
mod game_component;
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
#[path = "Plugins/WorldPlugin.rs"]
mod world_plugin;
#[path = "Systems/WorldSystem.rs"]
mod world_system;

// Setup: create the conventional Bevy game app.
pub fn create_app() -> App {
    let mut app = App::new();
    app.add_plugins((
        DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Bevy Project Template".to_owned(),
                resolution: (800.0, 600.0).into(),
                ..Default::default()
            }),
            ..Default::default()
        }),
        game_plugin::GamePlugin,
    ));
    app
}
