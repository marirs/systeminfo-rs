#[cfg(target_os = "windows")]
#[path = "win/mod.rs"]
mod platform;

#[cfg(target_os = "linux")]
#[path = "nix/mod.rs"]
mod platform;

#[cfg(target_os = "macos")]
#[path = "mac/mod.rs"]
mod platform;

mod common;

pub mod consts;
pub use self::platform::*;
