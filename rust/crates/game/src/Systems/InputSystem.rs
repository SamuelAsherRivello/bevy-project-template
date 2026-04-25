use bevy::prelude::*;

use crate::input_component::InputComponent;

pub fn input_startup_system(mut commands: Commands) {
    commands.spawn(InputComponent::default());
}

pub fn input_update_system(
    keys: Res<ButtonInput<KeyCode>>,
    mut input_query: Query<&mut InputComponent>,
) {
    let Ok(mut input) = input_query.get_single_mut() else {
        return;
    };

    input.is_left_arrow_pressed = keys.pressed(KeyCode::ArrowLeft) || keys.pressed(KeyCode::KeyA);
    input.is_right_arrow_pressed = keys.pressed(KeyCode::ArrowRight) || keys.pressed(KeyCode::KeyD);
}
