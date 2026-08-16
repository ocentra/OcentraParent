use crate::constants;
use crate::household_mesh::{HouseholdMeshBridgeRejectionReason, HouseholdMeshTransportEnvelope};

const LOCAL_EVENT_TO_LAN_MESSAGE: [(&str, &str); 13] = [
    (
        constants::household_mesh::LOCAL_EVENT_DEVICE_DISCOVERY,
        constants::household_mesh::LAN_MESSAGE_DEVICE_DISCOVERY,
    ),
    (
        constants::household_mesh::LOCAL_EVENT_PROVIDER_ADVERTISEMENT,
        constants::household_mesh::LAN_MESSAGE_PROVIDER_ADVERTISEMENT,
    ),
    (
        constants::household_mesh::LOCAL_EVENT_PROVIDER_HEARTBEAT,
        constants::household_mesh::LAN_MESSAGE_PROVIDER_HEARTBEAT,
    ),
    (
        constants::household_mesh::LOCAL_EVENT_PROVIDER_CAPABILITY,
        constants::household_mesh::LAN_MESSAGE_PROVIDER_CAPABILITY,
    ),
    (
        constants::household_mesh::LOCAL_EVENT_AI_WORK_OFFER,
        constants::household_mesh::LAN_MESSAGE_AI_WORK_OFFER,
    ),
    (
        constants::household_mesh::LOCAL_EVENT_AI_WORK_CLAIM_REQUEST,
        constants::household_mesh::LAN_MESSAGE_AI_WORK_CLAIM_REQUEST,
    ),
    (
        constants::household_mesh::LOCAL_EVENT_AI_WORK_CLAIM_DECISION,
        constants::household_mesh::LAN_MESSAGE_AI_WORK_CLAIM_DECISION,
    ),
    (
        constants::household_mesh::LOCAL_EVENT_AI_WORK_LEASE_STATE,
        constants::household_mesh::LAN_MESSAGE_AI_WORK_LEASE_STATE,
    ),
    (
        constants::household_mesh::LOCAL_EVENT_AI_JOB_PAYLOAD_TRANSFER,
        constants::household_mesh::LAN_MESSAGE_AI_JOB_PAYLOAD_TRANSFER,
    ),
    (
        constants::household_mesh::LOCAL_EVENT_AI_RESULT_RETURN,
        constants::household_mesh::LAN_MESSAGE_AI_RESULT_RETURN,
    ),
    (
        constants::household_mesh::LOCAL_EVENT_CONFIG_COMMAND,
        constants::household_mesh::LAN_MESSAGE_CONFIG_COMMAND,
    ),
    (
        constants::household_mesh::LOCAL_EVENT_APPROVAL_OVERRIDE_COMMAND,
        constants::household_mesh::LAN_MESSAGE_APPROVAL_OVERRIDE_COMMAND,
    ),
    (
        constants::household_mesh::LOCAL_EVENT_READ_MODEL_QUERY_REQUEST,
        constants::household_mesh::LAN_MESSAGE_READ_MODEL_QUERY_REQUEST,
    ),
];

pub(super) fn rejection_reason(
    message: &HouseholdMeshTransportEnvelope,
) -> Option<HouseholdMeshBridgeRejectionReason> {
    let Some(expected_lan_message_type) = expected_lan_message_type(&message.local_event_ref)
    else {
        return Some(HouseholdMeshBridgeRejectionReason::UnselectedEvent);
    };
    if message.lan_message_type != expected_lan_message_type {
        return Some(HouseholdMeshBridgeRejectionReason::MismatchedMessageRef);
    }
    None
}

fn expected_lan_message_type(local_event_ref: &str) -> Option<&'static str> {
    LOCAL_EVENT_TO_LAN_MESSAGE
        .iter()
        .find_map(|(event_ref, message_type)| {
            (*event_ref == local_event_ref).then_some(*message_type)
        })
}
