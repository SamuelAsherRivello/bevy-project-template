# AGENTS.md — AI Agent Guidance

This file tells AI coding agents how to work effectively in this repository.
Read it before making any changes.

The active project root is `Bevy/`. Unless stated otherwise, all project paths below are relative to `Bevy/`.

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
bevy-project-template/
├── AGENTS.md                  ← you are here (canonical)
└── Bevy/
    ├── Cargo.toml             ← workspace root + main crate dependencies
    ├── Cargo.lock
    ├── build.rs               ← embeds Windows icon resource
    ├── index.html             ← Trunk web entry point
    ├── Trunk.toml             ← Trunk (WASM bundler) config
    │
    ├── documentation/
    │   ├── credits/           ← CREDITS.md + third-party licenses
    │   └── images/            ← screenshots, diagrams
    │
    ├── assets/
    │   ├── audio/
    │   │   ├── Click01.ogg
    │   │   └── Click02.mp3
    │   └── textures/
    │       └── bevy.png
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
    │   │   │   │   ├── CustomDefaultsPlugin.rs ← replaces DefaultPlugins defaults
    │   │   │   │   ├── GamePlugin.rs           ← top-level game plugin
    │   │   │   │   ├── HudPlugin.rs
    │   │   │   │   ├── InputPlugin.rs
    │   │   │   │   ├── InspectorPlugin.rs      ← bevy-inspector-egui integration
    │   │   │   │   ├── LoadingPlugin.rs        ← asset loading via bevy_asset_loader
    │   │   │   │   ├── MenuPlugin.rs
    │   │   │   │   └── PlayerPlugin.rs
    │   │   │   ├── Resources/          ← Resources, typed asset handles, input intent
    │   │   │   │   ├── Mod.rs
    │   │   │   │   ├── ActionsResource.rs
    │   │   │   │   ├── AssetsResource.rs
    │   │   │   │   └── DataResource.rs
    │   │   │   └── Systems/            ← Low-level system functions (non-plugin)
    │   │   │       ├── Mod.rs
    │   │   │       └── RotationSystem.rs
    │   │   └── Server/                 ← Future headless/server scaffold
    │   │       └── Mod.rs
    │   └── Tests/                      ← Headless in-crate test modules
    │       ├── Mod.rs
    │       └── PlayerPluginTests.rs    ← PlayerPlugin + GameState integration tests
    │
    ├── mobile/                    ← Android + iOS workspace crate
    └── build/                     ← Platform-specific build assets (icons, installer)
```

---

## Key conventions

| Convention | Detail |
|---|---|
| **Naming** | Within `src/`, every folder and file uses Title Case except `src` itself. |
| **Classification** | Put components in `Components/`, resources in `Resources/`, low-level system functions in `Systems/`, feature plugins in `Plugins/`. |
| **Suffixes** | Component files must be named `*Component.rs`, resource files `*Resource.rs`, system files `*System.rs`, and plugin files `*Plugin.rs`. |
| **Data vs behaviour** | `Components/` and `Resources/` hold data. `Systems/` and `Plugins/` hold `Plugin` impls and system functions. |
| **One plugin per feature** | Each file in `Systems/` and `Plugins/` is one self-contained feature plugin. |
| **State-gated systems** | All gameplay systems use `.run_if(in_state(GameState::Playing))`. |
| **Input abstraction** | Systems read `Res<Actions>`, never raw `ButtonInput` directly (except `Plugins/InputPlugin.rs`). |
| **Tests are headless** | Use `MinimalPlugins` in tests. Never require a display or audio device. |
| **`GameState` is `pub`** | It lives in `Client/Lib.rs` and must stay `pub` so tests and `Main.rs` can reference it. |

---

## How to run

```cmd
# Native (dev, fast iteration — dynamic linking + file watcher)
cargo run --features dev

# Native (without dev features)
cargo run

# Native (release)
cargo build --profile dist

# Web dev server (port 8080, auto-reload)
trunk serve

# Web dev server with WASM-safe dev features
trunk serve --features dev_wasm

# Web production build → dist/
trunk build --release

# Tests (headless, all platforms)
cargo test
```

> **Note:** `cargo run-dev`, `cargo check-wasm`, and `cargo build-wasm-release` aliases are **not currently present** — `.cargo/config.toml` does not exist in this repository. Use the explicit commands above instead.

Native fast iteration uses the `dev_native` feature (aliased as `dev`) which enables `bevy/dynamic_linking` + `bevy/file_watcher`.
The `dev_wasm` feature is a separate WASM-safe variant (no dynamic linking) intended for `trunk serve --features dev_wasm`.

---

## Key dependencies

| Crate | Purpose |
|---|---|
| `bevy 0.18` | Core engine (custom feature set — no default audio) |
| `bevy_kira_audio 0.25` | Audio (replaces `bevy_audio`; supports MP3 + OGG) |
| `bevy_asset_loader 0.25` | Declarative asset loading in `LoadingPlugin` |
| `bevy-inspector-egui 0.36` | Runtime inspector, toggled via `InspectorPlugin` |
| `rand 0.9` | General randomness |
| `webbrowser 1` | Open URLs from native/WASM via `HudPlugin` |

Do **not** use `bevy_audio` — it is intentionally excluded and will conflict with `bevy_kira_audio`.

---

## Before making changes

1. Check `Bevy/Cargo.toml` for the current Bevy version — do not upgrade it silently.
2. Run `cargo test` before and after your changes.
3. If you add a new game feature, follow the pattern:
   - Add components to `Components/` and resources to `Resources/` as needed
   - Add a feature plugin in `Plugins/` (or a low-level system function in `Systems/` if no plugin wrapper is needed)
   - Register the new plugin in `Client/Lib.rs` `GamePlugin::build`
   - Add at least one headless test module in `src/Tests/`

---

## Special Commands

### `/create_commit_message`

When the user message is exactly `/create_commit_message` or `create_commit_message` (or starts with either), do the following:

1. Run these git commands from the repository root (`bevy-project-template/`) to gather change context:
   ```
   git status --short
   git diff --staged
   git diff
   ```
2. If `git diff --staged` is empty, use `git diff` (unstaged changes) as the diff source.
   If both are empty, run `git diff HEAD~1 HEAD` to describe the most recent commit instead.
3. Read the full diff of changed files. For each changed file understand:
   - What was added, removed, or restructured
   - Which layer it belongs to (Component / Resource / Plugin / System / Test / Asset / Config)
4. Generate one high-quality commit message that accurately reflects the actual changes.
5. Return **only** the Markdown block below — no preamble, no explanation:

```md
## Commit Message

<type>(<scope>): <summary>

- <bullet 1>
- <bullet 2>
- <bullet 3>
```

**Rules:**
- Use [Conventional Commits](https://www.conventionalcommits.org/) types: `feat`, `fix`, `refactor`, `docs`, `chore`, `test`, `style`, `perf`, `ci`.
- `<scope>` should name the subsystem affected, e.g. `player`, `hud`, `loading`, `audio`, `inspector`, `wasm`, `agents`, `deps`.
- Summary line: imperative mood, ≤ 72 characters, no trailing period.
- Bullets: one meaningful fact per line; omit bullets if there is only one change.
- If changes span multiple concerns, pick the dominant intent as the type/scope and list secondary items in bullets.