# AGENTS.md

## Project Overview

Minimal Rust hot-reload demo for Windows. A long-running shell executable (`game_shell`) dynamically loads and reloads a game DLL (`game`) at runtime without restarting the window. Shared types live in `game_api`. The same crates also support a browser workflow (`wasm32`) with page live reload.

## Architecture

### Three-Crate Structure

| Crate | Path | Output | Hot Reloadable |
|---|---|---|---|
| `game-api` | `rust/crates/game_api` | `rlib` | ❌ — must stay ABI-stable |
| `game` | `rust/crates/game` | `cdylib` + `lib` | ✅ — only crate rebuilt on save |
| `game-shell` | `rust/crates/game_shell` | binary (`game-shell`) + wasm app | ❌ — owns the runtime/window |

**Critical constraint:** `game-api` is the ABI boundary. It must never contain types that change layout between reloads. The shell and game both link against it statically; only the game DLL is swapped at runtime.

### Hot-Reload Data Flow

1. `GameSourceWatcher` polls `rust/crates/game/src/**/*.rs` modification times each frame.
2. On change, `game_shell` runs `cargo build -p game` in a subprocess.
3. Shell copies `target/debug/game.dll` → `target/hot-reload/game_hot_<timestamp>.dll` and loads the copy via `libloading` (Windows locks the original).
4. Exported C ABI symbols resolved by the shell: `AppInitialize`, `AppHotReload`, `hot_frame`, `hot_render_packet`.
5. Platform dispatch lives in `game_shell/src/Systems/PlatformSystem.rs`: native uses DLL swap (`NativePlatformSystem.rs`), wasm calls into `game` directly (`WasmPlatformSystem.rs`) and relies on dev-server/page reload.

## Developer Workflows

### First-Time Setup
```powershell
powershell.exe -ExecutionPolicy Bypass -File ./scripts/InstallProject.ps1
```

### Run with Hot Reload (primary workflow)
```powershell
powershell.exe -ExecutionPolicy Bypass -File ./scripts/RunProjectWithHotReload.ps1
```
Edit any `.rs` file under `rust/crates/game/src/` and save — the shell detects the change, rebuilds, and reloads without closing the window.

### Run in Browser (wasm live reload)
```powershell
powershell.exe -ExecutionPolicy Bypass -File ./scripts/RunProjectWithHotReloadWasm.ps1
```
Uses `trunk serve` with file polling and page live reload (not native DLL hot swap).

### Run Without Hot Reload
```powershell
powershell.exe -ExecutionPolicy Bypass -File ./scripts/RunProject.ps1
powershell.exe -ExecutionPolicy Bypass -File ./scripts/StopProject.ps1
```

### Build Only
```powershell
cargo build          # full workspace
cargo build -p game  # game DLL only (faster iteration)
cargo build -p game-shell
cargo build -p game-api
```

### Clean Hot-Reload Copies
```powershell
.\scripts\RunProjectWithHotReload.ps1 -CleanHotReloadCopies
```
Removes accumulated `target/hot-reload/game_hot_*.dll` files before starting.

## Key Files

- `rust/crates/game/src/lib.rs` — exported C ABI entry points and `GAME_STATE` lifecycle (`AppInitialize`, `AppHotReload`, `hot_frame`, `hot_render_packet`)
- `rust/crates/game/src/Plugins/PlayerPlugin.rs` + `rust/crates/game/src/Components/PlayerComponent.rs` — canonical gameplay state/setup example
- `rust/crates/game/src/Resources/UIResource.rs` — UI text resource and fixed-capacity byte packing into `RenderPacket`
- `rust/crates/game_shell/src/main.rs` — app bootstrap, plugin registration, and runtime startup
- `rust/crates/game_shell/src/Systems/NativePlatformSystem.rs` — DLL loading/copying, source watcher, and native hot-reload orchestration
- `rust/crates/game_shell/src/Systems/WasmPlatformSystem.rs` — wasm runtime path that calls `game` directly
- `rust/crates/game_shell/src/Plugins/ShellPlugin.rs` — schedule wiring for startup/update systems
- `rust/crates/game_api/src/lib.rs` — `Context` trait and ABI-safe render data types

## Conventions

- **Adding a new shell→game call:** Add an `extern "C"` function in `game/src/lib.rs`; in native path add a matching function-pointer type alias and symbol load in `game_shell/src/Systems/NativePlatformSystem.rs` (`GameRuntime::load`); in wasm path call the Rust function from `game_shell/src/Systems/WasmPlatformSystem.rs`.
- **Adding new game state:** Keep reload-reset game state inside `GAME_STATE` in `game/src/lib.rs` (`thread_local!` + `RefCell<Option<GameState>>`) and rebuild it from `AppHotReload`.
- **UI widgets/text:** Game-side UI text is pushed through `RenderPacket.ui_text`; shell-side overlay rendering lives in `game_shell/src/Systems/GameLoopSystem.rs` (`shell_update_overlay_text_update_system`).
- **Bevy usage:** `game` uses `bevy_app`/`bevy_ecs`, and `game_shell` uses full `bevy` + `bevy-inspector-egui`; keep `game_api` free of Bevy-specific ABI types.
- Rust edition `2024` is used across all crates.

## Naming Standards (Enforced)

Apply these conventions in `game`, `game_shell`, and `game_api`:

- **Components:** component type names end with `Component`; exactly one component type per file; component filenames must end with `Component.rs` but do not need to exactly match the type name.
- **Plugins:** type names end with `Plugin`; plugin filenames end with `Plugin.rs`.
- **Resources:** type names end with `Resource`; resource filenames end with `Resource.rs`.
- **Systems:** in `game_shell` and `game_api`, scheduled/public system functions live in files under `Systems/`; small local startup helpers may be colocated in plugin files (for example, `player_startup_system` in `game/src/Plugins/PlayerPlugin.rs`).
- **System files:** system filenames end with `System.rs`.
- **System file grouping:** one `*System.rs` file may contain one or more related systems.
- **System function names (scheduled systems):**
  - startup schedule: `*_startup_system`
  - update schedule: `*_update_system`
- **Plugin wiring style:**
  - `.add_systems(Startup, some_startup_system)`
  - `.add_systems(Update, some_update_system)`
