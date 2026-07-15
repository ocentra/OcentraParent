use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::household_mesh::{
    HouseholdMeshBridgeState, HouseholdMeshTransportEnvelope,
};

use crate::{
    household_mesh_event_bridge_lookup::{
        is_selected_local_event_ref, lan_message_type_for_ref, local_event_ref,
    },
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

pub fn validate_incoming_lan_message(
    message: &HouseholdMeshLanMessage,
    expected_family_id: &str,
    expected_target_child_device_id: &str,
    received_at_epoch_seconds: u64,
    seen_message_ids: &[&str],
    seen_idempotency_keys: &[&str],
) -> HouseholdMeshImportDecision {
    if message.authentication_state != HouseholdMeshAuthenticationState::PairedTrustedDevice {
        return HouseholdMeshImportDecision::Reject(
            HouseholdMeshBridgeRejection::UnauthenticatedMessage,
        );
    }
    if message.direct_remote_publish_requested {
        return HouseholdMeshImportDecision::Reject(
            HouseholdMeshBridgeRejection::DirectRemotePublish,
        );
    }
    if message.policy_authority != HouseholdMeshPolicyAuthority::ChildAgentOnly {
        return HouseholdMeshImportDecision::Reject(
            HouseholdMeshBridgeRejection::PolicyAuthorityEscalation,
        );
    }
    if message.raw_payload_included {
        return HouseholdMeshImportDecision::Reject(HouseholdMeshBridgeRejection::RawPayload);
    }
    if seen_message_ids.contains(&message.message_id.as_str())
        || seen_idempotency_keys.contains(&message.idempotency_key.as_str())
    {
        return HouseholdMeshImportDecision::Reject(HouseholdMeshBridgeRejection::ReplayedMessage);
    }
    if message.is_stale_at(received_at_epoch_seconds) {
        return HouseholdMeshImportDecision::Reject(HouseholdMeshBridgeRejection::StaleMessage);
    }
    if message.family_id != expected_family_id {
        return HouseholdMeshImportDecision::Reject(HouseholdMeshBridgeRejection::FamilyMismatch);
    }
    if message.target_child_device_id != expected_target_child_device_id {
        return HouseholdMeshImportDecision::Reject(
            HouseholdMeshBridgeRejection::WrongTargetDevice,
        );
    }
    if lan_message_type_for_ref(&message.local_event_ref) != Some(message.lan_message_type.as_str())
    {
        return if is_selected_local_event_ref(&message.local_event_ref) {
            HouseholdMeshImportDecision::Reject(HouseholdMeshBridgeRejection::MismatchedMessageRef)
        } else {
            HouseholdMeshImportDecision::Reject(HouseholdMeshBridgeRejection::UnselectedLocalEvent)
        };
    }
    HouseholdMeshImportDecision::Republish(HouseholdMeshLocalRepublish::from_validated_message(
        message,
    ))
}

impl HouseholdMeshBridgeRejection {
    pub fn as_str(self) -> &'static str {
        rejection_as_str(self)
    }
}
