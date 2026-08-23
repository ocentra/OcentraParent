#![forbid(unsafe_code)]

pub mod authority;
pub mod binding;
pub mod custody;
pub mod platform;

mod path_security;
mod storage;

pub(crate) const RECORD_NAMESPACE: &[u8] = b"ocentra.protected-capability-custody.v2";
pub(crate) const STORAGE_SCHEMA_VERSION: u32 = 2;
