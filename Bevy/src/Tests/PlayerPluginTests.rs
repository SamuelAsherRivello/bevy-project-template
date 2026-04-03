use bevy::prelude::*;

use crate::Components::PlayerComponent::Player;
use crate::Plugins::PlayerPlugin::PlayerPlugin;
use crate::Resources::ActionsResource::Actions;
use crate::Resources::AssetsResource::TextureAssets;
use crate::GameState;

const EXPECTED_PLAYER_BASE_SCALE: f32 = 0.25;

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
fn player_can_shrink_and_restart() {
    let mut app = App::new();
    app.add_plugins((MinimalPlugins, bevy::state::app::StatesPlugin))
        .init_state::<GameState>()
        .insert_resource(Actions::default())
        .insert_resource(TextureAssets {
            player: Handle::default(),
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
    assert_eq!(shrunk_scale, Vec3::splat(EXPECTED_PLAYER_BASE_SCALE * 0.9));
}

#[test]
fn player_restart_resets_translation_rotation_and_scale() {
    let mut app = App::new();
    app.add_plugins((MinimalPlugins, bevy::state::app::StatesPlugin))
        .init_state::<GameState>()
        .insert_resource(Actions::default())
        .insert_resource(TextureAssets {
            player: Handle::default(),
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
    assert_eq!(reset_transform.scale, Vec3::splat(EXPECTED_PLAYER_BASE_SCALE));
}