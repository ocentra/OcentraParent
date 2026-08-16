use std::fmt::Display;

use ocentra_parent_agent_protocol::constants::household_mesh as mesh;
use ocentra_parent_agent_protocol::household_mesh::HouseholdMeshBridgeState;

use crate::test_text::{TestResult, TestText};
use crate::{
    export_selected_local_event, HouseholdMeshAuthenticationState, HouseholdMeshBridgeRejection,
    HouseholdMeshExportDecision, HouseholdMeshLocalEventKind, HouseholdMeshPolicyAuthority,
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
fn household_mesh_rejection_strings_match_the_protocol_contract() {
    let cases = [
        (
            HouseholdMeshBridgeRejection::UnselectedLocalEvent,
            mesh::REJECTION_UNSELECTED_LOCAL_EVENT,
        ),
        (
            HouseholdMeshBridgeRejection::UnauthenticatedMessage,
            mesh::REJECTION_UNAUTHENTICATED_MESSAGE,
        ),
        (
            HouseholdMeshBridgeRejection::DirectRemotePublish,
            mesh::REJECTION_DIRECT_REMOTE_PUBLISH,
        ),
        (
            HouseholdMeshBridgeRejection::PolicyAuthorityEscalation,
            mesh::REJECTION_POLICY_AUTHORITY_ESCALATION,
        ),
        (
            HouseholdMeshBridgeRejection::RawPayload,
            mesh::REJECTION_RAW_PAYLOAD,
        ),
        (
            HouseholdMeshBridgeRejection::MismatchedMessageRef,
            mesh::REJECTION_MISMATCHED_MESSAGE_REF,
        ),
        (
            HouseholdMeshBridgeRejection::ReplayedMessage,
            mesh::REJECTION_REPLAYED_MESSAGE,
        ),
        (
            HouseholdMeshBridgeRejection::StaleMessage,
            mesh::REJECTION_STALE_MESSAGE,
        ),
        (
            HouseholdMeshBridgeRejection::FamilyMismatch,
            mesh::REJECTION_FAMILY_MISMATCH,
        ),
        (
            HouseholdMeshBridgeRejection::WrongTargetDevice,
            mesh::REJECTION_WRONG_TARGET_DEVICE,
        ),
    ];
    for (rejection, expected) in cases {
        assert_eq!(rejection.as_str(), expected);
    }
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
