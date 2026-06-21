use serde::{Deserialize, Serialize};

use super::HouseholdMeshTransportEnvelope;
use crate::constants;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HouseholdMeshBridgeInput {
    pub correlation_id: String,
    pub local_event_type: String,
    pub family_id: String,
    pub target_child_device_id: String,
    pub outbound_message_id: String,
    pub outbound_idempotency_key: String,
    pub child_agent_peer_id: String,
    pub provider_peer_id: String,
    pub payload_ref: String,
    pub observed_at: String,
    pub received_at_epoch_seconds: u64,
    pub inbound_message: HouseholdMeshTransportEnvelope,
    pub seen_message_ids: Vec<String>,
    pub seen_idempotency_keys: Vec<String>,
}

impl HouseholdMeshBridgeInput {
    pub fn proof_fixture() -> Self {
        let mut inbound_message = HouseholdMeshTransportEnvelope::proof_fixture_for(
            constants::household_mesh::LOCAL_EVENT_AI_RESULT_RETURN,
            constants::household_mesh::LAN_MESSAGE_AI_RESULT_RETURN,
        );
        inbound_message.source_peer_id =
            constants::household_mesh::TEST_BRIDGE_PROVIDER_PEER_ID.to_string();
        Self {
            correlation_id: constants::household_mesh::TEST_BRIDGE_CORRELATION_ID.to_string(),
            local_event_type: constants::screen_flow::EVENT_SCREEN_MESH_OFFER_PUBLISHED.to_string(),
            family_id: constants::household_mesh::TEST_BRIDGE_FAMILY_ID.to_string(),
            target_child_device_id: constants::household_mesh::TEST_BRIDGE_TARGET_CHILD_DEVICE_ID
                .to_string(),
            outbound_message_id: constants::household_mesh::TEST_BRIDGE_OUTBOUND_MESSAGE_ID
                .to_string(),
            outbound_idempotency_key: constants::household_mesh::TEST_BRIDGE_IDEMPOTENCY_KEY
                .to_string(),
            child_agent_peer_id: constants::household_mesh::TEST_BRIDGE_CHILD_AGENT_PEER_ID
                .to_string(),
            provider_peer_id: constants::household_mesh::TEST_BRIDGE_PROVIDER_PEER_ID.to_string(),
            payload_ref: constants::household_mesh::TEST_BRIDGE_PAYLOAD_REF.to_string(),
            observed_at: constants::activity_store::TEST_FIRST_OBSERVED_AT.to_string(),
            received_at_epoch_seconds:
                constants::household_mesh::TEST_BRIDGE_RECEIVED_AT_EPOCH_SECONDS,
            inbound_message,
            seen_message_ids: Vec::new(),
            seen_idempotency_keys: Vec::new(),
        }
    }

    pub fn inbound_envelope(&self) -> HouseholdMeshBridgeInboundEnvelope {
        HouseholdMeshBridgeInboundEnvelope {
            message: self.inbound_message.clone(),
            expected_family_id: self.family_id.clone(),
            expected_target_child_device_id: self.target_child_device_id.clone(),
            received_at_epoch_seconds: self.received_at_epoch_seconds,
            authorized: true,
            seen_message_ids: self.seen_message_ids.clone(),
            seen_idempotency_keys: self.seen_idempotency_keys.clone(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HouseholdMeshBridgeExportCandidate {
    pub local_event_type: String,
    pub contains_raw_screenshot: bool,
    pub private_local_event: bool,
}

impl HouseholdMeshBridgeExportCandidate {
    pub fn selected_offer() -> Self {
        Self {
            local_event_type: constants::screen_flow::EVENT_SCREEN_MESH_OFFER_PUBLISHED.to_string(),
            contains_raw_screenshot: false,
            private_local_event: false,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HouseholdMeshBridgeInboundEnvelope {
    pub message: HouseholdMeshTransportEnvelope,
    pub expected_family_id: String,
    pub expected_target_child_device_id: String,
    pub received_at_epoch_seconds: u64,
    pub authorized: bool,
    pub seen_message_ids: Vec<String>,
    pub seen_idempotency_keys: Vec<String>,
}

impl HouseholdMeshBridgeInboundEnvelope {
    pub fn accepted_offer() -> Self {
        let mut message = HouseholdMeshTransportEnvelope::proof_fixture_for(
            constants::household_mesh::LOCAL_EVENT_AI_WORK_OFFER,
            constants::household_mesh::LAN_MESSAGE_AI_WORK_OFFER,
        );
        message.source_peer_id =
            constants::household_mesh::TEST_BRIDGE_PROVIDER_PEER_ID.to_string();
        Self {
            message,
            expected_family_id: constants::household_mesh::TEST_BRIDGE_FAMILY_ID.to_string(),
            expected_target_child_device_id:
                constants::household_mesh::TEST_BRIDGE_TARGET_CHILD_DEVICE_ID.to_string(),
            received_at_epoch_seconds:
                constants::household_mesh::TEST_BRIDGE_RECEIVED_AT_EPOCH_SECONDS,
            authorized: true,
            seen_message_ids: Vec::new(),
            seen_idempotency_keys: Vec::new(),
        }
    }

    pub fn accepted_result() -> Self {
        let mut message = HouseholdMeshTransportEnvelope::proof_fixture_for(
            constants::household_mesh::LOCAL_EVENT_AI_RESULT_RETURN,
            constants::household_mesh::LAN_MESSAGE_AI_RESULT_RETURN,
        );
        message.source_peer_id =
            constants::household_mesh::TEST_BRIDGE_PROVIDER_PEER_ID.to_string();
        Self {
            message,
            expected_family_id: constants::household_mesh::TEST_BRIDGE_FAMILY_ID.to_string(),
            expected_target_child_device_id:
                constants::household_mesh::TEST_BRIDGE_TARGET_CHILD_DEVICE_ID.to_string(),
            received_at_epoch_seconds:
                constants::household_mesh::TEST_BRIDGE_RECEIVED_AT_EPOCH_SECONDS,
            authorized: true,
            seen_message_ids: Vec::new(),
            seen_idempotency_keys: Vec::new(),
        }
    }
}
