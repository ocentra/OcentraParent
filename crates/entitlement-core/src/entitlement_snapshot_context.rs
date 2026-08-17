use super::{
    EntitlementSnapshotContext, EntitlementSnapshotVerificationContext, SignedEntitlementSnapshot,
};
use crate::entitlement_snapshot_values::EntitlementDeviceTrustRequirementState;

pub(super) fn snapshot_context_from_signed_snapshot(
    snapshot: &SignedEntitlementSnapshot,
    verification: EntitlementSnapshotVerificationContext,
) -> EntitlementSnapshotContext {
    EntitlementSnapshotContext {
        signature_state: verification.signature_state,
        freshness_state: verification.freshness_state,
        household_binding_state: verification.household_binding_state,
        device_binding_state: verification.device_binding_state,
        device_trust_requirement_state: match snapshot.device_trust_required {
            true => EntitlementDeviceTrustRequirementState::Required,
            false => EntitlementDeviceTrustRequirementState::NotRequired,
        },
        device_trust_state: verification.device_trust_state,
        package_build_state: verification.package_build_state,
    }
}
