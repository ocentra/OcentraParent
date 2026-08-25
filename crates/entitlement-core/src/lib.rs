#![forbid(unsafe_code)]

#[macro_use]
mod entitlement_text_id;
#[macro_use]
mod entitlement_snapshot_text_id;
pub mod entitlement_access;
mod entitlement_access_reasons;
mod entitlement_access_reasons_policy;
pub mod entitlement_snapshot;
pub mod entitlement_snapshot_authority;
pub mod entitlement_snapshot_cache;
pub(crate) mod entitlement_snapshot_issuer;
pub mod entitlement_snapshot_values;
