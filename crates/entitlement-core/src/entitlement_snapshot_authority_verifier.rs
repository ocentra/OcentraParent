#![forbid(unsafe_code)]

//! Signed snapshot shape, account/device binding, and signature verification.

use crate::{
    entitlement_snapshot::{EntitlementSnapshotContext, SignedEntitlementSnapshot},
    entitlement_snapshot_values::{
        EntitlementDeviceTrustRequirementState, EntitlementDeviceTrustState,
        EntitlementPackageBuildState, EntitlementSnapshotBindingState,
        EntitlementSnapshotSignatureState,
    },
};

use super::{
    verifier_binding, verifier_currentness, verifier_request, verifier_signature,
    EntitlementSnapshotAuthority, EntitlementSnapshotVerificationFailure,
};

/// Opaque result of cryptographic, currentness, account, and device binding
/// verification. It cannot be serialized or reconstructed by a command.
pub struct VerifiedEntitlementSnapshot {
    context: EntitlementSnapshotContext,
}

impl std::fmt::Debug for VerifiedEntitlementSnapshot {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("VerifiedEntitlementSnapshot")
            .field("authority", &"opaque")
            .finish()
    }
}

pub(crate) fn verify(
    authority: &EntitlementSnapshotAuthority,
    snapshot: &SignedEntitlementSnapshot,
    request: &verifier_request::EntitlementSnapshotVerificationRequest,
) -> Result<VerifiedEntitlementSnapshot, EntitlementSnapshotVerificationFailure> {
    verifier_binding::validate_snapshot_binding(snapshot, request)?;
    verifier_signature::verify_snapshot_signature(authority, snapshot)?;
    let currentness = verifier_currentness::verify_snapshot_currentness(authority, snapshot)?;

    Ok(VerifiedEntitlementSnapshot {
        context: EntitlementSnapshotContext {
            signature_state: EntitlementSnapshotSignatureState::Trusted,
            freshness_state: currentness.freshness,
            household_binding_state: EntitlementSnapshotBindingState::Matched,
            device_binding_state: EntitlementSnapshotBindingState::Matched,
            device_trust_requirement_state: if snapshot.device_trust_required {
                EntitlementDeviceTrustRequirementState::Required
            } else {
                EntitlementDeviceTrustRequirementState::NotRequired
            },
            device_trust_state: EntitlementDeviceTrustState::Present,
            package_build_state: EntitlementPackageBuildState::Valid,
        },
    })
}
