use crate::GameState;
use crate::Resources::AssetsResource::{AudioAssets, TextureAssets};
use crate::Resources::DataResource::DataResource;
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

/// Loads the assets required by the template before showing the menu.
pub struct LoadingPlugin;

impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<DataResource>()
            .add_loading_state(
                LoadingState::new(GameState::Loading)
                    .continue_to_state(GameState::Menu)
                    .load_collection::<AudioAssets>()
                    .load_collection::<TextureAssets>(),
            )
            .add_systems(OnEnter(GameState::Menu), apply_skip_menu);
    }
}

/// Immediately advances to `Playing` when `DataResource::will_skip_menu`
/// is enabled. Runs on the same frame as `OnEnter(Menu)` so there is no flicker.
fn apply_skip_menu(
    config: Res<DataResource>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if config.will_skip_menu {
        next_state.set(GameState::Playing);
    }
}