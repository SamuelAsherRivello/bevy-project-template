use bevy_ecs::prelude::{Commands, Query};

use crate::{input_component::InputComponent, player_component::PlayerComponent};

pub fn player_startup_system(mut commands: Commands) {
    commands.spawn(PlayerComponent::default());
}

pub fn player_update_system(
    input_query: Query<&InputComponent>,
    mut player_query: Query<&mut PlayerComponent>,
) {
    let Ok(input) = input_query.get_single() else {
        return;
    };

    let key_turn = match (input.left_pressed, input.right_pressed) {
        (true, false) => 1.0,
        (false, true) => -1.0,
        _ => 0.0,
    };
    let turn_input = if key_turn != 0.0 { key_turn } else { input.turn_input };

    for mut player in &mut player_query {
        player.angle_degrees = (player.angle_degrees
            + player.speed_degrees_per_second * turn_input * input.delta_seconds)
            .rem_euclid(360.0);

        let pulse = (input.elapsed_seconds * player.pulse_speed).sin().abs();
        player.scale = 1.0 + pulse * player.pulse_amount;
    }
}

