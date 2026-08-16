use crate::household_mesh_bridge_runtime::HouseholdMeshBridgeValidation;
use crate::household_mesh_bridge_runtime_state::{
    HouseholdMeshBridgeRejectionReason, HouseholdMeshBridgeValidationState,
};

pub(crate) fn bridge_validation_for_rejection(
    rejection_reason: Option<HouseholdMeshBridgeRejectionReason>,
) -> HouseholdMeshBridgeValidation {
    HouseholdMeshBridgeValidation {
        state: if rejection_reason.is_none() {
            HouseholdMeshBridgeValidationState::Accepted
        } else {
            HouseholdMeshBridgeValidationState::Rejected
        },
        rejection_reason,
    }
}
