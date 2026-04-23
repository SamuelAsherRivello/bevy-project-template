use bevy_app::{App, Plugin};

use crate::{
    input_plugin::InputPlugin, player_plugin::PlayerPlugin, ui::UiTextResource,
};

pub struct GamePlugin {
    pub reload_count: u64,
}

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(UiTextResource {
            text: reload_text(self.reload_count),
        })
        .add_plugins((InputPlugin, PlayerPlugin));
    }
}

fn reload_text(reload_count: u64) -> String {
    format!("Bevy Project Template\nReloads : {reload_count}\nFrame: 0")
}
