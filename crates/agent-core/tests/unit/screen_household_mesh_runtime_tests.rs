use ocentra_eventing::expect_value::ExpectValue;
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::screen_evidence::{
    ScreenHouseholdMeshPhase, ScreenMeshChildValidationState, ScreenMeshClaimState,
    ScreenMeshLeaseState, ScreenMeshPolicyState, ScreenMeshProviderResultState,
    ScreenMeshResultRejectionReason,
};

use ocentra_parent_agent_core::screen_household_mesh_runtime::{
    publish_screen_household_mesh_chain_for_input, validate_screen_household_mesh_result,
    ScreenHouseholdMeshEventPayload, ScreenHouseholdMeshInput, ScreenHouseholdMeshReport,
    ScreenHouseholdMeshResultSubmission,
};

#[tokio::test]
async fn screen_household_mesh_chain_publishes_claim_lease_and_child_validation() {
    let report =
        publish_screen_household_mesh_chain_for_input(ScreenHouseholdMeshInput::proof_fixture())
            .await
            .expect_value(constants::screen_flow::ERROR_SCREEN_MESH_CHAIN_PUBLISHES);
    let payloads = decode_payloads(&report);

    assert_eq!(
        report.publish_reports.len(),
        ScreenHouseholdMeshPhase::ordered_chain().len()
    );
    assert_eq!(
        report.stored_events.len(),
        ScreenHouseholdMeshPhase::ordered_chain().len()
    );
    assert!(report.dead_letters.is_empty());
    assert!(!report.raw_screenshot_escaped());

    let claim = payload_for_phase(&payloads, ScreenHouseholdMeshPhase::ClaimGranted);
    assert_eq!(claim.claim_state, ScreenMeshClaimState::Granted);
    assert_eq!(claim.lease_state, ScreenMeshLeaseState::NotCreated);

    let lease = payload_for_phase(&payloads, ScreenHouseholdMeshPhase::LeaseCreated);
    assert_eq!(lease.lease_state, ScreenMeshLeaseState::Active);

    let accepted = payload_for_phase(&payloads, ScreenHouseholdMeshPhase::ChildResultAccepted);
    assert_eq!(
        accepted.child_validation_state,
        ScreenMeshChildValidationState::Accepted
    );
    assert_eq!(
        accepted.provider_result_state,
        ScreenMeshProviderResultState::Returned
    );
}

#[tokio::test]
async fn screen_household_mesh_policy_waits_for_child_accepted_result() {
    let report =
        publish_screen_household_mesh_chain_for_input(ScreenHouseholdMeshInput::proof_fixture())
            .await
            .expect_value(constants::screen_flow::ERROR_SCREEN_MESH_CHAIN_PUBLISHES);
    let payloads = decode_payloads(&report);

    let provider_result =
        payload_for_phase(&payloads, ScreenHouseholdMeshPhase::ProviderResultReturned);
    assert_eq!(provider_result.policy_decision_ref, None);
    assert_eq!(
        provider_result.policy_state,
        ScreenMeshPolicyState::NotReady
    );

    let policy = payload_for_phase(&payloads, ScreenHouseholdMeshPhase::PolicyRequested);
    assert_eq!(
        policy.previous_phase_ref,
        Some(constants::screen_flow::SCREEN_MESH_CHILD_ACCEPTED_EVENT_REF.to_string())
    );
    assert_eq!(
        policy.provider_result_ref,
        Some(constants::screen_flow::TEST_SCREEN_MESH_RESULT_REF.to_string())
    );
    assert_eq!(
        policy.policy_decision_ref,
        Some(constants::activity_store::TEST_POLICY_DECISION_ID.to_string())
    );
    assert_eq!(policy.policy_state, ScreenMeshPolicyState::Ready);
}

#[test]
fn screen_household_mesh_rejects_invalid_provider_results_before_policy() {
    let input = ScreenHouseholdMeshInput::proof_fixture();
    let accepted = ScreenHouseholdMeshResultSubmission::accepted_for(&input);
    assert!(validate_screen_household_mesh_result(&input, &accepted).policy_may_run);

    assert_rejection(
        &input,
        |submission| submission.duplicate_result = true,
        ScreenMeshResultRejectionReason::DuplicateResult,
    );
    assert_rejection(
        &input,
        |submission| submission.completed_after_lease_expiry = true,
        ScreenMeshResultRejectionReason::ExpiredLease,
    );
    assert_rejection(
        &input,
        |submission| {
            submission
                .provider_peer_id
                .push_str(constants::screen_flow::TEST_SCREEN_MESH_WRONG_SUFFIX)
        },
        ScreenMeshResultRejectionReason::WrongProvider,
    );
    assert_rejection(
        &input,
        |submission| {
            submission
                .claim_id
                .push_str(constants::screen_flow::TEST_SCREEN_MESH_WRONG_SUFFIX)
        },
        ScreenMeshResultRejectionReason::WrongClaim,
    );
    assert_rejection(
        &input,
        |submission| {
            submission
                .screen_evidence_ref
                .push_str(constants::screen_flow::TEST_SCREEN_MESH_WRONG_SUFFIX)
        },
        ScreenMeshResultRejectionReason::EvidenceMismatch,
    );
    assert_rejection(
        &input,
        |submission| {
            submission
                .custody_label
                .push_str(constants::screen_flow::TEST_SCREEN_MESH_WRONG_SUFFIX)
        },
        ScreenMeshResultRejectionReason::CustodyMismatch,
    );
    assert_rejection(
        &input,
        |submission| submission.raw_screenshot_transferred = true,
        ScreenMeshResultRejectionReason::RawImageTransfer,
    );
    assert_rejection(
        &input,
        |submission| submission.provider_policy_event_attempted = true,
        ScreenMeshResultRejectionReason::ProviderAuthorityViolation,
    );
}

#[tokio::test]
async fn screen_household_mesh_keeps_provider_worker_only_and_no_raw_transfer() {
    let report =
        publish_screen_household_mesh_chain_for_input(ScreenHouseholdMeshInput::proof_fixture())
            .await
            .expect_value(constants::screen_flow::ERROR_SCREEN_MESH_CHAIN_PUBLISHES);
    let payloads = decode_payloads(&report);

    assert!(payloads.iter().all(|payload| {
        !payload.custody_boundary.raw_screenshot_transferred
            && !payload.custody_boundary.raw_screenshot_retained_by_provider
            && !payload.custody_boundary.provider_can_publish_policy
            && !payload.custody_boundary.provider_can_publish_enforcement
            && payload.custody_boundary.child_agent_validates_before_policy
    }));
}

fn assert_rejection(
    input: &ScreenHouseholdMeshInput,
    mutate: impl FnOnce(&mut ScreenHouseholdMeshResultSubmission),
    expected: ScreenMeshResultRejectionReason,
) {
    let mut submission = ScreenHouseholdMeshResultSubmission::accepted_for(input);
    mutate(&mut submission);
    let validation = validate_screen_household_mesh_result(input, &submission);
    assert!(!validation.accepted);
    assert!(!validation.policy_may_run);
    assert_eq!(validation.rejection_reason, Some(expected));
}

fn decode_payloads(report: &ScreenHouseholdMeshReport) -> Vec<ScreenHouseholdMeshEventPayload> {
    report
        .stored_events
        .iter()
        .map(|event| {
            let envelope: ocentra_eventing::envelope::EventEnvelope<
                ScreenHouseholdMeshEventPayload,
            > = event
                .decode()
                .expect_value(constants::screen_flow::ERROR_SCREEN_MESH_PAYLOAD_DECODES);
            envelope.payload
        })
        .collect()
}

fn payload_for_phase(
    payloads: &[ScreenHouseholdMeshEventPayload],
    phase: ScreenHouseholdMeshPhase,
) -> &ScreenHouseholdMeshEventPayload {
    payloads
        .iter()
        .find(|payload| payload.phase == phase)
        .expect_value(constants::screen_flow::ERROR_SCREEN_MESH_PAYLOAD_DECODES)
}
