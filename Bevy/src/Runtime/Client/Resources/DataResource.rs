use bevy::prelude::*;

/// High-level application states for the template.
#[derive(States, Default, Clone, Eq, PartialEq, Debug, Hash)]
pub enum GameState {
    #[default]
    Loading,
    Menu,
    Playing,
}

/// Core gameplay counters that drive the HUD display.
///
/// Modify these fields from any gameplay system to have the HUD update
/// automatically on the next frame.
#[derive(Resource, Debug, Clone)]
pub struct DataResource {
    pub lives_current: u32,
    pub lives_max: u32,
    pub score_current: u32,
    pub score_max: u32,
    /// When `true` the main menu is skipped and gameplay starts immediately
    /// after assets have finished loading. Defaults to `true` for fast
    /// iteration during development.
    pub will_skip_menu: bool,
}

impl Default for DataResource {
    fn default() -> Self {
        Self {
            lives_current: 3,
            lives_max: 3,
            score_current: 0,
            score_max: 5,
            will_skip_menu: true,
        }
    }
}