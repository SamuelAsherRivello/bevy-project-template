use bevy::prelude::{Commands, DespawnRecursiveExt, Entity, Query, With};

use crate::game_component::GameComponent;

pub fn game_cleanup_system(
    mut commands: Commands,
    game_entities: Query<Entity, With<GameComponent>>,
) {
    for entity in &game_entities {
        commands.entity(entity).despawn_recursive();
    }
}
