use bevy::prelude::*;

/// Marker component for the player-controlled entity.
///
/// Keep marker components zero-sized like `Player`.
/// If this component later needs gameplay data, prefer named fields over a tuple
/// struct so the intent of each value stays obvious at call sites and in queries.
#[derive(Component, Reflect, Default, Debug, Clone, Copy, PartialEq, Eq)]
#[reflect(Component, Default)]
pub struct Player;
