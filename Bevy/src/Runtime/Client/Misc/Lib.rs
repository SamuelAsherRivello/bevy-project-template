#![allow(clippy::type_complexity, non_snake_case)]

#[path = "../Components/Mod.rs"]
pub mod Components;
#[path = "Mod.rs"]
pub mod Misc;
#[path = "../Resources/Mod.rs"]
pub mod Resources;
#[path = "../../Server/Mod.rs"]
pub mod Server;
#[path = "../../Shared/Mod.rs"]
pub mod Shared;
#[path = "../Systems/Mod.rs"]
pub mod Systems;
#[cfg(test)]
#[path = "../../../Tests/Mod.rs"]
pub mod Tests;

use crate::Systems::AudioSystem::InternalAudioPlugin;
use crate::Systems::InputSystem::ActionsPlugin;
use crate::Systems::LoadingSystem::LoadingPlugin;
use crate::Systems::PlayerSystem::PlayerPlugin;
use crate::Systems::RotationSystem::RotationPlugin;
use crate::Misc::Hud::HudPlugin;

use bevy::app::App;
#[cfg(debug_assertions)]
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;

/// High-level application states for the template.
#[derive(States, Default, Clone, Eq, PartialEq, Debug, Hash)]
pub enum GameState {
    #[default]
    Loading,
    Playing,
}

/// Registers the core feature plugins used by the template.
pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>()
            .add_systems(Startup, spawn_camera)
            .add_plugins((
                LoadingPlugin,
                HudPlugin,
                ActionsPlugin,
                InternalAudioPlugin,
                PlayerPlugin,
                RotationPlugin,
            ));

        #[cfg(debug_assertions)]
        {
            app.add_plugins((
                FrameTimeDiagnosticsPlugin::default(),
                LogDiagnosticsPlugin::default(),
            ));
        }
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn((Camera2d, Msaa::Off));
}
