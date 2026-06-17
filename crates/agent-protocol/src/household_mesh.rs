use serde::{Deserialize, Serialize};

use crate::constants::household_mesh as mesh;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum HouseholdMeshBridgeState {
    ExportSelected,
    LocalRepublishRequired,
}

impl HouseholdMeshBridgeState {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::ExportSelected => mesh::BRIDGE_STATE_EXPORT_SELECTED,
            Self::LocalRepublishRequired => mesh::BRIDGE_STATE_LOCAL_REPUBLISH_REQUIRED,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum HouseholdMeshAuthenticationState {
    PairedTrustedDevice,
    Anonymous,
    StaleOrRevoked,
}

impl HouseholdMeshAuthenticationState {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::PairedTrustedDevice => mesh::AUTHENTICATION_PAIRED_TRUSTED_DEVICE,
            Self::Anonymous => mesh::AUTHENTICATION_ANONYMOUS,
            Self::StaleOrRevoked => mesh::AUTHENTICATION_STALE_OR_REVOKED,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum HouseholdMeshPolicyAuthority {
    ChildAgentOnly,
    ProviderClaimed,
    ParentUiClaimed,
}

impl HouseholdMeshPolicyAuthority {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::ChildAgentOnly => mesh::POLICY_AUTHORITY_CHILD_AGENT_ONLY,
            Self::ProviderClaimed => mesh::POLICY_AUTHORITY_PROVIDER_CLAIMED,
            Self::ParentUiClaimed => mesh::POLICY_AUTHORITY_PARENT_UI_CLAIMED,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HouseholdMeshTransportEnvelope {
    pub schema_version: u16,
    pub message_id: String,
    pub idempotency_key: String,
    pub family_id: String,
    pub target_child_device_id: String,
    pub source_peer_id: String,
    pub local_event_ref: String,
    pub lan_message_type: String,
    pub bridge_state: HouseholdMeshBridgeState,
    pub authentication_state: HouseholdMeshAuthenticationState,
    pub policy_authority: HouseholdMeshPolicyAuthority,
    pub direct_remote_publish_requested: bool,
    pub raw_payload_included: bool,
    pub sent_at_epoch_seconds: u64,
    pub stale_after_seconds: u64,
}

impl HouseholdMeshTransportEnvelope {
    pub fn proof_fixture_for(local_event_ref: &str, lan_message_type: &str) -> Self {
        Self {
            schema_version: mesh::EVENT_SCHEMA_VERSION,
            message_id: mesh::TEST_BRIDGE_INBOUND_MESSAGE_ID.to_string(),
            idempotency_key: mesh::TEST_BRIDGE_IDEMPOTENCY_KEY.to_string(),
            family_id: mesh::TEST_BRIDGE_FAMILY_ID.to_string(),
            target_child_device_id: mesh::TEST_BRIDGE_TARGET_CHILD_DEVICE_ID.to_string(),
            source_peer_id: mesh::TEST_BRIDGE_CHILD_AGENT_PEER_ID.to_string(),
            local_event_ref: local_event_ref.to_string(),
            lan_message_type: lan_message_type.to_string(),
            bridge_state: HouseholdMeshBridgeState::ExportSelected,
            authentication_state: HouseholdMeshAuthenticationState::PairedTrustedDevice,
            policy_authority: HouseholdMeshPolicyAuthority::ChildAgentOnly,
            direct_remote_publish_requested: false,
            raw_payload_included: false,
            sent_at_epoch_seconds: mesh::TEST_BRIDGE_SENT_AT_EPOCH_SECONDS,
            stale_after_seconds: mesh::TEST_BRIDGE_STALE_AFTER_SECONDS,
        }
    }

    pub fn age_at_seconds(&self, received_at_epoch_seconds: u64) -> u64 {
        received_at_epoch_seconds.saturating_sub(self.sent_at_epoch_seconds)
    }

    pub fn is_stale_at(&self, received_at_epoch_seconds: u64) -> bool {
        self.age_at_seconds(received_at_epoch_seconds) > self.stale_after_seconds
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HouseholdMeshLocalRepublish {
    pub family_id: String,
    pub target_child_device_id: String,
    pub source_peer_id: String,
    pub local_event_ref: String,
    pub lan_message_type: String,
    pub bridge_state: HouseholdMeshBridgeState,
    pub policy_authority: HouseholdMeshPolicyAuthority,
    pub validated_before_republish: bool,
    pub child_agent_policy_authority_preserved: bool,
}

impl HouseholdMeshLocalRepublish {
    pub fn from_validated_message(message: &HouseholdMeshTransportEnvelope) -> Self {
        Self {
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
}
