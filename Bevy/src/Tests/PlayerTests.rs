use bevy::prelude::*;
use bevy::time::TimeUpdateStrategy;
use std::time::Duration;

use crate::Components::RotationComponent::Rotation;
use crate::Resources::ActionsResource::Actions;
use crate::Resources::AssetsResource::TextureAssets;
use crate::Systems::PlayerSystem::PlayerPlugin;
use crate::Systems::RotationSystem::RotationPlugin;
use crate::{Components::PlayerComponent::Player, GameState};

#[test]
fn player_component_can_be_spawned_and_queried() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins);

    let player_entity = app.world_mut().spawn((Player, Transform::default())).id();

    app.update();

    assert!(
        app.world().get::<Player>(player_entity).is_some(),
        "Entity should carry the Player marker component"
    );
    assert!(
        app.world().get::<Transform>(player_entity).is_some(),
        "Entity should carry a Transform"
    );
}

#[test]
fn player_transform_can_be_mutated() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins);

    let player_entity = app
        .world_mut()
        .spawn((Player, Transform::from_translation(Vec3::ZERO)))
        .id();

    app.world_mut()
        .get_mut::<Transform>(player_entity)
        .unwrap()
        .translation = Vec3::new(42.0, 0.0, 0.0);

    let translation = app
        .world()
        .get::<Transform>(player_entity)
        .unwrap()
        .translation;

    assert_eq!(translation.x, 42.0);
}

#[test]
fn game_state_starts_in_loading() {
    let mut app = App::new();
    app.add_plugins((MinimalPlugins, bevy::state::app::StatesPlugin))
        .init_state::<GameState>();

    app.update();

    let state = app.world().resource::<State<GameState>>();
    assert_eq!(
        *state.get(),
        GameState::Loading,
        "Game must start in the Loading state"
    );
}

#[test]
fn player_can_shrink_and_restart() {
    let mut app = App::new();
    app.add_plugins((MinimalPlugins, bevy::state::app::StatesPlugin))
        .init_state::<GameState>()
        .insert_resource(Actions::default())
        .insert_resource(TextureAssets {
            bevy: Handle::default(),
            github: Handle::default(),
        })
        .add_plugins(PlayerPlugin);

    app.world_mut()
        .resource_mut::<NextState<GameState>>()
        .set(GameState::Playing);
    app.update();

    app.world_mut().resource_mut::<Actions>().shrink_player = true;
    app.update();

    let mut player_query = app.world_mut().query_filtered::<&Transform, With<Player>>();
    let shrunk_scale = player_query.single(app.world()).unwrap().scale;
    assert_eq!(shrunk_scale, Vec3::splat(0.9));
}

#[test]
fn player_restart_resets_translation_rotation_and_scale() {
    let mut app = App::new();
    app.add_plugins((MinimalPlugins, bevy::state::app::StatesPlugin))
        .init_state::<GameState>()
        .insert_resource(Actions::default())
        .insert_resource(TextureAssets {
            bevy: Handle::default(),
            github: Handle::default(),
        })
        .add_plugins(PlayerPlugin);

    app.world_mut()
        .resource_mut::<NextState<GameState>>()
        .set(GameState::Playing);
    app.update();

    let player_entity = app
        .world_mut()
        .query_filtered::<Entity, With<Player>>()
        .single(app.world())
        .unwrap();

    {
        let mut transform = app.world_mut().get_mut::<Transform>(player_entity).unwrap();
        transform.translation = Vec3::new(24.0, -12.0, 8.0);
        transform.rotation = Quat::from_rotation_z(1.25);
        transform.scale = Vec3::splat(0.4);
    }

    {
        let mut actions = app.world_mut().resource_mut::<Actions>();
        actions.restart_game = true;
    }

    app.update();

    let mut player_query = app.world_mut().query_filtered::<&Transform, With<Player>>();
    let reset_transform = player_query.single(app.world()).unwrap();
    assert_eq!(reset_transform.translation, Vec3::new(0.0, 0.0, 1.0));
    assert_eq!(reset_transform.rotation, Quat::IDENTITY);
    assert_eq!(reset_transform.scale, Vec3::ONE);
}

#[test]
fn player_rotates_slowly_during_gameplay() {
    let mut app = App::new();
    app.add_plugins((MinimalPlugins, bevy::state::app::StatesPlugin))
        .init_state::<GameState>()
        .insert_resource(Actions::default())
        .insert_resource(TimeUpdateStrategy::ManualDuration(
            Duration::from_secs_f32(1.0),
        ))
        .insert_resource(TextureAssets {
            bevy: Handle::default(),
            github: Handle::default(),
        })
        .add_plugins((PlayerPlugin, RotationPlugin));

    app.world_mut()
        .resource_mut::<NextState<GameState>>()
        .set(GameState::Playing);
    app.update();

    let player_entity = app
        .world_mut()
        .query_filtered::<Entity, With<Player>>()
        .single(app.world())
        .unwrap();

    let rotation = *app.world().get::<Rotation>(player_entity).unwrap();
    let before = app.world().get::<Transform>(player_entity).unwrap().rotation;

    app.update();

    let after = app.world().get::<Transform>(player_entity).unwrap().rotation;
    let delta = before.inverse() * after;
    let (_, _, angle) = delta.to_euler(EulerRot::XYZ);

    assert_ne!(after, before);
    assert_eq!(rotation, Rotation::default());
    assert!(angle > 0.0, "Player should rotate forward during gameplay");
}
