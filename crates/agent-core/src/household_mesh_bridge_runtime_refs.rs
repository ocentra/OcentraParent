use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::household_mesh::{
    HouseholdMeshBridgeEnvelopeState, HouseholdMeshBridgePhase,
};

use crate::HouseholdMeshLocalEventKind;

pub(crate) fn bridge_aggregate_key(correlation_id: &str) -> String {
    let mut value = String::from(constants::household_mesh::AGGREGATE_HOUSEHOLD_MESH_PREFIX);
    value.push_str(correlation_id);
    value
}

pub(crate) fn bridge_message_type_for_local_event(event_type: &str) -> Option<&'static str> {
    match event_type {
        constants::screen_flow::EVENT_SCREEN_MESH_OFFER_PUBLISHED => {
            Some(constants::household_mesh::LAN_MESSAGE_AI_WORK_OFFER)
        }
        constants::screen_flow::EVENT_SCREEN_MESH_PROVIDER_RESULT_RETURNED => {
            Some(constants::household_mesh::LAN_MESSAGE_AI_RESULT_RETURN)
        }
        _ => None,
    }
}

pub(crate) fn bridge_local_event_kind_for_local_event(
    event_type: &str,
) -> Option<HouseholdMeshLocalEventKind> {
    match event_type {
        constants::screen_flow::EVENT_SCREEN_MESH_OFFER_PUBLISHED => {
            Some(HouseholdMeshLocalEventKind::AiWorkOffer)
        }
        constants::screen_flow::EVENT_SCREEN_MESH_PROVIDER_RESULT_RETURNED => {
            Some(HouseholdMeshLocalEventKind::AiResultReturn)
        }
        _ => None,
    }
}

pub(crate) fn bridge_event_state(
    phase: HouseholdMeshBridgePhase,
) -> HouseholdMeshBridgeEnvelopeState {
    match phase {
        HouseholdMeshBridgePhase::LocalEventSelected => {
            HouseholdMeshBridgeEnvelopeState::LocalSelected
        }
        HouseholdMeshBridgePhase::LanMessageExported => {
            HouseholdMeshBridgeEnvelopeState::LanExported
        }
        HouseholdMeshBridgePhase::LanMessageReceived => {
            HouseholdMeshBridgeEnvelopeState::LanReceived
        }
        HouseholdMeshBridgePhase::LocalEventRepublished => {
            HouseholdMeshBridgeEnvelopeState::LocalRepublished
        }
    }
}

pub(crate) fn previous_bridge_phase_ref(phase: HouseholdMeshBridgePhase) -> Option<String> {
    let value = match phase {
        HouseholdMeshBridgePhase::LocalEventSelected => return None,
        HouseholdMeshBridgePhase::LanMessageExported => {
            constants::household_mesh::TEST_BRIDGE_SELECTED_EVENT_REF
        }
        HouseholdMeshBridgePhase::LanMessageReceived => {
            constants::household_mesh::TEST_BRIDGE_EXPORTED_MESSAGE_REF
        }
        HouseholdMeshBridgePhase::LocalEventRepublished => {
            constants::household_mesh::TEST_BRIDGE_RECEIVED_MESSAGE_REF
        }
    };
    Some(value.to_string())
}
