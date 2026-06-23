use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::household_mesh::{
    HouseholdMeshAuthenticationState, HouseholdMeshBridgePhase,
    HouseholdMeshBridgeRejectionReason, HouseholdMeshBridgeValidationState,
};

use crate::household_mesh_bridge_runtime::{
    publish_household_mesh_bridge_chain_for_input, validate_household_mesh_bridge_export,
    validate_household_mesh_bridge_import, HouseholdMeshBridgeEventPayload,
    HouseholdMeshBridgeExportCandidate, HouseholdMeshBridgeInboundEnvelope,
    HouseholdMeshBridgeInput, HouseholdMeshBridgeReport,
};

type TestResult = Result<(), String>;

#[tokio::test]
async fn household_mesh_bridge_exports_selected_events_and_republishes_validated_imports() -> TestResult {
    let report = ok(
        publish_household_mesh_bridge_chain_for_input(HouseholdMeshBridgeInput::proof_fixture())
            .await,
        constants::household_mesh::ERROR_BRIDGE_CHAIN_PUBLISHES,
    )?;
    let payloads = decode_payloads(&report)?;

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
        HouseholdMeshBridgeValidationState::Accepted
    );

    let republished =
        payload_for_phase(&payloads, HouseholdMeshBridgePhase::LocalEventRepublished)?;
    assert_eq!(
        republished.previous_phase_ref,
        Some(constants::household_mesh::TEST_BRIDGE_RECEIVED_MESSAGE_REF.to_string())
    );
    assert_eq!(
        republished.validation_state,
        HouseholdMeshBridgeValidationState::Accepted
    );
    assert_eq!(republished.rejection_reason, None);

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
        validate_household_mesh_bridge_import(
            &HouseholdMeshBridgeInboundEnvelope::accepted_result()
        )
        .state,
        HouseholdMeshBridgeValidationState::Accepted
    );

    assert_import_rejection(
        |envelope| envelope.message.direct_remote_publish_requested = true,
        HouseholdMeshBridgeRejectionReason::DirectRemotePublish,
    );
    assert_import_rejection(
        |envelope| {
            envelope.message.authentication_state = HouseholdMeshAuthenticationState::Anonymous
        },
        HouseholdMeshBridgeRejectionReason::UnauthenticatedPeer,
    );
    assert_import_rejection(
        |envelope| envelope.authorized = false,
        HouseholdMeshBridgeRejectionReason::UnauthorizedPeer,
    );
    assert_import_rejection(
        |envelope| envelope.message.raw_payload_included = true,
        HouseholdMeshBridgeRejectionReason::RawScreenPayload,
    );
    assert_import_rejection(
        |envelope| {
            envelope.message.policy_authority =
                ocentra_parent_agent_protocol::household_mesh::HouseholdMeshPolicyAuthority::ProviderClaimed
        },
        HouseholdMeshBridgeRejectionReason::PolicyAuthorityEscalation,
    );
    assert_import_rejection(
        |envelope| {
            envelope
                .seen_message_ids
                .push(envelope.message.message_id.clone())
        },
        HouseholdMeshBridgeRejectionReason::ReplayedMessage,
    );
    assert_import_rejection(
        |envelope| {
            envelope
                .seen_idempotency_keys
                .push(envelope.message.idempotency_key.clone())
        },
        HouseholdMeshBridgeRejectionReason::ReplayedMessage,
    );
    assert_import_rejection(
        |envelope| {
            envelope.received_at_epoch_seconds =
                envelope.message.sent_at_epoch_seconds + envelope.message.stale_after_seconds + 1
        },
        HouseholdMeshBridgeRejectionReason::StaleMessage,
    );
    assert_import_rejection(
        |envelope| {
            envelope.expected_family_id =
                constants::household_mesh::TEST_BRIDGE_OTHER_FAMILY_ID.to_string()
        },
        HouseholdMeshBridgeRejectionReason::FamilyMismatch,
    );
    assert_import_rejection(
        |envelope| {
            envelope.expected_target_child_device_id =
                constants::household_mesh::TEST_BRIDGE_OTHER_CHILD_DEVICE_ID.to_string()
        },
        HouseholdMeshBridgeRejectionReason::WrongTargetDevice,
    );
    assert_import_rejection(
        |envelope| {
            envelope.message.lan_message_type =
                constants::household_mesh::LAN_MESSAGE_PROVIDER_ADVERTISEMENT.to_string()
        },
        HouseholdMeshBridgeRejectionReason::MismatchedMessageRef,
    );
    assert_import_rejection(
        |envelope| {
            envelope.message.local_event_ref =
                constants::household_mesh::LOCAL_EVENT_RAW_CAPTURE_INTERNAL.to_string()
        },
        HouseholdMeshBridgeRejectionReason::UnselectedEvent,
    );
}

#[tokio::test]
async fn household_mesh_bridge_topology_uses_bridge_targets_not_direct_remote_bus() -> TestResult {
    let report = ok(
        publish_household_mesh_bridge_chain_for_input(HouseholdMeshBridgeInput::proof_fixture())
            .await,
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

fn assert_import_rejection(
    mutate: impl FnOnce(&mut HouseholdMeshBridgeInboundEnvelope),
    expected: HouseholdMeshBridgeRejectionReason,
) {
    let mut envelope = HouseholdMeshBridgeInboundEnvelope::accepted_result();
    mutate(&mut envelope);
    let validation = validate_household_mesh_bridge_import(&envelope);
    assert_eq!(
        validation.state,
        HouseholdMeshBridgeValidationState::Rejected
    );
    assert_eq!(validation.rejection_reason, Some(expected));
}

fn decode_payloads(
    report: &HouseholdMeshBridgeReport,
) -> Result<Vec<HouseholdMeshBridgeEventPayload>, String> {
    report
        .stored_events
        .iter()
        .map(|event| {
            let envelope: ocentra_eventing::envelope::EventEnvelope<
                HouseholdMeshBridgeEventPayload,
            > = ok(
                event.decode(),
                constants::household_mesh::ERROR_BRIDGE_PAYLOAD_DECODES,
            )?;
            Ok(envelope.payload)
        })
        .collect()
}

fn payload_for_phase(
    payloads: &[HouseholdMeshBridgeEventPayload],
    phase: HouseholdMeshBridgePhase,
) -> Result<&HouseholdMeshBridgeEventPayload, String> {
    some(
        payloads
        .iter()
        .find(|payload| payload.phase == phase),
        constants::household_mesh::ERROR_BRIDGE_PAYLOAD_DECODES,
    )
}

fn ok<T, E: core::fmt::Debug>(result: Result<T, E>, context: &str) -> Result<T, String> {
    result.map_err(|error| format!("{context}: {error:?}"))
}

fn some<T>(value: Option<T>, context: &str) -> Result<T, String> {
    value.ok_or_else(|| context.to_string())
}
