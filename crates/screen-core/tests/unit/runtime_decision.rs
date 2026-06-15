use ocentra_parent_agent_protocol::ActivityCaptureCapabilityStatus;
use ocentra_parent_screen_capture_adapter::{
    trigger_scheduler::{
        ScreenCaptureScheduleDecision, ScreenCaptureScheduleTrigger, ScreenCaptureSuppressionReason,
    },
    CapturedScreenImage, ScreenCaptureAttempt, ScreenCaptureMetadata, ScreenCaptureScope,
};
use ocentra_screen_core::{
    screen_ai_analysis_requested_event, screen_evidence_recorded_event,
    screen_policy_evaluation_requested_event, ScreenObservationIntent,
};
use ocentra_screen_core::runtime_decision::{
    evaluate_screen_runtime, screen_runtime_decision_recorded_event,
    screen_runtime_input_from_capture, screen_runtime_observed_event, ScreenAggregateId,
    ScreenAiHandoffState, ScreenContentSignalState, ScreenPolicyHandoffState,
    ScreenRuntimeActionState, ScreenRuntimeDecisionId,
};

#[test]
fn suppressed_capture_stays_idle_without_handoffs() {
    let input = screen_runtime_input_from_capture(
        ScreenCaptureScheduleDecision::SuppressCapture {
            reason: ScreenCaptureSuppressionReason::DisabledByParent,
        },
        None,
        ScreenContentSignalState::AmbiguousContent,
    );

    let decision = evaluate_screen_runtime(input.clone());
    let observed = screen_runtime_observed_event(input);
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
fn degraded_capture_requires_policy_without_ai() {
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

    let decision = evaluate_screen_runtime(input.clone());
    let observed = screen_runtime_observed_event(input);
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
    assert_eq!(
        screen_policy_evaluation_requested_event(&evidence)
            .expect("degraded capture should publish policy evidence")
            .evidence_refs,
        vec![evidence.evidence_ref]
    );
}

#[test]
fn available_ambiguous_capture_routes_to_ai_boundary() {
    let captured = captured_attempt();
    let input = screen_runtime_input_from_capture(
        ScreenCaptureScheduleDecision::EnqueueCapture {
            reason: ScreenCaptureScheduleTrigger::ManagedBrowserUrlChange,
            scope: ScreenCaptureScope::ActiveWindow,
        },
        Some(&captured),
        ScreenContentSignalState::AmbiguousContent,
    );

    let decision = evaluate_screen_runtime(input.clone());
    let observed = screen_runtime_observed_event(input);
    let evidence = screen_evidence_recorded_event(&observed);

    assert_eq!(
        decision.observation_intent,
        ScreenObservationIntent::AmbiguousContentRequiresAi
    );
    assert_eq!(
        decision.runtime_action_state,
        ScreenRuntimeActionState::RecordCapturedEvidence
    );
    assert_eq!(
        screen_ai_analysis_requested_event(&evidence)
            .expect("ambiguous capture should request AI")
            .evidence_refs,
        vec![evidence.evidence_ref.clone()]
    );
    assert_eq!(screen_policy_evaluation_requested_event(&evidence), None);
}

#[test]
fn available_known_policy_capture_publishes_policy_without_ai() {
    let captured = captured_attempt();
    let input = screen_runtime_input_from_capture(
        ScreenCaptureScheduleDecision::EnqueueCapture {
            reason: ScreenCaptureScheduleTrigger::NativeAppForegroundStart,
            scope: ScreenCaptureScope::ActiveWindow,
        },
        Some(&captured),
        ScreenContentSignalState::KnownPolicyState,
    );

    let decision = evaluate_screen_runtime(input.clone());
    let observed = screen_runtime_observed_event(input);
    let evidence = screen_evidence_recorded_event(&observed);

    assert_eq!(
        decision.observation_intent,
        ScreenObservationIntent::KnownPolicyStateRequiresPolicy
    );
    assert_eq!(screen_ai_analysis_requested_event(&evidence), None);
    assert_eq!(
        screen_policy_evaluation_requested_event(&evidence)
            .expect("known policy capture should publish policy evidence")
            .evidence_refs,
        vec![evidence.evidence_ref]
    );
}

#[test]
fn runtime_decision_recorded_event_preserves_input_and_decision() {
    let input = screen_runtime_input_from_capture(
        ScreenCaptureScheduleDecision::EnqueueCapture {
            reason: ScreenCaptureScheduleTrigger::TimedCadence,
            scope: ScreenCaptureScope::PrimaryDisplay,
        },
        Some(&captured_attempt()),
        ScreenContentSignalState::ObservationOnly,
    );
    let event = screen_runtime_decision_recorded_event(
        ScreenAggregateId::parse("screen.aggregate.1").expect("aggregate id"),
        ScreenRuntimeDecisionId::parse("screen.runtime-decision.1").expect("decision id"),
        input,
    );

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
