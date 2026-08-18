use crate::entitlement_access::{EntitlementCapabilityInput, EntitlementCapabilityRejectionReason};
use crate::entitlement_snapshot_values::{
    EntitlementSnapshotFreshnessState, EntitlementSnapshotSignatureState,
};

pub(crate) fn entitlement_rejection_reason(
    input: &EntitlementCapabilityInput,
) -> Option<EntitlementCapabilityRejectionReason> {
    signature_or_freshness_reason(input)
        .or_else(|| crate::entitlement_access_reasons_policy::binding_or_trust_reason(input))
        .or_else(|| {
            crate::entitlement_access_reasons_policy::package_family_policy_scope_subscription_reason(input)
        })
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
