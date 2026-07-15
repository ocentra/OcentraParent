use crate::household_mesh_bridge_runtime::{
    HouseholdMeshBridgeInboundEnvelope, HouseholdMeshBridgeValidation,
};
use crate::household_mesh_bridge_runtime_state::HouseholdMeshBridgeRejectionReason;
use crate::household_mesh_event_bridge::{
    validate_incoming_lan_message, HouseholdMeshImportDecision,
};

#[path = "household_mesh_bridge_runtime_validation_import_rejection.rs"]
mod household_mesh_bridge_runtime_validation_import_rejection;

pub(crate) fn validate_household_mesh_bridge_import(
    envelope: &HouseholdMeshBridgeInboundEnvelope,
) -> HouseholdMeshBridgeValidation {
    let rejection_reason = if !envelope.authorized {
        Some(HouseholdMeshBridgeRejectionReason::UnauthorizedPeer)
    } else {
        let seen_message_ids = envelope
            .seen_message_ids
            .iter()
            .map(String::as_str)
            .collect::<Vec<_>>();
        let seen_idempotency_keys = envelope
            .seen_idempotency_keys
            .iter()
            .map(String::as_str)
            .collect::<Vec<_>>();
        match validate_incoming_lan_message(
            &envelope.message,
            &envelope.expected_family_id,
            &envelope.expected_target_child_device_id,
            envelope.received_at_epoch_seconds,
            &seen_message_ids,
            &seen_idempotency_keys,
        ) {
            HouseholdMeshImportDecision::Republish(_) => None,
            HouseholdMeshImportDecision::Reject(rejection) => Some(
                household_mesh_bridge_runtime_validation_import_rejection::import_rejection_reason(
                    rejection,
                ),
            ),
        }
    };
    super::bridge_validation_for_rejection(rejection_reason)
}
