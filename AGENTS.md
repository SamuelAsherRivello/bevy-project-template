﻿# AGENTS.md — AI Agent Guidance

> Migration note: the canonical project now lives in `Bevy/`.
> Use `Bevy/AGENTS.md` for current structure and conventions.
> Naming rule: inside `Bevy/src`, every file and folder must use Title Case except the root `src` folder itself.

This file tells AI coding agents how to work effectively in this repository.
Read it before making any changes.

---

## What this project is

A **Bevy 0.18 / Rust game template** with out-of-the-box builds for:
- Native (Windows, Linux, macOS) via `cargo run`
- Web (WASM) via `trunk serve`
- Mobile (Android / iOS) via the `mobile/` workspace crate

The code is structured to mirror the `model / systems / view` separation familiar
from web front-end templates (Babylon.js, React, etc.), adapted for Bevy's ECS.

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
│   ├── project_spec.md        ← project requirements and architecture decisions
│   └── images/                ← screenshots, diagrams
│
├── assets/
│   ├── audio/flying.ogg
│   └── textures/bevy.png, github.png
│
├── src/
│   ├── main.rs                ← binary entry point (window, icon, GamePlugin)
│   ├── lib.rs                 ← GamePlugin + GameState machine
│   │
│   ├── model/                 ← DATA LAYER: pure types, no Plugin impls
│   │   ├── mod.rs
│   │   ├── actions.rs         ← Actions resource, GameControl enum, get_movement
│   │   ├── assets.rs          ← AudioAssets, TextureAssets (bevy_asset_loader)
│   │   └── player.rs          ← Player marker component
│   │
│   ├── systems/               ← SYSTEMS LAYER: Bevy Plugins + Systems ("addX" pattern)
│   │   ├── mod.rs
│   │   ├── actions.rs         ← ActionsPlugin, set_movement_actions
│   │   ├── audio.rs           ← InternalAudioPlugin (bevy_kira_audio)
│   │   ├── loading.rs         ← LoadingPlugin (bevy_asset_loader)
│   │   └── player.rs          ← PlayerPlugin (spawn + movement)
│   │
│   ├── view/                  ← VIEW LAYER: UI Plugins and display logic
│   │   ├── mod.rs
│   │   └── menu.rs            ← MenuPlugin (main menu, buttons, state transitions)
│   │
│   ├── shared/                ← Shared utilities (math helpers, events, constants)
│   │   └── mod.rs             ← empty — populate as needed
│   │
│   └── server/                ← Future server/headless logic
│       └── mod.rs             ← empty — would become its own workspace crate
│
├── tests/                     ← Cargo integration tests (headless, no window)
│   ├── model_tests.rs         ← Actions, GameControl, get_movement
│   └── player_tests.rs        ← Player component, GameState machine
│
├── mobile/                    ← Android + iOS workspace crate
└── build/                     ← Platform-specific build assets (icons, installer)
```

---

## Key conventions

| Convention | Detail |
|---|---|
| **Data vs behaviour** | `model/` holds plain data types. `systems/` holds all `Plugin` impls and `fn system()` bodies. Never put Plugin impls in `model/`. |
| **One plugin per feature** | Each file in `systems/` is one self-contained plugin. Match the Babylon `addX.ts` naming spirit. |
| **State-gated systems** | All gameplay systems use `.run_if(in_state(GameState::Playing))`. |
| **Input abstraction** | Systems read `Res<Actions>`, never raw `ButtonInput` directly (except `systems/actions.rs`). |
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

1. Read `documentation/project_spec.md` for the project vision and constraints.
2. Check `Cargo.toml` for the current Bevy version — do not upgrade it silently.
3. Run `cargo test` before and after your changes.
4. If you add a new game feature, follow the pattern:
   - Add data types to `model/`
   - Add a `Plugin` in `systems/`
   - Register the plugin in `lib.rs` `GamePlugin::build`
   - Add at least one integration test in `tests/`
