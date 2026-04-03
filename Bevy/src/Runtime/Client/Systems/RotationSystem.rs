use crate::Components::RotationComponent::Rotation;
use crate::GameState;
use bevy::prelude::*;

/// Rotates entities that opt into the `Rotation` component.
pub struct RotationPlugin;

impl Plugin for RotationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, rotate_entities.run_if(in_state(GameState::Playing)));
    }
}

fn rotate_entities(time: Res<Time>, mut rotating_entities: Query<(&Rotation, &mut Transform)>) {
    let delta_rotation = time.delta_secs();
    if delta_rotation == 0.0 {
        return;
    }

    for (rotation, mut transform) in &mut rotating_entities {
        transform.rotate_z(rotation.radians_per_second * delta_rotation);
    }
}
