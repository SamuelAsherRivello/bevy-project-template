#![allow(clippy::type_complexity, non_snake_case)]

#[path = "Components/Mod.rs"]
pub mod Components;
#[path = "Plugins/Mod.rs"]
pub mod Plugins;
#[path = "Resources/Mod.rs"]
pub mod Resources;
#[path = "../Server/Mod.rs"]
pub mod Server;
#[path = "Systems/Mod.rs"]
pub mod Systems;
#[cfg(test)]
#[path = "../../Tests/Mod.rs"]
pub mod Tests;

pub use crate::Plugins::GamePlugin::GamePlugin;
pub use crate::Resources::GameState;