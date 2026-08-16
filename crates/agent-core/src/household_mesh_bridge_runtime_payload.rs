use ocentra_parent_agent_protocol::household_mesh::HouseholdMeshBridgePhase;

use crate::{
    household_mesh_bridge_runtime::{HouseholdMeshBridgeEventPayload, HouseholdMeshBridgeInput},
    household_mesh_bridge_runtime_message::{
        bridge_export_decision_for_input, bridge_message_for_phase,
    },
    household_mesh_bridge_runtime_refs::{bridge_event_state, previous_bridge_phase_ref},
    household_mesh_bridge_runtime_state::HouseholdMeshBridgeCustody,
    household_mesh_bridge_runtime_validation::{
        bridge_direction_for_phase, bridge_validation_for_phase,
    },
};

pub(crate) fn household_mesh_bridge_event_payload_from_input(
    phase: HouseholdMeshBridgePhase,
    input: &HouseholdMeshBridgeInput,
) -> HouseholdMeshBridgeEventPayload {
    let export_decision = bridge_export_decision_for_input(input);
    let bridge_message = bridge_message_for_phase(phase, input, &export_decision);
    let validation = bridge_validation_for_phase(phase, input, &export_decision);
    HouseholdMeshBridgeEventPayload {
        phase,
        envelope_state: bridge_event_state(phase),
        direction: bridge_direction_for_phase(phase),
        local_event_type: input.local_event_type.clone(),
        local_event_ref: bridge_message.local_event_ref,
        lan_message_type: bridge_message.lan_message_type,
        family_id: bridge_message.family_id,
        target_child_device_id: bridge_message.target_child_device_id,
        source_peer_id: bridge_message.source_peer_id,
        idempotency_key: bridge_message.idempotency_key,
        outbound_message_id: input.outbound_message_id.clone(),
        inbound_message_id: input.inbound_message.message_id.clone(),
        child_agent_peer_id: input.child_agent_peer_id.clone(),
        provider_peer_id: input.provider_peer_id.clone(),
        payload_ref: input.payload_ref.clone(),
        previous_phase_ref: previous_bridge_phase_ref(phase),
        validation_state: validation.state,
        rejection_reason: validation.rejection_reason,
        custody: HouseholdMeshBridgeCustody::selected_bridge_only(),
        observed_at: input.observed_at.clone(),
    }
}
