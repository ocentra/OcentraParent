use std::{
    fs,
    sync::atomic::{AtomicU64, Ordering},
};

use ocentra_eventing::journal::ndjson::{NdjsonEventJournal, NdjsonJournalOptions};
use ocentra_eventing::replay::ReplayFilter;
use ocentra_parent_agent_protocol::activity::ActivityEvidenceRef;
use ocentra_parent_agent_protocol::activity_surface::ActivityReadModelState;
use ocentra_parent_agent_protocol::activity_surface::ActivityScreenReadModelRow;
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::screen_evidence::ScreenAiAuditState;
use ocentra_parent_agent_protocol::screen_evidence::ScreenRuntimeEventPayload;
use ocentra_parent_agent_protocol::screen_evidence::ScreenRuntimePhase;
use ocentra_parent_agent_protocol::screen_evidence::SCREEN_CAPABILITY_READY;
use ocentra_parent_agent_protocol::screen_evidence::SCREEN_CAPTURE_SCOPE_ACTIVE_WINDOW;
use ocentra_parent_agent_protocol::screen_evidence::SCREEN_CATEGORY_SCHOOL;
use ocentra_parent_agent_protocol::screen_evidence::SCREEN_CUSTODY_JOURNAL;
use ocentra_parent_agent_protocol::screen_evidence::SCREEN_DELETION_DELETED;
use ocentra_parent_agent_protocol::screen_evidence::SCREEN_POLICY_CONFIDENCE_READY;
use ocentra_parent_agent_protocol::screen_evidence::SCREEN_PROVIDER_LOCAL_OCR;
use ocentra_parent_agent_protocol::screen_evidence::SCREEN_PROVIDER_LOCAL_VISION;

use super::screen_ai_service_event_bridge::{
    publish_screen_capture_queue_event_chain, publish_screen_degraded_event_chain,
    publish_screen_service_row_event_chain, screen_runtime_capture_input_from_service_row,
    screen_runtime_degraded_input_from_service_row, screen_runtime_deletion_input_from_service_row,
    screen_runtime_input_from_service_row, ScreenAiServiceEventBridgeError,
    ScreenAiServiceEventBridgeRefs,
};
use super::screen_ai_service_event_subscription::{
    ActionRefText, ObservedAtText, ScreenAiServiceEventRuntime,
};
use crate::test_invariants::require_ok;

static SCREEN_SERVICE_EVENT_TEST_SEQUENCE: AtomicU64 = AtomicU64::new(0);

#[test]
fn screen_service_event_bridge_maps_service_row_to_existing_screen_runtime_input() {
    let input = require_ok(
        screen_runtime_input_from_service_row(service_screen_row(), service_bridge_refs()),
        constants::screen_flow::ERROR_SCREEN_SERVICE_EVENT_BRIDGE_MAPS,
    );

    assert_eq!(
        input.queue_job_id,
        constants::activity_store::TEST_SCREEN_QUEUE_JOB_ID
    );
    assert_eq!(
        input.screen_analysis_result_id,
        constants::activity_store::TEST_SCREEN_RESULT_ID
    );
    assert_eq!(
        input.model_runtime_ref,
        constants::activity_store::TEST_SCREEN_MODEL_RUNTIME_REF
    );
    assert_eq!(
        input.model_id,
        constants::activity_store::TEST_SCREEN_MODEL_ID
    );
    assert_eq!(
        input.prompt_or_template_version,
        constants::activity_store::TEST_SCREEN_TEMPLATE_VERSION
    );
    assert_eq!(
        input.policy_decision_ref,
        constants::activity_store::TEST_POLICY_DECISION_ID
    );
    assert_eq!(
        input.parent_rule_ref,
        constants::screen_flow::TEST_SCREEN_POLICY_RULE_REF
    );
    assert_eq!(
        input.portal_read_model_ref,
        constants::activity_store::TEST_SCREEN_RESULT_ID
    );
}

#[tokio::test]
async fn screen_service_event_bridge_publishes_ordered_chain_from_service_read_model_row() {
    let report = publish_screen_service_row_event_chain(
        service_screen_row(),
        ObservedAtText(constants::activity_store::TEST_FIRST_OBSERVED_AT.to_string()),
        service_bridge_refs(),
    )
    .await;
    let report = require_ok(
        report,
        constants::screen_flow::ERROR_SCREEN_SERVICE_EVENT_BRIDGE_PUBLISHES,
    );
    let phases = report
        .stored_events
        .iter()
        .map(|event| {
            require_ok(
                event.decode::<ScreenRuntimeEventPayload>(),
                constants::screen_flow::ERROR_SCREEN_RUNTIME_PAYLOAD_DECODES,
            )
            .payload
            .phase
        })
        .collect::<Vec<_>>();

    assert_eq!(phases, ScreenRuntimePhase::ordered_chain());
    assert_eq!(report.publish_reports.len(), phases.len());
    assert_eq!(report.dead_letters.len(), 0);
    assert!(!report.raw_image_escaped());
}

#[tokio::test]
async fn screen_service_event_bridge_publishes_capture_queue_events_from_capture_row() {
    let report = publish_screen_capture_queue_event_chain(
        service_screen_row(),
        ObservedAtText(constants::activity_store::TEST_FIRST_OBSERVED_AT.to_string()),
    )
    .await;
    let report = require_ok(
        report,
        constants::screen_flow::ERROR_SCREEN_SERVICE_EVENT_BRIDGE_PUBLISHES,
    );
    let phases = report
        .stored_events
        .iter()
        .map(|event| {
            require_ok(
                event.decode::<ScreenRuntimeEventPayload>(),
                constants::screen_flow::ERROR_SCREEN_RUNTIME_PAYLOAD_DECODES,
            )
            .payload
            .phase
        })
        .collect::<Vec<_>>();

    assert_eq!(
        phases,
        vec![
            ScreenRuntimePhase::CaptureObserved,
            ScreenRuntimePhase::QueueEncrypted
        ]
    );
    assert_eq!(report.publish_reports.len(), phases.len());
    assert_eq!(report.dead_letters.len(), 0);
    assert!(!report.raw_image_escaped());
}

#[tokio::test]
async fn screen_service_event_bridge_publishes_deletion_event_from_retention_row() {
    let journal_path = screen_deletion_journal_path("retention-row");
    let _ = fs::remove_file(&journal_path);
    let runtime = require_ok(
        ScreenAiServiceEventRuntime::start().await,
        constants::screen_flow::ERROR_SCREEN_SERVICE_EVENT_BRIDGE_PUBLISHES,
    );
    let report = runtime
        .publish_deletion_row(
            service_screen_row(),
            ObservedAtText(constants::activity_store::TEST_FIRST_OBSERVED_AT.to_string()),
            &journal_path,
        )
        .await;
    let report = require_ok(
        report,
        constants::screen_flow::ERROR_SCREEN_SERVICE_EVENT_BRIDGE_PUBLISHES,
    );
    let payload = require_ok(
        report.stored_events[0].decode::<ScreenRuntimeEventPayload>(),
        constants::screen_flow::ERROR_SCREEN_RUNTIME_PAYLOAD_DECODES,
    )
    .payload;
    let _ = fs::remove_file(&journal_path);

    assert_eq!(payload.phase, ScreenRuntimePhase::DeletionCommitted);
    assert_eq!(payload.policy_decision_ref, None);
    assert_eq!(payload.action_ref, None);
    assert_eq!(
        payload.deletion_proof_ref,
        Some(constants::activity_store::TEST_SCREEN_DELETION_REASONS.to_string())
    );
    assert_eq!(report.publish_reports.len(), 1);
    assert_eq!(report.dead_letters.len(), 0);
    assert!(!report.raw_image_escaped());
}

#[tokio::test]
async fn screen_service_event_runtime_bounds_each_deletion_publication_report() {
    let journal_path = screen_deletion_journal_path("bounded-report");
    let _ = fs::remove_file(&journal_path);
    let runtime = require_ok(
        ScreenAiServiceEventRuntime::start().await,
        constants::screen_flow::ERROR_SCREEN_SERVICE_EVENT_BRIDGE_PUBLISHES,
    );
    let first = require_ok(
        runtime
            .publish_deletion_row(
                service_screen_row(),
                ObservedAtText(constants::activity_store::TEST_FIRST_OBSERVED_AT.to_string()),
                &journal_path,
            )
            .await,
        constants::screen_flow::ERROR_SCREEN_SERVICE_EVENT_BRIDGE_PUBLISHES,
    );
    let mut second_row = service_screen_row();
    second_row.queue_job_id.push_str("-second");
    second_row.row_id.push_str("-second");
    let second = require_ok(
        runtime
            .publish_deletion_row(
                second_row,
                ObservedAtText(constants::activity_store::TEST_SECOND_OBSERVED_AT.to_string()),
                &journal_path,
            )
            .await,
        constants::screen_flow::ERROR_SCREEN_SERVICE_EVENT_BRIDGE_PUBLISHES,
    );

    assert_eq!(first.publish_reports.len(), 1);
    assert_eq!(first.stored_events.len(), 1);
    assert_eq!(second.publish_reports.len(), 1);
    assert_eq!(second.stored_events.len(), 1);
    assert!(!second.raw_image_escaped());
    let _ = fs::remove_file(&journal_path);
}

#[tokio::test]
async fn screen_service_event_runtime_bounds_memory_while_durable_journal_retains_proof() {
    let journal_path = screen_deletion_journal_path("bounded-memory");
    let _ = fs::remove_file(&journal_path);
    let runtime = require_ok(
        ScreenAiServiceEventRuntime::start().await,
        constants::screen_flow::ERROR_SCREEN_SERVICE_EVENT_BRIDGE_PUBLISHES,
    );
    let first = require_ok(
        runtime
            .publish_deletion_row(
                service_screen_row(),
                ObservedAtText(constants::activity_store::TEST_FIRST_OBSERVED_AT.to_string()),
                &journal_path,
            )
            .await,
        constants::screen_flow::ERROR_SCREEN_SERVICE_EVENT_BRIDGE_PUBLISHES,
    );
    let mut second_row = service_screen_row();
    second_row.queue_job_id.push_str("-retained");
    second_row.row_id.push_str("-retained");
    let second = require_ok(
        runtime
            .publish_deletion_row(
                second_row,
                ObservedAtText(constants::activity_store::TEST_SECOND_OBSERVED_AT.to_string()),
                &journal_path,
            )
            .await,
        constants::screen_flow::ERROR_SCREEN_SERVICE_EVENT_BRIDGE_PUBLISHES,
    );

    let replay = require_ok(
        NdjsonEventJournal::with_options(&journal_path, NdjsonJournalOptions::hash_chain())
            .replay_projection(ReplayFilter::all())
            .await,
        constants::screen_flow::ERROR_SCREEN_RUNTIME_CHAIN_PUBLISHES,
    );
    let _ = fs::remove_file(&journal_path);

    assert_eq!(first.stored_events.len(), 1);
    assert_eq!(second.stored_events.len(), 1);
    assert_eq!(replay.records.len(), 2);
}

#[tokio::test]
async fn screen_service_event_runtime_isolates_concurrent_deletion_publication_reports() {
    let journal_path = screen_deletion_journal_path("concurrent");
    let _ = fs::remove_file(&journal_path);
    let runtime = require_ok(
        ScreenAiServiceEventRuntime::start().await,
        constants::screen_flow::ERROR_SCREEN_SERVICE_EVENT_BRIDGE_PUBLISHES,
    );
    let first_row = service_screen_row();
    let first_queue_job_id = first_row.queue_job_id.clone();
    let mut second_row = service_screen_row();
    second_row.queue_job_id.push_str("-concurrent");
    second_row.row_id.push_str("-concurrent");
    let second_queue_job_id = second_row.queue_job_id.clone();

    let (first, second) = tokio::join!(
        runtime.publish_deletion_row(
            first_row,
            ObservedAtText(constants::activity_store::TEST_FIRST_OBSERVED_AT.to_string()),
            &journal_path,
        ),
        runtime.publish_deletion_row(
            second_row,
            ObservedAtText(constants::activity_store::TEST_SECOND_OBSERVED_AT.to_string()),
            &journal_path,
        )
    );
    let first = require_ok(
        first,
        constants::screen_flow::ERROR_SCREEN_SERVICE_EVENT_BRIDGE_PUBLISHES,
    );
    let second = require_ok(
        second,
        constants::screen_flow::ERROR_SCREEN_SERVICE_EVENT_BRIDGE_PUBLISHES,
    );
    let first_payload = require_ok(
        first.stored_events[0].decode::<ScreenRuntimeEventPayload>(),
        constants::screen_flow::ERROR_SCREEN_RUNTIME_PAYLOAD_DECODES,
    )
    .payload;
    let second_payload = require_ok(
        second.stored_events[0].decode::<ScreenRuntimeEventPayload>(),
        constants::screen_flow::ERROR_SCREEN_RUNTIME_PAYLOAD_DECODES,
    )
    .payload;

    assert_eq!(first.stored_events.len(), 1);
    assert_eq!(second.stored_events.len(), 1);
    assert_eq!(first_payload.queue_job_id, first_queue_job_id);
    assert_eq!(second_payload.queue_job_id, second_queue_job_id);
    let _ = fs::remove_file(&journal_path);
}

fn screen_deletion_journal_path(suffix: &str) -> std::path::PathBuf {
    let sequence = SCREEN_SERVICE_EVENT_TEST_SEQUENCE.fetch_add(1, Ordering::Relaxed);
    std::env::temp_dir().join(format!(
        "ocentra-screen-service-deletion-{suffix}-{}-{sequence}.ndjson",
        std::process::id()
    ))
}

#[tokio::test]
async fn screen_service_event_bridge_publishes_degraded_non_ai_event_path() {
    let report = publish_screen_degraded_event_chain(
        degraded_service_screen_row(),
        ObservedAtText(constants::activity_store::TEST_FIRST_OBSERVED_AT.to_string()),
    )
    .await;
    let report = require_ok(
        report,
        constants::screen_flow::ERROR_SCREEN_SERVICE_EVENT_BRIDGE_PUBLISHES,
    );
    let payloads = report
        .stored_events
        .iter()
        .map(|event| {
            require_ok(
                event.decode::<ScreenRuntimeEventPayload>(),
                constants::screen_flow::ERROR_SCREEN_RUNTIME_PAYLOAD_DECODES,
            )
            .payload
        })
        .collect::<Vec<_>>();
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
    assert!(payloads.iter().all(|payload| {
        payload.policy_decision_ref.is_none()
            && payload.policy_action.is_none()
            && payload.parent_rule_ref.is_none()
            && payload.action_ref.is_none()
            && payload.ai_request_ref.is_none()
            && payload.ai_result_ref.is_none()
            && payload.ai_audit_state == ScreenAiAuditState::NotRequested
    }));
    assert_eq!(
        payloads
            .last()
            .and_then(|payload| payload.portal_read_model_ref.clone()),
        Some(constants::activity_store::TEST_SCREEN_RESULT_ID.to_string())
    );
    assert_eq!(report.publish_reports.len(), phases.len());
    assert_eq!(report.dead_letters.len(), 0);
    assert!(!report.raw_image_escaped());
}

#[test]
fn screen_service_event_bridge_rejects_raw_retention_and_missing_policy_refs() {
    let mut raw_retained = service_screen_row();
    raw_retained.raw_image_retained = true;
    assert!(matches!(
        screen_runtime_input_from_service_row(raw_retained, service_bridge_refs()),
        Err(ScreenAiServiceEventBridgeError::RawImageRetained)
    ));
    let mut raw_capture = service_screen_row();
    raw_capture.raw_image_retained = true;
    assert!(matches!(
        screen_runtime_capture_input_from_service_row(raw_capture),
        Err(ScreenAiServiceEventBridgeError::RawImageRetained)
    ));
    let mut raw_deletion = service_screen_row();
    raw_deletion.raw_image_retained = true;
    assert!(matches!(
        screen_runtime_deletion_input_from_service_row(raw_deletion),
        Err(ScreenAiServiceEventBridgeError::RawImageRetained)
    ));

    let mut missing_policy = service_screen_row();
    missing_policy.policy_decision_ref = None;
    assert!(matches!(
        screen_runtime_input_from_service_row(missing_policy, service_bridge_refs()),
        Err(ScreenAiServiceEventBridgeError::MissingPolicyDecision)
    ));

    let mut missing_deletion = service_screen_row();
    missing_deletion.deletion_reasons = Vec::new();
    assert!(matches!(
        screen_runtime_deletion_input_from_service_row(missing_deletion),
        Err(ScreenAiServiceEventBridgeError::MissingDeletionProof)
    ));

    let mut raw_degraded = degraded_service_screen_row();
    raw_degraded.raw_image_retained = true;
    assert!(matches!(
        screen_runtime_degraded_input_from_service_row(raw_degraded),
        Err(ScreenAiServiceEventBridgeError::RawImageRetained)
    ));
}

fn service_screen_row() -> ActivityScreenReadModelRow {
    ActivityScreenReadModelRow {
        row_id: constants::activity_store::TEST_SCREEN_RESULT_ID.to_string(),
        label: constants::activity_store::TEST_SCREEN_SUMMARY.to_string(),
        device_id: constants::activity_surface::DEFAULT_DEVICE_ID.to_string(),
        state: ActivityReadModelState::Ready,
        total_ms: 0,
        foreground_ms: 0,
        background_ms: 0,
        capture_reason: constants::activity_capture::SCREEN_TRIGGER_TIMED_CADENCE.to_string(),
        capture_scope: SCREEN_CAPTURE_SCOPE_ACTIVE_WINDOW.to_string(),
        capability_status: SCREEN_CAPABILITY_READY.to_string(),
        queue_job_id: constants::activity_store::TEST_SCREEN_QUEUE_JOB_ID.to_string(),
        model_runtime_ref: constants::activity_store::TEST_SCREEN_MODEL_RUNTIME_REF.to_string(),
        model_id: constants::activity_store::TEST_SCREEN_MODEL_ID.to_string(),
        provider_kind: SCREEN_PROVIDER_LOCAL_OCR.to_string(),
        prompt_or_template_version: constants::activity_store::TEST_SCREEN_TEMPLATE_VERSION
            .to_string(),
        primary_category: Some(SCREEN_CATEGORY_SCHOOL.to_string()),
        confidence: SCREEN_POLICY_CONFIDENCE_READY,
        image_deletion_state: SCREEN_DELETION_DELETED.to_string(),
        raw_image_retained: false,
        policy_eligible: true,
        image_digest: constants::activity_store::TEST_SCREEN_IMAGE_DIGEST.to_string(),
        custody_state: SCREEN_CUSTODY_JOURNAL.to_string(),
        evidence: Vec::<ActivityEvidenceRef>::new(),
        policy_decision_ref: Some(constants::activity_store::TEST_POLICY_DECISION_ID.to_string()),
        policy_action: Some(constants::activity_store::TEST_POLICY_ACTION_ALLOW.to_string()),
        policy_reason_codes: Vec::new(),
        parent_rule_refs: vec![constants::screen_flow::TEST_SCREEN_POLICY_RULE_REF.to_string()],
        local_model_runtime_refs: vec![
            constants::activity_store::TEST_SCREEN_MODEL_RUNTIME_REF.to_string()
        ],
        parent_explanation_refs: Vec::new(),
        explanation_reasons: Vec::new(),
        deletion_reasons: vec![constants::activity_store::TEST_SCREEN_DELETION_REASONS.to_string()],
        ocr_text_snippets: Vec::new(),
        redaction_notes: Vec::new(),
    }
}

fn service_bridge_refs() -> ScreenAiServiceEventBridgeRefs {
    ScreenAiServiceEventBridgeRefs {
        action_ref: ActionRefText(constants::screen_flow::TEST_SCREEN_ACTION_REF.to_string()),
    }
}

fn degraded_service_screen_row() -> ActivityScreenReadModelRow {
    let mut row = service_screen_row();
    row.capability_status = constants::activity_surface::SAVED_STATE_DEGRADED.to_string();
    row.provider_kind = SCREEN_PROVIDER_LOCAL_VISION.to_string();
    row.primary_category = None;
    row.policy_eligible = false;
    row.policy_decision_ref = None;
    row.policy_action = None;
    row.parent_rule_refs = Vec::new();
    row
}
