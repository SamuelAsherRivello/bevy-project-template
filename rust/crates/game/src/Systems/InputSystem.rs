use bevy_ecs::prelude::Commands;

use crate::input_component::InputComponent;

pub fn input_startup_system(mut commands: Commands) {
    commands.spawn(InputComponent::default());
}


