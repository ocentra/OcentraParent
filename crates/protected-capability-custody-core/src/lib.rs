#![forbid(unsafe_code)]

pub mod account_issuer;

mod authority;
pub mod binding;
pub mod broker_admission;
pub mod custody;
mod platform;

mod path_security;
mod storage;

mod account_issuer_signing;

pub(crate) const RECORD_NAMESPACE: &[u8] = b"ocentra.protected-capability-custody.v3";
pub(crate) const STORAGE_SCHEMA_VERSION: u32 = 3;
