use ocentra_parent_agent_protocol::household_mesh::HouseholdMeshBridgePhase;

use crate::household_mesh_bridge_runtime::{
    HouseholdMeshBridgeExportCandidate, HouseholdMeshBridgeInboundEnvelope,
    HouseholdMeshBridgeInput, HouseholdMeshBridgeValidation,
};
use crate::household_mesh_bridge_runtime_state::{
    HouseholdMeshBridgeDirection, HouseholdMeshBridgeRejectionReason,
};
use crate::household_mesh_event_bridge::HouseholdMeshExportDecision;

#[path = "household_mesh_bridge_runtime_validation_direction.rs"]
mod household_mesh_bridge_runtime_validation_direction;
#[path = "household_mesh_bridge_runtime_validation_export.rs"]
mod household_mesh_bridge_runtime_validation_export;
#[path = "household_mesh_bridge_runtime_validation_import.rs"]
mod household_mesh_bridge_runtime_validation_import;
#[path = "household_mesh_bridge_runtime_validation_rejection.rs"]
mod household_mesh_bridge_runtime_validation_rejection;

pub(crate) fn validate_household_mesh_bridge_export(
    candidate: &HouseholdMeshBridgeExportCandidate,
) -> HouseholdMeshBridgeValidation {
    household_mesh_bridge_runtime_validation_export::validate_household_mesh_bridge_export(
        candidate,
    )
}

pub(crate) fn validate_household_mesh_bridge_import(
    envelope: &HouseholdMeshBridgeInboundEnvelope,
) -> HouseholdMeshBridgeValidation {
    household_mesh_bridge_runtime_validation_import::validate_household_mesh_bridge_import(envelope)
}

pub(crate) fn bridge_validation_for_phase(
    phase: HouseholdMeshBridgePhase,
    input: &HouseholdMeshBridgeInput,
    export_decision: &HouseholdMeshExportDecision,
) -> HouseholdMeshBridgeValidation {
    match phase {
        HouseholdMeshBridgePhase::LanMessageReceived
        | HouseholdMeshBridgePhase::LocalEventRepublished => {
            validate_household_mesh_bridge_import(&input.inbound_envelope())
        }
        HouseholdMeshBridgePhase::LocalEventSelected
        | HouseholdMeshBridgePhase::LanMessageExported => match export_decision {
            HouseholdMeshExportDecision::Export(_) => bridge_validation_for_rejection(None),
            HouseholdMeshExportDecision::Reject(rejection) => {
                bridge_validation_for_rejection(Some(
                    crate::household_mesh_bridge_runtime_rejection::bridge_rejection_reason(
                        *rejection,
                    ),
                ))
            }
        },
    }
}

pub(crate) fn bridge_validation_for_rejection(
    rejection_reason: Option<HouseholdMeshBridgeRejectionReason>,
) -> HouseholdMeshBridgeValidation {
    household_mesh_bridge_runtime_validation_rejection::bridge_validation_for_rejection(
        rejection_reason,
    )
}

pub(crate) fn bridge_direction_for_phase(
    phase: HouseholdMeshBridgePhase,
) -> HouseholdMeshBridgeDirection {
    household_mesh_bridge_runtime_validation_direction::bridge_direction_for_phase(phase)
}
