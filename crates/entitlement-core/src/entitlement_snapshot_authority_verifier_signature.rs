#![forbid(unsafe_code)]

use ed25519_dalek::{Signature, Verifier};

use crate::entitlement_snapshot::SignedEntitlementSnapshot;

use super::{
    revocation::derive_entitlement_signature_key_id, EntitlementSnapshotAuthority,
    EntitlementSnapshotVerificationFailure,
};

pub(super) fn verify_snapshot_signature(
    authority: &EntitlementSnapshotAuthority,
    snapshot: &SignedEntitlementSnapshot,
) -> Result<(), EntitlementSnapshotVerificationFailure> {
    let verifying_key = authority
        .key_provider
        .verifying_key(&snapshot.signature_key_id)?;
    if verifying_key.is_weak() {
        return Err(EntitlementSnapshotVerificationFailure::WeakVerificationKey);
    }
    if derive_entitlement_signature_key_id(&verifying_key)? != snapshot.signature_key_id {
        return Err(EntitlementSnapshotVerificationFailure::SignatureKeyMismatch);
    }
    let signature = Signature::from_slice(&snapshot.signature)
        .map_err(|_error| EntitlementSnapshotVerificationFailure::InvalidSignature)?;
    verifying_key
        .verify_strict(&snapshot.signing_bytes(), &signature)
        .map_err(|_error| EntitlementSnapshotVerificationFailure::InvalidSignature)
}
