//! Explicit fail-closed API on non-Windows targets.

#![cfg(not(windows))]

#[path = "unsupported_process.rs"]
mod process;
#[path = "unsupported_registry.rs"]
mod registry;
#[path = "unsupported_service.rs"]
mod service;
#[path = "unsupported_tpm.rs"]
mod tpm;
