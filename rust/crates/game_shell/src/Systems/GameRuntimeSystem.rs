use std::time::Duration;

use bevy::prelude::*;
#[cfg(not(target_arch = "wasm32"))]
use std::time::Instant;
#[cfg(target_arch = "wasm32")]
use web_time::Instant;

use crate::{platform_system, shell_runtime_resource::ShellRuntimeResource};

pub fn shell_drive_game_update_system(world: &mut World) {
    let mut runtime = world
        .remove_non_send_resource::<ShellRuntimeResource>()
        .expect("ShellRuntimeResource should exist before driving game");

    reload_game_if_needed(world, &mut runtime);
    update_game_frame_if_needed(world, &mut runtime);

    world.insert_non_send_resource(runtime);
}

fn reload_game_if_needed(world: &mut World, runtime: &mut ShellRuntimeResource) {
    #[cfg(not(target_arch = "wasm32"))]
    let should_reload = if runtime.hot_reload_enabled {
        runtime.game_source.rebuild_if_changed(&mut runtime.context)
            && runtime.game.artifact_changed()
    } else {
        false
    };

    #[cfg(target_arch = "wasm32")]
    let should_reload = false;

    if !should_reload {
        return;
    }

    println!("change detected; reloading game DLL");
    runtime.reload_count += 1;
    runtime.context.reload_count = runtime.reload_count;
    runtime.context.frame_local_count = 0;
    runtime.game.cleanup(world);
    runtime.game.unload(&mut runtime.context);

    match platform_system::GameRuntime::load(
        world,
        &mut runtime.context,
        false,
        runtime.reload_count,
    ) {
        Ok(game) => runtime.game = game,
        Err(error) => eprintln!("failed to load game DLL: {error}"),
    }
}

fn update_game_frame_if_needed(world: &mut World, runtime: &mut ShellRuntimeResource) {
    let elapsed = runtime.last_game_frame.elapsed();
    if elapsed < Duration::from_secs_f32(1.0 / 60.0) {
        return;
    }

    runtime.context.frame_global_count += 1;
    runtime.context.frame_local_count += 1;
    runtime.game.frame(world, &mut runtime.context);
    runtime.last_game_frame = Instant::now();
}
