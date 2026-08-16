use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::household_mesh::{
    HouseholdMeshBridgeState, HouseholdMeshStructurallyValidatedTransportEnvelope,
    HouseholdMeshTransportEnvelope,
};

use crate::{
    household_mesh_event_bridge_lookup::{lan_message_type_for_ref, local_event_ref},
    household_mesh_event_bridge_rejection::rejection_as_str,
};

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
    let inbound = ocentra_parent_agent_protocol::household_mesh::household_mesh_bridge_input::
        HouseholdMeshBridgeInboundEnvelope::for_structural_validation(
            message.clone(),
            expected_family_id.to_string(),
            expected_target_child_device_id.to_string(),
            received_at_epoch_seconds,
            seen_message_ids
                .iter()
                .map(|value| (*value).to_string())
                .collect(),
            seen_idempotency_keys
                .iter()
                .map(|value| (*value).to_string())
                .collect(),
        );

    match inbound.validate_structure() {
        Ok(structural_message) => {
            let Some(authorization) = authorization else {
                return HouseholdMeshImportDecision::Reject(
                    HouseholdMeshBridgeRejection::UnauthenticatedMessage,
                );
            };
            if !authorization.matches_source(&structural_message.message().source_peer_id) {
                return HouseholdMeshImportDecision::Reject(
                    HouseholdMeshBridgeRejection::UnauthenticatedMessage,
                );
            }
            HouseholdMeshImportDecision::Republish(local_republish_from_authorized_structure(
                structural_message,
                authorization,
            ))
        }
        Err(validation) => {
            HouseholdMeshImportDecision::Reject(rejection_from_protocol_validation(validation))
        }
    }
}

fn local_republish_from_authorized_structure(
    structural_message: HouseholdMeshStructurallyValidatedTransportEnvelope,
    _authorization: &HouseholdMeshPeerAuthorization,
) -> HouseholdMeshLocalRepublish {
    let message = structural_message.message();
    HouseholdMeshLocalRepublish {
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

fn rejection_from_protocol_validation(
    validation: ocentra_parent_agent_protocol::household_mesh::HouseholdMeshBridgeValidation,
) -> HouseholdMeshBridgeRejection {
    use ocentra_parent_agent_protocol::household_mesh::HouseholdMeshBridgeRejectionReason as Reason;

    match validation.rejection_reason {
        Some(Reason::UnauthenticatedPeer | Reason::UnauthorizedPeer) => {
            HouseholdMeshBridgeRejection::UnauthenticatedMessage
        }
        Some(Reason::DirectRemotePublish) => HouseholdMeshBridgeRejection::DirectRemotePublish,
        Some(Reason::PolicyAuthorityEscalation) => {
            HouseholdMeshBridgeRejection::PolicyAuthorityEscalation
        }
        Some(Reason::RawScreenPayload) => HouseholdMeshBridgeRejection::RawPayload,
        Some(Reason::ReplayedMessage) => HouseholdMeshBridgeRejection::ReplayedMessage,
        Some(Reason::StaleMessage) => HouseholdMeshBridgeRejection::StaleMessage,
        Some(Reason::FamilyMismatch) => HouseholdMeshBridgeRejection::FamilyMismatch,
        Some(Reason::WrongTargetDevice) => HouseholdMeshBridgeRejection::WrongTargetDevice,
        Some(Reason::UnselectedEvent | Reason::UnsupportedLanMessage) => {
            HouseholdMeshBridgeRejection::UnselectedLocalEvent
        }
        Some(Reason::MismatchedMessageRef) | None => {
            HouseholdMeshBridgeRejection::MismatchedMessageRef
        }
    }
}

impl HouseholdMeshBridgeRejection {
    pub fn as_str(self) -> &'static str {
        rejection_as_str(self)
    }
}
