use ocentra_eventing::envelope::{DomainEvent, EventContract};
use ocentra_eventing::error::EventingError;
use ocentra_eventing::ids::{AggregateKey, EventType, IdempotencyKey, SchemaVersion};
use serde::{Deserialize, Serialize};

use crate::constants;
use crate::constants::household_mesh as mesh;

pub mod household_mesh_bridge_input;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u8)]
#[serde(rename_all = "kebab-case")]
pub enum HouseholdMeshBridgeState {
    ExportSelected,
    LocalRepublishRequired,
}

impl HouseholdMeshBridgeState {
    const PROTOCOL_STRINGS: [&'static str; 2] = [
        mesh::BRIDGE_STATE_EXPORT_SELECTED,
        mesh::BRIDGE_STATE_LOCAL_REPUBLISH_REQUIRED,
    ];

    pub fn as_str(self) -> &'static str {
        Self::PROTOCOL_STRINGS[self as usize]
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u8)]
#[serde(rename_all = "kebab-case")]
pub enum HouseholdMeshAuthenticationState {
    PairedTrustedDevice,
    Anonymous,
    StaleOrRevoked,
}

impl HouseholdMeshAuthenticationState {
    const PROTOCOL_STRINGS: [&'static str; 3] = [
        mesh::AUTHENTICATION_PAIRED_TRUSTED_DEVICE,
        mesh::AUTHENTICATION_ANONYMOUS,
        mesh::AUTHENTICATION_STALE_OR_REVOKED,
    ];

    pub fn as_str(self) -> &'static str {
        Self::PROTOCOL_STRINGS[self as usize]
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u8)]
#[serde(rename_all = "kebab-case")]
pub enum HouseholdMeshPolicyAuthority {
    ChildAgentOnly,
    ProviderClaimed,
    ParentUiClaimed,
}

impl HouseholdMeshPolicyAuthority {
    const PROTOCOL_STRINGS: [&'static str; 3] = [
        mesh::POLICY_AUTHORITY_CHILD_AGENT_ONLY,
        mesh::POLICY_AUTHORITY_PROVIDER_CLAIMED,
        mesh::POLICY_AUTHORITY_PARENT_UI_CLAIMED,
    ];

    pub fn as_str(self) -> &'static str {
        Self::PROTOCOL_STRINGS[self as usize]
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
    #[cfg(any(test, feature = "test-support"))]
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

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u8)]
pub enum HouseholdMeshBridgePhase {
    LocalEventSelected,
    LanMessageExported,
    LanMessageReceived,
    LocalEventRepublished,
}

impl HouseholdMeshBridgePhase {
    pub fn ordered_chain() -> &'static [Self] {
        &[
            Self::LocalEventSelected,
            Self::LanMessageExported,
            Self::LanMessageReceived,
            Self::LocalEventRepublished,
        ]
    }

    pub fn event_type(self) -> &'static str {
        Self::EVENT_TYPES[self as usize]
    }

    pub fn subscriber_id(self) -> &'static str {
        Self::SUBSCRIBER_IDS[self as usize]
    }

    pub fn target_handler(self) -> &'static str {
        Self::TARGET_HANDLERS[self as usize]
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u8)]
pub enum HouseholdMeshBridgeDirection {
    Export,
    Import,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u8)]
pub enum HouseholdMeshBridgeEnvelopeState {
    LocalSelected,
    LanExported,
    LanReceived,
    LocalRepublished,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u8)]
pub enum HouseholdMeshBridgeValidationState {
    Accepted,
    Rejected,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u8)]
pub enum HouseholdMeshBridgeRejectionReason {
    UnselectedEvent,
    PrivateLocalEvent,
    RawScreenPayload,
    UnauthenticatedPeer,
    UnauthorizedPeer,
    DirectRemotePublish,
    PolicyAuthorityEscalation,
    MismatchedMessageRef,
    ReplayedMessage,
    StaleMessage,
    FamilyMismatch,
    WrongTargetDevice,
    UnsupportedLanMessage,
}

impl HouseholdMeshBridgePhase {
    const EVENT_TYPES: [&'static str; 4] = [
        constants::household_mesh::EVENT_BRIDGE_LOCAL_SELECTED,
        constants::household_mesh::EVENT_BRIDGE_LAN_EXPORTED,
        constants::household_mesh::EVENT_BRIDGE_LAN_RECEIVED,
        constants::household_mesh::EVENT_BRIDGE_LOCAL_REPUBLISHED,
    ];

    const SUBSCRIBER_IDS: [&'static str; 4] = [
        constants::household_mesh::SUBSCRIBER_BRIDGE_LOCAL_SELECTED,
        constants::household_mesh::SUBSCRIBER_BRIDGE_LAN_EXPORTED,
        constants::household_mesh::SUBSCRIBER_BRIDGE_LAN_RECEIVED,
        constants::household_mesh::SUBSCRIBER_BRIDGE_LOCAL_REPUBLISHED,
    ];

    const TARGET_HANDLERS: [&'static str; 4] = [
        constants::household_mesh::TARGET_BRIDGE_EXPORT_VALIDATOR,
        constants::household_mesh::TARGET_BRIDGE_LAN_TRANSPORT,
        constants::household_mesh::TARGET_BRIDGE_IMPORT_VALIDATOR,
        constants::household_mesh::TARGET_LOCAL_EVENT_REPUBLISHER,
    ];
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct HouseholdMeshBridgeCustody {
    pub selected_event_only: bool,
    pub remote_direct_publish_allowed: bool,
    pub raw_screenshot_transferred: bool,
    pub private_local_event_exported: bool,
}

impl HouseholdMeshBridgeCustody {
    pub fn selected_bridge_only() -> Self {
        Self {
            selected_event_only: true,
            remote_direct_publish_allowed: false,
            raw_screenshot_transferred: false,
            private_local_event_exported: false,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct HouseholdMeshBridgeValidation {
    pub state: HouseholdMeshBridgeValidationState,
    pub rejection_reason: Option<HouseholdMeshBridgeRejectionReason>,
}

impl HouseholdMeshBridgeValidation {
    pub(crate) fn rejected(reason: HouseholdMeshBridgeRejectionReason) -> Self {
        Self {
            state: HouseholdMeshBridgeValidationState::Rejected,
            rejection_reason: Some(reason),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HouseholdMeshStructurallyValidatedTransportEnvelope {
    message: HouseholdMeshTransportEnvelope,
}

impl HouseholdMeshStructurallyValidatedTransportEnvelope {
    pub fn message(&self) -> &HouseholdMeshTransportEnvelope {
        &self.message
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HouseholdMeshBridgeEventPayload {
    pub phase: HouseholdMeshBridgePhase,
    pub envelope_state: HouseholdMeshBridgeEnvelopeState,
    pub direction: HouseholdMeshBridgeDirection,
    pub local_event_type: String,
    pub local_event_ref: String,
    pub lan_message_type: String,
    pub family_id: String,
    pub target_child_device_id: String,
    pub source_peer_id: String,
    pub idempotency_key: String,
    pub outbound_message_id: String,
    pub inbound_message_id: String,
    pub child_agent_peer_id: String,
    pub provider_peer_id: String,
    pub payload_ref: String,
    pub previous_phase_ref: Option<String>,
    pub validation_state: HouseholdMeshBridgeValidationState,
    pub rejection_reason: Option<HouseholdMeshBridgeRejectionReason>,
    pub custody: HouseholdMeshBridgeCustody,
    pub observed_at: String,
}

impl DomainEvent for HouseholdMeshBridgeEventPayload {
    fn contract(&self) -> Result<EventContract, EventingError> {
        Ok(EventContract::new(
            EventType::parse(self.phase.event_type())?,
            SchemaVersion::new(constants::household_mesh::EVENT_SCHEMA_VERSION)?,
        ))
    }

    fn aggregate_key(&self) -> Result<AggregateKey, EventingError> {
        AggregateKey::parse(household_mesh_bridge_aggregate_key(
            &self.outbound_message_id,
        ))
    }

    fn idempotency_key(&self) -> Result<IdempotencyKey, EventingError> {
        let mut value = String::from(constants::household_mesh::IDEMPOTENCY_HOUSEHOLD_MESH_PREFIX);
        value.push_str(self.phase.event_type());
        value.push(constants::delimiter::HYPHEN);
        value.push_str(&self.outbound_message_id);
        value.push(constants::delimiter::HYPHEN);
        value.push_str(&self.observed_at);
        IdempotencyKey::parse(value)
    }
}

fn household_mesh_bridge_aggregate_key(correlation_id: &str) -> String {
    let mut value = String::from(constants::household_mesh::AGGREGATE_HOUSEHOLD_MESH_PREFIX);
    value.push_str(correlation_id);
    value
}
