use std::fmt::Display;

use ocentra_parent_agent_protocol::constants::household_mesh as mesh;
use ocentra_parent_agent_protocol::household_mesh::HouseholdMeshBridgeState;

use crate::test_text::{TestResult, TestText};
use crate::{
    export_selected_local_event, validate_incoming_lan_message, HouseholdMeshAuthenticationState,
    HouseholdMeshBridgeRejection, HouseholdMeshExportDecision, HouseholdMeshImportDecision,
    HouseholdMeshLanMessage, HouseholdMeshLocalEventKind, HouseholdMeshPolicyAuthority,
};

type SelectedEventCase = (HouseholdMeshLocalEventKind, TestText, TestText);

#[test]
fn household_mesh_exports_all_selected_local_events() -> TestResult {
    for (event_kind, expected_local_ref, expected_lan_message) in selected_event_cases() {
        assert_selected_export(event_kind, expected_local_ref, expected_lan_message)?;
    }

    Ok(())
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
            export_selected_local_event(
                event_kind,
                mesh::TEST_BRIDGE_FAMILY_ID,
                mesh::TEST_BRIDGE_TARGET_CHILD_DEVICE_ID,
                mesh::TEST_BRIDGE_CHILD_AGENT_PEER_ID,
                mesh::TEST_BRIDGE_OUTBOUND_MESSAGE_ID,
                mesh::TEST_BRIDGE_IDEMPOTENCY_KEY,
                (
                    mesh::TEST_BRIDGE_SENT_AT_EPOCH_SECONDS,
                    mesh::TEST_BRIDGE_STALE_AFTER_SECONDS,
                ),
            ),
            HouseholdMeshExportDecision::Reject(HouseholdMeshBridgeRejection::UnselectedLocalEvent)
        );
    }
}

#[test]
fn household_mesh_validates_incoming_before_local_republish() -> TestResult {
    let decision = validate_incoming_lan_message(
        &provider_result_message(),
        mesh::TEST_BRIDGE_FAMILY_ID,
        mesh::TEST_BRIDGE_TARGET_CHILD_DEVICE_ID,
        mesh::TEST_BRIDGE_RECEIVED_AT_EPOCH_SECONDS,
        &[],
        &[],
    );
    let HouseholdMeshImportDecision::Republish(republish) = decision else {
        return Err(TestText::from_display(mesh::TEST_INCOMING_VALIDATES_EXPECT));
    };
    assert_eq!(republish.family_id, mesh::TEST_BRIDGE_FAMILY_ID);
    assert_eq!(
        republish.target_child_device_id,
        mesh::TEST_BRIDGE_TARGET_CHILD_DEVICE_ID
    );
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
        HouseholdMeshBridgeState::LocalRepublishRequired
    );
    assert_eq!(
        republish.policy_authority,
        HouseholdMeshPolicyAuthority::ChildAgentOnly
    );
    assert!(republish.validated_before_republish);
    assert!(republish.child_agent_policy_authority_preserved);

    Ok(())
}

#[test]
fn household_mesh_rejects_unauthenticated_incoming_messages() {
    assert_eq!(
        validate_incoming_lan_message(
            &HouseholdMeshLanMessage {
                authentication_state: HouseholdMeshAuthenticationState::Anonymous,
                ..provider_result_message()
            },
            mesh::TEST_BRIDGE_FAMILY_ID,
            mesh::TEST_BRIDGE_TARGET_CHILD_DEVICE_ID,
            mesh::TEST_BRIDGE_RECEIVED_AT_EPOCH_SECONDS,
            &[],
            &[],
        ),
        HouseholdMeshImportDecision::Reject(HouseholdMeshBridgeRejection::UnauthenticatedMessage)
    );
    assert_eq!(
        validate_incoming_lan_message(
            &HouseholdMeshLanMessage {
                authentication_state: HouseholdMeshAuthenticationState::StaleOrRevoked,
                ..provider_result_message()
            },
            mesh::TEST_BRIDGE_FAMILY_ID,
            mesh::TEST_BRIDGE_TARGET_CHILD_DEVICE_ID,
            mesh::TEST_BRIDGE_RECEIVED_AT_EPOCH_SECONDS,
            &[],
            &[],
        ),
        HouseholdMeshImportDecision::Reject(HouseholdMeshBridgeRejection::UnauthenticatedMessage)
    );
}

#[test]
fn household_mesh_rejects_direct_publish_and_policy_escalation() {
    assert_eq!(
        validate_incoming_lan_message(
            &HouseholdMeshLanMessage {
                direct_remote_publish_requested: true,
                ..provider_result_message()
            },
            mesh::TEST_BRIDGE_FAMILY_ID,
            mesh::TEST_BRIDGE_TARGET_CHILD_DEVICE_ID,
            mesh::TEST_BRIDGE_RECEIVED_AT_EPOCH_SECONDS,
            &[],
            &[],
        ),
        HouseholdMeshImportDecision::Reject(HouseholdMeshBridgeRejection::DirectRemotePublish)
    );
    assert_eq!(
        HouseholdMeshBridgeRejection::DirectRemotePublish.as_str(),
        mesh::REJECTION_DIRECT_REMOTE_PUBLISH
    );

    assert_eq!(
        validate_incoming_lan_message(
            &HouseholdMeshLanMessage {
                policy_authority: HouseholdMeshPolicyAuthority::ProviderClaimed,
                ..provider_result_message()
            },
            mesh::TEST_BRIDGE_FAMILY_ID,
            mesh::TEST_BRIDGE_TARGET_CHILD_DEVICE_ID,
            mesh::TEST_BRIDGE_RECEIVED_AT_EPOCH_SECONDS,
            &[],
            &[],
        ),
        HouseholdMeshImportDecision::Reject(
            HouseholdMeshBridgeRejection::PolicyAuthorityEscalation
        )
    );
    assert_eq!(
        validate_incoming_lan_message(
            &HouseholdMeshLanMessage {
                policy_authority: HouseholdMeshPolicyAuthority::ParentUiClaimed,
                ..provider_result_message()
            },
            mesh::TEST_BRIDGE_FAMILY_ID,
            mesh::TEST_BRIDGE_TARGET_CHILD_DEVICE_ID,
            mesh::TEST_BRIDGE_RECEIVED_AT_EPOCH_SECONDS,
            &[],
            &[],
        ),
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
fn household_mesh_rejects_raw_payload_invalid_refs_and_route_mismatches() {
    assert_eq!(
        validate_incoming_lan_message(
            &HouseholdMeshLanMessage {
                raw_payload_included: true,
                ..provider_result_message()
            },
            mesh::TEST_BRIDGE_FAMILY_ID,
            mesh::TEST_BRIDGE_TARGET_CHILD_DEVICE_ID,
            mesh::TEST_BRIDGE_RECEIVED_AT_EPOCH_SECONDS,
            &[],
            &[],
        ),
        HouseholdMeshImportDecision::Reject(HouseholdMeshBridgeRejection::RawPayload)
    );

    assert_eq!(
        validate_incoming_lan_message(
            &HouseholdMeshLanMessage {
                local_event_ref: mesh::LOCAL_EVENT_RAW_CAPTURE_INTERNAL.to_string(),
                ..provider_result_message()
            },
            mesh::TEST_BRIDGE_FAMILY_ID,
            mesh::TEST_BRIDGE_TARGET_CHILD_DEVICE_ID,
            mesh::TEST_BRIDGE_RECEIVED_AT_EPOCH_SECONDS,
            &[],
            &[],
        ),
        HouseholdMeshImportDecision::Reject(HouseholdMeshBridgeRejection::UnselectedLocalEvent)
    );

    assert_eq!(
        validate_incoming_lan_message(
            &HouseholdMeshLanMessage {
                lan_message_type: mesh::LAN_MESSAGE_PROVIDER_ADVERTISEMENT.to_string(),
                ..provider_result_message()
            },
            mesh::TEST_BRIDGE_FAMILY_ID,
            mesh::TEST_BRIDGE_TARGET_CHILD_DEVICE_ID,
            mesh::TEST_BRIDGE_RECEIVED_AT_EPOCH_SECONDS,
            &[],
            &[],
        ),
        HouseholdMeshImportDecision::Reject(HouseholdMeshBridgeRejection::MismatchedMessageRef)
    );
    assert_eq!(
        HouseholdMeshBridgeRejection::MismatchedMessageRef.as_str(),
        mesh::REJECTION_MISMATCHED_MESSAGE_REF
    );
}

#[test]
fn household_mesh_rejects_replay_stale_family_and_device_mismatches() {
    assert_eq!(
        validate_incoming_lan_message(
            &provider_result_message(),
            mesh::TEST_BRIDGE_FAMILY_ID,
            mesh::TEST_BRIDGE_TARGET_CHILD_DEVICE_ID,
            mesh::TEST_BRIDGE_RECEIVED_AT_EPOCH_SECONDS,
            &[mesh::TEST_BRIDGE_INBOUND_MESSAGE_ID],
            &[],
        ),
        HouseholdMeshImportDecision::Reject(HouseholdMeshBridgeRejection::ReplayedMessage)
    );
    assert_eq!(
        validate_incoming_lan_message(
            &provider_result_message(),
            mesh::TEST_BRIDGE_FAMILY_ID,
            mesh::TEST_BRIDGE_TARGET_CHILD_DEVICE_ID,
            mesh::TEST_BRIDGE_RECEIVED_AT_EPOCH_SECONDS,
            &[],
            &[mesh::TEST_BRIDGE_IDEMPOTENCY_KEY],
        ),
        HouseholdMeshImportDecision::Reject(HouseholdMeshBridgeRejection::ReplayedMessage)
    );
    assert_eq!(
        validate_incoming_lan_message(
            &provider_result_message(),
            mesh::TEST_BRIDGE_FAMILY_ID,
            mesh::TEST_BRIDGE_TARGET_CHILD_DEVICE_ID,
            mesh::TEST_BRIDGE_SENT_AT_EPOCH_SECONDS + mesh::TEST_BRIDGE_STALE_AFTER_SECONDS + 1,
            &[],
            &[],
        ),
        HouseholdMeshImportDecision::Reject(HouseholdMeshBridgeRejection::StaleMessage)
    );
    assert_eq!(
        validate_incoming_lan_message(
            &provider_result_message(),
            mesh::TEST_BRIDGE_OTHER_FAMILY_ID,
            mesh::TEST_BRIDGE_TARGET_CHILD_DEVICE_ID,
            mesh::TEST_BRIDGE_RECEIVED_AT_EPOCH_SECONDS,
            &[],
            &[],
        ),
        HouseholdMeshImportDecision::Reject(HouseholdMeshBridgeRejection::FamilyMismatch)
    );
    assert_eq!(
        validate_incoming_lan_message(
            &provider_result_message(),
            mesh::TEST_BRIDGE_FAMILY_ID,
            mesh::TEST_BRIDGE_OTHER_CHILD_DEVICE_ID,
            mesh::TEST_BRIDGE_RECEIVED_AT_EPOCH_SECONDS,
            &[],
            &[],
        ),
        HouseholdMeshImportDecision::Reject(HouseholdMeshBridgeRejection::WrongTargetDevice)
    );
}

fn assert_selected_export(
    event_kind: HouseholdMeshLocalEventKind,
    expected_local_ref: impl Display,
    expected_lan_message: impl Display,
) -> TestResult {
    let decision = export_selected_local_event(
        event_kind,
        mesh::TEST_BRIDGE_FAMILY_ID,
        mesh::TEST_BRIDGE_TARGET_CHILD_DEVICE_ID,
        mesh::TEST_BRIDGE_CHILD_AGENT_PEER_ID,
        mesh::TEST_BRIDGE_OUTBOUND_MESSAGE_ID,
        mesh::TEST_BRIDGE_IDEMPOTENCY_KEY,
        (
            mesh::TEST_BRIDGE_SENT_AT_EPOCH_SECONDS,
            mesh::TEST_BRIDGE_STALE_AFTER_SECONDS,
        ),
    );
    let HouseholdMeshExportDecision::Export(message) = decision else {
        return Err(TestText::from_display(mesh::TEST_SELECTED_EXPORTS_EXPECT));
    };
    assert_eq!(message.message_id, mesh::TEST_BRIDGE_OUTBOUND_MESSAGE_ID);
    assert_eq!(message.idempotency_key, mesh::TEST_BRIDGE_IDEMPOTENCY_KEY);
    assert_eq!(message.family_id, mesh::TEST_BRIDGE_FAMILY_ID);
    assert_eq!(
        message.target_child_device_id,
        mesh::TEST_BRIDGE_TARGET_CHILD_DEVICE_ID
    );
    assert_eq!(
        message.source_peer_id,
        mesh::TEST_BRIDGE_CHILD_AGENT_PEER_ID
    );
    assert_eq!(message.local_event_ref, expected_local_ref.to_string());
    assert_eq!(message.lan_message_type, expected_lan_message.to_string());
    assert_eq!(
        message.bridge_state,
        HouseholdMeshBridgeState::ExportSelected
    );
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
    assert_eq!(
        message.sent_at_epoch_seconds,
        mesh::TEST_BRIDGE_SENT_AT_EPOCH_SECONDS
    );
    assert_eq!(
        message.stale_after_seconds,
        mesh::TEST_BRIDGE_STALE_AFTER_SECONDS
    );
    assert!(!message.direct_remote_publish_requested);
    assert!(!message.raw_payload_included);

    Ok(())
}

fn selected_event_cases() -> [SelectedEventCase; 13] {
    [
        (
            HouseholdMeshLocalEventKind::DeviceDiscovery,
            TestText::from_display(mesh::LOCAL_EVENT_DEVICE_DISCOVERY),
            TestText::from_display(mesh::LAN_MESSAGE_DEVICE_DISCOVERY),
        ),
        (
            HouseholdMeshLocalEventKind::ProviderAdvertisement,
            TestText::from_display(mesh::LOCAL_EVENT_PROVIDER_ADVERTISEMENT),
            TestText::from_display(mesh::LAN_MESSAGE_PROVIDER_ADVERTISEMENT),
        ),
        (
            HouseholdMeshLocalEventKind::ProviderHeartbeat,
            TestText::from_display(mesh::LOCAL_EVENT_PROVIDER_HEARTBEAT),
            TestText::from_display(mesh::LAN_MESSAGE_PROVIDER_HEARTBEAT),
        ),
        (
            HouseholdMeshLocalEventKind::ProviderCapability,
            TestText::from_display(mesh::LOCAL_EVENT_PROVIDER_CAPABILITY),
            TestText::from_display(mesh::LAN_MESSAGE_PROVIDER_CAPABILITY),
        ),
        (
            HouseholdMeshLocalEventKind::AiWorkOffer,
            TestText::from_display(mesh::LOCAL_EVENT_AI_WORK_OFFER),
            TestText::from_display(mesh::LAN_MESSAGE_AI_WORK_OFFER),
        ),
        (
            HouseholdMeshLocalEventKind::AiWorkClaimRequest,
            TestText::from_display(mesh::LOCAL_EVENT_AI_WORK_CLAIM_REQUEST),
            TestText::from_display(mesh::LAN_MESSAGE_AI_WORK_CLAIM_REQUEST),
        ),
        (
            HouseholdMeshLocalEventKind::AiWorkClaimDecision,
            TestText::from_display(mesh::LOCAL_EVENT_AI_WORK_CLAIM_DECISION),
            TestText::from_display(mesh::LAN_MESSAGE_AI_WORK_CLAIM_DECISION),
        ),
        (
            HouseholdMeshLocalEventKind::AiWorkLeaseState,
            TestText::from_display(mesh::LOCAL_EVENT_AI_WORK_LEASE_STATE),
            TestText::from_display(mesh::LAN_MESSAGE_AI_WORK_LEASE_STATE),
        ),
        (
            HouseholdMeshLocalEventKind::AiJobPayloadTransfer,
            TestText::from_display(mesh::LOCAL_EVENT_AI_JOB_PAYLOAD_TRANSFER),
            TestText::from_display(mesh::LAN_MESSAGE_AI_JOB_PAYLOAD_TRANSFER),
        ),
        (
            HouseholdMeshLocalEventKind::AiResultReturn,
            TestText::from_display(mesh::LOCAL_EVENT_AI_RESULT_RETURN),
            TestText::from_display(mesh::LAN_MESSAGE_AI_RESULT_RETURN),
        ),
        (
            HouseholdMeshLocalEventKind::ConfigCommand,
            TestText::from_display(mesh::LOCAL_EVENT_CONFIG_COMMAND),
            TestText::from_display(mesh::LAN_MESSAGE_CONFIG_COMMAND),
        ),
        (
            HouseholdMeshLocalEventKind::ApprovalOverrideCommand,
            TestText::from_display(mesh::LOCAL_EVENT_APPROVAL_OVERRIDE_COMMAND),
            TestText::from_display(mesh::LAN_MESSAGE_APPROVAL_OVERRIDE_COMMAND),
        ),
        (
            HouseholdMeshLocalEventKind::ReadModelQueryRequest,
            TestText::from_display(mesh::LOCAL_EVENT_READ_MODEL_QUERY_REQUEST),
            TestText::from_display(mesh::LAN_MESSAGE_READ_MODEL_QUERY_REQUEST),
        ),
    ]
}

fn provider_result_message() -> HouseholdMeshLanMessage {
    HouseholdMeshLanMessage::proof_fixture_for(
        mesh::LOCAL_EVENT_AI_RESULT_RETURN,
        mesh::LAN_MESSAGE_AI_RESULT_RETURN,
    )
}
