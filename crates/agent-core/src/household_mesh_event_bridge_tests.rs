use std::panic::panic_any;

use ocentra_parent_agent_protocol::constants::household_mesh as mesh;

use crate::{
    export_selected_local_event, validate_incoming_lan_message, HouseholdMeshAuthenticationState,
    HouseholdMeshBridgeRejection, HouseholdMeshExportDecision, HouseholdMeshImportDecision,
    HouseholdMeshLanMessage, HouseholdMeshLocalEventKind, HouseholdMeshPolicyAuthority,
};

type SelectedEventCase = (HouseholdMeshLocalEventKind, &'static str, &'static str);

#[test]
fn household_mesh_exports_all_selected_local_events() {
    for (event_kind, expected_local_ref, expected_lan_message) in selected_event_cases() {
        assert_selected_export(event_kind, expected_local_ref, expected_lan_message);
    }
}

#[test]
fn household_mesh_rejects_unselected_local_events() {
    let unselected_events = [
        HouseholdMeshLocalEventKind::RawCaptureInternal,
        HouseholdMeshLocalEventKind::AdapterInternal,
        HouseholdMeshLocalEventKind::PrivateQueueMechanic,
        HouseholdMeshLocalEventKind::PolicyDecision,
        HouseholdMeshLocalEventKind::EnforcementCommand,
    ];

    for event_kind in unselected_events {
        assert_eq!(
            export_selected_local_event(event_kind),
            HouseholdMeshExportDecision::Reject(HouseholdMeshBridgeRejection::UnselectedLocalEvent)
        );
    }
}

#[test]
fn household_mesh_validates_incoming_before_local_republish() {
    let decision = validate_incoming_lan_message(provider_result_message());
    let HouseholdMeshImportDecision::Republish(republish) = decision else {
        panic_any(mesh::TEST_INCOMING_VALIDATES_EXPECT);
    };
    assert_eq!(
        republish.local_event_ref,
        mesh::LOCAL_EVENT_AI_RESULT_RETURN
    );
    assert_eq!(
        republish.lan_message_type,
        mesh::LAN_MESSAGE_AI_RESULT_RETURN
    );
    assert_eq!(
        republish.bridge_state,
        mesh::BRIDGE_STATE_LOCAL_REPUBLISH_REQUIRED
    );
    assert_eq!(
        republish.policy_authority,
        HouseholdMeshPolicyAuthority::ChildAgentOnly
    );
    assert!(republish.validated_before_republish);
    assert!(republish.child_agent_policy_authority_preserved);
}

#[test]
fn household_mesh_rejects_unauthenticated_incoming_messages() {
    assert_eq!(
        validate_incoming_lan_message(HouseholdMeshLanMessage {
            authentication_state: HouseholdMeshAuthenticationState::Anonymous,
            ..provider_result_message()
        }),
        HouseholdMeshImportDecision::Reject(HouseholdMeshBridgeRejection::UnauthenticatedMessage)
    );
    assert_eq!(
        validate_incoming_lan_message(HouseholdMeshLanMessage {
            authentication_state: HouseholdMeshAuthenticationState::StaleOrRevoked,
            ..provider_result_message()
        }),
        HouseholdMeshImportDecision::Reject(HouseholdMeshBridgeRejection::UnauthenticatedMessage)
    );
}

#[test]
fn household_mesh_rejects_direct_publish_and_policy_escalation() {
    assert_eq!(
        validate_incoming_lan_message(HouseholdMeshLanMessage {
            direct_remote_publish_requested: true,
            ..provider_result_message()
        }),
        HouseholdMeshImportDecision::Reject(HouseholdMeshBridgeRejection::DirectRemotePublish)
    );
    assert_eq!(
        HouseholdMeshBridgeRejection::DirectRemotePublish.as_str(),
        mesh::REJECTION_DIRECT_REMOTE_PUBLISH
    );

    assert_eq!(
        validate_incoming_lan_message(HouseholdMeshLanMessage {
            policy_authority: HouseholdMeshPolicyAuthority::ProviderClaimed,
            ..provider_result_message()
        }),
        HouseholdMeshImportDecision::Reject(
            HouseholdMeshBridgeRejection::PolicyAuthorityEscalation
        )
    );
    assert_eq!(
        validate_incoming_lan_message(HouseholdMeshLanMessage {
            policy_authority: HouseholdMeshPolicyAuthority::ParentUiClaimed,
            ..provider_result_message()
        }),
        HouseholdMeshImportDecision::Reject(
            HouseholdMeshBridgeRejection::PolicyAuthorityEscalation
        )
    );
    assert_eq!(
        HouseholdMeshBridgeRejection::PolicyAuthorityEscalation.as_str(),
        mesh::REJECTION_POLICY_AUTHORITY_ESCALATION
    );
}

#[test]
fn household_mesh_rejects_raw_payload_and_invalid_refs() {
    assert_eq!(
        validate_incoming_lan_message(HouseholdMeshLanMessage {
            raw_payload_included: true,
            ..provider_result_message()
        }),
        HouseholdMeshImportDecision::Reject(HouseholdMeshBridgeRejection::RawPayload)
    );

    assert_eq!(
        validate_incoming_lan_message(HouseholdMeshLanMessage {
            local_event_ref: mesh::LOCAL_EVENT_RAW_CAPTURE_INTERNAL,
            ..provider_result_message()
        }),
        HouseholdMeshImportDecision::Reject(HouseholdMeshBridgeRejection::UnselectedLocalEvent)
    );

    assert_eq!(
        validate_incoming_lan_message(HouseholdMeshLanMessage {
            lan_message_type: mesh::LAN_MESSAGE_PROVIDER_ADVERTISEMENT,
            ..provider_result_message()
        }),
        HouseholdMeshImportDecision::Reject(HouseholdMeshBridgeRejection::MismatchedMessageRef)
    );
    assert_eq!(
        HouseholdMeshBridgeRejection::MismatchedMessageRef.as_str(),
        mesh::REJECTION_MISMATCHED_MESSAGE_REF
    );
}

fn assert_selected_export(
    event_kind: HouseholdMeshLocalEventKind,
    expected_local_ref: &'static str,
    expected_lan_message: &'static str,
) {
    let decision = export_selected_local_event(event_kind);
    let HouseholdMeshExportDecision::Export(message) = decision else {
        panic_any(mesh::TEST_SELECTED_EXPORTS_EXPECT);
    };
    assert_eq!(message.local_event_ref, expected_local_ref);
    assert_eq!(message.lan_message_type, expected_lan_message);
    assert_eq!(message.bridge_state, mesh::BRIDGE_STATE_EXPORT_SELECTED);
    assert_eq!(
        message.authentication_state,
        HouseholdMeshAuthenticationState::PairedTrustedDevice
    );
    assert_eq!(
        message.authentication_state.as_str(),
        mesh::AUTHENTICATION_PAIRED_TRUSTED_DEVICE
    );
    assert_eq!(
        message.policy_authority,
        HouseholdMeshPolicyAuthority::ChildAgentOnly
    );
    assert_eq!(
        message.policy_authority.as_str(),
        mesh::POLICY_AUTHORITY_CHILD_AGENT_ONLY
    );
    assert!(!message.direct_remote_publish_requested);
    assert!(!message.raw_payload_included);
}

fn selected_event_cases() -> [SelectedEventCase; 13] {
    [
        (
            HouseholdMeshLocalEventKind::DeviceDiscovery,
            mesh::LOCAL_EVENT_DEVICE_DISCOVERY,
            mesh::LAN_MESSAGE_DEVICE_DISCOVERY,
        ),
        (
            HouseholdMeshLocalEventKind::ProviderAdvertisement,
            mesh::LOCAL_EVENT_PROVIDER_ADVERTISEMENT,
            mesh::LAN_MESSAGE_PROVIDER_ADVERTISEMENT,
        ),
        (
            HouseholdMeshLocalEventKind::ProviderHeartbeat,
            mesh::LOCAL_EVENT_PROVIDER_HEARTBEAT,
            mesh::LAN_MESSAGE_PROVIDER_HEARTBEAT,
        ),
        (
            HouseholdMeshLocalEventKind::ProviderCapability,
            mesh::LOCAL_EVENT_PROVIDER_CAPABILITY,
            mesh::LAN_MESSAGE_PROVIDER_CAPABILITY,
        ),
        (
            HouseholdMeshLocalEventKind::AiWorkOffer,
            mesh::LOCAL_EVENT_AI_WORK_OFFER,
            mesh::LAN_MESSAGE_AI_WORK_OFFER,
        ),
        (
            HouseholdMeshLocalEventKind::AiWorkClaimRequest,
            mesh::LOCAL_EVENT_AI_WORK_CLAIM_REQUEST,
            mesh::LAN_MESSAGE_AI_WORK_CLAIM_REQUEST,
        ),
        (
            HouseholdMeshLocalEventKind::AiWorkClaimDecision,
            mesh::LOCAL_EVENT_AI_WORK_CLAIM_DECISION,
            mesh::LAN_MESSAGE_AI_WORK_CLAIM_DECISION,
        ),
        (
            HouseholdMeshLocalEventKind::AiWorkLeaseState,
            mesh::LOCAL_EVENT_AI_WORK_LEASE_STATE,
            mesh::LAN_MESSAGE_AI_WORK_LEASE_STATE,
        ),
        (
            HouseholdMeshLocalEventKind::AiJobPayloadTransfer,
            mesh::LOCAL_EVENT_AI_JOB_PAYLOAD_TRANSFER,
            mesh::LAN_MESSAGE_AI_JOB_PAYLOAD_TRANSFER,
        ),
        (
            HouseholdMeshLocalEventKind::AiResultReturn,
            mesh::LOCAL_EVENT_AI_RESULT_RETURN,
            mesh::LAN_MESSAGE_AI_RESULT_RETURN,
        ),
        (
            HouseholdMeshLocalEventKind::ConfigCommand,
            mesh::LOCAL_EVENT_CONFIG_COMMAND,
            mesh::LAN_MESSAGE_CONFIG_COMMAND,
        ),
        (
            HouseholdMeshLocalEventKind::ApprovalOverrideCommand,
            mesh::LOCAL_EVENT_APPROVAL_OVERRIDE_COMMAND,
            mesh::LAN_MESSAGE_APPROVAL_OVERRIDE_COMMAND,
        ),
        (
            HouseholdMeshLocalEventKind::ReadModelQueryRequest,
            mesh::LOCAL_EVENT_READ_MODEL_QUERY_REQUEST,
            mesh::LAN_MESSAGE_READ_MODEL_QUERY_REQUEST,
        ),
    ]
}

fn provider_result_message() -> HouseholdMeshLanMessage {
    HouseholdMeshLanMessage {
        local_event_ref: mesh::LOCAL_EVENT_AI_RESULT_RETURN,
        lan_message_type: mesh::LAN_MESSAGE_AI_RESULT_RETURN,
        bridge_state: mesh::BRIDGE_STATE_EXPORT_SELECTED,
        authentication_state: HouseholdMeshAuthenticationState::PairedTrustedDevice,
        policy_authority: HouseholdMeshPolicyAuthority::ChildAgentOnly,
        direct_remote_publish_requested: false,
        raw_payload_included: false,
    }
}
