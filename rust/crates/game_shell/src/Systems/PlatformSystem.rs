#[cfg(not(target_arch = "wasm32"))]
#[path = "NativePlatformSystem.rs"]
mod native_platform_system;
#[cfg(target_arch = "wasm32")]
#[path = "WasmPlatformSystem.rs"]
mod wasm_platform_system;

#[cfg(not(target_arch = "wasm32"))]
pub use native_platform_system::*;
#[cfg(target_arch = "wasm32")]
pub use wasm_platform_system::*;
