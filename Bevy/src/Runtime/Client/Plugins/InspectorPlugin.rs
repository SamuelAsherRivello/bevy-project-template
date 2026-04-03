use bevy::prelude::*;
use bevy_inspector_egui::bevy_egui::{EguiContext, EguiPlugin, EguiPrimaryContextPass, PrimaryEguiContext};
use bevy_inspector_egui::{bevy_inspector, DefaultInspectorConfigPlugin};

const INSPECTOR_DEFAULT_X: f32 = 10.0;
const INSPECTOR_DEFAULT_Y: f32 = 50.0;
const INSPECTOR_DEFAULT_WIDTH: f32 = 224.0;
const INSPECTOR_DEFAULT_HEIGHT: f32 = 416.0;

#[derive(Resource, Default)]
struct InspectorPanelState {
    visible: bool,
}

/// Adds a world inspector for dev-time debugging.
///
/// Press T to toggle the inspector panel.
pub struct InspectorPlugin;

impl Plugin for InspectorPlugin {
    fn build(&self, app: &mut App) {
        if !app.is_plugin_added::<EguiPlugin>() {
            app.add_plugins(EguiPlugin::default());
        }

        if !app.is_plugin_added::<DefaultInspectorConfigPlugin>() {
            app.add_plugins(DefaultInspectorConfigPlugin);
        }

        app.init_resource::<InspectorPanelState>()
            .add_systems(Update, toggle_inspector)
            .add_systems(EguiPrimaryContextPass, world_inspector_ui);
    }
}

fn toggle_inspector(input: Res<ButtonInput<KeyCode>>, mut state: ResMut<InspectorPanelState>) {
    if input.just_pressed(KeyCode::KeyT) {
        state.visible = !state.visible;
    }
}

fn world_inspector_ui(world: &mut World) {
    let is_visible = world
        .get_resource::<InspectorPanelState>()
        .map(|state| state.visible)
        .unwrap_or(false);

    if !is_visible {
        return;
    }

    let egui_context = world
        .query_filtered::<&mut EguiContext, With<PrimaryEguiContext>>()
        .single(world);

    let Ok(egui_context) = egui_context else {
        return;
    };
    let mut egui_context = egui_context.clone();

    bevy_inspector_egui::bevy_egui::egui::Window::new("World Inspector")
        .default_pos((INSPECTOR_DEFAULT_X, INSPECTOR_DEFAULT_Y))
        .default_size((INSPECTOR_DEFAULT_WIDTH, INSPECTOR_DEFAULT_HEIGHT))
        .show(egui_context.get_mut(), |ui| {
            bevy_inspector_egui::bevy_egui::egui::ScrollArea::both().show(ui, |ui| {
                bevy_inspector::ui_for_world_entities_filtered::<(
                    Without<bevy::ecs::observer::Observer>,
                    Without<Node>,
                )>(world, ui, true);
                ui.allocate_space(ui.available_size());
            });
        });
}
