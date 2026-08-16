use crate::household_mesh::{
    HouseholdMeshBridgeValidation, HouseholdMeshStructurallyValidatedTransportEnvelope,
};

use super::HouseholdMeshBridgeInboundEnvelope;

#[path = "household_mesh_bridge_input_validation_context.rs"]
mod household_mesh_bridge_input_validation_context;
#[path = "household_mesh_bridge_input_validation_core.rs"]
mod household_mesh_bridge_input_validation_core;
#[path = "household_mesh_bridge_input_validation_event_ref.rs"]
mod household_mesh_bridge_input_validation_event_ref;
#[path = "household_mesh_bridge_input_validation_policy.rs"]
mod household_mesh_bridge_input_validation_policy;

pub(super) fn validate_structure(
    input: HouseholdMeshBridgeInboundEnvelope,
) -> Result<HouseholdMeshStructurallyValidatedTransportEnvelope, HouseholdMeshBridgeValidation> {
    let message = &input.message;
    if let Some(reason) = household_mesh_bridge_input_validation_core::rejection_reason(message) {
        return Err(HouseholdMeshBridgeValidation::rejected(reason));
    }
    if let Some(reason) = household_mesh_bridge_input_validation_policy::rejection_reason(message) {
        return Err(HouseholdMeshBridgeValidation::rejected(reason));
    }
    if let Some(reason) =
        household_mesh_bridge_input_validation_context::rejection_reason(message, &input)
    {
        return Err(HouseholdMeshBridgeValidation::rejected(reason));
    }
    if let Some(reason) =
        household_mesh_bridge_input_validation_event_ref::rejection_reason(message)
    {
        return Err(HouseholdMeshBridgeValidation::rejected(reason));
    }

    Ok(HouseholdMeshStructurallyValidatedTransportEnvelope {
        message: input.message,
    })
}
