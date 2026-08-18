#![forbid(unsafe_code)]

//! Signature-key binding and error mapping for durable revocation state.

use std::fmt::Write;

use ed25519_dalek::{Signature, Verifier, VerifyingKey};
use sha2::{Digest, Sha256};

use crate::{
    entitlement_snapshot_cache::{
        EntitlementSnapshotCacheError, SignedEntitlementRevocationUpdate,
    },
    entitlement_snapshot_values::EntitlementSignatureKeyId,
};

use super::{
    ports::EntitlementSnapshotVerificationKeyProvider, EntitlementSnapshotVerificationFailure,
};

const REVOCATION_UPDATE_SCHEMA_VERSION: u16 = 1;

pub(crate) fn verify_revocation_update(
    update: &SignedEntitlementRevocationUpdate,
    key_provider: &dyn EntitlementSnapshotVerificationKeyProvider,
) -> Result<(), EntitlementSnapshotVerificationFailure> {
    update
        .validate_shape()
        .map_err(|_error| EntitlementSnapshotVerificationFailure::RevocationStateCorrupt)?;
    if update.schema_version != REVOCATION_UPDATE_SCHEMA_VERSION {
        return Err(EntitlementSnapshotVerificationFailure::RevocationStateCorrupt);
    }
    let verifying_key = key_provider.verifying_key(&update.issuer_key_id)?;
    if verifying_key.is_weak() {
        return Err(EntitlementSnapshotVerificationFailure::WeakVerificationKey);
    }
    if derive_entitlement_signature_key_id(&verifying_key)? != update.issuer_key_id {
        return Err(EntitlementSnapshotVerificationFailure::SignatureKeyMismatch);
    }
    let signature = Signature::from_slice(&update.signature)
        .map_err(|_error| EntitlementSnapshotVerificationFailure::RevocationStateCorrupt)?;
    verifying_key
        .verify_strict(&update.signing_bytes(), &signature)
        .map_err(|_error| EntitlementSnapshotVerificationFailure::RevocationStateCorrupt)
}

pub fn derive_entitlement_signature_key_id(
    verifying_key: &VerifyingKey,
) -> Result<EntitlementSignatureKeyId, EntitlementSnapshotVerificationFailure> {
    if verifying_key.is_weak() {
        return Err(EntitlementSnapshotVerificationFailure::WeakVerificationKey);
    }
    let digest = Sha256::digest(verifying_key.to_bytes());
    let mut value = String::with_capacity(32);
    for byte in &digest[..16] {
        write!(&mut value, "{byte:02x}")
            .map_err(|_error| EntitlementSnapshotVerificationFailure::AuthorityUnavailable)?;
    }
    EntitlementSignatureKeyId::parse(value)
        .map_err(|_error| EntitlementSnapshotVerificationFailure::AuthorityUnavailable)
}

pub(crate) fn map_cache_error(
    error: EntitlementSnapshotCacheError,
) -> EntitlementSnapshotVerificationFailure {
    match error {
        EntitlementSnapshotCacheError::StorageUnavailable
        | EntitlementSnapshotCacheError::InvalidPath => {
            EntitlementSnapshotVerificationFailure::AuthorityUnavailable
        }
        EntitlementSnapshotCacheError::CorruptState => {
            EntitlementSnapshotVerificationFailure::RevocationStateCorrupt
        }
        EntitlementSnapshotCacheError::StaleReplacement => {
            EntitlementSnapshotVerificationFailure::StaleAuthorityState
        }
    }
}
