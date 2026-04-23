use std::time::Duration;

use bevy::diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin};
use bevy::prelude::*;
use game_api::{MAX_RENDER_ITEMS, RENDER_ITEM_KIND_CUBE};
#[cfg(not(target_arch = "wasm32"))]
use std::time::Instant;
#[cfg(target_arch = "wasm32")]
use web_time::Instant;

use crate::{
    bevy_inspector_resource::BevyInspectorResource,
    demo_overlay_text_component::DemoOverlayTextComponent,
    demo_render_item_component::DemoRenderItemComponent,
    host_state_resource::HostStateResource,
    platform_system,
};

pub fn shell_drive_game_update_system(mut host: NonSendMut<HostStateResource>) {
    #[cfg(not(target_arch = "wasm32"))]
    let should_reload = if host.hot_reload_enabled {
        let game_changed = {
            let HostStateResource {
                context,
                game_source,
                ..
            } = &mut *host;
            game_source.rebuild_if_changed(context)
        };

        game_changed && host.game.artifact_changed()
    } else {
        false
    };

    #[cfg(target_arch = "wasm32")]
    let should_reload = false;

    if should_reload {
        println!("change detected; reloading game DLL");
        host.reload_count += 1;
        let reload_count = host.reload_count;
        {
            let HostStateResource { context, game, .. } = &mut *host;
            game.unload(context);
        }

        match platform_system::GameRuntime::load(&mut host.context, false, reload_count) {
            Ok(game) => host.game = game,
            Err(error) => eprintln!("failed to load game DLL: {error}"),
        }
    }

    let elapsed = host.last_game_frame.elapsed();
    if elapsed >= Duration::from_secs_f32(1.0 / 60.0) {
        let delta_seconds = elapsed.as_secs_f32().min(0.25);
        let latest_render_packet = {
            let HostStateResource {
                context,
                frame_resource,
                metrics_resource,
                game,
                ..
            } = &mut *host;

            frame_resource.delta_seconds = delta_seconds;
            frame_resource.shell_frame = context.frame;
            metrics_resource.elapsed_seconds += delta_seconds;
            metrics_resource.last_shell_frame = frame_resource.shell_frame;

            context.delta_seconds = frame_resource.delta_seconds;
            context.turn_input = frame_resource.turn_input;
            context.elapsed_seconds = metrics_resource.elapsed_seconds;

            game.frame(context);
            context.frame += 1;
            game.render_packet()
        };

        platform_system::debug_render_packet(host.context.frame, &latest_render_packet);
        host.latest_render_packet = latest_render_packet;
        host.last_game_frame = Instant::now();
    }
}

pub fn shell_apply_render_packet_update_system(
    host: NonSend<HostStateResource>,
    mut clear_color: ResMut<ClearColor>,
    mut render_items: Query<(
        &DemoRenderItemComponent,
        &mut Transform,
        &MeshMaterial3d<StandardMaterial>,
        &mut Visibility,
    )>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let packet = host.latest_render_packet;
    clear_color.0 = Color::srgba(
        packet.clear_color.red,
        packet.clear_color.green,
        packet.clear_color.blue,
        packet.clear_color.alpha,
    );

    let visible_count = packet.render_item_count.min(MAX_RENDER_ITEMS as u32) as usize;

    for (render_item, mut transform, material_handle, mut visibility) in &mut render_items {
        if render_item.slot >= visible_count {
            *visibility = Visibility::Hidden;
            continue;
        }

        let item = packet.render_items[render_item.slot];

        if item.kind != RENDER_ITEM_KIND_CUBE {
            *visibility = Visibility::Hidden;
            continue;
        }

        transform.translation = Vec3::new(item.translation.x, item.translation.y, item.translation.z);
        transform.rotation = Quat::from_euler(
            EulerRot::XYZ,
            item.rotation_radians.x,
            item.rotation_radians.y,
            item.rotation_radians.z,
        );
        transform.scale = Vec3::new(item.scale.x, item.scale.y, item.scale.z);

        if let Some(material) = materials.get_mut(&material_handle.0) {
            material.base_color = Color::srgba(
                item.color.red,
                item.color.green,
                item.color.blue,
                item.color.alpha,
            );
        }

        *visibility = Visibility::Visible;
    }
}

pub fn shell_update_overlay_text_update_system(
    host: NonSend<HostStateResource>,
    diagnostics: Res<DiagnosticsStore>,
    inspector: Res<BevyInspectorResource>,
    mut overlay_query: Query<&mut Text, With<DemoOverlayTextComponent>>,
) {
    let Ok(mut text) = overlay_query.get_single_mut() else {
        return;
    };

    let len = host
        .latest_render_packet
        .ui_text_len
        .min(game_api::UI_TEXT_CAPACITY as u32) as usize;
    let mut overlay_text = String::from_utf8_lossy(&host.latest_render_packet.ui_text[..len]).into_owned();

    if inspector.enabled {
        let fps_line = diagnostics
            .get(&FrameTimeDiagnosticsPlugin::FPS)
            .and_then(|fps| fps.smoothed())
            .map(|value| format!("FPS: {value:>4.0}"))
            .unwrap_or_else(|| "FPS:  N/A".to_owned());

        overlay_text.push('\n');
        overlay_text.push_str(&fps_line);
    }

    *text = Text::new(overlay_text);
}
