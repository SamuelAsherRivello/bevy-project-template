# AGENTS.md

## Project Overview

This template currently runs as a typical Bevy game from the `game` crate. The `game` crate owns the Bevy `App`, window, world setup, player, input, UI, and frame context.

Native hot reload was a project goal, but that goal is not currently met. The `game_shell` and `game_api` crates remain in the workspace as experimental scaffolding from the hot-reload effort, but they are not used by the primary run workflow.

## Active Architecture

| Crate | Path | Role | Active? |
|---|---|---|---|
| `game` | `rust/crates/game` | Main Bevy app and normal run target. | ✅ |
| `game-api` | `rust/crates/game_api` | Legacy ABI/shared-types experiment for hot reload. | ❌ |
| `game-shell` | `rust/crates/game_shell` | Legacy window-hosting/hot-reload shell experiment. | ❌ |

The active app is launched with:

```powershell
powershell.exe -ExecutionPolicy Bypass -File ./scripts/RunProject.ps1
```

Direct Cargo command:

```powershell
cargo run -p game
```

## Hot Reload Status

Do not describe native hot reload as working. It was a goal, but it is not currently implemented in the active workflow.

The previous DLL shell design conflicted with the goal of keeping `game` close to a typical Bevy project. Bevy resources/components such as `Assets<Mesh>`, `ButtonInput<KeyCode>`, and other ECS types do not safely cross a separately linked shell/DLL boundary. The template now prioritizes the typical Bevy project shape.

`RunProjectWithHotReload.ps1` is retained for compatibility. It prints that native hot reload no longer works and then starts the normal `game` app.

`RunProjectWithHotReloadWasm.ps1` and `StopProject.ps1` are obsolete. They print a message and exit.

## Developer Workflows

### First-Time Setup

```powershell
powershell.exe -ExecutionPolicy Bypass -File ./scripts/InstallProject.ps1
```

### Run Active Game

```powershell
powershell.exe -ExecutionPolicy Bypass -File ./scripts/RunProject.ps1
```

### Build Only

```powershell
cargo build -p game
cargo check -p game
```

## Key Active Files

- `rust/crates/game/src/main.rs` — runs the Bevy app and forces DX12 to avoid Vulkan overlay loader errors on Windows.
- `rust/crates/game/src/lib.rs` — module declarations and `create_app()`.
- `rust/crates/game/src/Plugins/GamePlugin.rs` — top-level game plugin wiring.
- `rust/crates/game/src/Plugins/WorldPlugin.rs` — world setup plugin.
- `rust/crates/game/src/Plugins/InputPlugin.rs` — input plugin.
- `rust/crates/game/src/Plugins/PlayerPlugin.rs` — player plugin.
- `rust/crates/game/src/Systems/WorldSystem.rs` — camera, light, and floor setup.
- `rust/crates/game/src/Systems/InputSystem.rs` — reads Bevy keyboard input into `InputComponent`.
- `rust/crates/game/src/Systems/PlayerSystem.rs` — moves/animates the player from `InputComponent`.
- `rust/crates/game/src/Systems/ContextSystem.rs` — increments frame counters.
- `rust/crates/game/src/Resources/ContextResource.rs` — `reload_count`, `frame_global_count`, and `frame_local_count`.
- `rust/crates/game/src/Systems/UISystem.rs` — UI text setup and updates.

## Conventions

Apply these conventions in active `game` code:

- **Components:** component type names end with `Component`; exactly one component type per file; component filenames end with `Component.rs`.
- **Plugins:** type names end with `Plugin`; plugin filenames end with `Plugin.rs`.
- **Resources:** type names end with `Resource`; resource filenames end with `Resource.rs`.
- **Systems:** scheduled/public system functions live under `Systems/`.
- **System files:** system filenames end with `System.rs`.
- **System function names:**
  - startup schedule: `*_startup_system`
  - update schedule: `*_update_system`
- **Plugin wiring style:**
  - `.add_systems(Startup, some_startup_system)`
  - `.add_systems(Update, some_update_system)`

Avoid adding new active dependencies on `game_shell` or `game_api` unless the user explicitly asks to revisit the hot-reload architecture.
