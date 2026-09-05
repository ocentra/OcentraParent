use ocentra_parent_agent_protocol::constants;

use crate::household_mesh_event_bridge::HouseholdMeshLocalEventKind;

struct LocalEventTypeLookup {
    event_type: &'static str,
    local_event_kind: HouseholdMeshLocalEventKind,
}

struct SelectedLocalEventLookup {
    local_event_kind: HouseholdMeshLocalEventKind,
    local_event_ref: &'static str,
    lan_message_type: &'static str,
}

const LOCAL_EVENT_TYPE_LOOKUPS: [LocalEventTypeLookup; 2] = [
    LocalEventTypeLookup {
        event_type: constants::screen_flow::EVENT_SCREEN_MESH_OFFER_PUBLISHED,
        local_event_kind: HouseholdMeshLocalEventKind::AiWorkOffer,
    },
    LocalEventTypeLookup {
        event_type: constants::screen_flow::EVENT_SCREEN_MESH_PROVIDER_RESULT_RETURNED,
        local_event_kind: HouseholdMeshLocalEventKind::AiResultReturn,
    },
];

const SELECTED_LOCAL_EVENT_LOOKUPS: [SelectedLocalEventLookup; 13] = [
    SelectedLocalEventLookup {
        local_event_kind: HouseholdMeshLocalEventKind::DeviceDiscovery,
        local_event_ref: constants::household_mesh::LOCAL_EVENT_DEVICE_DISCOVERY,
        lan_message_type: constants::household_mesh::LAN_MESSAGE_DEVICE_DISCOVERY,
    },
    SelectedLocalEventLookup {
        local_event_kind: HouseholdMeshLocalEventKind::ProviderAdvertisement,
        local_event_ref: constants::household_mesh::LOCAL_EVENT_PROVIDER_ADVERTISEMENT,
        lan_message_type: constants::household_mesh::LAN_MESSAGE_PROVIDER_ADVERTISEMENT,
    },
    SelectedLocalEventLookup {
        local_event_kind: HouseholdMeshLocalEventKind::ProviderHeartbeat,
        local_event_ref: constants::household_mesh::LOCAL_EVENT_PROVIDER_HEARTBEAT,
        lan_message_type: constants::household_mesh::LAN_MESSAGE_PROVIDER_HEARTBEAT,
    },
    SelectedLocalEventLookup {
        local_event_kind: HouseholdMeshLocalEventKind::ProviderCapability,
        local_event_ref: constants::household_mesh::LOCAL_EVENT_PROVIDER_CAPABILITY,
        lan_message_type: constants::household_mesh::LAN_MESSAGE_PROVIDER_CAPABILITY,
    },
    SelectedLocalEventLookup {
        local_event_kind: HouseholdMeshLocalEventKind::AiWorkOffer,
        local_event_ref: constants::household_mesh::LOCAL_EVENT_AI_WORK_OFFER,
        lan_message_type: constants::household_mesh::LAN_MESSAGE_AI_WORK_OFFER,
    },
    SelectedLocalEventLookup {
        local_event_kind: HouseholdMeshLocalEventKind::AiWorkClaimRequest,
        local_event_ref: constants::household_mesh::LOCAL_EVENT_AI_WORK_CLAIM_REQUEST,
        lan_message_type: constants::household_mesh::LAN_MESSAGE_AI_WORK_CLAIM_REQUEST,
    },
    SelectedLocalEventLookup {
        local_event_kind: HouseholdMeshLocalEventKind::AiWorkClaimDecision,
        local_event_ref: constants::household_mesh::LOCAL_EVENT_AI_WORK_CLAIM_DECISION,
        lan_message_type: constants::household_mesh::LAN_MESSAGE_AI_WORK_CLAIM_DECISION,
    },
    SelectedLocalEventLookup {
        local_event_kind: HouseholdMeshLocalEventKind::AiWorkLeaseState,
        local_event_ref: constants::household_mesh::LOCAL_EVENT_AI_WORK_LEASE_STATE,
        lan_message_type: constants::household_mesh::LAN_MESSAGE_AI_WORK_LEASE_STATE,
    },
    SelectedLocalEventLookup {
        local_event_kind: HouseholdMeshLocalEventKind::AiJobPayloadTransfer,
        local_event_ref: constants::household_mesh::LOCAL_EVENT_AI_JOB_PAYLOAD_TRANSFER,
        lan_message_type: constants::household_mesh::LAN_MESSAGE_AI_JOB_PAYLOAD_TRANSFER,
    },
    SelectedLocalEventLookup {
        local_event_kind: HouseholdMeshLocalEventKind::AiResultReturn,
        local_event_ref: constants::household_mesh::LOCAL_EVENT_AI_RESULT_RETURN,
        lan_message_type: constants::household_mesh::LAN_MESSAGE_AI_RESULT_RETURN,
    },
    SelectedLocalEventLookup {
        local_event_kind: HouseholdMeshLocalEventKind::ConfigCommand,
        local_event_ref: constants::household_mesh::LOCAL_EVENT_CONFIG_COMMAND,
        lan_message_type: constants::household_mesh::LAN_MESSAGE_CONFIG_COMMAND,
    },
    SelectedLocalEventLookup {
        local_event_kind: HouseholdMeshLocalEventKind::ApprovalOverrideCommand,
        local_event_ref: constants::household_mesh::LOCAL_EVENT_APPROVAL_OVERRIDE_COMMAND,
        lan_message_type: constants::household_mesh::LAN_MESSAGE_APPROVAL_OVERRIDE_COMMAND,
    },
    SelectedLocalEventLookup {
        local_event_kind: HouseholdMeshLocalEventKind::ReadModelQueryRequest,
        local_event_ref: constants::household_mesh::LOCAL_EVENT_READ_MODEL_QUERY_REQUEST,
        lan_message_type: constants::household_mesh::LAN_MESSAGE_READ_MODEL_QUERY_REQUEST,
    },
];

pub(crate) fn bridge_local_event_kind_for_local_event(
    event_type: &str,
) -> Option<HouseholdMeshLocalEventKind> {
    LOCAL_EVENT_TYPE_LOOKUPS
        .iter()
        .find_map(|lookup| (lookup.event_type == event_type).then_some(lookup.local_event_kind))
}

pub(crate) fn local_event_ref(event_kind: HouseholdMeshLocalEventKind) -> Option<&'static str> {
    SELECTED_LOCAL_EVENT_LOOKUPS.iter().find_map(|lookup| {
        (lookup.local_event_kind == event_kind).then_some(lookup.local_event_ref)
    })
}

pub(crate) fn lan_message_type_for_ref(local_event_ref: &str) -> Option<&'static str> {
    SELECTED_LOCAL_EVENT_LOOKUPS.iter().find_map(|lookup| {
        (lookup.local_event_ref == local_event_ref).then_some(lookup.lan_message_type)
    })
}
