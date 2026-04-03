// Disable the console window for release builds on Windows.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use bevy::prelude::*;
use bevy_game::GamePlugin;
use bevy_game::Plugins::CustomDefaultsPlugin::CustomDefaultsPlugin;
use bevy_game::Plugins::MenuPlugin::MenuPlugin;
use bevy_game::Plugins::PlayerPlugin::PlayerPlugin;

fn main() {
    App::new()
        .add_plugins((
            CustomDefaultsPlugin,
            GamePlugin,
            MenuPlugin,
            PlayerPlugin,
        ))
        .run();
}