//! The platform module owns the Windows ABI boundary; its implementation is
//! isolated in a dedicated Windows-only source unit.

#[cfg(windows)]
#[path = "platform_windows.rs"]
pub(crate) mod windows;
