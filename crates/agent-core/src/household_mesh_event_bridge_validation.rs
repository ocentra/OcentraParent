use ocentra_parent_agent_protocol::household_mesh::{
    HouseholdMeshBridgeState, HouseholdMeshStructurallyValidatedTransportEnvelope,
};

use crate::household_mesh_event_bridge::{
    HouseholdMeshBridgeRejection, HouseholdMeshImportDecision, HouseholdMeshLanMessage,
    HouseholdMeshLocalRepublish, HouseholdMeshPeerAuthorization, HouseholdMeshPolicyAuthority,
};

#[path = "household_mesh_event_bridge_validation_rejection.rs"]
mod household_mesh_event_bridge_validation_rejection;

pub(super) fn validate_incoming_lan_message(
    message: &HouseholdMeshLanMessage,
    expected_family_id: &str,
    expected_target_child_device_id: &str,
    received_at_epoch_seconds: u64,
    seen_message_ids: &[&str],
    seen_idempotency_keys: &[&str],
    authorization: Option<&HouseholdMeshPeerAuthorization>,
) -> HouseholdMeshImportDecision {
    let inbound = ocentra_parent_agent_protocol::household_mesh::household_mesh_bridge_input::
        HouseholdMeshBridgeInboundEnvelope::for_structural_validation(
            message.clone(),
            expected_family_id.to_string(),
            expected_target_child_device_id.to_string(),
            received_at_epoch_seconds,
            seen_message_ids
                .iter()
                .map(|value| (*value).to_string())
                .collect(),
            seen_idempotency_keys
                .iter()
                .map(|value| (*value).to_string())
                .collect(),
        );

    match inbound.validate_structure() {
        Ok(structural_message) => {
            let Some(authorization) = authorization else {
                return HouseholdMeshImportDecision::Reject(
                    HouseholdMeshBridgeRejection::UnauthenticatedMessage,
                );
            };
            if !authorization.matches_source(&structural_message.message().source_peer_id) {
                return HouseholdMeshImportDecision::Reject(
                    HouseholdMeshBridgeRejection::UnauthenticatedMessage,
                );
            }
            HouseholdMeshImportDecision::Republish(local_republish_from_authorized_structure(
                structural_message,
                authorization,
            ))
        }
        Err(validation) => HouseholdMeshImportDecision::Reject(
            household_mesh_event_bridge_validation_rejection::rejection_from_protocol_validation(
                validation,
            ),
        ),
    }
}

fn local_republish_from_authorized_structure(
    structural_message: HouseholdMeshStructurallyValidatedTransportEnvelope,
    _authorization: &HouseholdMeshPeerAuthorization,
) -> HouseholdMeshLocalRepublish {
    let message = structural_message.message();
    HouseholdMeshLocalRepublish {
        family_id: message.family_id.clone(),
        target_child_device_id: message.target_child_device_id.clone(),
        source_peer_id: message.source_peer_id.clone(),
        local_event_ref: message.local_event_ref.clone(),
        lan_message_type: message.lan_message_type.clone(),
        bridge_state: HouseholdMeshBridgeState::LocalRepublishRequired,
        policy_authority: HouseholdMeshPolicyAuthority::ChildAgentOnly,
        validated_before_republish: true,
        child_agent_policy_authority_preserved: true,
    }
}
