use crate::Components::PlayerComponent::Player;
use crate::Components::RotationComponent::Rotation;
use bevy::prelude::*;

/// Applies angular velocity from `Rotation` to the player's transform.
pub(crate) fn rotate_player(
    time: Res<Time>,
    player: Single<(&Rotation, &mut Transform), With<Player>>,
) {
    let delta_rotation = time.delta_secs();
    if delta_rotation == 0.0 {
        return;
    }

    let (rotation, mut transform) = player.into_inner();
    transform.rotate_z(rotation.radians_per_second * delta_rotation);
}