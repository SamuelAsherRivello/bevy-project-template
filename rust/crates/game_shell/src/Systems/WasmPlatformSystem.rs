use std::error::Error;

use bevy::prelude::*;
use game_api::Context;

use crate::shell_context_resource::ShellContextResource;

pub struct GameRuntime;

impl GameRuntime {
    pub fn load(
        world: &mut World,
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
        game::app_hot_reload(world, context, reload_count);
        debug_log("GameRuntime::load(): end");
        Ok(Self)
    }

    pub fn frame(&self, world: &mut World, context: &mut dyn Context) {
        game::hot_frame_rust(world, context);
    }

    pub fn cleanup(&self, world: &mut World) {
        game::app_cleanup(world);
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
