use serde::{Deserialize, Serialize};

use super::{
    HouseholdMeshBridgeValidation, HouseholdMeshStructurallyValidatedTransportEnvelope,
    HouseholdMeshTransportEnvelope,
};
use crate::constants;

#[path = "household_mesh_bridge_input_validation.rs"]
mod household_mesh_bridge_input_validation;

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
    pub fn inbound_envelope(&self) -> HouseholdMeshBridgeInboundEnvelope {
        HouseholdMeshBridgeInboundEnvelope {
            message: self.inbound_message.clone(),
            expected_family_id: self.family_id.clone(),
            expected_target_child_device_id: self.target_child_device_id.clone(),
            received_at_epoch_seconds: self.received_at_epoch_seconds,
            seen_message_ids: self.seen_message_ids.clone(),
            seen_idempotency_keys: self.seen_idempotency_keys.clone(),
        }
    }

    pub fn validate_inbound(
        &self,
    ) -> Result<HouseholdMeshStructurallyValidatedTransportEnvelope, HouseholdMeshBridgeValidation>
    {
        self.inbound_envelope().validate_structure()
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
    pub seen_message_ids: Vec<String>,
    pub seen_idempotency_keys: Vec<String>,
}

impl HouseholdMeshBridgeInboundEnvelope {
    pub fn for_structural_validation(
        message: HouseholdMeshTransportEnvelope,
        expected_family_id: String,
        expected_target_child_device_id: String,
        received_at_epoch_seconds: u64,
        seen_message_ids: Vec<String>,
        seen_idempotency_keys: Vec<String>,
    ) -> Self {
        Self {
            message,
            expected_family_id,
            expected_target_child_device_id,
            received_at_epoch_seconds,
            seen_message_ids,
            seen_idempotency_keys,
        }
    }

    pub fn validate_structure(
        self,
    ) -> Result<HouseholdMeshStructurallyValidatedTransportEnvelope, HouseholdMeshBridgeValidation>
    {
        household_mesh_bridge_input_validation::validate_structure(self)
    }
}
