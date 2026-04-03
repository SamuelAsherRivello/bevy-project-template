use crate::GameState;
use crate::Plugins::AudioPlugin::InternalAudioPlugin;
use crate::Plugins::InspectorPlugin::InspectorPlugin;
use crate::Plugins::HudPlugin::HudPlugin;
use crate::Plugins::InputPlugin::ActionsPlugin;
use crate::Plugins::LoadingPlugin::LoadingPlugin;

use bevy::app::App;
#[cfg(debug_assertions)]
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;

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
                InspectorPlugin,
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
