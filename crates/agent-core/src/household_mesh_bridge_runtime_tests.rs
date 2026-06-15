use ocentra_parent_agent_protocol::constants;
use ocentra_eventing::envelope::EventEnvelope;

use super::{
    publish_household_mesh_bridge_chain_for_input, validate_household_mesh_bridge_export,
    validate_household_mesh_bridge_import, HouseholdMeshBridgeEventPayload,
    HouseholdMeshBridgeExportCandidate, HouseholdMeshBridgeInboundEnvelope,
    HouseholdMeshBridgeInput, HouseholdMeshBridgePhase, HouseholdMeshBridgeRejectionReason,
    HouseholdMeshBridgeReport, HouseholdMeshBridgeValidationState,
};

#[tokio::test]
async fn household_mesh_bridge_exports_selected_events_and_republishes_validated_imports() {
    let report =
        publish_household_mesh_bridge_chain_for_input(HouseholdMeshBridgeInput::proof_fixture())
            .await
            .expect(constants::household_mesh::ERROR_BRIDGE_CHAIN_PUBLISHES);
    let payloads = decode_payloads(&report);

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

    let exported = payload_for_phase(&payloads, HouseholdMeshBridgePhase::LanMessageExported);
    assert_eq!(
        exported.lan_message_type,
        constants::household_mesh::MESSAGE_AI_WORK_OFFER
    );
    assert_eq!(
        exported.previous_phase_ref,
        Some(constants::household_mesh::TEST_BRIDGE_SELECTED_EVENT_REF.to_string())
    );

    let republished = payload_for_phase(&payloads, HouseholdMeshBridgePhase::LocalEventRepublished);
    assert_eq!(
        republished.previous_phase_ref,
        Some(constants::household_mesh::TEST_BRIDGE_RECEIVED_MESSAGE_REF.to_string())
    );
    assert_eq!(
        republished.validation_state,
        HouseholdMeshBridgeValidationState::Accepted
    );
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
fn household_mesh_bridge_rejects_untrusted_or_direct_remote_imports() {
    assert_eq!(
        validate_household_mesh_bridge_import(&HouseholdMeshBridgeInboundEnvelope::accepted_offer())
            .state,
        HouseholdMeshBridgeValidationState::Accepted
    );

    assert_import_rejection(
        |envelope| envelope.direct_remote_publish_attempted = true,
        HouseholdMeshBridgeRejectionReason::DirectRemotePublish,
    );
    assert_import_rejection(
        |envelope| envelope.authenticated = false,
        HouseholdMeshBridgeRejectionReason::UnauthenticatedPeer,
    );
    assert_import_rejection(
        |envelope| envelope.authorized = false,
        HouseholdMeshBridgeRejectionReason::UnauthorizedPeer,
    );
    assert_import_rejection(
        |envelope| envelope.contains_raw_screenshot = true,
        HouseholdMeshBridgeRejectionReason::RawScreenPayload,
    );
    assert_import_rejection(
        |envelope| {
            envelope.lan_message_type =
                constants::screen_flow::EVENT_SCREEN_POLICY_DECISION_COMPLETED.to_string()
        },
        HouseholdMeshBridgeRejectionReason::UnsupportedLanMessage,
    );
}

#[tokio::test]
async fn household_mesh_bridge_topology_uses_bridge_targets_not_direct_remote_bus() {
    let report =
        publish_household_mesh_bridge_chain_for_input(HouseholdMeshBridgeInput::proof_fixture())
            .await
            .expect(constants::household_mesh::ERROR_BRIDGE_TOPOLOGY_PROVES);
    let payloads = decode_payloads(&report);

    assert!(payloads.iter().all(|payload| {
        payload.custody.selected_event_only
            && !payload.custody.remote_direct_publish_allowed
            && !payload.custody.raw_screenshot_transferred
            && !payload.custody.private_local_event_exported
    }));
}

fn assert_import_rejection(
    mutate: impl FnOnce(&mut HouseholdMeshBridgeInboundEnvelope),
    expected: HouseholdMeshBridgeRejectionReason,
) {
    let mut envelope = HouseholdMeshBridgeInboundEnvelope::accepted_offer();
    mutate(&mut envelope);
    let validation = validate_household_mesh_bridge_import(&envelope);
    assert_eq!(
        validation.state,
        HouseholdMeshBridgeValidationState::Rejected
    );
    assert_eq!(validation.rejection_reason, Some(expected));
}

fn decode_payloads(report: &HouseholdMeshBridgeReport) -> Vec<HouseholdMeshBridgeEventPayload> {
    report
        .stored_events
        .iter()
        .map(|event| {
            let envelope: EventEnvelope<HouseholdMeshBridgeEventPayload> = event
                .decode()
                .expect(constants::household_mesh::ERROR_BRIDGE_PAYLOAD_DECODES);
            envelope.payload
        })
        .collect()
}

fn payload_for_phase(
    payloads: &[HouseholdMeshBridgeEventPayload],
    phase: HouseholdMeshBridgePhase,
) -> &HouseholdMeshBridgeEventPayload {
    payloads
        .iter()
        .find(|payload| payload.phase == phase)
        .expect(constants::household_mesh::ERROR_BRIDGE_PAYLOAD_DECODES)
}
