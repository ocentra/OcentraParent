use ocentra_parent_agent_protocol::constants::household_mesh as mesh;
use ocentra_parent_agent_protocol::household_mesh as protocol;

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

pub type HouseholdMeshAuthenticationState = protocol::HouseholdMeshAuthenticationState;
pub type HouseholdMeshPolicyAuthority = protocol::HouseholdMeshPolicyAuthority;

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

pub type HouseholdMeshLanMessage = protocol::HouseholdMeshTransportEnvelope;
pub type HouseholdMeshLocalRepublish = protocol::HouseholdMeshLocalRepublish;

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
    sent_at_epoch_seconds: u64,
    stale_after_seconds: u64,
) -> HouseholdMeshExportDecision {
    let Some(lan_message_type) = lan_message_type_for(event_kind) else {
        return HouseholdMeshExportDecision::Reject(
            HouseholdMeshBridgeRejection::UnselectedLocalEvent,
        );
    };
    HouseholdMeshExportDecision::Export(HouseholdMeshLanMessage {
        schema_version: mesh::EVENT_SCHEMA_VERSION,
        message_id: message_id.to_string(),
        idempotency_key: idempotency_key.to_string(),
        family_id: family_id.to_string(),
        target_child_device_id: target_child_device_id.to_string(),
        source_peer_id: source_peer_id.to_string(),
        local_event_ref: local_event_ref(event_kind).to_string(),
        lan_message_type: lan_message_type.to_string(),
        bridge_state: protocol::HouseholdMeshBridgeState::ExportSelected,
        authentication_state: HouseholdMeshAuthenticationState::PairedTrustedDevice,
        policy_authority: HouseholdMeshPolicyAuthority::ChildAgentOnly,
        direct_remote_publish_requested: false,
        raw_payload_included: false,
        sent_at_epoch_seconds,
        stale_after_seconds,
    })
}

pub fn validate_incoming_lan_message(
    message: HouseholdMeshLanMessage,
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
        &message,
    ))
}

pub fn local_event_ref(event_kind: HouseholdMeshLocalEventKind) -> &'static str {
    match event_kind {
        HouseholdMeshLocalEventKind::DeviceDiscovery => mesh::LOCAL_EVENT_DEVICE_DISCOVERY,
        HouseholdMeshLocalEventKind::ProviderAdvertisement => {
            mesh::LOCAL_EVENT_PROVIDER_ADVERTISEMENT
        }
        HouseholdMeshLocalEventKind::ProviderHeartbeat => mesh::LOCAL_EVENT_PROVIDER_HEARTBEAT,
        HouseholdMeshLocalEventKind::ProviderCapability => mesh::LOCAL_EVENT_PROVIDER_CAPABILITY,
        HouseholdMeshLocalEventKind::AiWorkOffer => mesh::LOCAL_EVENT_AI_WORK_OFFER,
        HouseholdMeshLocalEventKind::AiWorkClaimRequest => mesh::LOCAL_EVENT_AI_WORK_CLAIM_REQUEST,
        HouseholdMeshLocalEventKind::AiWorkClaimDecision => {
            mesh::LOCAL_EVENT_AI_WORK_CLAIM_DECISION
        }
        HouseholdMeshLocalEventKind::AiWorkLeaseState => mesh::LOCAL_EVENT_AI_WORK_LEASE_STATE,
        HouseholdMeshLocalEventKind::AiJobPayloadTransfer => {
            mesh::LOCAL_EVENT_AI_JOB_PAYLOAD_TRANSFER
        }
        HouseholdMeshLocalEventKind::AiResultReturn => mesh::LOCAL_EVENT_AI_RESULT_RETURN,
        HouseholdMeshLocalEventKind::ConfigCommand => mesh::LOCAL_EVENT_CONFIG_COMMAND,
        HouseholdMeshLocalEventKind::ApprovalOverrideCommand => {
            mesh::LOCAL_EVENT_APPROVAL_OVERRIDE_COMMAND
        }
        HouseholdMeshLocalEventKind::ReadModelQueryRequest => {
            mesh::LOCAL_EVENT_READ_MODEL_QUERY_REQUEST
        }
        HouseholdMeshLocalEventKind::RawCaptureInternal => mesh::LOCAL_EVENT_RAW_CAPTURE_INTERNAL,
        HouseholdMeshLocalEventKind::AdapterInternal => mesh::LOCAL_EVENT_ADAPTER_INTERNAL,
        HouseholdMeshLocalEventKind::PrivateQueueMechanic => {
            mesh::LOCAL_EVENT_PRIVATE_QUEUE_MECHANIC
        }
        HouseholdMeshLocalEventKind::PolicyDecision => mesh::LOCAL_EVENT_POLICY_DECISION,
        HouseholdMeshLocalEventKind::EnforcementCommand => mesh::LOCAL_EVENT_ENFORCEMENT_COMMAND,
    }
}

fn lan_message_type_for(event_kind: HouseholdMeshLocalEventKind) -> Option<&'static str> {
    Some(match event_kind {
        HouseholdMeshLocalEventKind::DeviceDiscovery => mesh::LAN_MESSAGE_DEVICE_DISCOVERY,
        HouseholdMeshLocalEventKind::ProviderAdvertisement => {
            mesh::LAN_MESSAGE_PROVIDER_ADVERTISEMENT
        }
        HouseholdMeshLocalEventKind::ProviderHeartbeat => mesh::LAN_MESSAGE_PROVIDER_HEARTBEAT,
        HouseholdMeshLocalEventKind::ProviderCapability => mesh::LAN_MESSAGE_PROVIDER_CAPABILITY,
        HouseholdMeshLocalEventKind::AiWorkOffer => mesh::LAN_MESSAGE_AI_WORK_OFFER,
        HouseholdMeshLocalEventKind::AiWorkClaimRequest => mesh::LAN_MESSAGE_AI_WORK_CLAIM_REQUEST,
        HouseholdMeshLocalEventKind::AiWorkClaimDecision => {
            mesh::LAN_MESSAGE_AI_WORK_CLAIM_DECISION
        }
        HouseholdMeshLocalEventKind::AiWorkLeaseState => mesh::LAN_MESSAGE_AI_WORK_LEASE_STATE,
        HouseholdMeshLocalEventKind::AiJobPayloadTransfer => {
            mesh::LAN_MESSAGE_AI_JOB_PAYLOAD_TRANSFER
        }
        HouseholdMeshLocalEventKind::AiResultReturn => mesh::LAN_MESSAGE_AI_RESULT_RETURN,
        HouseholdMeshLocalEventKind::ConfigCommand => mesh::LAN_MESSAGE_CONFIG_COMMAND,
        HouseholdMeshLocalEventKind::ApprovalOverrideCommand => {
            mesh::LAN_MESSAGE_APPROVAL_OVERRIDE_COMMAND
        }
        HouseholdMeshLocalEventKind::ReadModelQueryRequest => {
            mesh::LAN_MESSAGE_READ_MODEL_QUERY_REQUEST
        }
        HouseholdMeshLocalEventKind::RawCaptureInternal
        | HouseholdMeshLocalEventKind::AdapterInternal
        | HouseholdMeshLocalEventKind::PrivateQueueMechanic
        | HouseholdMeshLocalEventKind::PolicyDecision
        | HouseholdMeshLocalEventKind::EnforcementCommand => return None,
    })
}

fn lan_message_type_for_ref(local_event_ref: &str) -> Option<&'static str> {
    match local_event_ref {
        mesh::LOCAL_EVENT_DEVICE_DISCOVERY => Some(mesh::LAN_MESSAGE_DEVICE_DISCOVERY),
        mesh::LOCAL_EVENT_PROVIDER_ADVERTISEMENT => Some(mesh::LAN_MESSAGE_PROVIDER_ADVERTISEMENT),
        mesh::LOCAL_EVENT_PROVIDER_HEARTBEAT => Some(mesh::LAN_MESSAGE_PROVIDER_HEARTBEAT),
        mesh::LOCAL_EVENT_PROVIDER_CAPABILITY => Some(mesh::LAN_MESSAGE_PROVIDER_CAPABILITY),
        mesh::LOCAL_EVENT_AI_WORK_OFFER => Some(mesh::LAN_MESSAGE_AI_WORK_OFFER),
        mesh::LOCAL_EVENT_AI_WORK_CLAIM_REQUEST => Some(mesh::LAN_MESSAGE_AI_WORK_CLAIM_REQUEST),
        mesh::LOCAL_EVENT_AI_WORK_CLAIM_DECISION => Some(mesh::LAN_MESSAGE_AI_WORK_CLAIM_DECISION),
        mesh::LOCAL_EVENT_AI_WORK_LEASE_STATE => Some(mesh::LAN_MESSAGE_AI_WORK_LEASE_STATE),
        mesh::LOCAL_EVENT_AI_JOB_PAYLOAD_TRANSFER => {
            Some(mesh::LAN_MESSAGE_AI_JOB_PAYLOAD_TRANSFER)
        }
        mesh::LOCAL_EVENT_AI_RESULT_RETURN => Some(mesh::LAN_MESSAGE_AI_RESULT_RETURN),
        mesh::LOCAL_EVENT_CONFIG_COMMAND => Some(mesh::LAN_MESSAGE_CONFIG_COMMAND),
        mesh::LOCAL_EVENT_APPROVAL_OVERRIDE_COMMAND => {
            Some(mesh::LAN_MESSAGE_APPROVAL_OVERRIDE_COMMAND)
        }
        mesh::LOCAL_EVENT_READ_MODEL_QUERY_REQUEST => {
            Some(mesh::LAN_MESSAGE_READ_MODEL_QUERY_REQUEST)
        }
        _ => None,
    }
}

fn is_selected_local_event_ref(local_event_ref: &str) -> bool {
    lan_message_type_for_ref(local_event_ref).is_some()
}

impl HouseholdMeshBridgeRejection {
    pub fn as_str(self) -> &'static str {
        match self {
            HouseholdMeshBridgeRejection::UnselectedLocalEvent => {
                mesh::REJECTION_UNSELECTED_LOCAL_EVENT
            }
            HouseholdMeshBridgeRejection::UnauthenticatedMessage => {
                mesh::REJECTION_UNAUTHENTICATED_MESSAGE
            }
            HouseholdMeshBridgeRejection::DirectRemotePublish => {
                mesh::REJECTION_DIRECT_REMOTE_PUBLISH
            }
            HouseholdMeshBridgeRejection::PolicyAuthorityEscalation => {
                mesh::REJECTION_POLICY_AUTHORITY_ESCALATION
            }
            HouseholdMeshBridgeRejection::RawPayload => mesh::REJECTION_RAW_PAYLOAD,
            HouseholdMeshBridgeRejection::MismatchedMessageRef => {
                mesh::REJECTION_MISMATCHED_MESSAGE_REF
            }
            HouseholdMeshBridgeRejection::ReplayedMessage => mesh::REJECTION_REPLAYED_MESSAGE,
            HouseholdMeshBridgeRejection::StaleMessage => mesh::REJECTION_STALE_MESSAGE,
            HouseholdMeshBridgeRejection::FamilyMismatch => mesh::REJECTION_FAMILY_MISMATCH,
            HouseholdMeshBridgeRejection::WrongTargetDevice => mesh::REJECTION_WRONG_TARGET_DEVICE,
        }
    }
}
