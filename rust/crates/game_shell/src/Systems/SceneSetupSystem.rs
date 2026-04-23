use bevy::{math::primitives::Cuboid, prelude::*};
use bevy::window::PrimaryWindow;
use game_api::MAX_RENDER_ITEMS;

use crate::{
    bevy_inspector_component::BevyInspectorComponent,
    demo_overlay_text_component::DemoOverlayTextComponent,
    demo_render_item_component::DemoRenderItemComponent,
};

pub fn shell_startup_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    primary_window: Query<Entity, With<PrimaryWindow>>,
) {
    if let Ok(window_entity) = primary_window.get_single() {
        commands.entity(window_entity).insert(Name::new("Window"));
    }

    commands.spawn((
        Name::new("PointLight"),
        PointLight {
            intensity: 2_000_000.0,
            shadows_enabled: true,
            ..Default::default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0),
        BevyInspectorComponent,
    ));

    commands.spawn((
        Name::new("Camera3d"),
        Camera3d::default(),
        Msaa::Off,
        Transform::from_xyz(-5.0, 4.5, 9.0).looking_at(Vec3::new(0.0, 1.0, 0.0), Vec3::Y),
        BevyInspectorComponent,
    ));

    let cube_mesh = meshes.add(Cuboid::new(1.0, 1.0, 1.0));

    for slot in 0..MAX_RENDER_ITEMS {
        let name = match slot {
            0 => Name::new("Player"),
            1 => Name::new("Floor"),
            _ => Name::new(format!("RenderItemSlot{slot}")),
        };

        commands.spawn((
            name,
            Mesh3d(cube_mesh.clone()),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: Color::NONE,
                alpha_mode: AlphaMode::Blend,
                ..Default::default()
            })),
            Transform::from_scale(Vec3::ZERO),
            Visibility::Hidden,
            DemoRenderItemComponent { slot },
        ));
    }

    commands.spawn((
        Text::new("Waiting for game UI..."),
        TextFont {
            font_size: 22.0,
            ..Default::default()
        },
        TextColor(Color::WHITE),
        Node {
            position_type: PositionType::Absolute,
            left: Val::Px(24.0),
            top: Val::Px(24.0),
            padding: UiRect::axes(Val::Px(12.0), Val::Px(8.0)),
            ..Default::default()
        },
        BackgroundColor(Color::srgba(0.02, 0.02, 0.02, 0.72)),
        BorderRadius::all(Val::Px(8.0)),
        DemoOverlayTextComponent,
    ));
}
