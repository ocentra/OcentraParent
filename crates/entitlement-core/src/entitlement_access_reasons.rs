use crate::entitlement_access::{
    EntitlementCapability, EntitlementCapabilityInput, EntitlementCapabilityRejectionReason,
    OfflineGraceState,
};
use crate::entitlement_snapshot_values::{
    EntitlementSnapshotFreshnessState, EntitlementSnapshotSignatureState,
};

pub(crate) fn entitlement_rejection_reason(
    input: &EntitlementCapabilityInput,
) -> Option<EntitlementCapabilityRejectionReason> {
    signature_or_freshness_reason(input)
        .or_else(|| grace_capability_reason(input))
        .or_else(|| crate::entitlement_access_reasons_policy::binding_or_trust_reason(input))
        .or_else(|| {
            crate::entitlement_access_reasons_policy::package_family_policy_scope_subscription_reason(input)
        })
}

fn grace_capability_reason(
    input: &EntitlementCapabilityInput,
) -> Option<EntitlementCapabilityRejectionReason> {
    if input.snapshot_context.freshness_state == EntitlementSnapshotFreshnessState::Grace
        && (input.capability != EntitlementCapability::Tracking
            || input.offline_grace_state != OfflineGraceState::Active)
    {
        return Some(EntitlementCapabilityRejectionReason::GraceRestricted);
    }

    None
}

fn signature_or_freshness_reason(
    input: &EntitlementCapabilityInput,
) -> Option<EntitlementCapabilityRejectionReason> {
    match input.snapshot_context.signature_state {
        EntitlementSnapshotSignatureState::Missing => {
            Some(EntitlementCapabilityRejectionReason::MissingSignature)
        }
        EntitlementSnapshotSignatureState::Invalid => {
            Some(EntitlementCapabilityRejectionReason::InvalidSignature)
        }
        EntitlementSnapshotSignatureState::Trusted => {
            freshness_reason(input.snapshot_context.freshness_state)
        }
    }
}

fn freshness_reason(
    state: EntitlementSnapshotFreshnessState,
) -> Option<EntitlementCapabilityRejectionReason> {
    match state {
        EntitlementSnapshotFreshnessState::Stale => {
            Some(EntitlementCapabilityRejectionReason::StaleSnapshot)
        }
        EntitlementSnapshotFreshnessState::Expired => {
            Some(EntitlementCapabilityRejectionReason::ExpiredSnapshot)
        }
        EntitlementSnapshotFreshnessState::Revoked => {
            Some(EntitlementCapabilityRejectionReason::RevokedSnapshot)
        }
        EntitlementSnapshotFreshnessState::Fresh | EntitlementSnapshotFreshnessState::Grace => None,
    }
}
