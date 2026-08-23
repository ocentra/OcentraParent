use ocentra_eventing::bus::publisher::{EventContext, RootEventPublisher};
use ocentra_eventing::bus::reports::handler::HandlerOutcome;
use ocentra_eventing::bus::subscriber::EventSubscriber;
use ocentra_eventing::bus::EventBus;
use ocentra_eventing::envelope::{DomainEvent, EventContract, EventMetadata, EventSource};
use ocentra_eventing::error::EventingError;
use ocentra_eventing::expect_value::ExpectValue;
use ocentra_eventing::ids::{
    AggregateKey, CorrelationId, EventCustody, EventId, EventType, IdempotencyKey, RecordedAt,
    RuntimeInstanceId, RuntimeRole, SchemaVersion, SourceComponent, SourceService, SubscriberId,
    TargetHandler,
};
use ocentra_eventing::journal::ndjson::{NdjsonEventJournal, NdjsonJournalOptions};
use ocentra_eventing::replay::ReplayFilter;
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::screen_evidence::{
    ScreenActionState, ScreenAiAuditState, ScreenDeletionState, ScreenEvidenceScope,
    ScreenPolicyState, ScreenRuntimePhase,
};
use serde::{Deserialize, Serialize};
use std::{
    fmt::Display,
    fs,
    sync::atomic::{AtomicU64, Ordering},
    sync::{Arc, Mutex},
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

fn screen_runtime_input() -> ScreenRuntimeInput {
    ScreenRuntimeInput {
        queue_job_id: constants::activity_store::TEST_SCREEN_QUEUE_JOB_ID.to_string(),
        screen_analysis_result_id: constants::activity_store::TEST_SCREEN_RESULT_ID.to_string(),
        capture_reason: constants::activity_capture::SCREEN_TRIGGER_TIMED_CADENCE.to_string(),
        capture_scope: constants::activity_capture::OBSERVATION_MODE_ACTIVE_WINDOW.to_string(),
        image_digest: constants::activity_store::TEST_SCREEN_IMAGE_DIGEST.to_string(),
        summary: constants::activity_store::TEST_SCREEN_SUMMARY.to_string(),
        model_runtime_ref: constants::activity_store::TEST_SCREEN_MODEL_RUNTIME_REF.to_string(),
        model_id: constants::activity_store::TEST_SCREEN_MODEL_ID.to_string(),
        prompt_or_template_version: constants::activity_store::TEST_SCREEN_TEMPLATE_VERSION
            .to_string(),
        policy_decision_ref: constants::activity_store::TEST_POLICY_DECISION_ID.to_string(),
        policy_action: constants::activity_store::TEST_POLICY_ACTION_ALLOW.to_string(),
        parent_rule_ref: constants::screen_flow::TEST_SCREEN_POLICY_RULE_REF.to_string(),
        action_ref: constants::screen_flow::TEST_SCREEN_ACTION_REF.to_string(),
        deletion_proof_ref: constants::activity_store::TEST_SCREEN_DELETION_REASONS.to_string(),
        portal_read_model_ref: constants::screen_flow::TEST_SCREEN_PORTAL_READ_MODEL_REF
            .to_string(),
    }
}

fn screen_runtime_degraded_input() -> ScreenRuntimeDegradedInput {
    ScreenRuntimeDegradedInput {
        queue_job_id: constants::activity_store::TEST_SCREEN_QUEUE_JOB_ID.to_string(),
        screen_analysis_result_id: constants::activity_store::TEST_SCREEN_RESULT_ID.to_string(),
        capture_reason: constants::activity_capture::SCREEN_TRIGGER_TIMED_CADENCE.to_string(),
        capture_scope: constants::activity_capture::OBSERVATION_MODE_ACTIVE_WINDOW.to_string(),
        image_digest: constants::activity_store::TEST_SCREEN_IMAGE_DIGEST.to_string(),
        summary: constants::activity_store::TEST_SCREEN_SUMMARY.to_string(),
        model_runtime_ref: constants::activity_store::TEST_SCREEN_MODEL_RUNTIME_REF.to_string(),
        model_id: constants::activity_store::TEST_SCREEN_MODEL_ID.to_string(),
        prompt_or_template_version: constants::activity_store::TEST_SCREEN_TEMPLATE_VERSION
            .to_string(),
        deletion_proof_ref: constants::activity_store::TEST_SCREEN_DELETION_REASONS.to_string(),
        portal_read_model_ref: constants::screen_flow::TEST_SCREEN_PORTAL_READ_MODEL_REF
            .to_string(),
    }
}

#[tokio::test]
async fn screen_runtime_chain_publishes_uncoupled_lifecycle_flow() {
    let report = test_root_publish_screen_chain(
        screen_runtime_input(),
        screen_runtime_degraded_input(),
        false,
    )
    .await;
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
    let input = ScreenRuntimeCaptureInput::from(&screen_runtime_input());
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
    let input = ScreenRuntimeDeletionInput::from(&screen_runtime_input());
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
            ScreenRuntimeDeletionInput::from(&screen_runtime_input()),
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
    let report = test_root_publish_screen_chain(
        screen_runtime_input(),
        screen_runtime_degraded_input(),
        true,
    )
    .await;
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
    let report = test_root_publish_screen_chain(
        screen_runtime_input(),
        screen_runtime_degraded_input(),
        false,
    )
    .await;
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
    let report = test_root_publish_screen_chain(
        screen_runtime_input(),
        screen_runtime_degraded_input(),
        false,
    )
    .await;
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

#[derive(Clone, Debug, Serialize, Deserialize)]
struct ScreenRuntimeTestTrigger;

impl DomainEvent for ScreenRuntimeTestTrigger {
    fn contract(&self) -> Result<EventContract, EventingError> {
        Ok(EventContract::new(
            EventType::parse("eventing.screen-runtime.test-trigger")?,
            SchemaVersion::new(1)?,
        ))
    }

    fn aggregate_key(&self) -> Result<AggregateKey, EventingError> {
        AggregateKey::parse("screen-runtime-test-trigger")
    }

    fn idempotency_key(&self) -> Result<IdempotencyKey, EventingError> {
        IdempotencyKey::parse("screen-runtime-test-trigger-1")
    }
}

async fn test_root_publish_screen_chain(
    input: ScreenRuntimeInput,
    degraded_input: ScreenRuntimeDegradedInput,
    degraded: bool,
) -> ScreenRuntimeReport {
    let bus = EventBus::new();
    let target = bus.event_bus().clone();
    register_screen_runtime_test_owners(&bus).await;
    let captured = Arc::new(Mutex::new(None));
    let captured_for_handler = Arc::clone(&captured);
    let input_for_handler = input.clone();
    let degraded_input_for_handler = degraded_input.clone();
    bus.subscribe::<ScreenRuntimeTestTrigger, _, _>(
        EventSubscriber::new(
            SubscriberId::parse("screen-runtime-test-trigger-subscriber")
                .expect_value("screen trigger subscriber parses"),
            EventType::parse("eventing.screen-runtime.test-trigger")
                .expect_value("screen trigger event type parses"),
            TargetHandler::parse("screen-runtime-test-trigger-handler")
                .expect_value("screen trigger target parses"),
        ),
        move |context| {
            let target = target.clone();
            let captured = Arc::clone(&captured_for_handler);
            let input = input_for_handler.clone();
            let degraded_input = degraded_input_for_handler.clone();
            async move {
                let report = if degraded {
                    publish_screen_degraded_event_chain_for_input(
                        context.publisher(),
                        &target,
                        degraded_input,
                        constants::activity_store::TEST_FIRST_OBSERVED_AT,
                    )
                    .await
                } else {
                    publish_screen_runtime_chain_for_input(
                        context.publisher(),
                        &target,
                        input,
                        constants::activity_store::TEST_FIRST_OBSERVED_AT,
                    )
                    .await
                };
                *captured.lock().expect_value("screen report lock") = Some(report);
                Ok(())
            }
        },
    )
    .await
    .expect_value("screen trigger subscriber registers");
    bus.publish(ScreenRuntimeTestTrigger, screen_runtime_test_metadata())
        .await
        .expect_value("screen runtime trigger publishes");
    captured
        .lock()
        .expect_value("screen report lock")
        .take()
        .expect_value("screen runtime report captured")
        .expect_value(constants::screen_flow::ERROR_SCREEN_RUNTIME_CHAIN_PUBLISHES)
}

async fn register_screen_runtime_test_owners(bus: &RootEventPublisher) {
    let routes = [
        (
            constants::screen_flow::SUBSCRIBER_SCREEN_CAPTURE_OBSERVER,
            constants::screen_flow::EVENT_SCREEN_CAPTURE_OBSERVED,
            constants::screen_flow::TARGET_SCREEN_CAPTURE_OBSERVER,
        ),
        (
            constants::screen_flow::SUBSCRIBER_SCREEN_QUEUE_WRITER,
            constants::screen_flow::EVENT_SCREEN_QUEUE_ENCRYPTED,
            constants::screen_flow::TARGET_SCREEN_QUEUE_WRITER,
        ),
        (
            constants::screen_flow::SUBSCRIBER_SCREEN_AI_REQUEST,
            constants::screen_flow::EVENT_SCREEN_AI_ANALYSIS_REQUESTED,
            constants::screen_flow::TARGET_SCREEN_AI_ANALYZER,
        ),
        (
            constants::screen_flow::SUBSCRIBER_SCREEN_AI_COMPLETE,
            constants::screen_flow::EVENT_SCREEN_AI_ANALYSIS_COMPLETED,
            constants::screen_flow::TARGET_SCREEN_SUMMARY_WRITER,
        ),
        (
            constants::screen_flow::SUBSCRIBER_SCREEN_SUMMARY_WRITER,
            constants::screen_flow::EVENT_SCREEN_SUMMARY_COMMITTED,
            constants::screen_flow::TARGET_SCREEN_SUMMARY_WRITER,
        ),
        (
            constants::screen_flow::SUBSCRIBER_SCREEN_POLICY_DECISION,
            constants::screen_flow::EVENT_SCREEN_POLICY_DECISION_COMPLETED,
            constants::screen_flow::TARGET_SCREEN_POLICY_ENGINE,
        ),
        (
            constants::screen_flow::SUBSCRIBER_SCREEN_ACTION_DRY_RUN,
            constants::screen_flow::EVENT_SCREEN_ACTION_DRY_RUN_RECORDED,
            constants::screen_flow::TARGET_SCREEN_ACTION_DRY_RUN,
        ),
        (
            constants::screen_flow::SUBSCRIBER_SCREEN_DELETION_WORKER,
            constants::screen_flow::EVENT_SCREEN_DELETION_COMMITTED,
            constants::screen_flow::TARGET_SCREEN_DELETION_WORKER,
        ),
        (
            constants::screen_flow::SUBSCRIBER_SCREEN_PORTAL_READ_MODEL,
            constants::screen_flow::EVENT_SCREEN_PORTAL_READ_MODEL_UPDATED,
            constants::screen_flow::TARGET_SCREEN_PORTAL_READ_MODEL,
        ),
    ];
    for (subscriber_id, event_type, target_handler) in routes {
        bus.subscribe::<ScreenRuntimeEventPayload, _, _>(
            EventSubscriber::new(
                SubscriberId::parse(subscriber_id).expect_value("screen owner id parses"),
                EventType::parse(event_type).expect_value("screen owner event type parses"),
                TargetHandler::parse(target_handler).expect_value("screen owner target parses"),
            ),
            screen_runtime_test_owner_handler,
        )
        .await
        .expect_value("screen owner subscriber registers");
    }
}

async fn screen_runtime_test_owner_handler(
    context: EventContext<ScreenRuntimeEventPayload>,
) -> Result<(), EventingError> {
    let payload = context.payload();
    if payload.claim_boundary.raw_image_available_to_ai_provider
        || payload.claim_boundary.raw_image_available_to_policy
        || payload.claim_boundary.raw_image_available_to_portal
    {
        return Err(EventingError::InvalidValue {
            field: "screen_runtime_claim_boundary",
            value: "raw image escape",
        });
    }
    Ok(())
}

fn screen_runtime_test_metadata() -> EventMetadata {
    EventMetadata::from_parts(
        EventId::parse("screen-runtime-test-trigger-event-1")
            .expect_value("screen trigger event id parses"),
        CorrelationId::parse("screen-runtime-test-correlation-1")
            .expect_value("screen trigger correlation parses"),
        EventSource::new(
            EventCustody::parse("local-only").expect_value("screen trigger custody parses"),
            RuntimeRole::parse("agent").expect_value("screen trigger role parses"),
            SourceService::parse("agent-core-test").expect_value("screen trigger service parses"),
            SourceComponent::parse("screen-runtime-test")
                .expect_value("screen trigger component parses"),
            RuntimeInstanceId::parse("screen-runtime-test-instance")
                .expect_value("screen trigger instance parses"),
        ),
        RecordedAt::parse(constants::activity_store::TEST_FIRST_OBSERVED_AT)
            .expect_value("screen trigger observed time parses"),
        Some(
            TargetHandler::parse("screen-runtime-test-trigger-handler")
                .expect_value("screen trigger target parses"),
        ),
    )
}

fn screen_deletion_journal_path(suffix: &str) -> std::path::PathBuf {
    let sequence = SCREEN_EVENT_RUNTIME_TEST_SEQUENCE.fetch_add(1, Ordering::Relaxed);
    std::env::temp_dir().join(format!(
        "ocentra-screen-deletion-{suffix}-{}-{sequence}.ndjson",
        std::process::id()
    ))
}
