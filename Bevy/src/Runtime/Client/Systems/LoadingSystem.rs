use crate::GameState;
use crate::Resources::AssetsResource::{AudioAssets, TextureAssets};
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

/// Loads the assets required by the template before showing the menu.
pub struct LoadingPlugin;

impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        app.add_loading_state(
            LoadingState::new(GameState::Loading)
                .continue_to_state(GameState::Playing)
                .load_collection::<AudioAssets>()
                .load_collection::<TextureAssets>(),
        );
    }
}
