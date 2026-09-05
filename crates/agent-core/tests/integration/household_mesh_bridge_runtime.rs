use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::household_mesh::{
    HouseholdMeshAuthenticationState, HouseholdMeshBridgePhase, HouseholdMeshBridgeRejectionReason,
    HouseholdMeshBridgeState, HouseholdMeshBridgeValidationState, HouseholdMeshPolicyAuthority,
    HouseholdMeshTransportEnvelope,
};

use ocentra_parent_agent_core::household_mesh_bridge_runtime::{
    publish_household_mesh_bridge_chain_for_input, validate_household_mesh_bridge_export,
    validate_household_mesh_bridge_import, HouseholdMeshBridgeEventPayload,
    HouseholdMeshBridgeExportCandidate, HouseholdMeshBridgeInboundEnvelope,
    HouseholdMeshBridgeInput, HouseholdMeshBridgeReport,
};

use crate::test_text::{test_ok, test_some, TestText};

type TestResult = crate::test_text::TestResult;

#[tokio::test]
async fn household_mesh_bridge_exports_selected_events_and_rejects_unauthorized_imports(
) -> TestResult {
    let input = real_bridge_input();
    let report = test_ok(
        publish_household_mesh_bridge_chain_for_input(input.clone()).await,
        constants::household_mesh::ERROR_BRIDGE_CHAIN_PUBLISHES,
    )?;
    let payloads = decode_payloads(&report)?;

    let structurally_valid = test_ok(
        input.validate_inbound(),
        constants::household_mesh::ERROR_BRIDGE_PAYLOAD_DECODES,
    )?;
    assert_eq!(
        structurally_valid.message().message_id,
        constants::household_mesh::TEST_BRIDGE_INBOUND_MESSAGE_ID
    );

    assert_eq!(
        report.publish_reports.len(),
        HouseholdMeshBridgePhase::ordered_chain().len()
    );
    assert_eq!(
        report.stored_events.len(),
        HouseholdMeshBridgePhase::ordered_chain().len()
    );
    assert!(report.dead_letters.is_empty());
    assert!(!report.violates_bridge_custody());

    let exported = payload_for_phase(&payloads, HouseholdMeshBridgePhase::LanMessageExported)?;
    assert_eq!(
        exported.local_event_ref,
        constants::household_mesh::LOCAL_EVENT_AI_WORK_OFFER
    );
    assert_eq!(
        exported.lan_message_type,
        constants::household_mesh::LAN_MESSAGE_AI_WORK_OFFER
    );
    assert_eq!(
        exported.family_id,
        constants::household_mesh::TEST_BRIDGE_FAMILY_ID
    );
    assert_eq!(
        exported.target_child_device_id,
        constants::household_mesh::TEST_BRIDGE_TARGET_CHILD_DEVICE_ID
    );
    assert_eq!(
        exported.idempotency_key,
        constants::household_mesh::TEST_BRIDGE_IDEMPOTENCY_KEY
    );
    assert_eq!(
        exported.previous_phase_ref,
        Some(constants::household_mesh::TEST_BRIDGE_SELECTED_EVENT_REF.to_string())
    );
    assert_eq!(
        exported.validation_state,
        HouseholdMeshBridgeValidationState::Accepted
    );

    let received = payload_for_phase(&payloads, HouseholdMeshBridgePhase::LanMessageReceived)?;
    assert_eq!(
        received.local_event_ref,
        constants::household_mesh::LOCAL_EVENT_AI_RESULT_RETURN
    );
    assert_eq!(
        received.lan_message_type,
        constants::household_mesh::LAN_MESSAGE_AI_RESULT_RETURN
    );
    assert_eq!(
        received.source_peer_id,
        constants::household_mesh::TEST_BRIDGE_PROVIDER_PEER_ID
    );
    assert_eq!(
        received.validation_state,
        HouseholdMeshBridgeValidationState::Rejected
    );
    assert_eq!(
        received.rejection_reason,
        Some(HouseholdMeshBridgeRejectionReason::UnauthorizedPeer)
    );

    let republished =
        payload_for_phase(&payloads, HouseholdMeshBridgePhase::LocalEventRepublished)?;
    assert_eq!(
        republished.previous_phase_ref,
        Some(constants::household_mesh::TEST_BRIDGE_RECEIVED_MESSAGE_REF.to_string())
    );
    assert_eq!(
        republished.validation_state,
        HouseholdMeshBridgeValidationState::Rejected
    );
    assert_eq!(
        republished.rejection_reason,
        Some(HouseholdMeshBridgeRejectionReason::UnauthorizedPeer)
    );

    Ok(())
}

#[test]
fn household_mesh_bridge_rejects_export_of_unselected_private_or_raw_events() {
    assert_eq!(
        validate_household_mesh_bridge_export(&HouseholdMeshBridgeExportCandidate::selected_offer())
            .state,
        HouseholdMeshBridgeValidationState::Accepted
    );

    let mut unselected = HouseholdMeshBridgeExportCandidate::selected_offer();
    unselected.local_event_type = constants::screen_flow::EVENT_SCREEN_QUEUE_ENCRYPTED.to_string();
    assert_eq!(
        validate_household_mesh_bridge_export(&unselected).rejection_reason,
        Some(HouseholdMeshBridgeRejectionReason::UnselectedEvent)
    );

    let mut private = HouseholdMeshBridgeExportCandidate::selected_offer();
    private.private_local_event = true;
    assert_eq!(
        validate_household_mesh_bridge_export(&private).rejection_reason,
        Some(HouseholdMeshBridgeRejectionReason::PrivateLocalEvent)
    );

    let mut raw = HouseholdMeshBridgeExportCandidate::selected_offer();
    raw.contains_raw_screenshot = true;
    assert_eq!(
        validate_household_mesh_bridge_export(&raw).rejection_reason,
        Some(HouseholdMeshBridgeRejectionReason::RawScreenPayload)
    );
}

#[test]
fn household_mesh_bridge_rejects_untrusted_replayed_stale_and_mismatched_imports() {
    assert_eq!(
        validate_household_mesh_bridge_import(&real_bridge_input().inbound_envelope()).state,
        HouseholdMeshBridgeValidationState::Rejected
    );
    assert_eq!(
        validate_household_mesh_bridge_import(&real_bridge_input().inbound_envelope())
            .rejection_reason,
        Some(HouseholdMeshBridgeRejectionReason::UnauthorizedPeer)
    );

    assert_structural_rejection(
        |envelope| envelope.message.direct_remote_publish_requested = true,
        HouseholdMeshBridgeRejectionReason::DirectRemotePublish,
    );
    assert_structural_rejection(
        |envelope| {
            envelope.message.authentication_state = HouseholdMeshAuthenticationState::Anonymous
        },
        HouseholdMeshBridgeRejectionReason::UnauthenticatedPeer,
    );
    assert_structural_rejection(
        |envelope| envelope.message.raw_payload_included = true,
        HouseholdMeshBridgeRejectionReason::RawScreenPayload,
    );
    assert_structural_rejection(
        |envelope| {
            envelope.message.policy_authority = HouseholdMeshPolicyAuthority::ProviderClaimed
        },
        HouseholdMeshBridgeRejectionReason::PolicyAuthorityEscalation,
    );
    assert_structural_rejection(
        |envelope| {
            envelope
                .seen_message_ids
                .push(envelope.message.message_id.clone())
        },
        HouseholdMeshBridgeRejectionReason::ReplayedMessage,
    );
    assert_structural_rejection(
        |envelope| {
            envelope
                .seen_idempotency_keys
                .push(envelope.message.idempotency_key.clone())
        },
        HouseholdMeshBridgeRejectionReason::ReplayedMessage,
    );
    assert_structural_rejection(
        |envelope| {
            envelope.received_at_epoch_seconds =
                envelope.message.sent_at_epoch_seconds + envelope.message.stale_after_seconds + 1
        },
        HouseholdMeshBridgeRejectionReason::StaleMessage,
    );
    assert_structural_rejection(
        |envelope| {
            envelope.expected_family_id =
                constants::household_mesh::TEST_BRIDGE_OTHER_FAMILY_ID.to_string()
        },
        HouseholdMeshBridgeRejectionReason::FamilyMismatch,
    );
    assert_structural_rejection(
        |envelope| {
            envelope.expected_target_child_device_id =
                constants::household_mesh::TEST_BRIDGE_OTHER_CHILD_DEVICE_ID.to_string()
        },
        HouseholdMeshBridgeRejectionReason::WrongTargetDevice,
    );
    assert_structural_rejection(
        |envelope| {
            envelope.message.lan_message_type =
                constants::household_mesh::LAN_MESSAGE_PROVIDER_ADVERTISEMENT.to_string()
        },
        HouseholdMeshBridgeRejectionReason::MismatchedMessageRef,
    );
    assert_structural_rejection(
        |envelope| {
            envelope.message.local_event_ref =
                constants::household_mesh::LOCAL_EVENT_RAW_CAPTURE_INTERNAL.to_string()
        },
        HouseholdMeshBridgeRejectionReason::UnselectedEvent,
    );
}

#[tokio::test]
async fn household_mesh_bridge_topology_uses_bridge_targets_not_direct_remote_bus() -> TestResult {
    let input = real_bridge_input();
    let report = test_ok(
        publish_household_mesh_bridge_chain_for_input(input).await,
        constants::household_mesh::ERROR_BRIDGE_TOPOLOGY_PROVES,
    )?;
    let payloads = decode_payloads(&report)?;

    assert!(payloads.iter().all(|payload| {
        payload.custody.selected_event_only
            && !payload.custody.remote_direct_publish_allowed
            && !payload.custody.raw_screenshot_transferred
            && !payload.custody.private_local_event_exported
    }));

    Ok(())
}

fn assert_structural_rejection(
    mutate: impl FnOnce(&mut HouseholdMeshBridgeInboundEnvelope),
    expected: HouseholdMeshBridgeRejectionReason,
) {
    let mut envelope = real_bridge_input().inbound_envelope();
    mutate(&mut envelope);
    let validation = envelope.validate_structure();
    assert!(
        validation.is_err(),
        "structurally invalid household mesh envelope was accepted"
    );
    if let Err(validation) = validation {
        assert_eq!(
            validation.state,
            HouseholdMeshBridgeValidationState::Rejected
        );
        assert_eq!(validation.rejection_reason, Some(expected));
    }
}

fn real_bridge_input() -> HouseholdMeshBridgeInput {
    HouseholdMeshBridgeInput {
        correlation_id: constants::household_mesh::TEST_BRIDGE_CORRELATION_ID.to_string(),
        local_event_type: constants::screen_flow::EVENT_SCREEN_MESH_OFFER_PUBLISHED.to_string(),
        family_id: constants::household_mesh::TEST_BRIDGE_FAMILY_ID.to_string(),
        target_child_device_id: constants::household_mesh::TEST_BRIDGE_TARGET_CHILD_DEVICE_ID
            .to_string(),
        outbound_message_id: constants::household_mesh::TEST_BRIDGE_OUTBOUND_MESSAGE_ID.to_string(),
        outbound_idempotency_key: constants::household_mesh::TEST_BRIDGE_IDEMPOTENCY_KEY
            .to_string(),
        child_agent_peer_id: constants::household_mesh::TEST_BRIDGE_CHILD_AGENT_PEER_ID.to_string(),
        provider_peer_id: constants::household_mesh::TEST_BRIDGE_PROVIDER_PEER_ID.to_string(),
        payload_ref: constants::household_mesh::TEST_BRIDGE_PAYLOAD_REF.to_string(),
        observed_at: constants::activity_store::TEST_FIRST_OBSERVED_AT.to_string(),
        received_at_epoch_seconds: constants::household_mesh::TEST_BRIDGE_RECEIVED_AT_EPOCH_SECONDS,
        inbound_message: real_transport_envelope(),
        seen_message_ids: Vec::new(),
        seen_idempotency_keys: Vec::new(),
    }
}

fn real_transport_envelope() -> HouseholdMeshTransportEnvelope {
    HouseholdMeshTransportEnvelope {
        schema_version: constants::household_mesh::EVENT_SCHEMA_VERSION,
        message_id: constants::household_mesh::TEST_BRIDGE_INBOUND_MESSAGE_ID.to_string(),
        idempotency_key: constants::household_mesh::TEST_BRIDGE_IDEMPOTENCY_KEY.to_string(),
        family_id: constants::household_mesh::TEST_BRIDGE_FAMILY_ID.to_string(),
        target_child_device_id: constants::household_mesh::TEST_BRIDGE_TARGET_CHILD_DEVICE_ID
            .to_string(),
        source_peer_id: constants::household_mesh::TEST_BRIDGE_PROVIDER_PEER_ID.to_string(),
        local_event_ref: constants::household_mesh::LOCAL_EVENT_AI_RESULT_RETURN.to_string(),
        lan_message_type: constants::household_mesh::LAN_MESSAGE_AI_RESULT_RETURN.to_string(),
        bridge_state: HouseholdMeshBridgeState::ExportSelected,
        authentication_state: HouseholdMeshAuthenticationState::PairedTrustedDevice,
        policy_authority: HouseholdMeshPolicyAuthority::ChildAgentOnly,
        direct_remote_publish_requested: false,
        raw_payload_included: false,
        sent_at_epoch_seconds: constants::household_mesh::TEST_BRIDGE_SENT_AT_EPOCH_SECONDS,
        stale_after_seconds: constants::household_mesh::TEST_BRIDGE_STALE_AFTER_SECONDS,
    }
}

fn decode_payloads(
    report: &HouseholdMeshBridgeReport,
) -> Result<Vec<HouseholdMeshBridgeEventPayload>, TestText> {
    report
        .stored_events
        .iter()
        .map(|event| {
            let envelope: ocentra_eventing::envelope::EventEnvelope<
                HouseholdMeshBridgeEventPayload,
            > = test_ok(
                event.decode(),
                constants::household_mesh::ERROR_BRIDGE_PAYLOAD_DECODES,
            )?;
            Ok(envelope.into_payload())
        })
        .collect()
}

fn payload_for_phase(
    payloads: &[HouseholdMeshBridgeEventPayload],
    phase: HouseholdMeshBridgePhase,
) -> Result<&HouseholdMeshBridgeEventPayload, TestText> {
    test_some(
        payloads.iter().find(|payload| payload.phase == phase),
        constants::household_mesh::ERROR_BRIDGE_PAYLOAD_DECODES,
    )
}
