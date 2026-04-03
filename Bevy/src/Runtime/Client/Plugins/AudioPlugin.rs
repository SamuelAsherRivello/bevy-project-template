use crate::GameState;
use crate::Resources::ActionsResource::Actions;
use crate::Resources::AssetsResource::AudioAssets;
use crate::Plugins::InputPlugin::set_movement_actions;
use bevy::prelude::*;
use bevy_kira_audio::prelude::*;

const CLICK_VOLUME: f32 = 0.3;

/// Plays one-shot gameplay sounds for movement and restart actions.
pub struct InternalAudioPlugin;

impl Plugin for InternalAudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(AudioPlugin)
            .init_resource::<MovementAudioState>()
            .add_systems(OnEnter(GameState::Playing), reset_movement_audio_state)
            .add_systems(
                Update,
                play_gameplay_sounds
                    .after(set_movement_actions)
                    .run_if(in_state(GameState::Playing)),
            );
    }
}

/// Tracks whether movement was active on the previous frame to avoid replaying the sound continuously.
#[derive(Resource, Default)]
struct MovementAudioState {
    was_moving: bool,
}

fn reset_movement_audio_state(mut movement_audio_state: ResMut<MovementAudioState>) {
    movement_audio_state.was_moving = false;
}

fn play_gameplay_sounds(
    actions: Res<Actions>,
    audio_assets: Res<AudioAssets>,
    audio: Res<Audio>,
    mut movement_audio_state: ResMut<MovementAudioState>,
) {
    let is_moving = actions.player_movement.is_some();

    if is_moving && !movement_audio_state.was_moving {
        audio
            .play(audio_assets.click_01.clone())
            .with_volume(CLICK_VOLUME);
    }

    if actions.restart_game {
        audio
            .play(audio_assets.click_02.clone())
            .with_volume(CLICK_VOLUME);
    }

    movement_audio_state.was_moving = is_moving;
}