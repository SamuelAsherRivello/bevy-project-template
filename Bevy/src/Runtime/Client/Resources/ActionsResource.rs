use bevy::prelude::*;

/// Resource holding the current frame's player intent.
/// Systems read this instead of raw input, keeping input logic decoupled.
#[derive(Default, Resource, Debug, Clone)]
pub struct Actions {
    pub player_movement: Option<Vec2>,
    pub shrink_player: bool,
    pub restart_game: bool,
}

/// Logical input directions mapped from keyboard keys.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameControl {
    Up,
    Down,
    Left,
    Right,
}

impl GameControl {
    /// Returns true if the key(s) for this direction are currently held.
    /// Takes a plain `&ButtonInput` (not `Res`) so this can be called from tests.
    pub fn pressed(&self, keyboard_input: &ButtonInput<KeyCode>) -> bool {
        match self {
            GameControl::Up => {
                keyboard_input.pressed(KeyCode::KeyW) || keyboard_input.pressed(KeyCode::ArrowUp)
            }
            GameControl::Down => {
                keyboard_input.pressed(KeyCode::KeyS) || keyboard_input.pressed(KeyCode::ArrowDown)
            }
            GameControl::Left => {
                keyboard_input.pressed(KeyCode::KeyA) || keyboard_input.pressed(KeyCode::ArrowLeft)
            }
            GameControl::Right => {
                keyboard_input.pressed(KeyCode::KeyD) || keyboard_input.pressed(KeyCode::ArrowRight)
            }
        }
    }
}

/// Returns 1.0 if the control is pressed, 0.0 otherwise.
/// Takes a plain reference so it is unit-testable without a Bevy World.
pub fn get_movement(control: &GameControl, input: &ButtonInput<KeyCode>) -> f32 {
    if control.pressed(input) { 1.0 } else { 0.0 }
}
