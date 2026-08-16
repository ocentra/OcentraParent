use ocentra_parent_agent_protocol::activity_capture::ActivityCaptureCapabilityStatus;
use ocentra_parent_screen_capture_adapter::{
    trigger_scheduler::{
        ScreenCaptureScheduleDecision, ScreenCaptureScheduleTrigger, ScreenCaptureSuppressionReason,
    },
    CapturedScreenImage, ScreenCaptureAttempt, ScreenCaptureMetadata, ScreenCaptureScope,
};
use ocentra_screen_core::runtime_decision::{
    evaluate_screen_runtime, screen_runtime_decision_recorded_event,
    screen_runtime_input_from_capture, screen_runtime_observed_event, ScreenAggregateId,
    ScreenAiHandoffState, ScreenContentSignalState, ScreenPolicyHandoffState,
    ScreenRuntimeActionState, ScreenRuntimeDecisionId,
};
use ocentra_screen_core::{
    screen_ai_analysis_requested_event, screen_evidence_recorded_event,
    screen_policy_evaluation_requested_event, ScreenObservationIntent,
};

#[derive(Debug)]
struct TestError(String);

impl std::fmt::Display for TestError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(&self.0)
    }
}

impl std::error::Error for TestError {}

#[test]
fn suppressed_capture_stays_idle_without_handoffs() {
    let input = screen_runtime_input_from_capture(
        ScreenCaptureScheduleDecision::SuppressCapture {
            reason: ScreenCaptureSuppressionReason::DisabledByParent,
        },
        None,
        ScreenContentSignalState::AmbiguousContent,
    );

    let decision = evaluate_screen_runtime(&input);
    let observed = screen_runtime_observed_event(&input);
    let evidence = screen_evidence_recorded_event(&observed);

    assert_eq!(
        decision.observation_intent,
        ScreenObservationIntent::IdleObservationOnly
    );
    assert_eq!(
        decision.runtime_action_state,
        ScreenRuntimeActionState::SuppressCapture
    );
    assert_eq!(decision.ai_handoff_state, ScreenAiHandoffState::NotRequired);
    assert_eq!(
        decision.policy_handoff_state,
        ScreenPolicyHandoffState::DoNotPublish
    );
    assert_eq!(screen_ai_analysis_requested_event(&evidence), None);
    assert_eq!(screen_policy_evaluation_requested_event(&evidence), None);
}

#[test]
fn degraded_capture_requires_policy_without_ai() -> Result<(), TestError> {
    let degraded = ScreenCaptureAttempt::Degraded(ScreenCaptureMetadata {
        status: ActivityCaptureCapabilityStatus::AccessDenied,
        scope: ScreenCaptureScope::ActiveWindow,
        pid: None,
        app_name: None,
        title: None,
        window_id: None,
        monitor_id: None,
        monitor_name: None,
    });
    let input = screen_runtime_input_from_capture(
        ScreenCaptureScheduleDecision::EnqueueCapture {
            reason: ScreenCaptureScheduleTrigger::PolicyAmbiguity,
            scope: ScreenCaptureScope::ActiveWindow,
        },
        Some(&degraded),
        ScreenContentSignalState::KnownPolicyState,
    );

    let decision = evaluate_screen_runtime(&input);
    let observed = screen_runtime_observed_event(&input);
    let evidence = screen_evidence_recorded_event(&observed);

    assert_eq!(
        decision.observation_intent,
        ScreenObservationIntent::CaptureCapabilityRequiresPolicy
    );
    assert_eq!(
        decision.runtime_action_state,
        ScreenRuntimeActionState::RecordDegradedCapture
    );
    assert_eq!(screen_ai_analysis_requested_event(&evidence), None);
    let policy_request = screen_policy_evaluation_requested_event(&evidence).ok_or_else(|| {
        TestError(String::from(
            "degraded capture should publish policy evidence",
        ))
    })?;
    assert_eq!(policy_request.evidence_refs, vec![evidence.evidence_ref]);
    Ok(())
}

#[test]
fn available_ambiguous_capture_routes_to_ai_boundary() -> Result<(), TestError> {
    let captured = captured_attempt();
    let input = screen_runtime_input_from_capture(
        ScreenCaptureScheduleDecision::EnqueueCapture {
            reason: ScreenCaptureScheduleTrigger::ManagedBrowserUrlChange,
            scope: ScreenCaptureScope::ActiveWindow,
        },
        Some(&captured),
        ScreenContentSignalState::AmbiguousContent,
    );

    let decision = evaluate_screen_runtime(&input);
    let observed = screen_runtime_observed_event(&input);
    let evidence = screen_evidence_recorded_event(&observed);

    assert_eq!(
        decision.observation_intent,
        ScreenObservationIntent::AmbiguousContentRequiresAi
    );
    assert_eq!(
        decision.runtime_action_state,
        ScreenRuntimeActionState::RecordCapturedEvidence
    );
    assert_eq!(screen_policy_evaluation_requested_event(&evidence), None);
    let ai_request = screen_ai_analysis_requested_event(&evidence)
        .ok_or_else(|| TestError(String::from("ambiguous capture should request AI")))?;
    assert_eq!(ai_request.evidence_refs, vec![evidence.evidence_ref]);
    Ok(())
}

#[test]
fn available_known_policy_capture_publishes_policy_without_ai() -> Result<(), TestError> {
    let captured = captured_attempt();
    let input = screen_runtime_input_from_capture(
        ScreenCaptureScheduleDecision::EnqueueCapture {
            reason: ScreenCaptureScheduleTrigger::NativeAppForegroundStart,
            scope: ScreenCaptureScope::ActiveWindow,
        },
        Some(&captured),
        ScreenContentSignalState::KnownPolicyState,
    );

    let decision = evaluate_screen_runtime(&input);
    let observed = screen_runtime_observed_event(&input);
    let evidence = screen_evidence_recorded_event(&observed);

    assert_eq!(
        decision.observation_intent,
        ScreenObservationIntent::KnownPolicyStateRequiresPolicy
    );
    assert_eq!(screen_ai_analysis_requested_event(&evidence), None);
    let policy_request = screen_policy_evaluation_requested_event(&evidence).ok_or_else(|| {
        TestError(String::from(
            "known policy capture should publish policy evidence",
        ))
    })?;
    assert_eq!(policy_request.evidence_refs, vec![evidence.evidence_ref]);
    Ok(())
}

#[test]
fn runtime_decision_recorded_event_preserves_input_and_decision() -> Result<(), TestError> {
    let input = screen_runtime_input_from_capture(
        ScreenCaptureScheduleDecision::EnqueueCapture {
            reason: ScreenCaptureScheduleTrigger::TimedCadence,
            scope: ScreenCaptureScope::PrimaryDisplay,
        },
        Some(&captured_attempt()),
        ScreenContentSignalState::ObservationOnly,
    );
    let aggregate_id = ScreenAggregateId::parse("screen.aggregate.1")
        .map_err(|error| TestError(format!("aggregate id should parse: {error}")))?;
    let decision_id = ScreenRuntimeDecisionId::parse("screen.runtime-decision.1")
        .map_err(|error| TestError(format!("decision id should parse: {error}")))?;
    let event = screen_runtime_decision_recorded_event(aggregate_id, decision_id, &input);

    assert_eq!(
        event.decision.runtime_action_state,
        ScreenRuntimeActionState::RecordCapturedEvidence
    );
    assert_eq!(
        event.decision.ai_handoff_state,
        ScreenAiHandoffState::NotRequired
    );
    assert_eq!(
        event.decision.policy_handoff_state,
        ScreenPolicyHandoffState::DoNotPublish
    );
    Ok(())
}

fn captured_attempt() -> ScreenCaptureAttempt {
    ScreenCaptureAttempt::Captured(CapturedScreenImage {
        metadata: ScreenCaptureMetadata {
            status: ActivityCaptureCapabilityStatus::Available,
            scope: ScreenCaptureScope::ActiveWindow,
            pid: Some(42),
            app_name: Some(String::from("Portal")),
            title: Some(String::from("Ocentra")),
            window_id: Some(7),
            monitor_id: Some(1),
            monitor_name: Some(String::from("Primary")),
        },
        width: 1,
        height: 1,
        png_bytes: vec![1],
    })
}
