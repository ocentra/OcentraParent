use ocentra_eventing::EventingError;
use ocentra_parent_agent_protocol::{
    constants, TrackingAlertSeverity, TrackingCheckInState, TrackingEvidenceRef,
    TrackingExpectedPlaceState, TrackingTransitionKind,
};

#[tokio::test]
async fn tracking_runtime_flow_keeps_ai_policy_and_notification_decoupled_by_events() {
    let flow_report = ocentra_child_runtime::publish_child_tracking_location_observed_event(
        ocentra_tracking_core::default_location_observed_event(),
    )
    .await
    .expect(constants::tracking_runtime::ERROR_TRACKING_RUNTIME_FLOW_RECORDED);

    assert_eq!(
        flow_report
            .ai_analysis_requested
            .as_ref()
            .expect(constants::tracking_runtime::ERROR_TRACKING_RUNTIME_FLOW_RECORDED)
            .evidence_refs,
        vec![
            TrackingEvidenceRef::parse(constants::tracking_runtime::DEFAULT_EVIDENCE_REF)
                .expect(constants::tracking_runtime::DEFAULT_EVIDENCE_REF)
        ]
    );
    assert_eq!(
        flow_report
            .geofence_transition_detected
            .as_ref()
            .expect(constants::tracking_runtime::ERROR_TRACKING_RUNTIME_FLOW_RECORDED)
            .transition_kind,
        TrackingTransitionKind::parse(constants::tracking_runtime::GEOFENCE_TRANSITION_AMBIGUOUS)
            .expect(constants::tracking_runtime::GEOFENCE_TRANSITION_AMBIGUOUS)
    );
    assert_eq!(
        flow_report
            .expected_place_state_evaluated
            .as_ref()
            .expect(constants::tracking_runtime::ERROR_TRACKING_RUNTIME_FLOW_RECORDED)
            .expected_place_state,
        TrackingExpectedPlaceState::parse(
            constants::tracking_runtime::EXPECTED_PLACE_STATE_UNKNOWN
        )
        .expect(constants::tracking_runtime::EXPECTED_PLACE_STATE_UNKNOWN)
    );
    assert_eq!(
        flow_report
            .child_check_in_recorded
            .as_ref()
            .expect(constants::tracking_runtime::ERROR_TRACKING_RUNTIME_FLOW_RECORDED)
            .check_in_state,
        TrackingCheckInState::parse(constants::tracking_runtime::CHECK_IN_STATE_RECEIVED)
            .expect(constants::tracking_runtime::CHECK_IN_STATE_RECEIVED)
    );
    assert_eq!(
        flow_report
            .nearby_place_classified
            .as_ref()
            .expect(constants::tracking_runtime::ERROR_TRACKING_RUNTIME_FLOW_RECORDED)
            .source_ai_request_id,
        flow_report
            .ai_analysis_requested
            .as_ref()
            .expect(constants::tracking_runtime::ERROR_TRACKING_RUNTIME_FLOW_RECORDED)
            .ai_request_id
    );
    assert_eq!(
        flow_report
            .ai_boundary_decision
            .as_ref()
            .expect(constants::tracking_runtime::ERROR_TRACKING_RUNTIME_FLOW_RECORDED)
            .decision_state,
        constants::tracking_runtime::AI_RESULT_ACCEPTED_AS_EVIDENCE
    );
    assert_eq!(
        flow_report
            .alert_decision
            .as_ref()
            .expect(constants::tracking_runtime::ERROR_TRACKING_RUNTIME_FLOW_RECORDED)
            .severity,
        TrackingAlertSeverity::parse(constants::tracking_runtime::ALERT_SEVERITY_REVIEW)
            .expect(constants::tracking_runtime::ALERT_SEVERITY_REVIEW)
    );
    assert_eq!(
        flow_report
            .policy_violation_detected
            .as_ref()
            .expect(constants::tracking_runtime::ERROR_TRACKING_RUNTIME_FLOW_RECORDED)
            .evidence_refs,
        flow_report
            .nearby_place_classified
            .as_ref()
            .expect(constants::tracking_runtime::ERROR_TRACKING_RUNTIME_FLOW_RECORDED)
            .evidence_refs
    );
    assert_eq!(
        flow_report
            .parent_notification_requested
            .as_ref()
            .expect(constants::tracking_runtime::ERROR_TRACKING_RUNTIME_FLOW_RECORDED)
            .source_policy_violation_id,
        flow_report
            .policy_violation_detected
            .as_ref()
            .expect(constants::tracking_runtime::ERROR_TRACKING_RUNTIME_FLOW_RECORDED)
            .violation_id
    );
}

#[tokio::test]
async fn tracking_runtime_flow_can_attach_once_to_runtime_owned_bus() {
    let runtime_flow = ocentra_child_runtime::TrackingRuntimeEventFlow::new()
        .await
        .expect(constants::tracking_runtime::ERROR_TRACKING_RUNTIME_FLOW_RECORDED);
    let metrics_before = runtime_flow.metrics_snapshot().await;

    let flow_report = runtime_flow
        .publish_location_observed(ocentra_tracking_core::default_location_observed_event())
        .await
        .expect(constants::tracking_runtime::ERROR_TRACKING_RUNTIME_FLOW_RECORDED);
    let metrics_after = runtime_flow.metrics_snapshot().await;

    assert_eq!(metrics_before.subscription_count, 5);
    assert_eq!(metrics_after.subscription_count, 5);
    assert_eq!(
        flow_report
            .tracking_subscription_report
            .subscriber_id
            .as_str(),
        constants::tracking_runtime::SUBSCRIBER_CHILD_TRACKING_OBSERVER
    );
    assert_eq!(
        flow_report
            .child_expected_place_policy_subscription_report
            .subscriber_id
            .as_str(),
        constants::tracking_runtime::SUBSCRIBER_CHILD_POLICY_EXPECTED_PLACE_EVALUATOR
    );
    assert_eq!(
        flow_report
            .geofence_transition_detected
            .expect(constants::tracking_runtime::ERROR_TRACKING_RUNTIME_FLOW_RECORDED)
            .evidence_refs,
        vec![flow_report.evidence_recorded.evidence_ref.clone()]
    );
    assert_eq!(
        flow_report
            .expected_place_state_evaluated
            .expect(constants::tracking_runtime::ERROR_TRACKING_RUNTIME_FLOW_RECORDED)
            .evidence_refs,
        vec![flow_report.evidence_recorded.evidence_ref.clone()]
    );
    assert_eq!(
        flow_report
            .child_check_in_recorded
            .expect(constants::tracking_runtime::ERROR_TRACKING_RUNTIME_FLOW_RECORDED)
            .source_observation_id,
        flow_report.evidence_recorded.source_observation_id
    );
    assert_eq!(
        flow_report
            .ai_analysis_requested
            .expect(constants::tracking_runtime::ERROR_TRACKING_RUNTIME_FLOW_RECORDED)
            .evidence_refs,
        vec![flow_report.evidence_recorded.evidence_ref.clone()]
    );
    assert_eq!(
        flow_report
            .policy_violation_detected
            .as_ref()
            .expect(constants::tracking_runtime::ERROR_TRACKING_RUNTIME_FLOW_RECORDED)
            .evidence_refs,
        flow_report
            .parent_notification_requested
            .as_ref()
            .expect(constants::tracking_runtime::ERROR_TRACKING_RUNTIME_FLOW_RECORDED)
            .evidence_refs
    );
}

#[tokio::test]
async fn tracking_runtime_flow_can_route_away_from_expected_place_without_ai_boundary() {
    let flow_report = ocentra_child_runtime::publish_child_tracking_location_observed_event(
        ocentra_tracking_core::default_away_from_expected_place_location_observed_event(),
    )
    .await
    .expect(constants::tracking_runtime::ERROR_TRACKING_RUNTIME_FLOW_RECORDED);

    assert!(flow_report.ai_analysis_requested.is_none());
    assert!(flow_report.nearby_place_classified.is_none());
    assert!(flow_report.ai_boundary_decision.is_none());
    assert_eq!(
        flow_report
            .alert_decision
            .as_ref()
            .expect(constants::tracking_runtime::ERROR_TRACKING_RUNTIME_FLOW_RECORDED)
            .severity,
        TrackingAlertSeverity::parse(constants::tracking_runtime::ALERT_SEVERITY_REVIEW)
            .expect(constants::tracking_runtime::ALERT_SEVERITY_REVIEW)
    );
    assert_eq!(
        flow_report
            .geofence_transition_detected
            .as_ref()
            .expect(constants::tracking_runtime::ERROR_TRACKING_RUNTIME_FLOW_RECORDED)
            .transition_kind,
        TrackingTransitionKind::parse(constants::tracking_runtime::GEOFENCE_TRANSITION_EXIT)
            .expect(constants::tracking_runtime::GEOFENCE_TRANSITION_EXIT)
    );
    assert_eq!(
        flow_report
            .expected_place_state_evaluated
            .as_ref()
            .expect(constants::tracking_runtime::ERROR_TRACKING_RUNTIME_FLOW_RECORDED)
            .expected_place_state,
        TrackingExpectedPlaceState::parse(
            constants::tracking_runtime::EXPECTED_PLACE_STATE_LEFT_EXPECTED_PLACE
        )
        .expect(constants::tracking_runtime::EXPECTED_PLACE_STATE_LEFT_EXPECTED_PLACE)
    );
    assert_eq!(
        flow_report
            .policy_violation_detected
            .as_ref()
            .expect(constants::tracking_runtime::ERROR_TRACKING_RUNTIME_FLOW_RECORDED)
            .evidence_refs,
        flow_report
            .expected_place_state_evaluated
            .as_ref()
            .expect(constants::tracking_runtime::ERROR_TRACKING_RUNTIME_FLOW_RECORDED)
            .evidence_refs
    );
    assert_eq!(
        flow_report
            .parent_notification_requested
            .as_ref()
            .expect(constants::tracking_runtime::ERROR_TRACKING_RUNTIME_FLOW_RECORDED)
            .source_policy_violation_id,
        flow_report
            .policy_violation_detected
            .as_ref()
            .expect(constants::tracking_runtime::ERROR_TRACKING_RUNTIME_FLOW_RECORDED)
            .violation_id
    );
}

#[tokio::test]
async fn tracking_runtime_flow_clears_optional_state_between_observations() {
    let runtime_flow = ocentra_child_runtime::TrackingRuntimeEventFlow::new()
        .await
        .expect(constants::tracking_runtime::ERROR_TRACKING_RUNTIME_FLOW_RECORDED);

    let first_report = runtime_flow
        .publish_location_observed(ocentra_tracking_core::default_location_observed_event())
        .await
        .expect(constants::tracking_runtime::ERROR_TRACKING_RUNTIME_FLOW_RECORDED);
    let second_report = runtime_flow
        .publish_location_observed(
            ocentra_tracking_core::default_at_expected_place_location_observed_event(),
        )
        .await
        .expect(constants::tracking_runtime::ERROR_TRACKING_RUNTIME_FLOW_RECORDED);

    assert!(first_report.ai_analysis_requested.is_some());
    assert!(first_report.policy_violation_detected.is_some());
    assert!(first_report.parent_notification_requested.is_some());
    assert!(second_report.ai_analysis_requested.is_none());
    assert!(second_report.nearby_place_classified.is_none());
    assert!(second_report.ai_boundary_decision.is_none());
    assert!(second_report.alert_decision.is_none());
    assert!(second_report.policy_violation_detected.is_none());
    assert!(second_report.parent_notification_requested.is_none());
    assert_eq!(
        second_report
            .expected_place_state_evaluated
            .as_ref()
            .expect(constants::tracking_runtime::ERROR_TRACKING_RUNTIME_FLOW_RECORDED)
            .expected_place_state,
        TrackingExpectedPlaceState::parse(
            constants::tracking_runtime::EXPECTED_PLACE_STATE_WHERE_EXPECTED
        )
        .expect(constants::tracking_runtime::EXPECTED_PLACE_STATE_WHERE_EXPECTED)
    );
}

#[tokio::test]
async fn tracking_runtime_flow_suppresses_duplicate_parent_notifications_on_repeated_violation() {
    let runtime_flow = ocentra_child_runtime::TrackingRuntimeEventFlow::new()
        .await
        .expect(constants::tracking_runtime::ERROR_TRACKING_RUNTIME_FLOW_RECORDED);

    let first_report = runtime_flow
        .publish_location_observed(ocentra_tracking_core::default_location_observed_event())
        .await
        .expect(constants::tracking_runtime::ERROR_TRACKING_RUNTIME_FLOW_RECORDED);
    let second_report = runtime_flow
        .publish_location_observed(ocentra_tracking_core::default_location_observed_event())
        .await
        .expect(constants::tracking_runtime::ERROR_TRACKING_RUNTIME_FLOW_RECORDED);

    assert_eq!(
        first_report
            .alert_decision
            .as_ref()
            .expect(constants::tracking_runtime::ERROR_TRACKING_RUNTIME_FLOW_RECORDED)
            .parent_notification_state,
        ocentra_tracking_core::TrackingParentNotificationDecisionState::Allowed
    );
    assert!(first_report.parent_notification_requested.is_some());
    assert_eq!(
        second_report
            .alert_decision
            .as_ref()
            .expect(constants::tracking_runtime::ERROR_TRACKING_RUNTIME_FLOW_RECORDED)
            .severity,
        TrackingAlertSeverity::parse(constants::tracking_runtime::ALERT_SEVERITY_NONE)
            .expect(constants::tracking_runtime::ALERT_SEVERITY_NONE)
    );
    assert_eq!(
        second_report
            .alert_decision
            .as_ref()
            .expect(constants::tracking_runtime::ERROR_TRACKING_RUNTIME_FLOW_RECORDED)
            .parent_notification_state,
        ocentra_tracking_core::TrackingParentNotificationDecisionState::Suppressed
    );
    assert!(second_report.policy_violation_detected.is_some());
    assert!(second_report.parent_notification_requested.is_none());
}

#[tokio::test]
async fn tracking_runtime_flow_rejects_invalid_location_before_recording_evidence() {
    let mut observed = ocentra_tracking_core::default_location_observed_event();
    observed.horizontal_accuracy_meters = 0;

    let error = ocentra_child_runtime::publish_child_tracking_location_observed_event(observed)
        .await
        .expect_err("invalid tracking observation should fail");

    assert_eq!(
        error,
        EventingError::InvalidValue {
            field: "tracking.location.validation",
            value: String::from(constants::tracking_runtime::LOCATION_VALIDATION_REJECTED_ACCURACY),
        }
    );
}
