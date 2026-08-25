//! Rust-owned protocol constants for the Account current-authority producer.
//!
//! The producer wire is an authenticated transport envelope, not an authority
//! DTO. `ocentra-family-identity-core` owns issuance from its opaque verified
//! capability; consumers must verify the signature and then re-check durable
//! currentness before treating the handoff as a binding.

use serde::{Deserialize, Serialize};

pub const ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_SCHEMA_VERSION: &str =
    "ocentra.account-authority-producer.v1";
pub const ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_AUDIENCE: &str = "ocentra.account.authority";
pub const ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_ENVIRONMENT: &str = "account-owned";
pub const ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_SIGNATURE_ALGORITHM: &str = "ed25519";
pub const ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_SIGNATURE_BYTES: usize = 64;
pub const ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_MAX_FIELD_BYTES: usize = 1_024;
pub const ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_MAX_PAYLOAD_BYTES: usize = 16 * 1_024;
pub const ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_MAX_LIFETIME_SECONDS: i64 = 5 * 60;
pub const ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_MAX_FUTURE_ISSUED_SKEW_SECONDS: i64 = 30;
pub const ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_MAX_WIRE_BYTES: usize =
    ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_MAX_PAYLOAD_BYTES
        + (ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_MAX_FIELD_BYTES * 7)
        + 128
        + 64;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename = "v1")]
pub enum AccountIdentityAuthorityProducerSchemaVersion {
    V1,
}
