use serde::{Deserialize, Serialize};

use super::{
    HouseholdMeshAuthenticationState, HouseholdMeshBridgeRejectionReason, HouseholdMeshBridgeState,
    HouseholdMeshBridgeValidation, HouseholdMeshPolicyAuthority,
    HouseholdMeshStructurallyValidatedTransportEnvelope, HouseholdMeshTransportEnvelope,
};
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
    #[cfg(any(test, feature = "test-support"))]
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
        let message = &self.message;
        if message.schema_version != constants::household_mesh::EVENT_SCHEMA_VERSION {
            return Err(HouseholdMeshBridgeValidation::rejected(
                HouseholdMeshBridgeRejectionReason::UnsupportedLanMessage,
            ));
        }
        if message.message_id.is_empty()
            || message.idempotency_key.is_empty()
            || message.local_event_ref.is_empty()
            || message.source_peer_id.is_empty()
        {
            return Err(HouseholdMeshBridgeValidation::rejected(
                HouseholdMeshBridgeRejectionReason::MismatchedMessageRef,
            ));
        }
        if message.bridge_state != HouseholdMeshBridgeState::ExportSelected {
            return Err(HouseholdMeshBridgeValidation::rejected(
                HouseholdMeshBridgeRejectionReason::MismatchedMessageRef,
            ));
        }
        if message.authentication_state != HouseholdMeshAuthenticationState::PairedTrustedDevice {
            return Err(HouseholdMeshBridgeValidation::rejected(
                HouseholdMeshBridgeRejectionReason::UnauthenticatedPeer,
            ));
        }
        if message.direct_remote_publish_requested {
            return Err(HouseholdMeshBridgeValidation::rejected(
                HouseholdMeshBridgeRejectionReason::DirectRemotePublish,
            ));
        }
        if message.raw_payload_included {
            return Err(HouseholdMeshBridgeValidation::rejected(
                HouseholdMeshBridgeRejectionReason::RawScreenPayload,
            ));
        }
        if message.policy_authority != HouseholdMeshPolicyAuthority::ChildAgentOnly {
            return Err(HouseholdMeshBridgeValidation::rejected(
                HouseholdMeshBridgeRejectionReason::PolicyAuthorityEscalation,
            ));
        }
        if message.family_id != self.expected_family_id {
            return Err(HouseholdMeshBridgeValidation::rejected(
                HouseholdMeshBridgeRejectionReason::FamilyMismatch,
            ));
        }
        if message.target_child_device_id != self.expected_target_child_device_id {
            return Err(HouseholdMeshBridgeValidation::rejected(
                HouseholdMeshBridgeRejectionReason::WrongTargetDevice,
            ));
        }
        if message.is_stale_at(self.received_at_epoch_seconds) {
            return Err(HouseholdMeshBridgeValidation::rejected(
                HouseholdMeshBridgeRejectionReason::StaleMessage,
            ));
        }
        if self
            .seen_message_ids
            .iter()
            .any(|seen| seen == &message.message_id)
            || self
                .seen_idempotency_keys
                .iter()
                .any(|seen| seen == &message.idempotency_key)
        {
            return Err(HouseholdMeshBridgeValidation::rejected(
                HouseholdMeshBridgeRejectionReason::ReplayedMessage,
            ));
        }
        let Some(expected_lan_message_type) = expected_lan_message_type(&message.local_event_ref)
        else {
            return Err(HouseholdMeshBridgeValidation::rejected(
                HouseholdMeshBridgeRejectionReason::UnselectedEvent,
            ));
        };
        if message.lan_message_type != expected_lan_message_type {
            return Err(HouseholdMeshBridgeValidation::rejected(
                HouseholdMeshBridgeRejectionReason::MismatchedMessageRef,
            ));
        }

        Ok(HouseholdMeshStructurallyValidatedTransportEnvelope {
            message: self.message,
        })
    }

    #[cfg(any(test, feature = "test-support"))]
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
            seen_message_ids: Vec::new(),
            seen_idempotency_keys: Vec::new(),
        }
    }

    #[cfg(any(test, feature = "test-support"))]
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
            seen_message_ids: Vec::new(),
            seen_idempotency_keys: Vec::new(),
        }
    }
}

fn expected_lan_message_type(local_event_ref: &str) -> Option<&'static str> {
    match local_event_ref {
        constants::household_mesh::LOCAL_EVENT_DEVICE_DISCOVERY => {
            Some(constants::household_mesh::LAN_MESSAGE_DEVICE_DISCOVERY)
        }
        constants::household_mesh::LOCAL_EVENT_PROVIDER_ADVERTISEMENT => {
            Some(constants::household_mesh::LAN_MESSAGE_PROVIDER_ADVERTISEMENT)
        }
        constants::household_mesh::LOCAL_EVENT_PROVIDER_HEARTBEAT => {
            Some(constants::household_mesh::LAN_MESSAGE_PROVIDER_HEARTBEAT)
        }
        constants::household_mesh::LOCAL_EVENT_PROVIDER_CAPABILITY => {
            Some(constants::household_mesh::LAN_MESSAGE_PROVIDER_CAPABILITY)
        }
        constants::household_mesh::LOCAL_EVENT_AI_WORK_OFFER => {
            Some(constants::household_mesh::LAN_MESSAGE_AI_WORK_OFFER)
        }
        constants::household_mesh::LOCAL_EVENT_AI_WORK_CLAIM_REQUEST => {
            Some(constants::household_mesh::LAN_MESSAGE_AI_WORK_CLAIM_REQUEST)
        }
        constants::household_mesh::LOCAL_EVENT_AI_WORK_CLAIM_DECISION => {
            Some(constants::household_mesh::LAN_MESSAGE_AI_WORK_CLAIM_DECISION)
        }
        constants::household_mesh::LOCAL_EVENT_AI_WORK_LEASE_STATE => {
            Some(constants::household_mesh::LAN_MESSAGE_AI_WORK_LEASE_STATE)
        }
        constants::household_mesh::LOCAL_EVENT_AI_JOB_PAYLOAD_TRANSFER => {
            Some(constants::household_mesh::LAN_MESSAGE_AI_JOB_PAYLOAD_TRANSFER)
        }
        constants::household_mesh::LOCAL_EVENT_AI_RESULT_RETURN => {
            Some(constants::household_mesh::LAN_MESSAGE_AI_RESULT_RETURN)
        }
        constants::household_mesh::LOCAL_EVENT_CONFIG_COMMAND => {
            Some(constants::household_mesh::LAN_MESSAGE_CONFIG_COMMAND)
        }
        constants::household_mesh::LOCAL_EVENT_APPROVAL_OVERRIDE_COMMAND => {
            Some(constants::household_mesh::LAN_MESSAGE_APPROVAL_OVERRIDE_COMMAND)
        }
        constants::household_mesh::LOCAL_EVENT_READ_MODEL_QUERY_REQUEST => {
            Some(constants::household_mesh::LAN_MESSAGE_READ_MODEL_QUERY_REQUEST)
        }
        _ => None,
    }
}
