// Disable the console window for release builds on Windows.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use bevy::DefaultPlugins;
use bevy::asset::AssetMetaCheck;
#[cfg(not(target_arch = "wasm32"))]
use bevy::ecs::system::NonSendMarker;
use bevy::prelude::*;
use bevy::window::WindowPosition;
#[cfg(not(target_arch = "wasm32"))]
use bevy::window::{MonitorSelection, PrimaryWindow};
#[cfg(not(target_arch = "wasm32"))]
use bevy::winit::WINIT_WINDOWS;
use bevy_game::GamePlugin; // ToDo: Replace bevy_game with your new crate name.
#[cfg(not(target_arch = "wasm32"))]
use std::io::Cursor;
#[cfg(not(target_arch = "wasm32"))]
use winit::dpi::PhysicalPosition;
#[cfg(not(target_arch = "wasm32"))]
use winit::window::Icon;

const BACKGROUND_COLOR: Color = Color::linear_rgb(0.4, 0.4, 0.4);
const WINDOW_TITLE: &str = "Bevy game";
const WINDOW_WIDTH: u32 = 800;
const WINDOW_HEIGHT: u32 = 600;
#[cfg(not(target_arch = "wasm32"))]
const WINDOW_X_OFFSET: i32 = 100;

fn main() {
    let mut app = App::new();

    app.insert_resource(ClearColor(BACKGROUND_COLOR))
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: WINDOW_TITLE.to_string(), // ToDo
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
                        // Bind to canvas included in `documentation/index.html`
                        canvas: Some("#bevy".to_owned()),
                        fit_canvas_to_parent: true,
                        // Tells wasm not to override default event handling, like F5 and Ctrl+R
                        prevent_default_event_handling: false,
                        ..default()
                    }),
                    ..default()
                })
                .set(AssetPlugin {
                    meta_check: AssetMetaCheck::Never,
                    ..default()
                }),
        )
        .add_plugins(GamePlugin);

    // Desktop only: winit window icon APIs are not used on Web.
    #[cfg(not(target_arch = "wasm32"))]
    app.add_systems(Startup, set_window_icon);

    app.run();
}

/// Sets the icon on Windows and X11, then nudges the window near the monitor center.
#[cfg(not(target_arch = "wasm32"))]
fn set_window_icon(
    primary_window: Single<Entity, With<PrimaryWindow>>,
    _non_send_marker: NonSendMarker,
) -> Result {
    WINIT_WINDOWS.with_borrow(|windows| {
        let Some(primary) = windows.get_window(*primary_window) else {
            return Err(BevyError::from("No primary window!"));
        };
        let icon_buf = Cursor::new(include_bytes!(
            "../../../../build/macos/AppIcon.iconset/icon_256x256.png"
        ));
        if let Ok(image) = image::load(icon_buf, image::ImageFormat::Png) {
            let image = image.into_rgba8();
            let (width, height) = image.dimensions();
            let rgba = image.into_raw();
            let icon = Icon::from_rgba(rgba, width, height).unwrap();
            primary.set_window_icon(Some(icon));
        }

        if let Some(monitor) = primary.current_monitor() {
            let monitor_size = monitor.size();
            let monitor_pos = monitor.position();
            let window_size = primary.outer_size();
            let centered_x =
                monitor_pos.x + ((monitor_size.width as i32 - window_size.width as i32) / 2);
            let centered_y =
                monitor_pos.y + ((monitor_size.height as i32 - window_size.height as i32) / 2);
            primary.set_outer_position(PhysicalPosition::new(
                centered_x + WINDOW_X_OFFSET,
                centered_y,
            ));
        }

        Ok(())
    })
}
