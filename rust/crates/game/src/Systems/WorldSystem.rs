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
        Transform::from_xyz(
            camera.translation.x,
            camera.translation.y,
            camera.translation.z,
        )
        .looking_at(
            Vec3::new(camera.look_at.x, camera.look_at.y, camera.look_at.z),
            Vec3::Y,
        ),
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
        Transform::from_xyz(
            light.translation.x,
            light.translation.y,
            light.translation.z,
        ),
        light,
        GameComponent,
    ));

    let floor = FloorComponent::default();
    let cube_mesh = meshes.add(Cuboid::new(1.0, 1.0, 1.0));
    commands.spawn((
        Name::new("Floor"),
        Mesh3d(cube_mesh),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgba(
                floor.color.red,
                floor.color.green,
                floor.color.blue,
                floor.color.alpha,
            ),
            ..Default::default()
        })),
        Transform::from_xyz(
            floor.translation.x,
            floor.translation.y,
            floor.translation.z,
        )
        .with_scale(Vec3::new(floor.scale.x, floor.scale.y, floor.scale.z)),
        floor,
        GameComponent,
    ));
}
