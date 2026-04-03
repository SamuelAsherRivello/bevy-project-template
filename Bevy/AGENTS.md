﻿# AGENTS.md — AI Agent Guidance

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
bevy_game_template_fork/
├── AGENTS.md                  ← you are here
├── README.md                  ← build instructions + platform guide
├── Cargo.toml                 ← workspace root + main crate dependencies
├── Cargo.lock
├── build.rs                   ← embeds Windows icon resource
├── index.html                 ← Trunk web entry point
├── Trunk.toml                 ← Trunk (WASM bundler) config
├── .cargo/config.toml         ← getrandom wasm_js backend cfg flag
│
├── documentation/
│   └── images/                ← screenshots, diagrams
│
├── assets/
│   ├── audio/flying.ogg
│   └── textures/bevy.png, github.png
│
├── src/
│   ├── Runtime/
│   │   ├── Client/
│   │   ├── Components/
│   │   │   ├── Mod.rs
│   │   │   └── PlayerComponent.rs
│   │   ├── Resources/
│   │   │   ├── Mod.rs
│   │   │   ├── ActionsResource.rs
│   │   │   └── AssetsResource.rs
│   │   ├── Systems/
│   │   │   ├── Mod.rs
│   │   │   ├── InputSystem.rs
│   │   │   ├── AudioSystem.rs
│   │   │   ├── LoadingSystem.rs
│   │   │   └── PlayerSystem.rs
│   │   └── Misc/
│   │       ├── Mod.rs
│   │       ├── Lib.rs
│   │       ├── Main.rs
│   │       └── Menu.rs
│   │   ├── Shared/
│   │   │   └── Mod.rs
│   │   └── Server/
│   │       └── Mod.rs
│   └── Tests/                 ← Headless in-crate test modules
│       ├── Mod.rs
│       ├── ModelTests.rs      ← Actions, GameControl, get_movement
│       └── PlayerTests.rs     ← Player component, GameState machine
│
├── mobile/                    ← Android + iOS workspace crate
└── build/                     ← Platform-specific build assets (icons, installer)
```

---

## Key conventions

| Convention | Detail |
|---|---|
| **Naming** | Within `src/`, every folder and file uses Title Case except `src` itself. |
| **Classification** | Put components in `Components/`, resources in `Resources/`, systems/plugins in `Systems/`, and everything else in `Misc/`. |
| **Suffixes** | Component files must be named `*Component.rs`, resource files `*Resource.rs`, and system files `*System.rs`. |
| **Data vs behaviour** | `Components/` and `Resources/` hold data. `Systems/` holds `Plugin` impls and system functions. |
| **One plugin per feature** | Each file in `Systems/` is one self-contained feature plugin or system entrypoint. |
| **State-gated systems** | All gameplay systems use `.run_if(in_state(GameState::Playing))`. |
| **Input abstraction** | Systems read `Res<Actions>`, never raw `ButtonInput` directly (except `Systems/InputSystem.rs`). |
| **Tests are headless** | Use `MinimalPlugins` in tests. Never require a display or audio device. |
| **`GameState` is `pub`** | It lives in `lib.rs` and must stay `pub` so integration tests and `main.rs` can reference it. |

---

## How to run

```cmd
# Native (dev)
cargo run

# Native (release)
cargo build --profile dist

# Web dev server (port 8080, auto-reload)
trunk serve

# Web production build → dist/
trunk build --release

# Tests (headless, all platforms)
cargo test
```

---

## Before making changes

1. Check `Cargo.toml` for the current Bevy version — do not upgrade it silently.
2. Run `cargo test` before and after your changes.
3. If you add a new game feature, follow the pattern:
   - Add components to `Components/` and resources to `Resources/` as needed
   - Add a system/plugin file in `Systems/`
   - Put non-component/resource/system files in `Misc/`
   - Register the plugin in `Misc/Lib.rs` `GamePlugin::build`
   - Add at least one headless test module in `src/Tests/`
