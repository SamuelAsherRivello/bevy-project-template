use bevy::prelude::*;

use crate::Resources::ActionsResource::{Actions, GameControl, get_movement};

#[test]
fn actions_defaults_to_no_movement() {
    let actions = Actions::default();
    assert!(
        actions.player_movement.is_none(),
        "A fresh Actions resource must not carry any movement intent"
    );
    assert!(!actions.shrink_player);
    assert!(!actions.restart_game);
}

#[test]
fn actions_stores_and_returns_movement() {
    let mut actions = Actions::default();
    actions.player_movement = Some(Vec2::new(1.0, 0.0));
    assert_eq!(actions.player_movement, Some(Vec2::new(1.0, 0.0)));
}

#[test]
fn actions_movement_can_be_cleared() {
    let mut actions = Actions::default();
    actions.player_movement = Some(Vec2::ONE);
    actions.player_movement = None;
    assert!(actions.player_movement.is_none());
}

#[test]
fn get_movement_returns_zero_when_no_keys_pressed() {
    let input = ButtonInput::<KeyCode>::default();
    assert_eq!(get_movement(&GameControl::Up, &input), 0.0);
    assert_eq!(get_movement(&GameControl::Down, &input), 0.0);
    assert_eq!(get_movement(&GameControl::Left, &input), 0.0);
    assert_eq!(get_movement(&GameControl::Right, &input), 0.0);
}

#[test]
fn game_control_not_pressed_by_default() {
    let input = ButtonInput::<KeyCode>::default();
    assert!(!GameControl::Up.pressed(&input));
    assert!(!GameControl::Down.pressed(&input));
    assert!(!GameControl::Left.pressed(&input));
    assert!(!GameControl::Right.pressed(&input));
}
