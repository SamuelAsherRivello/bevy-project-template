use bevy::{math::primitives::Cuboid, prelude::*};
use bevy_simple_subsecond_system as hot_reload;
use hot_reload::prelude::hot;
use shared::input_component::InputComponent;

use crate::player_component::PlayerComponent;

const PLAYER_MESH_SIZE: f32 = 1.0;
const PLAYER_BASE_SCALE: f32 = 1.0;
const PLAYER_COLOR: Color = Color::srgba(0.25, 0.72, 1.0, 1.0);

/// Spawns the player mesh and attaches its gameplay state.
pub fn player_startup_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Name::new("Player"),
        Mesh3d(meshes.add(Cuboid::new(
            PLAYER_MESH_SIZE,
            PLAYER_MESH_SIZE,
            PLAYER_MESH_SIZE,
        ))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: PLAYER_COLOR,
            ..Default::default()
        })),
        Transform::from_scale(Vec3::splat(PLAYER_BASE_SCALE)),
        PlayerComponent::default(),
    ));
}

/// Turns the player from current input and applies the idle pulse animation.
#[hot]
pub fn player_update_system(
    time: Res<Time>,
    input_query: Query<&InputComponent>,
    mut player_query: Query<(&mut PlayerComponent, &mut Transform)>,
) {
    let Ok(input) = input_query.single() else {
        return;
    };

    let turn_input = match (input.is_left_arrow_pressed, input.is_right_arrow_pressed) {
        (true, false) => 1.0,
        (false, true) => -1.0,
        _ => 0.0,
    };

    for (mut player, mut transform) in &mut player_query {
        player.angle_degrees = (player.angle_degrees
            + player.speed_degrees_per_second * turn_input * time.delta_secs())
        .rem_euclid(360.0);

        let pulse = (time.elapsed_secs() * player.pulse_speed).sin().abs();
        player.scale = PLAYER_BASE_SCALE + pulse * player.pulse_amount;

        transform.rotation = Quat::from_rotation_y(player.angle_degrees.to_radians());
        transform.scale = Vec3::splat(PLAYER_BASE_SCALE * player.scale);
    }
}
