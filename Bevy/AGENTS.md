# AGENTS.md — AI Agent Guidance

This file tells AI coding agents how to work effectively in this repository.
Read it before making any changes.

---

## What this project is

A **Bevy 0.18 / Rust game template** with out-of-the-box builds for:
- Native (Windows, Linux, macOS) via `cargo run`
- Web (WASM) via `trunk serve`
- Mobile (Android / iOS) via the `mobile/` workspace crate

The code is structured around Bevy ECS concepts instead of MVC labels.
Inside `src/`, use **Title Case** for every folder and file name except the root `src` folder itself.

---

## Repository layout

```
Bevy/
├── AGENTS.md                  ← you are here
├── Cargo.toml                 ← workspace root + main crate dependencies
├── Cargo.lock
├── build.rs                   ← embeds Windows icon resource
├── index.html                 ← Trunk web entry point
├── Trunk.toml                 ← Trunk (WASM bundler) config
│
├── documentation/
│   ├── credits/               ← CREDITS.md + third-party licenses
│   └── images/                ← screenshots, diagrams
│
├── assets/
│   ├── audio/
│   │   ├── Click01.ogg
│   │   └── Click02.mp3
│   └── textures/
│       ├── bevy.png
│       └── github.png
│
├── src/
│   ├── Runtime/
│   │   ├── Client/
│   │   │   ├── Lib.rs              ← GamePlugin, GameState
│   │   │   ├── Main.rs             ← binary + web entrypoint
│   │   │   ├── Components/         ← ECS component types
│   │   │   │   ├── Mod.rs
│   │   │   │   ├── PlayerComponent.rs
│   │   │   │   └── RotationComponent.rs
│   │   │   ├── Plugins/            ← Feature-level Bevy Plugin impls
│   │   │   │   ├── Mod.rs
│   │   │   │   ├── MenuPlugin.rs
│   │   │   │   └── PlayerPlugin.rs
│   │   │   ├── Resources/          ← Resources, typed asset handles, input intent
│   │   │   │   ├── Mod.rs
│   │   │   │   ├── ActionsResource.rs
│   │   │   │   ├── AssetsResource.rs
│   │   │   │   └── DataResource.rs
│   │   │   └── Systems/            ← System-function-heavy plugin entrypoints
│   │   │       ├── Mod.rs
│   │   │       ├── AudioSystem.rs
│   │   │       ├── HudSystem.rs
│   │   │       ├── InputSystem.rs
│   │   │       ├── LoadingSystem.rs
│   │   │       ├── PlayerSystem.rs
│   │   │       └── RotationSystem.rs
│   │   └── Server/                 ← Future headless/server scaffold
│   │       └── Mod.rs
│   └── Tests/                      ← Headless in-crate test modules
│       ├── Mod.rs
│       ├── ModelTests.rs           ← Actions, GameControl, get_movement
│       └── PlayerTests.rs          ← Player component, GameState machine
│
├── mobile/                    ← Android + iOS workspace crate
└── build/                     ← Platform-specific build assets (icons, installer)
```

---

## Key conventions

| Convention | Detail |
|---|---|
| **Naming** | Within `src/`, every folder and file uses Title Case except `src` itself. |
| **Classification** | Put components in `Components/`, resources in `Resources/`, system-heavy plugins in `Systems/`, feature plugins in `Plugins/`. |
| **Suffixes** | Component files must be named `*Component.rs`, resource files `*Resource.rs`, system files `*System.rs`, and plugin files `*Plugin.rs`. |
| **Data vs behaviour** | `Components/` and `Resources/` hold data. `Systems/` and `Plugins/` hold `Plugin` impls and system functions. |
| **One plugin per feature** | Each file in `Systems/` and `Plugins/` is one self-contained feature plugin. |
| **State-gated systems** | All gameplay systems use `.run_if(in_state(GameState::Playing))`. |
| **Input abstraction** | Systems read `Res<Actions>`, never raw `ButtonInput` directly (except `Systems/InputSystem.rs`). |
| **Tests are headless** | Use `MinimalPlugins` in tests. Never require a display or audio device. |
| **`GameState` is `pub`** | It lives in `Client/Lib.rs` and must stay `pub` so tests and `Main.rs` can reference it. |

---

## How to run

```cmd
# Native (dev, fast iteration)
cargo run-dev

# Native (without dev feature path)
cargo run

# Native (release)
cargo build --profile dist

# Web dev server (port 8080, auto-reload)
trunk serve

# Wasm compile-only check (fast feedback)
cargo check-wasm

# Web production build → dist/
trunk build --release

# Optimized wasm binary (without Trunk bundling)
cargo build-wasm-release

# Tests (headless, all platforms)
cargo test
```

Cargo aliases are configured in `Bevy/.cargo/config.toml`.
Native fast iteration uses the `dev_native` feature (`bevy/dynamic_linking` + `bevy/file_watcher`).

---

## Before making changes

1. Check `Cargo.toml` for the current Bevy version — do not upgrade it silently.
2. Run `cargo test` before and after your changes.
3. If you add a new game feature, follow the pattern:
   - Add components to `Components/` and resources to `Resources/` as needed
   - Add a system-entrypoint file in `Systems/` or a feature plugin in `Plugins/`
   - Register the new plugin in `Client/Lib.rs` `GamePlugin::build`
   - Add at least one headless test module in `src/Tests/`