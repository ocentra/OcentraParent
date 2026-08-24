#![forbid(unsafe_code)]

mod authority;
pub mod binding;
pub mod custody;
mod platform;

mod path_security;
mod storage;

pub(crate) const RECORD_NAMESPACE: &[u8] = b"ocentra.protected-capability-custody.v3";
pub(crate) const STORAGE_SCHEMA_VERSION: u32 = 3;
