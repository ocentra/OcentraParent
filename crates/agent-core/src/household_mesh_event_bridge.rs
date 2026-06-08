use ocentra_parent_agent_protocol::constants::household_mesh as mesh;

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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HouseholdMeshAuthenticationState {
    PairedTrustedDevice,
    Anonymous,
    StaleOrRevoked,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HouseholdMeshPolicyAuthority {
    ChildAgentOnly,
    ProviderClaimed,
    ParentUiClaimed,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HouseholdMeshBridgeRejection {
    UnselectedLocalEvent,
    UnauthenticatedMessage,
    DirectRemotePublish,
    PolicyAuthorityEscalation,
    RawPayload,
    MismatchedMessageRef,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HouseholdMeshLanMessage {
    pub local_event_ref: &'static str,
    pub lan_message_type: &'static str,
    pub bridge_state: &'static str,
    pub authentication_state: HouseholdMeshAuthenticationState,
    pub policy_authority: HouseholdMeshPolicyAuthority,
    pub direct_remote_publish_requested: bool,
    pub raw_payload_included: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HouseholdMeshLocalRepublish {
    pub local_event_ref: &'static str,
    pub lan_message_type: &'static str,
    pub bridge_state: &'static str,
    pub policy_authority: HouseholdMeshPolicyAuthority,
    pub validated_before_republish: bool,
    pub child_agent_policy_authority_preserved: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HouseholdMeshExportDecision {
    Export(HouseholdMeshLanMessage),
    Reject(HouseholdMeshBridgeRejection),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HouseholdMeshImportDecision {
    Republish(HouseholdMeshLocalRepublish),
    Reject(HouseholdMeshBridgeRejection),
}

pub fn export_selected_local_event(
    event_kind: HouseholdMeshLocalEventKind,
) -> HouseholdMeshExportDecision {
    let Some(lan_message_type) = lan_message_type_for(event_kind) else {
        return HouseholdMeshExportDecision::Reject(
            HouseholdMeshBridgeRejection::UnselectedLocalEvent,
        );
    };
    HouseholdMeshExportDecision::Export(HouseholdMeshLanMessage {
        local_event_ref: local_event_ref(event_kind),
        lan_message_type,
        bridge_state: mesh::BRIDGE_STATE_EXPORT_SELECTED,
        authentication_state: HouseholdMeshAuthenticationState::PairedTrustedDevice,
        policy_authority: HouseholdMeshPolicyAuthority::ChildAgentOnly,
        direct_remote_publish_requested: false,
        raw_payload_included: false,
    })
}

pub fn validate_incoming_lan_message(
    message: HouseholdMeshLanMessage,
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
    if lan_message_type_for_ref(message.local_event_ref) != Some(message.lan_message_type) {
        return if is_selected_local_event_ref(message.local_event_ref) {
            HouseholdMeshImportDecision::Reject(HouseholdMeshBridgeRejection::MismatchedMessageRef)
        } else {
            HouseholdMeshImportDecision::Reject(HouseholdMeshBridgeRejection::UnselectedLocalEvent)
        };
    }
    HouseholdMeshImportDecision::Republish(HouseholdMeshLocalRepublish {
        local_event_ref: message.local_event_ref,
        lan_message_type: message.lan_message_type,
        bridge_state: mesh::BRIDGE_STATE_LOCAL_REPUBLISH_REQUIRED,
        policy_authority: HouseholdMeshPolicyAuthority::ChildAgentOnly,
        validated_before_republish: true,
        child_agent_policy_authority_preserved: true,
    })
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

fn lan_message_type_for_ref(local_event_ref: &'static str) -> Option<&'static str> {
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

fn is_selected_local_event_ref(local_event_ref: &'static str) -> bool {
    lan_message_type_for_ref(local_event_ref).is_some()
}

impl HouseholdMeshAuthenticationState {
    pub fn as_str(self) -> &'static str {
        match self {
            HouseholdMeshAuthenticationState::PairedTrustedDevice => {
                mesh::AUTHENTICATION_PAIRED_TRUSTED_DEVICE
            }
            HouseholdMeshAuthenticationState::Anonymous => mesh::AUTHENTICATION_ANONYMOUS,
            HouseholdMeshAuthenticationState::StaleOrRevoked => {
                mesh::AUTHENTICATION_STALE_OR_REVOKED
            }
        }
    }
}

impl HouseholdMeshPolicyAuthority {
    pub fn as_str(self) -> &'static str {
        match self {
            HouseholdMeshPolicyAuthority::ChildAgentOnly => mesh::POLICY_AUTHORITY_CHILD_AGENT_ONLY,
            HouseholdMeshPolicyAuthority::ProviderClaimed => {
                mesh::POLICY_AUTHORITY_PROVIDER_CLAIMED
            }
            HouseholdMeshPolicyAuthority::ParentUiClaimed => {
                mesh::POLICY_AUTHORITY_PARENT_UI_CLAIMED
            }
        }
    }
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
        }
    }
}
