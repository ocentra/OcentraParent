use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::household_mesh::{
    HouseholdMeshBridgeEnvelopeState, HouseholdMeshBridgePhase,
};

use crate::HouseholdMeshLocalEventKind;

const LOCAL_EVENT_MAPPINGS: [(&str, HouseholdMeshLocalEventKind, &str); 2] = [
    (
        constants::screen_flow::EVENT_SCREEN_MESH_OFFER_PUBLISHED,
        HouseholdMeshLocalEventKind::AiWorkOffer,
        constants::household_mesh::LAN_MESSAGE_AI_WORK_OFFER,
    ),
    (
        constants::screen_flow::EVENT_SCREEN_MESH_PROVIDER_RESULT_RETURNED,
        HouseholdMeshLocalEventKind::AiResultReturn,
        constants::household_mesh::LAN_MESSAGE_AI_RESULT_RETURN,
    ),
];

pub(crate) fn bridge_aggregate_key(correlation_id: &str) -> String {
    let mut value = String::from(constants::household_mesh::AGGREGATE_HOUSEHOLD_MESH_PREFIX);
    value.push_str(correlation_id);
    value
}

pub(crate) fn bridge_message_type_for_local_event(event_type: &str) -> Option<&'static str> {
    LOCAL_EVENT_MAPPINGS
        .iter()
        .find_map(|(known_event_type, _, lan_message_type)| {
            (*known_event_type == event_type).then_some(*lan_message_type)
        })
}

pub(crate) fn bridge_local_event_kind_for_local_event(
    event_type: &str,
) -> Option<HouseholdMeshLocalEventKind> {
    LOCAL_EVENT_MAPPINGS
        .iter()
        .find_map(|(known_event_type, local_event_kind, _)| {
            (*known_event_type == event_type).then_some(*local_event_kind)
        })
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
