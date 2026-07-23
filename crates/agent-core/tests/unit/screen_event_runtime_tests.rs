use ocentra_eventing::bus::reports::handler::HandlerOutcome;
use ocentra_eventing::expect_value::ExpectValue;
use ocentra_eventing::journal::ndjson::{NdjsonEventJournal, NdjsonJournalOptions};
use ocentra_eventing::replay::ReplayFilter;
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::screen_evidence::{
    ScreenActionState, ScreenAiAuditState, ScreenDeletionState, ScreenEvidenceScope,
    ScreenPolicyState, ScreenRuntimePhase,
};
use std::{
    fmt::Display,
    fs,
    sync::atomic::{AtomicU64, Ordering},
};

use ocentra_parent_agent_core::screen_event_runtime::{
    publish_screen_capture_queue_events_for_input, publish_screen_degraded_event_chain_for_input,
    publish_screen_deletion_event_for_input, publish_screen_runtime_chain_for_input,
    ScreenRuntimeEventPayload, ScreenRuntimeReport, ScreenRuntimeSpine,
};
use ocentra_parent_agent_core::screen_event_runtime_input::{
    ScreenRuntimeCaptureInput, ScreenRuntimeDegradedInput, ScreenRuntimeDeletionInput,
    ScreenRuntimeInput,
};

static SCREEN_EVENT_RUNTIME_TEST_SEQUENCE: AtomicU64 = AtomicU64::new(0);

#[tokio::test]
async fn screen_runtime_chain_publishes_uncoupled_lifecycle_flow() {
    let report = publish_screen_runtime_chain_for_input(
        ScreenRuntimeInput::proof_fixture(),
        constants::activity_store::TEST_FIRST_OBSERVED_AT,
    )
    .await
    .expect_value(constants::screen_flow::ERROR_SCREEN_RUNTIME_CHAIN_PUBLISHES);
    let payloads = decode_payloads(&report);

    assert_eq!(
        report.publish_reports.len(),
        ScreenRuntimePhase::ordered_chain().len()
    );
    assert_eq!(
        report.stored_events.len(),
        ScreenRuntimePhase::ordered_chain().len()
    );
    assert!(report.dead_letters.is_empty());
    assert!(!report.raw_image_escaped());
    assert_eq!(payloads[0].phase, ScreenRuntimePhase::CaptureObserved);
    assert_eq!(payloads[2].ai_audit_state, ScreenAiAuditState::Requested);
    assert_eq!(payloads[3].ai_audit_state, ScreenAiAuditState::Completed);
    assert_eq!(
        count_event_type(
            &report,
            constants::screen_flow::EVENT_SCREEN_ACTION_DRY_RUN_RECORDED
        ),
        1
    );
    assert_eq!(
        count_event_type(
            &report,
            constants::screen_flow::EVENT_SCREEN_DELETION_COMMITTED
        ),
        1
    );
}

#[tokio::test]
async fn screen_capture_queue_events_publish_without_ai_policy_or_action_refs() {
    let input = ScreenRuntimeCaptureInput::from(&ScreenRuntimeInput::proof_fixture());
    let report = publish_screen_capture_queue_events_for_input(
        input,
        constants::activity_store::TEST_FIRST_OBSERVED_AT,
    )
    .await
    .expect_value(constants::screen_flow::ERROR_SCREEN_RUNTIME_CHAIN_PUBLISHES);
    let payloads = decode_payloads(&report);

    assert_eq!(report.publish_reports.len(), 2);
    assert_eq!(report.stored_events.len(), 2);
    assert!(report.dead_letters.is_empty());
    assert_eq!(payloads[0].phase, ScreenRuntimePhase::CaptureObserved);
    assert_eq!(payloads[1].phase, ScreenRuntimePhase::QueueEncrypted);
    assert!(payloads.iter().all(|payload| {
        payload.ai_request_ref.is_none()
            && payload.ai_result_ref.is_none()
            && payload.policy_decision_ref.is_none()
            && payload.action_ref.is_none()
            && payload.deletion_proof_ref.is_none()
            && payload.evidence_scope == ScreenEvidenceScope::EncryptedLocalImage
            && payload.ai_audit_state == ScreenAiAuditState::NotRequested
            && payload.policy_state == ScreenPolicyState::NotReady
            && payload.deletion_state == ScreenDeletionState::Pending
    }));
    assert_eq!(
        payloads[1].queue_event_ref,
        Some(constants::screen_flow::SCREEN_QUEUE_EVENT_REF.to_string())
    );
    assert!(!report.raw_image_escaped());
}

#[tokio::test]
async fn screen_deletion_event_publishes_without_policy_or_action_claims() {
    let input = ScreenRuntimeDeletionInput::from(&ScreenRuntimeInput::proof_fixture());
    let report = publish_screen_deletion_event_for_input(
        input,
        constants::activity_store::TEST_FIRST_OBSERVED_AT,
    )
    .await
    .expect_value(constants::screen_flow::ERROR_SCREEN_RUNTIME_CHAIN_PUBLISHES);
    let payloads = decode_payloads(&report);

    assert_eq!(report.publish_reports.len(), 1);
    assert_eq!(report.stored_events.len(), 1);
    assert!(report.dead_letters.is_empty());
    assert_eq!(payloads[0].phase, ScreenRuntimePhase::DeletionCommitted);
    assert_eq!(
        payloads[0].previous_phase_ref,
        Some(constants::screen_flow::SCREEN_QUEUE_EVENT_REF.to_string())
    );
    assert_eq!(payloads[0].policy_decision_ref, None);
    assert_eq!(payloads[0].policy_action, None);
    assert_eq!(payloads[0].action_ref, None);
    assert_eq!(
        payloads[0].deletion_proof_ref,
        Some(constants::activity_store::TEST_SCREEN_DELETION_REASONS.to_string())
    );
    assert_eq!(payloads[0].ai_audit_state, ScreenAiAuditState::NotRequested);
    assert_eq!(payloads[0].policy_state, ScreenPolicyState::NotReady);
    assert_eq!(payloads[0].action_state, ScreenActionState::NotReady);
    assert_eq!(payloads[0].deletion_state, ScreenDeletionState::Committed);
    assert_eq!(
        payloads[0].evidence_scope,
        ScreenEvidenceScope::DeletedQueryStoreSummary
    );
    assert!(!report.raw_image_escaped());
}

#[tokio::test]
async fn screen_deletion_delivery_is_handled_and_survives_journal_reopen() {
    let journal_path = screen_deletion_journal_path("handled-reopen");
    let _ = fs::remove_file(&journal_path);
    let spine = ScreenRuntimeSpine::with_durable_deletion_handler(
        NdjsonEventJournal::with_options(&journal_path, NdjsonJournalOptions::hash_chain()),
    )
    .await
    .expect_value(constants::screen_flow::ERROR_SCREEN_RUNTIME_CHAIN_PUBLISHES);
    let report = spine
        .publish_deletion_event(
            ScreenRuntimeDeletionInput::from(&ScreenRuntimeInput::proof_fixture()),
            constants::activity_store::TEST_FIRST_OBSERVED_AT,
        )
        .await
        .expect_value(constants::screen_flow::ERROR_SCREEN_RUNTIME_CHAIN_PUBLISHES);
    drop(spine);

    let reopened =
        NdjsonEventJournal::with_options(&journal_path, NdjsonJournalOptions::hash_chain());
    let replay = reopened
        .replay_projection(ReplayFilter::all())
        .await
        .expect_value(constants::screen_flow::ERROR_SCREEN_RUNTIME_CHAIN_PUBLISHES);
    let _ = fs::remove_file(&journal_path);

    assert_eq!(report.publish_reports.len(), 1);
    assert_eq!(report.publish_reports[0].subscriber_count, 1);
    assert_eq!(report.publish_reports[0].handled_count, 1);
    assert_eq!(report.publish_reports[0].dead_letter_count, 0);
    assert_eq!(
        report.publish_reports[0].handler_reports[0].outcome,
        HandlerOutcome::Handled
    );
    assert_eq!(replay.records.len(), 1);
    assert_eq!(
        replay.records[0].envelope.event_id,
        report.publish_reports[0].event_id
    );
}

#[tokio::test]
async fn screen_degraded_event_chain_publishes_without_policy_or_action_claims() {
    let report = publish_screen_degraded_event_chain_for_input(
        ScreenRuntimeDegradedInput::proof_fixture(),
        constants::activity_store::TEST_FIRST_OBSERVED_AT,
    )
    .await
    .expect_value(constants::screen_flow::ERROR_SCREEN_RUNTIME_CHAIN_PUBLISHES);
    let payloads = decode_payloads(&report);
    let phases = payloads
        .iter()
        .map(|payload| payload.phase)
        .collect::<Vec<_>>();

    assert_eq!(
        phases,
        vec![
            ScreenRuntimePhase::CaptureObserved,
            ScreenRuntimePhase::QueueEncrypted,
            ScreenRuntimePhase::AiAnalysisRequested,
            ScreenRuntimePhase::AiAnalysisCompleted,
            ScreenRuntimePhase::DeletionCommitted,
            ScreenRuntimePhase::PortalReadModelUpdated,
        ]
    );
    assert!(report.dead_letters.is_empty());
    assert!(payloads.iter().all(|payload| {
        payload.policy_decision_ref.is_none()
            && payload.policy_action.is_none()
            && payload.parent_rule_ref.is_none()
            && payload.action_ref.is_none()
            && payload.policy_state == ScreenPolicyState::NotReady
            && payload.action_state == ScreenActionState::NotReady
    }));
    let ai_completed = payload_for_phase(&payloads, ScreenRuntimePhase::AiAnalysisCompleted);
    assert_eq!(ai_completed.ai_audit_state, ScreenAiAuditState::Completed);
    assert_eq!(
        ai_completed.deletion_proof_ref,
        Some(constants::activity_store::TEST_SCREEN_DELETION_REASONS.to_string())
    );
    let portal = payload_for_phase(&payloads, ScreenRuntimePhase::PortalReadModelUpdated);
    assert_eq!(portal.deletion_state, ScreenDeletionState::Committed);
    assert_eq!(
        portal.portal_read_model_ref,
        Some(constants::screen_flow::TEST_SCREEN_PORTAL_READ_MODEL_REF.to_string())
    );
    assert!(!report.raw_image_escaped());
}

#[tokio::test]
async fn screen_runtime_chain_carries_refs_without_direct_ai_to_policy_shortcut() {
    let report = publish_screen_runtime_chain_for_input(
        ScreenRuntimeInput::proof_fixture(),
        constants::activity_store::TEST_FIRST_OBSERVED_AT,
    )
    .await
    .expect_value(constants::screen_flow::ERROR_SCREEN_RUNTIME_CHAIN_PUBLISHES);
    let payloads = decode_payloads(&report);

    let ai_request = payload_for_phase(&payloads, ScreenRuntimePhase::AiAnalysisRequested);
    assert_eq!(
        ai_request.previous_phase_ref,
        Some(constants::screen_flow::SCREEN_QUEUE_EVENT_REF.to_string())
    );
    assert_eq!(ai_request.policy_decision_ref, None);
    assert_eq!(ai_request.action_ref, None);

    let policy = payload_for_phase(&payloads, ScreenRuntimePhase::PolicyDecisionCompleted);
    assert_eq!(
        policy.previous_phase_ref,
        Some(constants::screen_flow::SCREEN_SUMMARY_EVENT_REF.to_string())
    );
    assert_eq!(
        policy.ai_result_ref,
        Some(constants::screen_flow::SCREEN_AI_RESULT_EVENT_REF.to_string())
    );
    assert_eq!(
        policy.summary_ref,
        Some(constants::screen_flow::SCREEN_SUMMARY_EVENT_REF.to_string())
    );
    assert_eq!(
        policy.policy_decision_ref,
        Some(constants::activity_store::TEST_POLICY_DECISION_ID.to_string())
    );
    assert_eq!(policy.action_ref, None);

    let action = payload_for_phase(&payloads, ScreenRuntimePhase::ActionDryRunRecorded);
    assert_eq!(
        action.previous_phase_ref,
        Some(constants::screen_flow::SCREEN_POLICY_EVENT_REF.to_string())
    );
    assert_eq!(
        action.policy_decision_ref,
        Some(constants::activity_store::TEST_POLICY_DECISION_ID.to_string())
    );
    assert_eq!(
        action.action_ref,
        Some(constants::screen_flow::TEST_SCREEN_ACTION_REF.to_string())
    );
}

#[tokio::test]
async fn screen_runtime_chain_keeps_raw_image_out_of_policy_portal_and_provider() {
    let report = publish_screen_runtime_chain_for_input(
        ScreenRuntimeInput::proof_fixture(),
        constants::activity_store::TEST_FIRST_OBSERVED_AT,
    )
    .await
    .expect_value(constants::screen_flow::ERROR_SCREEN_RUNTIME_CHAIN_PUBLISHES);
    let payloads = decode_payloads(&report);

    assert!(payloads.iter().all(|payload| {
        !payload.claim_boundary.raw_image_available_to_ai_provider
            && !payload.claim_boundary.raw_image_available_to_policy
            && !payload.claim_boundary.raw_image_available_to_portal
            && !payload.claim_boundary.adapter_action_executed
    }));

    let queue = payload_for_phase(&payloads, ScreenRuntimePhase::QueueEncrypted);
    assert_eq!(
        queue.evidence_scope,
        ScreenEvidenceScope::EncryptedLocalImage
    );
    assert_eq!(queue.deletion_state, ScreenDeletionState::Pending);

    let portal = payload_for_phase(&payloads, ScreenRuntimePhase::PortalReadModelUpdated);
    assert_eq!(
        portal.evidence_scope,
        ScreenEvidenceScope::DeletedQueryStoreSummary
    );
    assert_eq!(portal.policy_state, ScreenPolicyState::Completed);
    assert_eq!(portal.action_state, ScreenActionState::DryRunRecorded);
    assert_eq!(portal.deletion_state, ScreenDeletionState::Committed);
    assert_eq!(
        portal.deletion_proof_ref,
        Some(constants::activity_store::TEST_SCREEN_DELETION_REASONS.to_string())
    );
    assert_eq!(
        portal.portal_read_model_ref,
        Some(constants::screen_flow::TEST_SCREEN_PORTAL_READ_MODEL_REF.to_string())
    );
}

fn decode_payloads(report: &ScreenRuntimeReport) -> Vec<ScreenRuntimeEventPayload> {
    report
        .stored_events
        .iter()
        .map(|event| {
            let envelope: ocentra_eventing::envelope::EventEnvelope<ScreenRuntimeEventPayload> =
                event
                    .decode()
                    .expect_value(constants::screen_flow::ERROR_SCREEN_RUNTIME_PAYLOAD_DECODES);
            envelope.payload
        })
        .collect()
}

fn count_event_type(report: &ScreenRuntimeReport, event_type: impl Display) -> usize {
    let event_type = event_type.to_string();
    report
        .stored_events
        .iter()
        .filter(|event| event.contract.event_type.as_str() == event_type.as_str())
        .count()
}

fn payload_for_phase(
    payloads: &[ScreenRuntimeEventPayload],
    phase: ScreenRuntimePhase,
) -> &ScreenRuntimeEventPayload {
    payloads
        .iter()
        .find(|payload| payload.phase == phase)
        .expect_value(constants::screen_flow::ERROR_SCREEN_RUNTIME_PAYLOAD_DECODES)
}

fn screen_deletion_journal_path(suffix: &str) -> std::path::PathBuf {
    let sequence = SCREEN_EVENT_RUNTIME_TEST_SEQUENCE.fetch_add(1, Ordering::Relaxed);
    std::env::temp_dir().join(format!(
        "ocentra-screen-deletion-{suffix}-{}-{sequence}.ndjson",
        std::process::id()
    ))
}
