use crate::Components::PlayerComponent::Player;
use crate::Components::RotationComponent::Rotation;
use crate::GameState;
use crate::Resources::ActionsResource::Actions;
use crate::Resources::AssetsResource::TextureAssets;
use crate::Systems::RotationSystem::rotate_player;
use bevy::prelude::*;

const PLAYER_SPEED: f32 = 150.0;
const PLAYER_Z_INDEX: f32 = 1.0;
const PLAYER_BASE_SCALE: f32 = 0.5;
const PLAYER_DEFAULT_SCALE: Vec3 = Vec3::splat(PLAYER_BASE_SCALE);
const PLAYER_SHRINK_FACTOR: f32 = 0.9;

/// Spawns and moves the player entity during gameplay.
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), spawn_player)
            .add_systems(
                Update,
                (
                    handle_player_actions,
                    move_player.after(handle_player_actions),
                    rotate_player,
                )
                    .run_if(in_state(GameState::Playing)),
            )
            .add_systems(OnExit(GameState::Playing), cleanup_player);
    }
}

/// Spawns the single player entity used by this template.
fn spawn_player(mut commands: Commands, textures: Res<TextureAssets>) {
    commands.spawn((
        Sprite::from_image(textures.player.clone()),
        Transform::from_xyz(0.0, 0.0, PLAYER_Z_INDEX).with_scale(PLAYER_DEFAULT_SCALE),
        Player,
        Rotation::default(),
    ));
}

/// Applies one-shot gameplay actions like shrinking or restarting the player.
fn handle_player_actions(actions: Res<Actions>, mut player: Single<&mut Transform, With<Player>>) {
    if actions.restart_game {
        player.translation = Vec3::new(0.0, 0.0, PLAYER_Z_INDEX);
        player.rotation = Quat::IDENTITY;
        player.scale = PLAYER_DEFAULT_SCALE;
        return;
    }

    if actions.shrink_player {
        player.scale *= PLAYER_SHRINK_FACTOR;
    }
}

/// Applies the movement intent stored in `Actions` to the player transform.
fn move_player(
    time: Res<Time>,
    actions: Res<Actions>,
    mut player: Single<&mut Transform, With<Player>>,
) {
    if actions.restart_game {
        return;
    }

    let Some(movement) = actions.player_movement else {
        return;
    };

    let frame_movement = Vec3::new(
        movement.x * PLAYER_SPEED * time.delta_secs(),
        movement.y * PLAYER_SPEED * time.delta_secs(),
        0.0,
    );

    player.translation += frame_movement;
}


/// Removes the player entity when leaving gameplay.
fn cleanup_player(mut commands: Commands, players: Query<Entity, With<Player>>) {
    for entity in &players {
        commands.entity(entity).despawn();
    }
}