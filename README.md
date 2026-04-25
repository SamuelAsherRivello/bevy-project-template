# Bevy Project Template

Bevy project template focused on a typical Bevy game crate.

## Current Status

The active app is [`game`](./rust/crates/game). It creates and runs a normal Bevy `App`, owns the window, creates the world/camera/lights/floor/player/UI, and reads keyboard input through normal Bevy resources.

Native hot reload was a goal for this template, but that goal is not currently met. The existing `game_shell` and `game_api` crates remain in the workspace as experimental scaffolding from that effort, but they are not used by the primary run workflow.

## Features

* Typical Bevy app structure in the `game` crate
* Game-owned world setup through `WorldPlugin`
* Game-owned input through `InputPlugin` and `InputComponent`
* Game-owned player setup through `PlayerPlugin`
* Game-owned UI and frame context resources
* Windows native run workflow
* Faster dev compile settings in Cargo config

## Getting Started

| Script | Description | Required? |
| ------ | ----------- | --------- |
| [`InstallProject.ps1`](./scripts/InstallProject.ps1) | Installs Rust with `rustup` if needed and builds the workspace once. | ✅ |
| [`RunProject.ps1`](./scripts/RunProject.ps1) | Builds and runs the active `game` Bevy app. | ✅ |
| [`RunProjectWithHotReload.ps1`](./scripts/RunProjectWithHotReload.ps1) | Prints that hot reload no longer works, then runs `game` normally. | ❌ |
| [`BuildProject.ps1`](./scripts/BuildProject.ps1) | Builds the workspace without running it. | ❌ |
| [`RunProjectWithHotReloadWasm.ps1`](./scripts/RunProjectWithHotReloadWasm.ps1) | Obsolete; prints a message and exits. | ❌ |
| [`StopProject.ps1`](./scripts/StopProject.ps1) | Obsolete; prints a message and exits. | ❌ |

## Structure

| Crate | Description | Active? |
| ----- | ----------- | ------- |
| [`game`](./rust/crates/game) | Main Bevy app. This is the crate to edit and run. | ✅ |
| [`game_api`](./rust/crates/game_api) | ABI/shared-types experiment from the hot-reload goal. Not used by the active game app. | ❌ |
| [`game_shell`](./rust/crates/game_shell) | Window-hosting/hot-reload shell experiment. Not used by the active run workflow. | ❌ |

## Build And Run

```powershell
powershell.exe -ExecutionPolicy Bypass -File ./scripts/RunProject.ps1
```

Direct Cargo command:

```powershell
cargo run -p game
```

## Hot Reload Note

The intended native hot-reload design was to keep the window open while swapping game code. That is not currently implemented in the active workflow.

The previous DLL approach conflicted with the goal of making `game` feel like a typical Bevy project because Bevy resources and component type identities do not safely cross the shell/DLL boundary when both sides link Bevy separately. For now, the template chooses the typical Bevy app experience over native DLL hot reload.

## Resources

- Rust hot reload background:<br> https://johnaustin.io/articles/2022/hot-reloading-rust
- Inspired by:<br> https://github.com/SamuelAsherRivello/rust-project-template

---

## Credits

**Created By**

- Samuel Asher Rivello
- Over 25 years XP with game development (2025)
- Over 10 years XP with Unity (2025)

**Contact**

- Twitter - [@srivello](https://twitter.com/srivello)
- Git - [Github.com/SamuelAsherRivello](https://github.com/SamuelAsherRivello)
- Resume & Portfolio - [SamuelAsherRivello.com](https://www.SamuelAsherRivello.com)
- LinkedIn - [Linkedin.com/in/SamuelAsherRivello](https://www.linkedin.com/in/SamuelAsherRivello)

**License**

Provided as-is under [MIT License](./LICENSE) | Copyright ™ & © 2006 - 2026 Rivello Multimedia Consulting, LLC
