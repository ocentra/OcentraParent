//! Windows-native ownership boundary for local logging artifacts.
//!
//! Safe callers receive typed operations and opaque lifetime-bound guards;
//! raw Windows handles never cross this crate's public API.

pub mod error;
#[path = "ffi/transport/mod.rs"]
pub mod transport;

#[cfg(windows)]
mod constants;

#[cfg(windows)]
#[path = "ffi/transport/owner.rs"]
pub mod owner;
#[cfg(windows)]
mod owner_journal;
#[cfg(windows)]
#[path = "ffi/transport/owner_mutations.rs"]
mod owner_mutations;
#[cfg(windows)]
#[path = "ffi/transport/owner_paths.rs"]
mod owner_paths;
#[cfg(windows)]
#[path = "ffi/transport/owner_types.rs"]
mod owner_types;
#[cfg(windows)]
#[path = "ffi/transport/platform.rs"]
mod platform;
