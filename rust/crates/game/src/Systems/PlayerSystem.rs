use bevy::{math::primitives::Cuboid, prelude::*};

use crate::{
    game_component::GameComponent, input_component::InputComponent,
    player_component::PlayerComponent,
};

pub fn player_startup_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Name::new("Player"),
        Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgba(0.22, 0.72, 1.0, 1.0),
            ..Default::default()
        })),
        Transform::from_scale(Vec3::splat(1.5)),
        PlayerComponent::default(),
        GameComponent,
    ));
}

pub fn player_update_system(
    time: Res<Time>,
    input_query: Query<&InputComponent>,
    mut player_query: Query<(&mut PlayerComponent, &mut Transform)>,
) {
    let Ok(input) = input_query.get_single() else {
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
        player.scale = 1.0 + pulse * player.pulse_amount;

        transform.rotation = Quat::from_rotation_y(player.angle_degrees.to_radians());
        transform.scale = Vec3::splat(1.5 * player.scale);
    }
}
