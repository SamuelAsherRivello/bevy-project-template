use bevy::asset::AssetMetaCheck;
use bevy::prelude::*;
use bevy::window::WindowPosition;
#[cfg(not(target_arch = "wasm32"))]
use bevy::window::MonitorSelection;

const WINDOW_TITLE: &str = "Bevy game";
const WINDOW_WIDTH: u32 = 800;
const WINDOW_HEIGHT: u32 = 600;
const BACKGROUND_COLOR: Color = Color::linear_rgb(0.4, 0.4, 0.4);

/// Registers the project's customized Bevy default plugin stack.
pub struct CustomDefaultsPlugin;

impl Plugin for CustomDefaultsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ClearColor(BACKGROUND_COLOR)).add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: WINDOW_TITLE.to_string(),
                        resolution: (WINDOW_WIDTH, WINDOW_HEIGHT).into(),
                        position: {
                            #[cfg(target_arch = "wasm32")]
                            {
                                WindowPosition::Automatic
                            }
                            #[cfg(not(target_arch = "wasm32"))]
                            {
                                WindowPosition::Centered(MonitorSelection::Current)
                            }
                        },
                        canvas: Some("#bevy".to_owned()),
                        fit_canvas_to_parent: true,
                        prevent_default_event_handling: false,
                        ..default()
                    }),
                    ..default()
                })
                .set(AssetPlugin {
                    meta_check: AssetMetaCheck::Never,
                    ..default()
                }),
        );
    }
}