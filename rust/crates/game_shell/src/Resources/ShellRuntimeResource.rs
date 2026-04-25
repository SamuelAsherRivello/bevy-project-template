use crate::{platform_system, shell_context_resource::ShellContextResource};

#[cfg(not(target_arch = "wasm32"))]
use bevy::prelude::IVec2;
#[cfg(not(target_arch = "wasm32"))]
use std::time::Instant;
#[cfg(target_arch = "wasm32")]
use web_time::Instant;

pub struct ShellRuntimeResource {
    pub context: ShellContextResource,
    pub game: platform_system::GameRuntime,
    pub reload_count: u64,
    #[cfg(not(target_arch = "wasm32"))]
    pub game_source: platform_system::GameSourceWatcher,
    #[cfg(not(target_arch = "wasm32"))]
    pub hot_reload_enabled: bool,
    pub last_game_frame: Instant,
    #[cfg(not(target_arch = "wasm32"))]
    pub last_window_position: Option<IVec2>,
}
