#![forbid(unsafe_code)]

//! Durable revocation-state custody plus a read-only snapshot storage
//! primitive for a future owner.
//!
//! These stores persist issuer-signed wire material only.  Reading either
//! file never establishes authority; the verifier authenticates the signature
//! again on every use. Missing, malformed, or tampered state is surfaced as an
//! explicit unavailable/corrupt result. Snapshot mutation is intentionally not
//! exposed here because no legal owner ingestion or handle-safe platform
//! custody path is mounted; revocation membership cannot shrink during its
//! locked replacement.

use serde::{Deserialize, Serialize};

use crate::entitlement_snapshot_values::{
    EntitlementRevocationCursor, EntitlementSignatureKeyId, EntitlementSnapshotId,
};

#[path = "entitlement_snapshot_cache_path.rs"]
pub(crate) mod path;
#[path = "entitlement_snapshot_cache_revocation.rs"]
pub(crate) mod revocation_state;
#[path = "entitlement_snapshot_cache_storage.rs"]
mod storage;

const REVOCATION_STATE_SCHEMA_VERSION: u16 = 1;
const REVOCATION_SIGNATURE_BYTES: usize = 64;
const MAX_REVOCATION_ENTRIES: usize = 16_384;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum EntitlementSnapshotCacheError {
    StorageUnavailable,
    CorruptState,
    InvalidPath,
    StaleReplacement,
}

/// Signed revocation cursor state persisted by the entitlement owner.
///
/// The update is intentionally public as wire data, but its signature is
/// checked by `EntitlementSnapshotAuthority` before it influences currentness.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct SignedEntitlementRevocationUpdate {
    pub schema_version: u16,
    pub issuer_key_id: EntitlementSignatureKeyId,
    pub revocation_cursor: EntitlementRevocationCursor,
    pub authority_generation: u64,
    pub revoked_snapshot_ids: Vec<EntitlementSnapshotId>,
    pub signature: Vec<u8>,
}

impl SignedEntitlementRevocationUpdate {
    pub(crate) fn signing_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(256);
        bytes.extend_from_slice(b"ocentra.entitlement.revocation.v1\0");
        append_u16(&mut bytes, self.schema_version);
        append_text(&mut bytes, self.issuer_key_id.as_str());
        append_text(&mut bytes, self.revocation_cursor.as_str());
        append_u64(&mut bytes, self.authority_generation);
        append_u32(&mut bytes, self.revoked_snapshot_ids.len() as u32);
        for snapshot_id in &self.revoked_snapshot_ids {
            append_text(&mut bytes, snapshot_id.as_str());
        }
        bytes
    }

    pub(crate) fn validate_shape(&self) -> Result<(), EntitlementSnapshotCacheError> {
        if self.schema_version != REVOCATION_STATE_SCHEMA_VERSION
            || self.signature.len() != REVOCATION_SIGNATURE_BYTES
            || self.authority_generation == 0
            || self.revoked_snapshot_ids.len() > MAX_REVOCATION_ENTRIES
        {
            return Err(EntitlementSnapshotCacheError::CorruptState);
        }
        if self
            .revoked_snapshot_ids
            .iter()
            .enumerate()
            .any(|(index, id)| {
                self.revoked_snapshot_ids[index + 1..]
                    .iter()
                    .any(|other| other == id)
            })
        {
            return Err(EntitlementSnapshotCacheError::CorruptState);
        }
        Ok(())
    }
}

fn append_text(bytes: &mut Vec<u8>, value: &str) {
    append_u64(bytes, value.len() as u64);
    bytes.extend_from_slice(value.as_bytes());
}

fn append_u16(bytes: &mut Vec<u8>, value: u16) {
    bytes.extend_from_slice(&value.to_be_bytes());
}

fn append_u32(bytes: &mut Vec<u8>, value: u32) {
    bytes.extend_from_slice(&value.to_be_bytes());
}

fn append_u64(bytes: &mut Vec<u8>, value: u64) {
    bytes.extend_from_slice(&value.to_be_bytes());
}
