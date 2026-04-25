use bevy::prelude::*;
use bevy_inspector_egui::{
    bevy_inspector::EntityFilter,
    bevy_egui::{EguiContext, PrimaryEguiContext, egui},
    bevy_inspector,
};

use crate::bevy_inspector_component::BevyInspectorComponent;

pub fn bevy_inspector_startup_system(mut commands: Commands) {
    commands.spawn((
        Name::new("Bevy Inspector"),
        BevyInspectorComponent::default(),
    ));
}

pub fn bevy_inspector_toggle_update_system(
    keys: Res<ButtonInput<KeyCode>>,
    mut inspector_query: Query<&mut BevyInspectorComponent>,
) {
    if !keys.just_pressed(KeyCode::KeyT) {
        return;
    }

    let Ok(mut inspector) = inspector_query.single_mut() else {
        return;
    };

    inspector.is_visible = !inspector.is_visible;
}

pub fn bevy_inspector_ui_system(world: &mut World) {
    let Some((is_visible, x, y, width, height)) = inspector_window_settings(world) else {
        return;
    };

    if !is_visible {
        return;
    }

    let Ok(mut egui_context) = world
        .query_filtered::<&mut EguiContext, With<PrimaryEguiContext>>()
        .single(world)
        .cloned()
    else {
        return;
    };

    egui::Window::new("Bevy Inspector")
        .default_pos(egui::pos2(x, y))
        .default_size(egui::vec2(width, height))
        .show(egui_context.get_mut(), |ui| {
            egui::ScrollArea::both().show(ui, |ui| {
                bevy_inspector::ui_for_entities_filtered(world, ui, false, &NamedEntityFilter);
                ui.allocate_space(ui.available_size());
            });
        });
}

struct NamedEntityFilter;

impl EntityFilter for NamedEntityFilter {
    type StaticFilter = ();

    fn filter_entity(&self, world: &mut World, entity: Entity) -> bool {
        let Some(name) = world.get::<Name>(entity) else {
            return false;
        };

        matches!(
            name.as_str(),
            "Camera3d" | "Lights" | "Player" | "Floor"
        )
    }
}

fn inspector_window_settings(world: &mut World) -> Option<(bool, f32, f32, f32, f32)> {
    let mut query = world.query::<&BevyInspectorComponent>();
    let inspector = query.iter(world).next()?;
    Some((
        inspector.is_visible,
        inspector.x,
        inspector.y,
        inspector.width,
        inspector.height,
    ))
}
