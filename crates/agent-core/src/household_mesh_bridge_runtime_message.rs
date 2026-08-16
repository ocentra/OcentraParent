use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::household_mesh::HouseholdMeshBridgePhase;

use crate::{
    household_mesh_bridge_runtime::HouseholdMeshBridgeInput,
    household_mesh_bridge_runtime_refs::{
        bridge_local_event_kind_for_local_event, bridge_message_type_for_local_event,
    },
    household_mesh_event_bridge::{
        export_selected_local_event, HouseholdMeshAuthenticationState,
        HouseholdMeshBridgeRejection, HouseholdMeshExportDecision, HouseholdMeshLanMessage,
        HouseholdMeshPolicyAuthority,
    },
};

pub(crate) fn bridge_export_decision_for_input(
    input: &HouseholdMeshBridgeInput,
) -> HouseholdMeshExportDecision {
    let Some(event_kind) = bridge_local_event_kind_for_local_event(&input.local_event_type) else {
        return HouseholdMeshExportDecision::Reject(
            HouseholdMeshBridgeRejection::UnselectedLocalEvent,
        );
    };
    export_selected_local_event(
        event_kind,
        &input.family_id,
        &input.target_child_device_id,
        &input.child_agent_peer_id,
        &input.outbound_message_id,
        &input.outbound_idempotency_key,
        (
            constants::household_mesh::TEST_BRIDGE_SENT_AT_EPOCH_SECONDS,
            constants::household_mesh::TEST_BRIDGE_STALE_AFTER_SECONDS,
        ),
    )
}

pub(crate) fn bridge_message_for_phase(
    phase: HouseholdMeshBridgePhase,
    input: &HouseholdMeshBridgeInput,
    export_decision: &HouseholdMeshExportDecision,
) -> HouseholdMeshLanMessage {
    match phase {
        HouseholdMeshBridgePhase::LocalEventSelected
        | HouseholdMeshBridgePhase::LanMessageExported => match export_decision {
            HouseholdMeshExportDecision::Export(message) => message.clone(),
            HouseholdMeshExportDecision::Reject(_) => fallback_export_message(input),
        },
        HouseholdMeshBridgePhase::LanMessageReceived
        | HouseholdMeshBridgePhase::LocalEventRepublished => input.inbound_message.clone(),
    }
}

fn fallback_export_message(input: &HouseholdMeshBridgeInput) -> HouseholdMeshLanMessage {
    let lan_message_type = bridge_message_type_for_local_event(&input.local_event_type)
        .unwrap_or(constants::household_mesh::LAN_MESSAGE_AI_WORK_OFFER);
    let local_event_ref = bridge_local_event_kind_for_local_event(&input.local_event_type)
        .and_then(crate::household_mesh_event_bridge_lookup::local_event_ref)
        .unwrap_or(constants::household_mesh::LOCAL_EVENT_AI_WORK_OFFER);
    HouseholdMeshLanMessage {
        schema_version: constants::household_mesh::EVENT_SCHEMA_VERSION,
        message_id: input.outbound_message_id.clone(),
        idempotency_key: input.outbound_idempotency_key.clone(),
        family_id: input.family_id.clone(),
        target_child_device_id: input.target_child_device_id.clone(),
        source_peer_id: input.child_agent_peer_id.clone(),
        local_event_ref: local_event_ref.to_string(),
        lan_message_type: lan_message_type.to_string(),
        bridge_state:
            ocentra_parent_agent_protocol::household_mesh::HouseholdMeshBridgeState::ExportSelected,
        authentication_state: HouseholdMeshAuthenticationState::PairedTrustedDevice,
        policy_authority: HouseholdMeshPolicyAuthority::ChildAgentOnly,
        direct_remote_publish_requested: false,
        raw_payload_included: false,
        sent_at_epoch_seconds: constants::household_mesh::TEST_BRIDGE_SENT_AT_EPOCH_SECONDS,
        stale_after_seconds: constants::household_mesh::TEST_BRIDGE_STALE_AFTER_SECONDS,
    }
}
