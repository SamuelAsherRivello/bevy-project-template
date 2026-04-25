use bevy::{math::primitives::Cuboid, prelude::*};

use crate::{
    camera_component::CameraComponent, floor_component::FloorComponent,
    game_component::GameComponent, light_component::LightComponent,
};

pub fn world_startup_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let camera = CameraComponent::default();
    commands.spawn((
        Name::new("Camera3d"),
        Camera3d::default(),
        Msaa::Off,
        Transform::from_translation(camera.translation).looking_at(camera.look_at, Vec3::Y),
        camera,
        GameComponent,
    ));

    let light = LightComponent::default();
    commands.spawn((
        Name::new("PointLight"),
        PointLight {
            intensity: light.intensity,
            shadows_enabled: true,
            ..Default::default()
        },
        Transform::from_translation(light.translation),
        light,
        GameComponent,
    ));

    let floor = FloorComponent::default();
    let cube_mesh = meshes.add(Cuboid::new(1.0, 1.0, 1.0));
    commands.spawn((
        Name::new("Floor"),
        Mesh3d(cube_mesh),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: floor.color,
            ..Default::default()
        })),
        Transform::from_translation(floor.translation).with_scale(floor.scale),
        floor,
        GameComponent,
    ));
}
