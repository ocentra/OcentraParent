use ocentra_eventing::{
    bus::reports::handler::HandlerOutcome, bus::EventBus, error::EventingError,
};
use ocentra_parent_agent_protocol::activity::ActivityEvidenceRef;
use ocentra_parent_agent_protocol::activity_surface::ActivityReadModelState;
use ocentra_parent_agent_protocol::activity_surface::ActivityScreenReadModelRow;
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::screen_evidence::SCREEN_CAPABILITY_READY;
use ocentra_parent_agent_protocol::screen_evidence::SCREEN_CAPTURE_SCOPE_ACTIVE_WINDOW;
use ocentra_parent_agent_protocol::screen_evidence::SCREEN_CATEGORY_SCHOOL;
use ocentra_parent_agent_protocol::screen_evidence::SCREEN_CUSTODY_JOURNAL;
use ocentra_parent_agent_protocol::screen_evidence::SCREEN_DELETION_DELETED;
use ocentra_parent_agent_protocol::screen_evidence::SCREEN_POLICY_CONFIDENCE_READY;
use ocentra_parent_agent_protocol::screen_evidence::SCREEN_PROVIDER_LOCAL_OCR;
use ocentra_parent_agent_protocol::screen_evidence::SCREEN_PROVIDER_LOCAL_VISION;

use super::screen_ai_service_event_bridge::ScreenAiServiceEventBridgeError;
use super::screen_ai_service_event_subscription::{
    publish_screen_service_row_ready_event, subscribe_screen_service_row_ready_events,
    ScreenAiServiceEventRuntime, ScreenAiServiceEventSubscriptionDispatch,
    ScreenAiServiceEventSubscriptionState, ScreenAiServiceRowReadyEvent,
};
use crate::screen_ai_service_event_subscription;
use crate::test_invariants::require_ok;

#[tokio::test]
async fn screen_service_event_runtime_start_registers_subscriber_for_production_startup() {
    let runtime = require_ok(
        ScreenAiServiceEventRuntime::start().await,
        constants::screen_flow::ERROR_SCREEN_SERVICE_EVENT_SUBSCRIBES,
    );

    let publish = runtime
        .publish_row_ready(
            service_screen_row(),
            screen_ai_service_event_subscription::ActionRefText(
                constants::screen_flow::TEST_SCREEN_ACTION_REF.to_string(),
            ),
            screen_ai_service_event_subscription::ObservedAtText(
                constants::activity_store::TEST_FIRST_OBSERVED_AT.to_string(),
            ),
        )
        .await;
    let error = publish.expect_err("row-ready must stop before root publication");
    assert_eq!(
        error,
        EventingError::InvalidValue {
            field: constants::screen_flow::FIELD_SCREEN_SERVICE_ROW_READY,
            value: constants::screen_flow::ERROR_SCREEN_RUNTIME_OWNER_UNAVAILABLE_MANUAL_REQUIRED
                .to_string(),
        }
    );
    let metrics = runtime.event_metrics_snapshot().await;
    assert_eq!(metrics.stored_event_count, 0);
    assert_eq!(metrics.dead_letter_count, 0);
    assert_eq!(metrics.queue.queued_event_count, 0);
    assert_eq!(metrics.queue.in_flight_event_id_count, 0);
}

#[tokio::test]
async fn screen_service_event_subscription_rejects_without_runtime_owner() {
    let bus = EventBus::new();
    let state = ScreenAiServiceEventSubscriptionState::default();
    let subscription = subscribe_screen_service_row_ready_events(&bus, state.clone()).await;
    require_ok(
        subscription,
        constants::screen_flow::ERROR_SCREEN_SERVICE_EVENT_SUBSCRIBES,
    );

    let publish = publish_screen_service_row_ready_event(
        &bus,
        ScreenAiServiceRowReadyEvent::new(
            service_screen_row(),
            screen_ai_service_event_subscription::ActionRefText(
                constants::screen_flow::TEST_SCREEN_ACTION_REF.to_string(),
            ),
        ),
        screen_ai_service_event_subscription::ObservedAtText(
            constants::activity_store::TEST_FIRST_OBSERVED_AT.to_string(),
        ),
    )
    .await;
    let publish = require_ok(
        publish,
        constants::screen_flow::ERROR_SCREEN_SERVICE_EVENT_SUBSCRIBER_PUBLISHES,
    );

    assert_eq!(publish.handler_reports.len(), 1);
    assert_eq!(publish.handler_reports[0].outcome, HandlerOutcome::Failed);
    assert_eq!(
        dispatches(&state),
        vec![ScreenAiServiceEventSubscriptionDispatch::Rejected {
            queue_job_id: constants::activity_store::TEST_SCREEN_QUEUE_JOB_ID.to_string(),
            screen_analysis_result_id: constants::activity_store::TEST_SCREEN_RESULT_ID.to_string(),
            reason: ScreenAiServiceEventBridgeError::RuntimeOwnerUnavailable,
        }]
    );
}

#[tokio::test]
async fn screen_service_event_subscription_rejects_degraded_without_runtime_owner() {
    let bus = EventBus::new();
    let state = ScreenAiServiceEventSubscriptionState::default();
    let subscription = subscribe_screen_service_row_ready_events(&bus, state.clone()).await;
    require_ok(
        subscription,
        constants::screen_flow::ERROR_SCREEN_SERVICE_EVENT_SUBSCRIBES,
    );

    let publish = publish_screen_service_row_ready_event(
        &bus,
        ScreenAiServiceRowReadyEvent::new(
            degraded_service_screen_row(),
            screen_ai_service_event_subscription::ActionRefText(
                constants::screen_flow::TEST_SCREEN_ACTION_REF.to_string(),
            ),
        ),
        screen_ai_service_event_subscription::ObservedAtText(
            constants::activity_store::TEST_FIRST_OBSERVED_AT.to_string(),
        ),
    )
    .await;
    let publish = require_ok(
        publish,
        constants::screen_flow::ERROR_SCREEN_SERVICE_EVENT_SUBSCRIBER_PUBLISHES,
    );

    assert_eq!(publish.handler_reports.len(), 1);
    assert_eq!(publish.handler_reports[0].outcome, HandlerOutcome::Failed);
    assert_eq!(
        dispatches(&state),
        vec![ScreenAiServiceEventSubscriptionDispatch::Rejected {
            queue_job_id: constants::activity_store::TEST_SCREEN_QUEUE_JOB_ID.to_string(),
            screen_analysis_result_id: constants::activity_store::TEST_SCREEN_RESULT_ID.to_string(),
            reason: ScreenAiServiceEventBridgeError::RuntimeOwnerUnavailable,
        }]
    );
}

#[tokio::test]
async fn screen_service_event_subscription_rejects_unsafe_rows_before_downstream_publish() {
    let bus = EventBus::new();
    let state = ScreenAiServiceEventSubscriptionState::default();
    let subscription = subscribe_screen_service_row_ready_events(&bus, state.clone()).await;
    require_ok(
        subscription,
        constants::screen_flow::ERROR_SCREEN_SERVICE_EVENT_SUBSCRIBES,
    );

    let mut row = service_screen_row();
    row.raw_image_retained = true;
    let publish = publish_screen_service_row_ready_event(
        &bus,
        ScreenAiServiceRowReadyEvent::new(
            row,
            screen_ai_service_event_subscription::ActionRefText(
                constants::screen_flow::TEST_SCREEN_ACTION_REF.to_string(),
            ),
        ),
        screen_ai_service_event_subscription::ObservedAtText(
            constants::activity_store::TEST_FIRST_OBSERVED_AT.to_string(),
        ),
    )
    .await;
    let publish = require_ok(
        publish,
        constants::screen_flow::ERROR_SCREEN_SERVICE_EVENT_SUBSCRIBER_REJECTS,
    );

    assert_eq!(publish.handler_reports.len(), 1);
    assert_eq!(publish.handler_reports[0].outcome, HandlerOutcome::Failed);
    assert_eq!(
        dispatches(&state),
        vec![ScreenAiServiceEventSubscriptionDispatch::Rejected {
            queue_job_id: constants::activity_store::TEST_SCREEN_QUEUE_JOB_ID.to_string(),
            screen_analysis_result_id: constants::activity_store::TEST_SCREEN_RESULT_ID.to_string(),
            reason: ScreenAiServiceEventBridgeError::RawImageRetained,
        }]
    );
}

fn dispatches(
    state: &ScreenAiServiceEventSubscriptionState,
) -> Vec<ScreenAiServiceEventSubscriptionDispatch> {
    state
        .dispatches
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
        .clone()
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
