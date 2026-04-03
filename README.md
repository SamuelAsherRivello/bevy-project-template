# A Bevy Game Template

This repo is a production-ready starting point for games built with the awesome [Bevy engine][bevy].

The template includes out-of-the-box builds for **Windows, Linux, macOS, Web (Wasm), Android, and iOS**.

![Bevy game screenshot](./Bevy/documentation/images/Screenshot.png)

<BR>

# Getting Started: Steps

### Build for Native Windows

1. Install [Rust](https://www.rust-lang.org/tools/install)
1. Clone or download this repository
1. Open a terminal in the repo root
1. Change into the active project with `cd Bevy`
1. Run the app with `cargo run`
1. Build an optimized version with `cargo build --profile dist`

### Build for Wasm Browser

1. Install [Rust](https://www.rust-lang.org/tools/install)
1. Install Trunk with `cargo install --locked trunk`
1. Install the wasm target with `rustup target add wasm32-unknown-unknown`
1. Clone or download this repository
1. Open a terminal in the repo root
1. Change into the active project with `cd Bevy`
1. Start the dev server with `trunk serve`
1. Build the production web version with `trunk build --release`

### Deploy Web Build To GitHub Pages

1. Trigger the `deploy-github-page` workflow
1. Activate GitHub Pages for your repository
1. Use the `gh-pages` branch created by the workflow
1. Wait a few minutes for the site to go live at `http://username.github.io/repository`

<BR>
<BR>

## Table of Contents

1. [About](#about)
1. [Overview](#overview)
1. [Best Practices](#best-practices)
1. [Resources](#resources)
1. [Credits](#credits)

<BR>
<BR>

## About

### Template Basics

This template contains the following:

| Name | Description |
|---|---|
| [`Bevy/AGENTS.md`](./Bevy/AGENTS.md) | Repo conventions, folder map, and guidance for AI coding agents. |
| [`Bevy/documentation/project_spec.md`](./Bevy/documentation/project_spec.md) | Project vision, architecture decisions, and extension guidance. |
| `Bevy/src/Runtime/Client/Components/` | ECS component types attached to entities. |
| `Bevy/src/Runtime/Client/Resources/` | Resources, typed asset handles, and input intent. |
| `Bevy/src/Runtime/Client/Systems/` | One feature plugin or system entrypoint per file. |
| `Bevy/src/Runtime/Client/Misc/` | UI, app entrypoints, and non-component/resource/system files. |
| `Bevy/src/Tests/` | Headless in-crate tests compiled only for test builds. |
| [`bevy_asset_loader`](https://github.com/NiklasEi/bevy_asset_loader) | Typed, state-driven asset loading. |
| [`bevy_kira_audio`](https://github.com/NiklasEi/bevy_kira_audio) | Cross-platform audio with good Web and Android support. |
| GitHub Actions workflows | CI plus release automation for native, web, and mobile targets. |

### Platform Support

| Platform | Workflow |
|---|---|
| Windows | `cargo build --profile dist` |
| Linux | `cargo build --profile dist` |
| macOS | `cargo build --profile dist` |
| Web (Wasm) | `trunk build --release` |
| Android | `cargo apk run --manifest-path Bevy/mobile/Cargo.toml` |
| iOS | `make run` inside `Bevy/mobile/` |

<BR>
<BR>

## Overview

### Features

This project is a living Bevy template. It gives you a clean starting point for architecture, input abstraction, typed asset loading, audio, tests, and multi-platform builds.

Core capabilities included today:

- A working **Bevy ECS-oriented layout** under `Bevy/src/`
- **`Components/`** for pure component data
- **`Resources/`** for actions, assets, and shared runtime state
- **`Systems/`** for plugins and systems, one feature per file
- **`Misc/`** for menu UI, app entrypoints, and non-ECS support code
- **`Shared/`** and **`Server/`** scaffolding ready to populate
- **`Tests/`** for headless Rust test modules
- Input abstraction from raw device input into an `Actions` resource
- A three-state flow: `Loading -> Menu -> Playing`
- A small example ["game"](https://niklasei.github.io/bevy_game_template/)
- Easy web iteration via [trunk] with `trunk serve`
- Native execution with `cargo run`
- CI checks for native platforms on every push
- Release workflows for Windows, Linux, macOS, Web, and mobile development/release pipelines

WARNING: if you work in a private repository, please be aware that macOS and Windows runners cost more build minutes. For public repositories the workflow runners are free.

### Structure

**Repository**
- `README.md` - Root entry point and migration-aware documentation
- `Bevy/` - Active project root

**Documentation**
- `Bevy/AGENTS.md` - Current conventions and folder map
- `Bevy/documentation/project_spec.md` - Architecture, state machine, dependencies, and test strategy

**Runtime**
- `Bevy/src/Runtime/Client/Components/` - ECS components
- `Bevy/src/Runtime/Client/Resources/` - Resources and input/action models
- `Bevy/src/Runtime/Client/Systems/` - Plugins and systems
- `Bevy/src/Runtime/Client/Misc/` - Main app files, menu UI, and support files
- `Bevy/src/Runtime/Shared/` - Shared utilities scaffold
- `Bevy/src/Runtime/Server/` - Future server/headless scaffold

**Testing**
- `Bevy/src/Tests/` - Headless in-crate test modules

**Build and Assets**
- `Bevy/build/` - Platform-specific build assets and packaging support
- `Bevy/assets/` - Runtime assets
- `Bevy/mobile/` - Android and iOS workspace crate

### Build Instructions

#### Native Build

Run from repo root:

```cmd
cd Bevy
cargo run
```

For an optimized distribution build from repo root:

```cmd
cd Bevy
cargo build --profile dist
```

#### Web Build

**Prerequisites (one-time setup):**

```cmd
cargo install --locked trunk
rustup target add wasm32-unknown-unknown
```

**Development server** (auto-rebuilds on code changes, served at `http://localhost:8080`) from repo root:

```cmd
cd Bevy
trunk serve
```

**Production web build** (outputs to `Bevy/dist/`) from repo root:

```cmd
cd Bevy
trunk build --release
```

### How To Use This Template

1. Click "Use this template" on the repository's page
1. Look for `ToDo` to use your own game name everywhere
1. [Update the icons as described below](#updating-the-icons)
1. Start coding

Platform entry points:

- Native app: `cargo run`
- Web build: `trunk serve`
- Android app: `cargo apk run --manifest-path Bevy/mobile/Cargo.toml`
- iOS app: `make run` inside `Bevy/mobile`

Web requirements:

- Install [trunk]: `cargo install --locked trunk`
- Install the wasm target: `rustup target add wasm32-unknown-unknown`
- The dev server runs on `8080` and automatically rebuilds and reloads

Android requirements:

- Follow the [Bevy example README Android setup][android-instructions]

iOS requirements:

- Install Xcode through the App Store
- Launch Xcode and install the iOS simulator
- Install the Rust targets with `rustup target add aarch64-apple-ios x86_64-apple-ios aarch64-apple-ios-sim`
- Run `make run` inside `Bevy/mobile`

You should keep the `Bevy/documentation/credits` directory up to date. The release workflow automatically includes the directory in every build.

### Updating The Icons

1. Replace `Bevy/build/macos/icon_1024x1024.png` with a `1024 x 1024` PNG icon and run `create_icns.sh` or `create_icns_linux.sh` inside `Bevy/build/macos`
1. Replace `Bevy/build/windows/icon.ico` for the Windows executable and the web favicon
1. Replace `Bevy/build/android/res/mipmap-mdpi/icon.png` with `macos/AppIcon.iconset/icon_256x256.png`, renamed to `icon.png`

Windows `.ico` creation steps:

1. Open `macos/AppIcon.iconset/icon_256x256.png` in [Gimp](https://www.gimp.org/downloads/)
1. Select `File > Export As`
1. Change the extension to `.ico`
1. Save as `Bevy/build/windows/icon.ico`

### Deploy Mobile Platforms

For general mobile support context, see [Notes on mobile development with Bevy][mobile_dev_with_bevy_2].

**Android**

Currently, `cargo-apk` is used to run the development app. APKs can no longer be published in the store and `cargo-apk` cannot produce the required AAB. Because of that, the repo includes setup for two Android-related tools:

- [`Bevy/mobile/Cargo.toml`](./Bevy/mobile/Cargo.toml) configures `cargo-apk` under `package.metadata.android`
- [`Bevy/mobile/manifest.yaml`](./Bevy/mobile/manifest.yaml) configures a custom fork of `xbuild` used by the `release-android-google-play` workflow to create an AAB

Additional reference: [Android release workflow setup][workflow_bevy_android]

**iOS**

The setup closely follows the Bevy mobile example.

Additional reference: [iOS release workflow setup][workflow_bevy_ios]

### Removing Mobile Platforms

If you do not want Android or iOS targets, delete:

- `Bevy/mobile`
- `Bevy/build/android`
- `Bevy/build/ios`

Then remove the `[workspace]` section from `Bevy/Cargo.toml`.

### Development Environments

#### Nix Support

`nixgl` is only used on non-NixOS Linux systems. When running there, use:

```cmd
nix develop --impure
```

If using `nixgl`, run `gl cargo run`; otherwise use `cargo` as usual.

<BR>
<BR>

## Best Practices

### Project Structure

The source is organized around Bevy ECS concepts instead of MVC naming.

Within `Bevy/src/`, every file and folder uses **Title Case** except the root `src` folder itself.

Scope includes:

- Folder structure
- Folder naming
- File naming
- Feature-oriented system/plugin files
- State-gated gameplay systems

Benefits:

- Consistency across the codebase
- Easier maintenance and onboarding
- Clearer communication about where new code belongs
- Better separation between data, resources, and behavior

### Coding And Architecture Standards

The project conventions are documented in [`Bevy/AGENTS.md`](./Bevy/AGENTS.md) and [`Bevy/documentation/project_spec.md`](./Bevy/documentation/project_spec.md).

Current rules include:

- `Components/` holds ECS components only
- `Resources/` holds resources and typed asset/input models
- `Systems/` holds `Plugin` implementations and system functions
- `Misc/` holds app entrypoints, menu UI, and everything that does not fit the other runtime folders
- Gameplay systems should use `.run_if(in_state(GameState::Playing))`
- Systems should consume `Res<Actions>` rather than raw `ButtonInput`, except in `Systems/InputSystem.rs`
- Tests stay headless and should use `MinimalPlugins`

Benefits:

- Cleaner code integration
- Easier testability
- Lower coupling between systems and devices
- More predictable feature growth
- Faster reuse of the template on new game projects

<BR>
<BR>

## Resources

### Learn Rust

If you want a fast Rust refresher before diving into the project, start with [A half-hour to learn Rust][learn-rust].

### Learn Bevy

For Bevy-specific learning, use the [Bevy learning page][bevy-learn], the [Bevy Cheat Book][Bevy Cheat Book], and the [official Bevy Discord server][bevy-discord].

Video tutorial: [Bevy introduction and walkthrough][bevy-youtube-intro].

For platform setup details, the Bevy example README covers [Android][android-instructions] and [iOS][ios-instructions].

### Known Issues

Audio in web builds can have issues in some browsers. This seems to be a general performance issue and not due to the audio itself. See [bevy_kira_audio/#9][firefox-sound-issue].

### License

This project is licensed under [CC0 1.0 Universal](LICENSE) except some content of `assets` and the Bevy icons in the `build` directory. See [Credits](./Bevy/documentation/credits/CREDITS.md).

<BR>
<BR>

## Credits

### Original

- Template source: <a href="https://github.com/NiklasEi/bevy_game_template">https://github.com/NiklasEi/bevy_game_template</a>

### New

**Created By**

- Samuel Asher Rivello
- Over 25 years XP with game development (2025)
- Over 10 years XP with Unity (2025)

**Contact**

- Twitter - <a href="https://twitter.com/srivello/">@srivello</a>
- Git - <a href="https://github.com/SamuelAsherRivello/">Github.com/SamuelAsherRivello</a>
- Resume & Portfolio - <a href="http://www.SamuelAsherRivello.com">SamuelAsherRivello.com</a>
- LinkedIn - <a href="https://Linkedin.com/in/SamuelAsherRivello">Linkedin.com/in/SamuelAsherRivello</a> <--- Say Hello! :)

**License**

Provided as-is under <a href="./LICENSE">MIT License</a> | Copyright ™ & © 2006 - 2026 Rivello Multimedia Consulting, LLC

[bevy]: https://bevyengine.org/
[learn-rust]: https://fasterthanli.me/articles/a-half-hour-to-learn-rust
[bevy-learn]: https://bevyengine.org/learn/
[bevy-youtube-intro]: https://www.youtube.com/watch?v=yFOPtYwnDjU&list=WL&index=9&t=2517s
[bevy-discord]: https://discord.gg/bevy
[nikl-twitter]: https://twitter.com/nikl_me
[nikl-mastodon]: https://mastodon.online/@nikl_me
[firefox-sound-issue]: https://github.com/NiklasEi/bevy_kira_audio/issues/9
[Bevy Cheat Book]: https://bevy-cheatbook.github.io/introduction.html
[trunk]: https://trunkrs.dev/
[android-instructions]: https://github.com/bevyengine/bevy/blob/latest/examples/README.md#setup
[ios-instructions]: https://github.com/bevyengine/bevy/blob/latest/examples/README.md#setup-1
[mobile_dev_with_bevy_2]: https://www.nikl.me/blog/2023/notes_on_mobile_development_with_bevy_2/
[workflow_bevy_android]: https://www.nikl.me/blog/2023/github_workflow_to_publish_android_app/
[workflow_bevy_ios]: https://www.nikl.me/blog/2023/github_workflow_to_publish_ios_app/
