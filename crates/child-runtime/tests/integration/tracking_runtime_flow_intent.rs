use ocentra_child_runtime::tracking_runtime_flow as ocentra_child_runtime;
use ocentra_eventing::{
    bus::EventBus, envelope::EventMetadata, envelope::EventSource, error::EventingError,
    ids::CorrelationId, ids::EventCustody, ids::EventId, ids::RecordedAt, ids::RuntimeInstanceId,
    ids::RuntimeRole, ids::SourceComponent, ids::SourceService, ids::TargetHandler,
    request::RequestCompletionOutcome,
};
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::tracking::identifiers::{
    tracking_ai_request_id_from_evidence_ref, tracking_check_in_id_from_observation_id,
    tracking_evaluation_id_from_observation_id, tracking_evidence_ref_from_observation_id,
    tracking_notification_id_from_violation_id, tracking_transition_id_from_observation_id,
    tracking_violation_id_from_ai_request_and_rule_ref, TrackingAlertSeverity, TrackingCheckInId,
    TrackingCheckInState, TrackingChildDeviceId, TrackingChildProfileId, TrackingEvidenceRef,
    TrackingExpectedPlaceState, TrackingPolicyViolationId, TrackingReasonCode, TrackingTimestamp,
    TrackingTransitionKind,
};
use ocentra_parent_agent_protocol::tracking::runtime_event::{
    TrackingChildCheckInDeliveryState, TrackingChildCheckInRequestState,
    TrackingChildCheckInRequestedEvent,
};
use ocentra_tracking_core::alerting::TrackingParentNotificationDecisionState;

trait OptionRequiredExt<T> {
    fn required(self, context: impl std::fmt::Display) -> T;
}

impl<T> OptionRequiredExt<T> for Option<T> {
    fn required(self, context: impl std::fmt::Display) -> T {
        let context = context.to_string();
        let _ = context;
        self.unwrap_or_else(|| std::process::abort())
    }
}

trait ResultRequiredExt<T, E> {
    fn required(self, context: impl std::fmt::Display) -> T;
    fn required_err(self, context: impl std::fmt::Display) -> E;
}

impl<T, E: std::fmt::Debug> ResultRequiredExt<T, E> for Result<T, E> {
    fn required(self, context: impl std::fmt::Display) -> T {
        let context = context.to_string();
        let _ = context;
        self.unwrap_or_else(|_| std::process::abort())
    }

    fn required_err(self, context: impl std::fmt::Display) -> E {
        let context = context.to_string();
        let _ = context;
        self.err().unwrap_or_else(|| std::process::abort())
    }
}

#[tokio::test]
async fn tracking_runtime_flow_keeps_ai_policy_and_notification_decoupled_by_events() {
    let flow_report = ocentra_child_runtime::publish_child_tracking_location_observed_event(
        ocentra_tracking_core::runtime_flow::default_location_observed_event(),
    )
    .await
    .required(constants::tracking_runtime::ERROR_TRACKING_RUNTIME_FLOW_RECORDED);
    assert_tracking_runtime_observation_branches(&flow_report);
    assert_tracking_runtime_policy_branches(&flow_report);
}

fn assert_tracking_runtime_observation_branches(
    flow_report: &ocentra_child_runtime::TrackingRuntimeEventFlowReport,
) {
    assert_tracking_runtime_event_identity(flow_report);
    assert_tracking_runtime_event_derivatives(flow_report);
}

fn assert_tracking_runtime_event_identity(
    flow_report: &ocentra_child_runtime::TrackingRuntimeEventFlowReport,
) {
    let expected_evidence_ref = tracking_evidence_ref_from_observation_id(
        &flow_report.evidence_recorded.source_observation_id,
    );
    let ai_request = flow_report
        .ai_analysis_requested
        .as_ref()
        .required(constants::tracking_runtime::ERROR_TRACKING_RUNTIME_FLOW_RECORDED);

    assert_eq!(
        ai_request.evidence_refs,
        vec![expected_evidence_ref.clone()]
    );
    assert_eq!(
        flow_report.evidence_recorded.source_observed_at,
        TrackingTimestamp::parse(constants::tracking_runtime::DEFAULT_OBSERVED_AT)
            .required(constants::tracking_runtime::DEFAULT_OBSERVED_AT)
    );
    assert_eq!(
        flow_report.evidence_recorded.evidence_ref,
        expected_evidence_ref
    );
    assert_eq!(
        ai_request.ai_request_id,
        tracking_ai_request_id_from_evidence_ref(&flow_report.evidence_recorded.evidence_ref)
    );
    assert_eq!(
        ai_request.source_observed_at,
        flow_report.evidence_recorded.source_observed_at
    );
}

fn assert_tracking_runtime_event_derivatives(
    flow_report: &ocentra_child_runtime::TrackingRuntimeEventFlowReport,
) {
    let ai_request = flow_report
        .ai_analysis_requested
        .as_ref()
        .required(constants::tracking_runtime::ERROR_TRACKING_RUNTIME_FLOW_RECORDED);
    let geofence = flow_report
        .geofence_transition_detected
        .as_ref()
        .required(constants::tracking_runtime::ERROR_TRACKING_RUNTIME_FLOW_RECORDED);
    let expected_place = flow_report
        .expected_place_state_evaluated
        .as_ref()
        .required(constants::tracking_runtime::ERROR_TRACKING_RUNTIME_FLOW_RECORDED);
    let check_in = flow_report
        .child_check_in_recorded
        .as_ref()
        .required(constants::tracking_runtime::ERROR_TRACKING_RUNTIME_FLOW_RECORDED);
    let nearby_place = flow_report
        .nearby_place_classified
        .as_ref()
        .required(constants::tracking_runtime::ERROR_TRACKING_RUNTIME_FLOW_RECORDED);

    assert_eq!(
        geofence.transition_kind,
        TrackingTransitionKind::parse(constants::tracking_runtime::GEOFENCE_TRANSITION_AMBIGUOUS)
            .required(constants::tracking_runtime::GEOFENCE_TRANSITION_AMBIGUOUS)
    );
    assert_eq!(
        geofence.transition_id,
        tracking_transition_id_from_observation_id(
            &flow_report.evidence_recorded.source_observation_id
        )
    );
    assert_eq!(
        expected_place.expected_place_state,
        TrackingExpectedPlaceState::parse(
            constants::tracking_runtime::EXPECTED_PLACE_STATE_UNKNOWN
        )
        .required(constants::tracking_runtime::EXPECTED_PLACE_STATE_UNKNOWN)
    );
    assert_eq!(
        expected_place.evaluation_id,
        tracking_evaluation_id_from_observation_id(
            &flow_report.evidence_recorded.source_observation_id
        )
    );
    assert_eq!(
        check_in.check_in_state,
        TrackingCheckInState::parse(constants::tracking_runtime::CHECK_IN_STATE_RECEIVED)
            .required(constants::tracking_runtime::CHECK_IN_STATE_RECEIVED)
    );
    assert_eq!(
        check_in.check_in_id,
        tracking_check_in_id_from_observation_id(
            &flow_report.evidence_recorded.source_observation_id
        )
    );
    assert_eq!(nearby_place.source_ai_request_id, ai_request.ai_request_id);
    assert_eq!(
        nearby_place.source_location_evidence_ref,
        flow_report.evidence_recorded.evidence_ref
    );
    assert_eq!(
        nearby_place.source_observed_at,
        flow_report.evidence_recorded.source_observed_at
    );
    assert_eq!(
        nearby_place.provider_kind,
        constants::tracking_runtime::NEARBY_PROVIDER_KIND_LOCAL_CACHE
    );
    assert_eq!(
        nearby_place.ambiguity_state,
        constants::tracking_runtime::NEARBY_PLACE_AMBIGUITY_CLEAR
    );
}

fn assert_tracking_runtime_policy_branches(
    flow_report: &ocentra_child_runtime::TrackingRuntimeEventFlowReport,
) {
    let ai_request = flow_report
        .ai_analysis_requested
        .as_ref()
        .required(constants::tracking_runtime::ERROR_TRACKING_RUNTIME_FLOW_RECORDED);
    let nearby_place = flow_report
        .nearby_place_classified
        .as_ref()
        .required(constants::tracking_runtime::ERROR_TRACKING_RUNTIME_FLOW_RECORDED);
    let ai_boundary_decision = flow_report
        .ai_boundary_decision
        .as_ref()
        .required(constants::tracking_runtime::ERROR_TRACKING_RUNTIME_FLOW_RECORDED);
    let alert_decision = flow_report
        .alert_decision
        .as_ref()
        .required(constants::tracking_runtime::ERROR_TRACKING_RUNTIME_FLOW_RECORDED);
    let policy_violation = flow_report
        .policy_violation_detected
        .as_ref()
        .required(constants::tracking_runtime::ERROR_TRACKING_RUNTIME_FLOW_RECORDED);
    let parent_notification = flow_report
        .parent_notification_requested
        .as_ref()
        .required(constants::tracking_runtime::ERROR_TRACKING_RUNTIME_FLOW_RECORDED);

    assert_eq!(
        ai_boundary_decision.decision_state,
        constants::tracking_runtime::AI_RESULT_ACCEPTED_AS_EVIDENCE
    );
    assert_eq!(
        alert_decision.severity,
        TrackingAlertSeverity::parse(constants::tracking_runtime::ALERT_SEVERITY_WATCH)
            .required(constants::tracking_runtime::ALERT_SEVERITY_WATCH)
    );
    assert_eq!(policy_violation.evidence_refs, nearby_place.evidence_refs);
    assert_eq!(
        parent_notification.source_policy_violation_id,
        policy_violation.violation_id
    );
    assert_eq!(
        policy_violation.violation_id,
        tracking_violation_id_from_ai_request_and_rule_ref(
            &ai_request.ai_request_id,
            &policy_violation.policy_rule_ref,
        )
    );
    assert_eq!(
        parent_notification.notification_id,
        tracking_notification_id_from_violation_id(&policy_violation.violation_id)
    );
}

#[tokio::test]
async fn tracking_runtime_flow_can_attach_once_to_runtime_owned_bus() {
    let runtime_flow = ocentra_child_runtime::TrackingRuntimeEventFlow::new()
        .await
        .required(constants::tracking_runtime::ERROR_TRACKING_RUNTIME_FLOW_RECORDED);
    let metrics_before = runtime_flow.metrics_snapshot().await;

    let flow_report = runtime_flow
        .publish_location_observed(
            ocentra_tracking_core::runtime_flow::default_location_observed_event(),
        )
        .await
        .required(constants::tracking_runtime::ERROR_TRACKING_RUNTIME_FLOW_RECORDED);
    let metrics_after = runtime_flow.metrics_snapshot().await;

    assert_eq!(metrics_before.subscription_count, 6);
    assert_eq!(metrics_after.subscription_count, 6);
    assert_eq!(
        flow_report
            .tracking_subscription_report
            .subscriber_id
            .as_str(),
        constants::tracking_runtime::SUBSCRIBER_CHILD_TRACKING_OBSERVER
    );
    assert_eq!(
        flow_report
            .child_check_in_request_subscription_report
            .subscriber_id
            .as_str(),
        constants::tracking_runtime::SUBSCRIBER_CHILD_TRACKING_CHECK_IN_REQUESTER
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
            .required(constants::tracking_runtime::ERROR_TRACKING_RUNTIME_FLOW_RECORDED)
            .evidence_refs,
        vec![flow_report.evidence_recorded.evidence_ref.clone()]
    );
    assert_eq!(
        flow_report
            .expected_place_state_evaluated
            .required(constants::tracking_runtime::ERROR_TRACKING_RUNTIME_FLOW_RECORDED)
            .evidence_refs,
        vec![flow_report.evidence_recorded.evidence_ref.clone()]
    );
    assert_eq!(
        flow_report
            .child_check_in_recorded
            .required(constants::tracking_runtime::ERROR_TRACKING_RUNTIME_FLOW_RECORDED)
            .source_observation_id,
        flow_report.evidence_recorded.source_observation_id
    );
    assert_eq!(
        flow_report
            .ai_analysis_requested
            .required(constants::tracking_runtime::ERROR_TRACKING_RUNTIME_FLOW_RECORDED)
            .evidence_refs,
        vec![flow_report.evidence_recorded.evidence_ref.clone()]
    );
    assert_eq!(
        flow_report
            .policy_violation_detected
            .as_ref()
            .required(constants::tracking_runtime::ERROR_TRACKING_RUNTIME_FLOW_RECORDED)
            .evidence_refs,
        flow_report
            .parent_notification_requested
            .as_ref()
            .required(constants::tracking_runtime::ERROR_TRACKING_RUNTIME_FLOW_RECORDED)
            .evidence_refs
    );
}

#[tokio::test]
async fn tracking_runtime_flow_can_route_away_from_expected_place_without_ai_boundary() {
    let flow_report = ocentra_child_runtime::publish_child_tracking_location_observed_event(
        ocentra_tracking_core::runtime_flow::default_away_from_expected_place_location_observed_event(),
    )
    .await
    .required(constants::tracking_runtime::ERROR_TRACKING_RUNTIME_FLOW_RECORDED);

    assert!(flow_report.ai_analysis_requested.is_none());
    assert!(flow_report.nearby_place_classified.is_none());
    assert!(flow_report.ai_boundary_decision.is_none());
    assert_eq!(
        flow_report
            .alert_decision
            .as_ref()
            .required(constants::tracking_runtime::ERROR_TRACKING_RUNTIME_FLOW_RECORDED)
            .severity,
        TrackingAlertSeverity::parse(constants::tracking_runtime::ALERT_SEVERITY_WATCH)
            .required(constants::tracking_runtime::ALERT_SEVERITY_WATCH)
    );
    assert_eq!(
        flow_report
            .geofence_transition_detected
            .as_ref()
            .required(constants::tracking_runtime::ERROR_TRACKING_RUNTIME_FLOW_RECORDED)
            .transition_kind,
        TrackingTransitionKind::parse(constants::tracking_runtime::GEOFENCE_TRANSITION_EXIT)
            .required(constants::tracking_runtime::GEOFENCE_TRANSITION_EXIT)
    );
    assert_eq!(
        flow_report
            .expected_place_state_evaluated
            .as_ref()
            .required(constants::tracking_runtime::ERROR_TRACKING_RUNTIME_FLOW_RECORDED)
            .expected_place_state,
        TrackingExpectedPlaceState::parse(
            constants::tracking_runtime::EXPECTED_PLACE_STATE_LEFT_EXPECTED_PLACE
        )
        .required(constants::tracking_runtime::EXPECTED_PLACE_STATE_LEFT_EXPECTED_PLACE)
    );
    assert_eq!(
        flow_report
            .policy_violation_detected
            .as_ref()
            .required(constants::tracking_runtime::ERROR_TRACKING_RUNTIME_FLOW_RECORDED)
            .evidence_refs,
        flow_report
            .expected_place_state_evaluated
            .as_ref()
            .required(constants::tracking_runtime::ERROR_TRACKING_RUNTIME_FLOW_RECORDED)
            .evidence_refs
    );
    assert_eq!(
        flow_report
            .parent_notification_requested
            .as_ref()
            .required(constants::tracking_runtime::ERROR_TRACKING_RUNTIME_FLOW_RECORDED)
            .source_policy_violation_id,
        flow_report
            .policy_violation_detected
            .as_ref()
            .required(constants::tracking_runtime::ERROR_TRACKING_RUNTIME_FLOW_RECORDED)
            .violation_id
    );
}

#[tokio::test]
async fn tracking_runtime_flow_clears_optional_state_between_observations() {
    let runtime_flow = ocentra_child_runtime::TrackingRuntimeEventFlow::new()
        .await
        .required(constants::tracking_runtime::ERROR_TRACKING_RUNTIME_FLOW_RECORDED);

    let first_report = runtime_flow
        .publish_location_observed(
            ocentra_tracking_core::runtime_flow::default_location_observed_event(),
        )
        .await
        .required(constants::tracking_runtime::ERROR_TRACKING_RUNTIME_FLOW_RECORDED);
    let second_report = runtime_flow
        .publish_location_observed(
            ocentra_tracking_core::runtime_flow::default_at_expected_place_location_observed_event(
            ),
        )
        .await
        .required(constants::tracking_runtime::ERROR_TRACKING_RUNTIME_FLOW_RECORDED);

    assert_eq!(
        first_report
            .ai_analysis_requested
            .as_ref()
            .required(constants::tracking_runtime::ERROR_TRACKING_RUNTIME_FLOW_RECORDED)
            .evidence_refs,
        vec![first_report.evidence_recorded.evidence_ref.clone()]
    );
    assert_eq!(
        first_report
            .policy_violation_detected
            .as_ref()
            .required(constants::tracking_runtime::ERROR_TRACKING_RUNTIME_FLOW_RECORDED)
            .evidence_refs,
        first_report
            .expected_place_state_evaluated
            .as_ref()
            .required(constants::tracking_runtime::ERROR_TRACKING_RUNTIME_FLOW_RECORDED)
            .evidence_refs
    );
    assert_eq!(
        first_report
            .parent_notification_requested
            .as_ref()
            .required(constants::tracking_runtime::ERROR_TRACKING_RUNTIME_FLOW_RECORDED)
            .source_policy_violation_id,
        first_report
            .policy_violation_detected
            .as_ref()
            .required(constants::tracking_runtime::ERROR_TRACKING_RUNTIME_FLOW_RECORDED)
            .violation_id
    );
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
            .required(constants::tracking_runtime::ERROR_TRACKING_RUNTIME_FLOW_RECORDED)
            .expected_place_state,
        TrackingExpectedPlaceState::parse(
            constants::tracking_runtime::EXPECTED_PLACE_STATE_WHERE_EXPECTED
        )
        .required(constants::tracking_runtime::EXPECTED_PLACE_STATE_WHERE_EXPECTED)
    );
}

#[tokio::test]
async fn tracking_runtime_flow_suppresses_duplicate_parent_notifications_on_repeated_violation() {
    let runtime_flow = ocentra_child_runtime::TrackingRuntimeEventFlow::new()
        .await
        .required(constants::tracking_runtime::ERROR_TRACKING_RUNTIME_FLOW_RECORDED);

    let first_report = runtime_flow
        .publish_location_observed(
            ocentra_tracking_core::runtime_flow::default_location_observed_event(),
        )
        .await
        .required(constants::tracking_runtime::ERROR_TRACKING_RUNTIME_FLOW_RECORDED);
    let second_report = runtime_flow
        .publish_location_observed(
            ocentra_tracking_core::runtime_flow::default_location_observed_event(),
        )
        .await
        .required(constants::tracking_runtime::ERROR_TRACKING_RUNTIME_FLOW_RECORDED);

    assert_eq!(
        first_report
            .alert_decision
            .as_ref()
            .required(constants::tracking_runtime::ERROR_TRACKING_RUNTIME_FLOW_RECORDED)
            .parent_notification_state,
        TrackingParentNotificationDecisionState::Allowed
    );
    assert_eq!(
        first_report
            .parent_notification_requested
            .as_ref()
            .required(constants::tracking_runtime::ERROR_TRACKING_RUNTIME_FLOW_RECORDED)
            .source_policy_violation_id,
        first_report
            .policy_violation_detected
            .as_ref()
            .required(constants::tracking_runtime::ERROR_TRACKING_RUNTIME_FLOW_RECORDED)
            .violation_id
    );
    assert_eq!(
        second_report
            .alert_decision
            .as_ref()
            .required(constants::tracking_runtime::ERROR_TRACKING_RUNTIME_FLOW_RECORDED)
            .severity,
        TrackingAlertSeverity::parse(constants::tracking_runtime::ALERT_SEVERITY_WATCH)
            .required(constants::tracking_runtime::ALERT_SEVERITY_WATCH)
    );
    assert_eq!(
        second_report
            .alert_decision
            .as_ref()
            .required(constants::tracking_runtime::ERROR_TRACKING_RUNTIME_FLOW_RECORDED)
            .parent_notification_state,
        TrackingParentNotificationDecisionState::SuppressedDuplicate
    );
    assert_eq!(
        second_report
            .policy_violation_detected
            .as_ref()
            .required(constants::tracking_runtime::ERROR_TRACKING_RUNTIME_FLOW_RECORDED)
            .evidence_refs,
        second_report
            .expected_place_state_evaluated
            .as_ref()
            .required(constants::tracking_runtime::ERROR_TRACKING_RUNTIME_FLOW_RECORDED)
            .evidence_refs
    );
    assert!(second_report.parent_notification_requested.is_none());
}

#[tokio::test]
async fn tracking_runtime_flow_rejects_invalid_location_before_recording_evidence() {
    let mut observed = ocentra_tracking_core::runtime_flow::default_location_observed_event();
    observed.horizontal_accuracy_meters = 0;

    let error = ocentra_child_runtime::publish_child_tracking_location_observed_event(observed)
        .await
        .required_err("invalid tracking observation should fail");

    assert_eq!(
        error,
        EventingError::InvalidValue {
            field: "tracking.location.validation",
            value: String::from(constants::tracking_runtime::LOCATION_VALIDATION_REJECTED_ACCURACY),
        }
    );
}

#[tokio::test]
async fn tracking_runtime_flow_marks_duplicate_parent_requested_check_in_receipts() {
    let bus = EventBus::new();
    let runtime_flow = ocentra_child_runtime::TrackingRuntimeEventFlow::with_bus(bus.clone())
        .await
        .required(constants::tracking_runtime::ERROR_TRACKING_RUNTIME_FLOW_RECORDED);
    let request = parent_requested_check_in_event(
        TrackingChildCheckInDeliveryState::Queued,
        TrackingChildCheckInRequestState::Pending,
        "2026-06-12T12:05:00Z",
    );

    bus.publish(
        request.clone(),
        parent_requested_check_in_metadata(
            request.check_in_id.as_str(),
            constants::tracking_runtime::DEFAULT_OBSERVED_AT,
        ),
    )
    .await
    .required(constants::tracking_runtime::ERROR_TRACKING_RUNTIME_FLOW_RECORDED);
    bus.publish(
        request,
        parent_requested_check_in_metadata(
            constants::tracking_runtime::DEFAULT_CHILD_CHECK_IN_ID,
            "2026-06-12T12:00:01Z",
        ),
    )
    .await
    .required(constants::tracking_runtime::ERROR_TRACKING_RUNTIME_FLOW_RECORDED);

    let (_, _, receipt, completion) = runtime_flow
        .latest_parent_requested_check_in()
        .required("duplicate request should be recorded");

    assert_eq!(
        receipt.delivery_state,
        TrackingChildCheckInDeliveryState::Duplicate
    );
    assert_eq!(
        receipt.reason_code,
        Some(
            TrackingReasonCode::parse(
                constants::tracking_runtime::REASON_DUPLICATE_CHECK_IN_REQUEST,
            )
            .required(constants::tracking_runtime::REASON_DUPLICATE_CHECK_IN_REQUEST),
        )
    );
    assert_eq!(completion.outcome, RequestCompletionOutcome::Late);
}

#[tokio::test]
async fn tracking_runtime_flow_marks_stale_parent_requested_check_in_receipts() {
    let bus = EventBus::new();
    let runtime_flow = ocentra_child_runtime::TrackingRuntimeEventFlow::with_bus(bus.clone())
        .await
        .required(constants::tracking_runtime::ERROR_TRACKING_RUNTIME_FLOW_RECORDED);
    let request = parent_requested_check_in_event(
        TrackingChildCheckInDeliveryState::Queued,
        TrackingChildCheckInRequestState::Pending,
        "2026-06-12T11:59:59Z",
    );

    bus.publish(
        request,
        parent_requested_check_in_metadata(
            constants::tracking_runtime::DEFAULT_CHILD_CHECK_IN_ID,
            constants::tracking_runtime::DEFAULT_OBSERVED_AT,
        ),
    )
    .await
    .required(constants::tracking_runtime::ERROR_TRACKING_RUNTIME_FLOW_RECORDED);

    let (_, _, receipt, completion) = runtime_flow
        .latest_parent_requested_check_in()
        .required("stale request should be recorded");

    assert_eq!(
        receipt.delivery_state,
        TrackingChildCheckInDeliveryState::Stale
    );
    assert_eq!(
        receipt.reason_code,
        Some(
            TrackingReasonCode::parse(constants::tracking_runtime::REASON_STALE_CHECK_IN_REQUEST)
                .required(constants::tracking_runtime::REASON_STALE_CHECK_IN_REQUEST),
        )
    );
    assert_eq!(completion.outcome, RequestCompletionOutcome::Late);
}

#[tokio::test]
async fn tracking_runtime_flow_marks_unsupported_parent_requested_check_in_delivery() {
    let bus = EventBus::new();
    let runtime_flow = ocentra_child_runtime::TrackingRuntimeEventFlow::with_bus(bus.clone())
        .await
        .required(constants::tracking_runtime::ERROR_TRACKING_RUNTIME_FLOW_RECORDED);
    let request = parent_requested_check_in_event(
        TrackingChildCheckInDeliveryState::Requested,
        TrackingChildCheckInRequestState::Pending,
        "2026-06-12T12:05:00Z",
    );

    bus.publish(
        request,
        parent_requested_check_in_metadata(
            constants::tracking_runtime::DEFAULT_CHILD_CHECK_IN_ID,
            constants::tracking_runtime::DEFAULT_OBSERVED_AT,
        ),
    )
    .await
    .required(constants::tracking_runtime::ERROR_TRACKING_RUNTIME_FLOW_RECORDED);

    let (_, _, receipt, completion) = runtime_flow
        .latest_parent_requested_check_in()
        .required("unsupported delivery should be recorded");

    assert_eq!(
        receipt.delivery_state,
        TrackingChildCheckInDeliveryState::UnsupportedDelivery
    );
    assert_eq!(
        receipt.reason_code,
        Some(
            TrackingReasonCode::parse(
                constants::tracking_runtime::REASON_UNSUPPORTED_CHECK_IN_DELIVERY,
            )
            .required(constants::tracking_runtime::REASON_UNSUPPORTED_CHECK_IN_DELIVERY),
        )
    );
    assert_eq!(completion.outcome, RequestCompletionOutcome::Late);
}

fn parent_requested_check_in_event(
    delivery_state: TrackingChildCheckInDeliveryState,
    request_state: TrackingChildCheckInRequestState,
    expires_at: impl std::fmt::Display,
) -> TrackingChildCheckInRequestedEvent {
    let expires_at = expires_at.to_string();
    TrackingChildCheckInRequestedEvent {
        child_device_id: TrackingChildDeviceId::parse(
            constants::tracking_runtime::DEFAULT_CHILD_DEVICE_ID,
        )
        .required(constants::tracking_runtime::DEFAULT_CHILD_DEVICE_ID),
        child_profile_id: TrackingChildProfileId::parse(
            constants::tracking_runtime::DEFAULT_CHILD_PROFILE_ID,
        )
        .required(constants::tracking_runtime::DEFAULT_CHILD_PROFILE_ID),
        check_in_id: TrackingCheckInId::parse(
            constants::tracking_runtime::DEFAULT_CHILD_CHECK_IN_ID,
        )
        .required(constants::tracking_runtime::DEFAULT_CHILD_CHECK_IN_ID),
        requested_at: TrackingTimestamp::parse(constants::tracking_runtime::DEFAULT_OBSERVED_AT)
            .required(constants::tracking_runtime::DEFAULT_OBSERVED_AT),
        request_state,
        delivery_state,
        related_alert_id: TrackingPolicyViolationId::parse(
            constants::tracking_runtime::DEFAULT_POLICY_VIOLATION_ID,
        )
        .required(constants::tracking_runtime::DEFAULT_POLICY_VIOLATION_ID),
        include_location_if_permitted: true,
        expires_at: TrackingTimestamp::parse(&expires_at).required(&expires_at),
        evidence_refs: vec![TrackingEvidenceRef::parse(
            constants::tracking_runtime::DEFAULT_EVIDENCE_REF,
        )
        .required(constants::tracking_runtime::DEFAULT_EVIDENCE_REF)],
        audit_refs: vec![String::from("audit.tracking.child-check-in.request")],
    }
}

fn parent_requested_check_in_metadata(
    check_in_id: impl std::fmt::Display,
    observed_at: impl std::fmt::Display,
) -> EventMetadata {
    let check_in_id = check_in_id.to_string();
    let observed_at = observed_at.to_string();
    EventMetadata::from_parts(
        EventId::generated(),
        CorrelationId::parse(format!(
            "{}{}",
            constants::tracking_runtime::CORRELATION_PREFIX,
            check_in_id
        ))
        .required(constants::tracking_runtime::CORRELATION_PREFIX),
        EventSource::new(
            EventCustody::parse(constants::eventing_source::CUSTODY_LOCAL_JOURNAL)
                .required(constants::eventing_source::CUSTODY_LOCAL_JOURNAL),
            RuntimeRole::parse(constants::eventing_source::ROLE_CONTROLLER)
                .required(constants::eventing_source::ROLE_CONTROLLER),
            SourceService::parse(constants::peer::LOCAL_DEV_AGENT)
                .required(constants::peer::LOCAL_DEV_AGENT),
            SourceComponent::parse(constants::tracking_runtime::SOURCE_COMPONENT_PARENT_RUNTIME)
                .required(constants::tracking_runtime::SOURCE_COMPONENT_PARENT_RUNTIME),
            RuntimeInstanceId::parse(constants::peer::PORTAL_DEV)
                .required(constants::peer::PORTAL_DEV),
        ),
        RecordedAt::parse(&observed_at).required(&observed_at),
        Some(
            TargetHandler::parse(
                constants::tracking_runtime::TARGET_HANDLER_CHILD_TRACKING_CHECK_IN_REQUESTER,
            )
            .required(
                constants::tracking_runtime::TARGET_HANDLER_CHILD_TRACKING_CHECK_IN_REQUESTER,
            ),
        ),
    )
}
