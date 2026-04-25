# Bevy Project Template

Bevy project template with hot reload.

## Features

* Bevy game structure with best practices
* Targets Windows 11
* Optimized project compile times for full build
* Optimized sub-second hot reload times 

## IDE Setup

You can use any IDE, but here is the VS Code setup:

1. Download [VS Code](https://code.visualstudio.com/)
2. Open [VS Code](https://code.visualstudio.com/docs)
3. Open [this repository folder](./)
4. Add the [rust-analyzer extension](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)

## Scripts

### Common

| # | Script | Platform | Required? | Use |
| -- | ------ | -------- | --------- | --- |
| 01 | [`InstallDependencies.ps1`](./Scripts/Common/InstallDependencies.ps1) | Windows | ✅ | First-time setup and validation build. |
| 02 | [`RunGameWithHotReload.ps1`](./Scripts/Common/RunGameWithHotReload.ps1) | Windows | ✅ | Build and run the active game with hot reload. |

> Try hot reload? Run 01 and 02, then edit line 12 in `Bevy/Crates/Game/Runtime/Systems/PlayerSystem.rs` (the `PLAYER_COLOR` constant) and save.

### Other

| # | Script | Platform | Required? | Use |
| -- | ------ | -------- | --------- | --- |
| 03 | [`RunGame.ps1`](./Scripts/Other/RunGame.ps1) | Windows | ❌ | Build and run the active game without hot reload. |
| 04 | [`RunGameTests.ps1`](./Scripts/Other/RunGameTests.ps1) | Windows | ❌ | Run the test suite. |
| 05 | [`BuildHotReloadCrate.ps1`](./Scripts/Other/BuildHotReloadCrate.ps1) | Windows | ❌ | Build the vendored hot-reload crate. |
| 06 | [`RunGameWithHotReloadWeb.ps1`](./Scripts/Other/RunGameWithHotReloadWeb.ps1) | Windows | ❌ | Build and run the game in browser/wasm with hot reload. |
| 07 | [`StopGame.ps1`](./Scripts/Other/StopGame.ps1) | Windows | ❌ | Stop running game processes. |

## Structure

### Crates

| Path | Description |
| ---- | ----------- |
| [`Bevy/Crates/Game`](./Bevy/Crates/Game) | Active Cargo package for the game |
| [`Bevy/Crates/HotReload/`](./Bevy/Crates/HotReload/) | 3rd party crate, imported so it can be updated to latest Bevy version |

### Details

| Path | Description |
| ---- | ----------- |
| [`Bevy/Crates/Game/Assets`](./Bevy/Crates/Game/Assets) | Game assets |
| [`Bevy/Crates/Game/Runtime/Components`](./Bevy/Crates/Game/Runtime/Components) | Bevy component types |
| [`Bevy/Crates/Game/Runtime/Plugins`](./Bevy/Crates/Game/Runtime/Plugins) | Bevy plugin wiring |
| [`Bevy/Crates/Game/Runtime/Resources`](./Bevy/Crates/Game/Runtime/Resources) | Bevy resource types |
| [`Bevy/Crates/Game/Runtime/Systems`](./Bevy/Crates/Game/Runtime/Systems) | Startup and update systems |
| [`Bevy/Crates/Game/Tests`](./Bevy/Crates/Game/Tests) | Unit tests for game behavior |

---

## Resources

| Resource | Link |
| -------- | ---- |
| Hot Reload | [`bevy_simple_subsecond_system`](https://crates.io/crates/bevy_simple_subsecond_system) |
| Inspiration | [`rust-project-template`](https://github.com/SamuelAsherRivello/rust-project-template) |

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
