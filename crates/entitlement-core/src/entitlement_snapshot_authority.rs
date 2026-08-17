use super::{
    EntitlementSnapshotVerificationContext, EntitlementSnapshotVerificationFailure,
    EntitlementSnapshotVerificationRequest, SignedEntitlementSnapshot,
};
use crate::entitlement_snapshot_values::{
    EntitlementAccountRef, EntitlementHouseholdRef, EntitlementRevocationCursor,
    EntitlementSignatureKeyId, EntitlementSnapshotBindingState, EntitlementTrustedDeviceRef,
};

/// Canonical identity and current-state result owned by the entitlement
/// verifier.  It cannot be deserialized or constructed by a request caller.
#[derive(Debug, PartialEq, Eq)]
pub(crate) struct EntitlementSnapshotAuthorityBinding {
    pub(super) account_ref: EntitlementAccountRef,
    pub(super) household_ref: EntitlementHouseholdRef,
    pub(super) trusted_device_ref: EntitlementTrustedDeviceRef,
    pub(super) signature_key_id: EntitlementSignatureKeyId,
    pub(super) revocation_cursor: EntitlementRevocationCursor,
    pub(super) lifecycle_generation: u64,
    pub(super) authority_generation: u64,
}

impl EntitlementSnapshotAuthorityBinding {
    pub(crate) fn from_verified_state(
        account_ref: EntitlementAccountRef,
        household_ref: EntitlementHouseholdRef,
        trusted_device_ref: EntitlementTrustedDeviceRef,
        signature_key_id: EntitlementSignatureKeyId,
        revocation_cursor: EntitlementRevocationCursor,
        lifecycle_generation: u64,
        authority_generation: u64,
    ) -> Self {
        Self {
            account_ref,
            household_ref,
            trusted_device_ref,
            signature_key_id,
            revocation_cursor,
            lifecycle_generation,
            authority_generation,
        }
    }
}

pub(super) fn validate_verifier_owned_binding(
    snapshot: &SignedEntitlementSnapshot,
    request: &EntitlementSnapshotVerificationRequest,
    verification: &EntitlementSnapshotVerificationContext,
) -> Result<(), EntitlementSnapshotVerificationFailure> {
    if verification.household_binding_state != EntitlementSnapshotBindingState::Matched {
        return Err(EntitlementSnapshotVerificationFailure::WrongHousehold);
    }
    if verification.device_binding_state != EntitlementSnapshotBindingState::Matched {
        return Err(EntitlementSnapshotVerificationFailure::WrongDevice);
    }
    let binding = &verification.authority_binding;
    if &binding.account_ref != &request.account_ref {
        return Err(EntitlementSnapshotVerificationFailure::WrongAccount);
    }
    if &binding.household_ref != &request.household_ref {
        return Err(EntitlementSnapshotVerificationFailure::WrongHousehold);
    }
    if &binding.trusted_device_ref != &request.trusted_device_ref {
        return Err(EntitlementSnapshotVerificationFailure::WrongDevice);
    }
    if &binding.signature_key_id != &snapshot.signature_key_id
        || &binding.revocation_cursor != &snapshot.revocation_cursor
        || binding.lifecycle_generation == 0
        || binding.authority_generation == 0
    {
        return Err(EntitlementSnapshotVerificationFailure::AuthorityUnavailable);
    }
    Ok(())
}
