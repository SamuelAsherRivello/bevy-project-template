use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_inspector_egui::{bevy_egui::EguiContext, bevy_inspector, egui};

use crate::{
    bevy_inspector_resource::BevyInspectorResource,
    host_state_resource::HostStateResource,
};

pub fn shell_capture_input_update_system(
    keys: Res<ButtonInput<KeyCode>>,
    mut host: NonSendMut<HostStateResource>,
) {
    host.context.left_pressed = keys.pressed(KeyCode::ArrowLeft) || keys.pressed(KeyCode::KeyA);
    host.context.right_pressed = keys.pressed(KeyCode::ArrowRight) || keys.pressed(KeyCode::KeyD);

    host.frame_resource.turn_input = match (host.context.left_pressed, host.context.right_pressed) {
        (true, false) => 1.0,
        (false, true) => -1.0,
        _ => 0.0,
    };
}

pub fn shell_toggle_bevy_inspector_update_system(
    keys: Res<ButtonInput<KeyCode>>,
    mut inspector: ResMut<BevyInspectorResource>,
) {
    if keys.just_pressed(KeyCode::KeyT) {
        inspector.enabled = !inspector.enabled;
    }
}

pub fn shell_bevy_inspector_ui_update_system(world: &mut World) {
    let inspector_settings = world.resource::<BevyInspectorResource>();
    if !inspector_settings.enabled {
        return;
    }
    let inspector_x = inspector_settings.x;
    let inspector_y = inspector_settings.y;
    let inspector_width = inspector_settings.width;
    let inspector_height = inspector_settings.height;

    let egui_context = world
        .query_filtered::<&mut EguiContext, With<PrimaryWindow>>()
        .get_single(world);

    let Ok(egui_context) = egui_context else {
        return;
    };
    let mut egui_context = egui_context.clone();

    let window_entity = named_entity(world, "Window");
    let point_light_entity = named_entity(world, "PointLight");
    let camera_entity = named_entity(world, "Camera3d");
    let player_entity = named_entity(world, "Player");
    let floor_entity = named_entity(world, "Floor");

    egui::Window::new("World Inspector")
        .default_pos(egui::pos2(inspector_x, inspector_y))
        .default_size(egui::vec2(inspector_width, inspector_height))
        .show(egui_context.get_mut(), |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                show_entity_inspector(ui, world, "Window", window_entity);
                show_entity_inspector(ui, world, "PointLight", point_light_entity);
                show_entity_inspector(ui, world, "Camera3d", camera_entity);
                show_entity_inspector(ui, world, "Player", player_entity);
                show_entity_inspector(ui, world, "Floor", floor_entity);
            });
        });
}

fn named_entity(world: &mut World, name: &str) -> Option<Entity> {
    let mut query = world.query::<(Entity, &Name)>();
    query
        .iter(world)
        .find_map(|(entity, entity_name)| (entity_name.as_str() == name).then_some(entity))
}

fn show_entity_inspector(
    ui: &mut egui::Ui,
    world: &mut World,
    label: &'static str,
    entity: Option<Entity>,
) {
    ui.collapsing(label, |ui| {
        if let Some(entity) = entity {
            bevy_inspector::ui_for_entity(world, entity, ui);
        } else {
            ui.label("Not found");
        }
    });
}
