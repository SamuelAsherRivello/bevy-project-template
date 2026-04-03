use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_kira_audio::AudioSource;

/// Audio asset handles loaded during `GameState::Loading`.
#[derive(AssetCollection, Resource, Clone)]
pub struct AudioAssets {
    #[asset(path = "audio/Click01.ogg")]
    pub click_01: Handle<AudioSource>,
    #[asset(path = "audio/Click02.mp3")]
    pub click_02: Handle<AudioSource>,
}

/// Texture asset handles loaded during `GameState::Loading`.
#[derive(AssetCollection, Resource, Clone)]
pub struct TextureAssets {
    #[asset(path = "textures/bevy.png")]
    pub player: Handle<Image>,
}