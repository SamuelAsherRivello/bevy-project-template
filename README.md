# Bevy Project Template - With Hot Reload

Bevy project template.

Features:

* Hot reload for fast iteration
* App window remembers screen position
* App window persists during hot reload
* App is Windows native
* App also runs in the browser via wasm

## Getting Started


| Script | Description | Required? |
| ------ | ----------- | --------- |
| [`InstallProject.ps1`](./scripts/InstallProject.ps1) | Installs Rust with `rustup` if needed and builds the workspace once. | ✅ |
| [`RunProjectWithHotReload.ps1`](./scripts/RunProjectWithHotReload.ps1) | Starts the app with hot reload enabled. Edit files under [`rust/crates/game/src`](./rust/crates/game/src) and save to rebuild the game DLL. | ✅ |
| [`RunProjectWithHotReloadWasm.ps1`](./scripts/RunProjectWithHotReloadWasm.ps1) | Starts the wasm build in a browser using `trunk serve` with rebuild and page live reload. | ❌ |
| [`BuildProject.ps1`](./scripts/BuildProject.ps1) | Builds the project without running it. | ❌ |
| [`RunProject.ps1`](./scripts/RunProject.ps1)  | Starts the project without hot reload. | ❌ |
| [`StopProject.ps1`](./scripts/StopProject.ps1) | Stops the non-hot-reload run. | ❌ |

## Structure


| Crate                                    | Description                                                                                                                 | Hot Reloaded? |
| ---------------------------------------- | --------------------------------------------------------------------------------------------------------------------------- | ------------- |
| [`game`](./rust/crates/game)             | Reloadable game DLL. This is the crate rebuilt and swapped during hot reload.                                               | ✅            |
| [`game_api`](./rust/crates/game_api)     | Shared ABI-safe runtime context used by both the shell and the game.                                                        | ❌            |
| [`game_shell`](./rust/crates/game_shell) | Long-running windowed host executable that owns the OS window, watches source changes, rebuilds`game`, and reloads the DLL. | ❌            |

---

## Resources

- Rust hot reload background:<BR> https://johnaustin.io/articles/2022/hot-reloading-rust
- Inspired by:<BR> https://github.com/SamuelAsherRivello/rust-project-template

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
- LinkedIn - [Linkedin.com/in/SamuelAsherRivello](https://www.linkedin.com/in/SamuelAsherRivello) <--- Say Hello! :)

**License**

Provided as-is under [MIT License](./LICENSE) | Copyright ™ & © 2006 - 2026 Rivello Multimedia Consulting, LLC
