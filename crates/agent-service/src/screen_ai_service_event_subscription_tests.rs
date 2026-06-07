use ocentra_eventing::{EventBus, HandlerOutcome};
use ocentra_parent_agent_core::ScreenRuntimePhase;
use ocentra_parent_agent_protocol::{
    constants, ActivityEvidenceRef, ActivityReadModelState, ActivityScreenReadModelRow,
    SCREEN_CAPABILITY_READY, SCREEN_CAPTURE_SCOPE_ACTIVE_WINDOW, SCREEN_CATEGORY_SCHOOL,
    SCREEN_CUSTODY_JOURNAL, SCREEN_DELETION_DELETED, SCREEN_POLICY_CONFIDENCE_READY,
    SCREEN_PROVIDER_LOCAL_OCR,
};

use super::screen_ai_service_event_bridge::ScreenAiServiceEventBridgeError;
use super::screen_ai_service_event_subscription::{
    publish_screen_service_row_ready_event, subscribe_screen_service_row_ready_events,
    ScreenAiServiceEventSubscriptionDispatch, ScreenAiServiceEventSubscriptionState,
    ScreenAiServiceRowReadyEvent,
};

#[tokio::test]
async fn screen_service_event_subscription_publishes_existing_runtime_chain() {
    let bus = EventBus::new();
    let state = ScreenAiServiceEventSubscriptionState::default();
    subscribe_screen_service_row_ready_events(&bus, state.clone())
        .await
        .expect(constants::screen_flow::ERROR_SCREEN_SERVICE_EVENT_SUBSCRIBES);

    let publish = publish_screen_service_row_ready_event(
        &bus,
        ScreenAiServiceRowReadyEvent::new(
            service_screen_row(),
            constants::screen_flow::TEST_SCREEN_ACTION_REF,
        ),
        constants::activity_store::TEST_FIRST_OBSERVED_AT,
    )
    .await
    .expect(constants::screen_flow::ERROR_SCREEN_SERVICE_EVENT_SUBSCRIBER_PUBLISHES);

    assert_eq!(publish.handler_reports.len(), 1);
    assert_eq!(publish.handler_reports[0].outcome, HandlerOutcome::Handled);
    assert_eq!(
        state.dispatches(),
        vec![ScreenAiServiceEventSubscriptionDispatch::Published {
            queue_job_id: constants::activity_store::TEST_SCREEN_QUEUE_JOB_ID.to_string(),
            screen_analysis_result_id: constants::activity_store::TEST_SCREEN_RESULT_ID.to_string(),
            downstream_event_count: ScreenRuntimePhase::ordered_chain().len(),
            raw_image_escaped: false,
        }]
    );
}

#[tokio::test]
async fn screen_service_event_subscription_rejects_unsafe_rows_before_downstream_publish() {
    let bus = EventBus::new();
    let state = ScreenAiServiceEventSubscriptionState::default();
    subscribe_screen_service_row_ready_events(&bus, state.clone())
        .await
        .expect(constants::screen_flow::ERROR_SCREEN_SERVICE_EVENT_SUBSCRIBES);

    let mut row = service_screen_row();
    row.raw_image_retained = true;
    let publish = publish_screen_service_row_ready_event(
        &bus,
        ScreenAiServiceRowReadyEvent::new(row, constants::screen_flow::TEST_SCREEN_ACTION_REF),
        constants::activity_store::TEST_FIRST_OBSERVED_AT,
    )
    .await
    .expect(constants::screen_flow::ERROR_SCREEN_SERVICE_EVENT_SUBSCRIBER_REJECTS);

    assert_eq!(publish.handler_reports.len(), 1);
    assert_eq!(publish.handler_reports[0].outcome, HandlerOutcome::Failed);
    assert_eq!(
        state.dispatches(),
        vec![ScreenAiServiceEventSubscriptionDispatch::Rejected {
            queue_job_id: constants::activity_store::TEST_SCREEN_QUEUE_JOB_ID.to_string(),
            screen_analysis_result_id: constants::activity_store::TEST_SCREEN_RESULT_ID.to_string(),
            reason: ScreenAiServiceEventBridgeError::RawImageRetained,
        }]
    );
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
