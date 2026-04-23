use std::error::Error;

use bevy::prelude::*;
use game_api::{Context, RenderPacket};

use crate::shell_context_resource::ShellContextResource;

pub struct GameRuntime;

impl GameRuntime {
    pub fn load(
        context: &mut ShellContextResource,
        run_initialize: bool,
        reload_count: u64,
    ) -> Result<Self, Box<dyn Error + Send + Sync>> {
        debug_log("GameRuntime::load(): begin");
        if run_initialize {
            debug_log("GameRuntime::load(): app_initialize()");
            game::app_initialize(context);
        }
        debug_log("GameRuntime::load(): app_hot_reload()");
        game::app_hot_reload(context, reload_count);
        debug_log("GameRuntime::load(): end");
        Ok(Self)
    }

    pub fn frame(&self, context: &mut dyn Context) {
        game::hot_frame_rust(context);
    }

    pub fn render_packet(&self) -> RenderPacket {
        game::hot_render_packet_rust()
    }

    pub fn unload(&mut self, context: &mut ShellContextResource) {
        context.log("browser build reloads the whole page instead of unloading a DLL");
    }
}

pub fn supports_hot_reload() -> bool {
    false
}

pub fn configure_runtime() {
    console_error_panic_hook::set_once();
    tracing_wasm::set_as_global_default();
    debug_log("configure_runtime(): panic hook and tracing installed");
}

pub fn ensure_game_ready() {}

pub fn debug_log(message: &str) {
    web_sys::console::log_1(&message.into());
}

pub fn debug_render_packet(frame: u64, packet: &RenderPacket) {
    if frame < 5 || frame % 120 == 0 {
        debug_log(&format!(
            "frame={frame} render_items={} reloads={} ui_text_len={} clear=({:.2}, {:.2}, {:.2}, {:.2})",
            packet.render_item_count,
            packet.reload_count,
            packet.ui_text_len,
            packet.clear_color.red,
            packet.clear_color.green,
            packet.clear_color.blue,
            packet.clear_color.alpha
        ));
    }
}

pub fn primary_window(_initial_window_position: Option<IVec2>) -> Window {
    Window {
        title: "Rust Hot Reload Demo - Bevy Web".to_owned(),
        resolution: (800.0, 600.0).into(),
        canvas: Some("#bevy".to_owned()),
        fit_canvas_to_parent: true,
        prevent_default_event_handling: false,
        ..Default::default()
    }
}

pub fn track_window_position_update_system() {}

pub fn persist_window_position_on_close_update_system() {}

pub fn load_saved_window_position() -> Option<IVec2> {
    None
}

