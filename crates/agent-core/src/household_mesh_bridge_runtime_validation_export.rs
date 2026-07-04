use crate::household_mesh_bridge_runtime::{
    HouseholdMeshBridgeExportCandidate, HouseholdMeshBridgeValidation,
};
use crate::household_mesh_bridge_runtime_state::HouseholdMeshBridgeRejectionReason;
use crate::household_mesh_event_bridge_lookup::bridge_local_event_kind_for_local_event;

pub(crate) fn validate_household_mesh_bridge_export(
    candidate: &HouseholdMeshBridgeExportCandidate,
) -> HouseholdMeshBridgeValidation {
    let rejection_reason = [
        candidate
            .private_local_event
            .then_some(HouseholdMeshBridgeRejectionReason::PrivateLocalEvent),
        candidate
            .contains_raw_screenshot
            .then_some(HouseholdMeshBridgeRejectionReason::RawScreenPayload),
        bridge_local_event_kind_for_local_event(&candidate.local_event_type)
            .is_none()
            .then_some(HouseholdMeshBridgeRejectionReason::UnselectedEvent),
    ]
    .into_iter()
    .flatten()
    .next();
    super::bridge_validation_for_rejection(rejection_reason)
}
