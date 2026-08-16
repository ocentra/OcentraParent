use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::household_mesh::{
    HouseholdMeshBridgeState, HouseholdMeshTransportEnvelope,
};

use crate::{
    household_mesh_event_bridge_lookup::{lan_message_type_for_ref, local_event_ref},
    household_mesh_event_bridge_rejection::rejection_as_str,
};

#[path = "household_mesh_event_bridge_validation.rs"]
mod household_mesh_event_bridge_validation;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HouseholdMeshLocalEventKind {
    DeviceDiscovery,
    ProviderAdvertisement,
    ProviderHeartbeat,
    ProviderCapability,
    AiWorkOffer,
    AiWorkClaimRequest,
    AiWorkClaimDecision,
    AiWorkLeaseState,
    AiJobPayloadTransfer,
    AiResultReturn,
    ConfigCommand,
    ApprovalOverrideCommand,
    ReadModelQueryRequest,
    RawCaptureInternal,
    AdapterInternal,
    PrivateQueueMechanic,
    PolicyDecision,
    EnforcementCommand,
}

pub type HouseholdMeshAuthenticationState =
    ocentra_parent_agent_protocol::household_mesh::HouseholdMeshAuthenticationState;
pub type HouseholdMeshPolicyAuthority =
    ocentra_parent_agent_protocol::household_mesh::HouseholdMeshPolicyAuthority;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HouseholdMeshBridgeRejection {
    UnselectedLocalEvent,
    UnauthenticatedMessage,
    DirectRemotePublish,
    PolicyAuthorityEscalation,
    RawPayload,
    MismatchedMessageRef,
    ReplayedMessage,
    StaleMessage,
    FamilyMismatch,
    WrongTargetDevice,
}

pub type HouseholdMeshLanMessage = HouseholdMeshTransportEnvelope;
pub type HouseholdMeshLocalRepublish =
    ocentra_parent_agent_protocol::household_mesh::HouseholdMeshLocalRepublish;

/// Runtime-owned proof that the inbound peer has been authorized by a
/// trusted composition boundary.  No constructor is exposed: the current
/// resolver is deliberately unavailable until the LAN authority/registry
/// integration exists.
pub(crate) struct HouseholdMeshPeerAuthorization {
    source_peer_id: String,
}

impl HouseholdMeshPeerAuthorization {
    fn matches_source(&self, source_peer_id: &str) -> bool {
        self.source_peer_id == source_peer_id
    }
}

pub(crate) fn resolve_household_mesh_peer_authorization(
    _message: &HouseholdMeshLanMessage,
    _expected_family_id: &str,
    _expected_target_child_device_id: &str,
) -> Option<HouseholdMeshPeerAuthorization> {
    // Do not promote envelope authentication fields or caller-supplied peer
    // IDs into runtime authority.  LAN pairing/account composition must
    // supply the non-forgeable token before republish can be enabled.
    None
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum HouseholdMeshExportDecision {
    Export(HouseholdMeshLanMessage),
    Reject(HouseholdMeshBridgeRejection),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum HouseholdMeshImportDecision {
    Republish(HouseholdMeshLocalRepublish),
    Reject(HouseholdMeshBridgeRejection),
}

pub fn export_selected_local_event(
    event_kind: HouseholdMeshLocalEventKind,
    family_id: &str,
    target_child_device_id: &str,
    source_peer_id: &str,
    message_id: &str,
    idempotency_key: &str,
    delivery_window_seconds: (u64, u64),
) -> HouseholdMeshExportDecision {
    let (sent_at_epoch_seconds, stale_after_seconds) = delivery_window_seconds;
    let Some(local_event_ref) = local_event_ref(event_kind) else {
        return HouseholdMeshExportDecision::Reject(
            HouseholdMeshBridgeRejection::UnselectedLocalEvent,
        );
    };
    let Some(lan_message_type) = lan_message_type_for_ref(local_event_ref) else {
        return HouseholdMeshExportDecision::Reject(
            HouseholdMeshBridgeRejection::UnselectedLocalEvent,
        );
    };
    HouseholdMeshExportDecision::Export(HouseholdMeshLanMessage {
        schema_version: constants::household_mesh::EVENT_SCHEMA_VERSION,
        message_id: message_id.to_string(),
        idempotency_key: idempotency_key.to_string(),
        family_id: family_id.to_string(),
        target_child_device_id: target_child_device_id.to_string(),
        source_peer_id: source_peer_id.to_string(),
        local_event_ref: local_event_ref.to_string(),
        lan_message_type: lan_message_type.to_string(),
        bridge_state: HouseholdMeshBridgeState::ExportSelected,
        authentication_state: HouseholdMeshAuthenticationState::PairedTrustedDevice,
        policy_authority: HouseholdMeshPolicyAuthority::ChildAgentOnly,
        direct_remote_publish_requested: false,
        raw_payload_included: false,
        sent_at_epoch_seconds,
        stale_after_seconds,
    })
}

pub(crate) fn validate_incoming_lan_message(
    message: &HouseholdMeshLanMessage,
    expected_family_id: &str,
    expected_target_child_device_id: &str,
    received_at_epoch_seconds: u64,
    seen_message_ids: &[&str],
    seen_idempotency_keys: &[&str],
    authorization: Option<&HouseholdMeshPeerAuthorization>,
) -> HouseholdMeshImportDecision {
    household_mesh_event_bridge_validation::validate_incoming_lan_message(
        message,
        expected_family_id,
        expected_target_child_device_id,
        received_at_epoch_seconds,
        seen_message_ids,
        seen_idempotency_keys,
        authorization,
    )
}

impl HouseholdMeshBridgeRejection {
    pub fn as_str(self) -> &'static str {
        rejection_as_str(self)
    }
}
