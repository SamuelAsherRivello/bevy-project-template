use bevy::math::Vec3Swizzles;
use bevy::prelude::*;

use crate::Components::PlayerComponent::Player;
use crate::GameState;
use crate::Resources::ActionsResource::{Actions, GameControl, get_movement};

const FOLLOW_DEAD_ZONE: f32 = 5.0;

/// Converts raw input into the higher-level `Actions` resource.
pub struct ActionsPlugin;

impl Plugin for ActionsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Actions>().add_systems(
            Update,
            set_movement_actions.run_if(in_state(GameState::Playing)),
        );
    }
}

/// Reads keyboard and touch input and stores normalized movement intent.
pub fn set_movement_actions(
    mut actions: ResMut<Actions>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    touch_input: Res<Touches>,
    player: Query<&Transform, With<Player>>,
    camera: Single<(&Camera, &GlobalTransform), With<Camera2d>>,
) -> Result {
    let mut player_movement = Vec2::new(
        get_movement(&GameControl::Right, &keyboard_input)
            - get_movement(&GameControl::Left, &keyboard_input),
        get_movement(&GameControl::Up, &keyboard_input)
            - get_movement(&GameControl::Down, &keyboard_input),
    );

    if let Some(touch_position) = touch_input.first_pressed_position()
        && let Ok(touch_position) = camera.0.viewport_to_world_2d(&camera.1, touch_position)
    {
        let offset_to_touch = touch_position
            - player
                .single()
                .map(|transform| transform.translation.xy())
                .unwrap_or(touch_position);

        if offset_to_touch.length() > FOLLOW_DEAD_ZONE {
            player_movement = offset_to_touch.normalize();
        }
    }

    if player_movement != Vec2::ZERO {
        actions.player_movement = Some(player_movement.normalize());
    } else {
        actions.player_movement = None;
    }

    actions.shrink_player = keyboard_input.just_pressed(KeyCode::Space);
    actions.restart_game = keyboard_input.just_pressed(KeyCode::KeyR);

    Ok(())
}
