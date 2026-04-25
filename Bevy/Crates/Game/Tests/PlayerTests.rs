use std::time::Duration;

use bevy::prelude::{App, Entity, Time, Transform, Update};
use shared::input_component::InputComponent;

use crate::{
    player_component::PlayerComponent, player_system::player_update_system,
};

#[test]
fn player_default_values_match_template_start() {
    let player = PlayerComponent::default();

    assert_close(player.angle_degrees, 0.0);
    assert_close(player.speed_degrees_per_second, 180.0);
    assert_close(player.scale, 1.0);
    assert_close(player.pulse_speed, 1.7);
    assert_close(player.pulse_amount, 0.35);
}

#[test]
fn player_update_turns_from_left_and_right_flags() {
    assert_close(
        updated_player_angle(
            InputComponent {
                is_left_arrow_pressed: true,
                is_right_arrow_pressed: false,
                ..Default::default()
            },
            PlayerComponent {
                angle_degrees: 350.0,
                ..Default::default()
            },
            0.5,
        ),
        80.0,
    );
    assert_close(
        updated_player_angle(
            InputComponent {
                is_left_arrow_pressed: false,
                is_right_arrow_pressed: true,
                ..Default::default()
            },
            PlayerComponent {
                angle_degrees: 10.0,
                ..Default::default()
            },
            0.25,
        ),
        325.0,
    );
    assert_close(
        updated_player_angle(
            InputComponent {
                is_left_arrow_pressed: true,
                is_right_arrow_pressed: true,
                ..Default::default()
            },
            PlayerComponent {
                angle_degrees: 10.0,
                ..Default::default()
            },
            0.25,
        ),
        10.0,
    );
    assert_close(
        updated_player_angle(
            InputComponent::default(),
            PlayerComponent {
                angle_degrees: 10.0,
                ..Default::default()
            },
            0.25,
        ),
        10.0,
    );
}

fn updated_player_angle(input: InputComponent, player: PlayerComponent, delta_secs: f32) -> f32 {
    let mut app = App::new();
    let mut time = Time::<()>::default();
    time.advance_by(Duration::from_secs_f32(delta_secs));
    app.insert_resource(time);
    app.add_systems(Update, player_update_system);

    app.world_mut().spawn(input);
    let player_entity = spawn_player(&mut app, player);

    app.update();

    app.world()
        .entity(player_entity)
        .get::<PlayerComponent>()
        .expect("player should still exist after update")
        .angle_degrees
}

fn spawn_player(app: &mut App, player: PlayerComponent) -> Entity {
    app.world_mut().spawn((player, Transform::default())).id()
}

fn assert_close(actual: f32, expected: f32) {
    assert!(
        (actual - expected).abs() < f32::EPSILON,
        "expected {expected}, got {actual}"
    );
}
