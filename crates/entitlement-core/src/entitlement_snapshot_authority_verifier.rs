#![forbid(unsafe_code)]

//! Signed snapshot shape, account/device binding, and signature verification.

use crate::{
    entitlement_access::EntitlementCapability,
    entitlement_snapshot::{EntitlementSnapshotContext, SignedEntitlementSnapshot},
    entitlement_snapshot_values::{
        EntitlementDeviceTrustRequirementState, EntitlementDeviceTrustState,
        EntitlementPackageBuildState, EntitlementSnapshotBindingState, EntitlementSnapshotId,
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
    snapshot_id: EntitlementSnapshotId,
    authority_generation: u64,
    context: EntitlementSnapshotContext,
    enabled_capabilities: Vec<EntitlementCapability>,
}

impl std::fmt::Debug for VerifiedEntitlementSnapshot {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("VerifiedEntitlementSnapshot")
            .field("authority", &"opaque")
            .finish()
    }
}

impl VerifiedEntitlementSnapshot {
    pub fn snapshot_id(&self) -> &EntitlementSnapshotId {
        &self.snapshot_id
    }

    pub(crate) fn authority_generation(&self) -> u64 {
        self.authority_generation
    }

    pub(crate) fn context(&self) -> EntitlementSnapshotContext {
        self.context
    }

    pub(crate) fn enables(&self, capability: EntitlementCapability) -> bool {
        self.enabled_capabilities.contains(&capability)
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

    let enabled_capabilities = snapshot
        .feature_flags
        .iter()
        .filter_map(|flag| flag.enabled.then_some(flag.capability))
        .collect();
    Ok(VerifiedEntitlementSnapshot {
        snapshot_id: snapshot.snapshot_id.clone(),
        authority_generation: currentness.authority_generation,
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
        enabled_capabilities,
    })
}
