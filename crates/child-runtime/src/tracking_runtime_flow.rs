use std::sync::{Arc, Mutex};

use ocentra_eventing::{
    CorrelationId, EventBus, EventCustody, EventId, EventMetadata, EventSource, EventSubscriber,
    EventType, EventingError, RecordedAt, RuntimeInstanceId, RuntimeRole, SourceComponent,
    SourceService, SubscriberId, SubscriptionReport, TargetHandler,
};
use ocentra_parent_agent_protocol::{
    constants, ParentNotificationRequestedEvent, TrackingAiAnalysisRequestedEvent,
    TrackingChildCheckInRecordedEvent, TrackingEvidenceRecordedEvent,
    TrackingExpectedPlaceStateEvaluatedEvent, TrackingGeofenceTransitionDetectedEvent,
    TrackingLocationObservedEvent, TrackingNearbyPlaceClassifiedEvent,
    TrackingNotificationMode, TrackingPolicyViolationDetectedEvent, TrackingTimestamp,
};
use ocentra_tracking_core::{
    TrackingAiBoundaryDecision, TrackingAlertDecision,
    TrackingParentNotificationDecisionState,
};

#[derive(Clone, Debug, PartialEq)]
pub struct TrackingRuntimeEventFlowReport {
    pub tracking_subscription_report: SubscriptionReport,
    pub child_ai_subscription_report: SubscriptionReport,
    pub child_policy_subscription_report: SubscriptionReport,
    pub child_expected_place_policy_subscription_report: SubscriptionReport,
    pub child_notification_subscription_report: SubscriptionReport,
    pub evidence_recorded: TrackingEvidenceRecordedEvent,
    pub geofence_transition_detected: Option<TrackingGeofenceTransitionDetectedEvent>,
    pub expected_place_state_evaluated: Option<TrackingExpectedPlaceStateEvaluatedEvent>,
    pub child_check_in_recorded: Option<TrackingChildCheckInRecordedEvent>,
    pub ai_analysis_requested: Option<TrackingAiAnalysisRequestedEvent>,
    pub nearby_place_classified: Option<TrackingNearbyPlaceClassifiedEvent>,
    pub ai_boundary_decision: Option<TrackingAiBoundaryDecision>,
    pub alert_decision: Option<TrackingAlertDecision>,
    pub policy_violation_detected: Option<TrackingPolicyViolationDetectedEvent>,
    pub parent_notification_requested: Option<ParentNotificationRequestedEvent>,
}

pub struct TrackingRuntimeEventFlow {
    bus: EventBus,
    state: TrackingRuntimeEventState,
    tracking_subscription_report: SubscriptionReport,
    child_ai_subscription_report: SubscriptionReport,
    child_policy_subscription_report: SubscriptionReport,
    child_expected_place_policy_subscription_report: SubscriptionReport,
    child_notification_subscription_report: SubscriptionReport,
}

impl TrackingRuntimeEventFlow {
    pub async fn new() -> Result<Self, EventingError> {
        Self::with_bus(EventBus::new()).await
    }

    pub async fn with_bus(bus: EventBus) -> Result<Self, EventingError> {
        let state = TrackingRuntimeEventState::default();
        let tracking_subscription_report =
            subscribe_tracking_location_observed_events(&bus, state.clone()).await?;
        let child_ai_subscription_report =
            subscribe_child_ai_tracking_analysis_events(&bus, state.clone()).await?;
        let child_expected_place_policy_subscription_report =
            subscribe_child_policy_tracking_expected_place_events(&bus, state.clone()).await?;
        let child_policy_subscription_report =
            subscribe_child_policy_tracking_analysis_events(&bus, state.clone()).await?;
        let child_notification_subscription_report =
            subscribe_child_notification_policy_events(&bus, state.clone()).await?;

        Ok(Self {
            bus,
            state,
            tracking_subscription_report,
            child_ai_subscription_report,
            child_policy_subscription_report,
            child_expected_place_policy_subscription_report,
            child_notification_subscription_report,
        })
    }

    pub async fn publish_location_observed(
        &self,
        event: TrackingLocationObservedEvent,
    ) -> Result<TrackingRuntimeEventFlowReport, EventingError> {
        self.state.reset_for_new_observation();
        let correlation_suffix = event.observation_id.as_str().to_owned();
        let recorded_at = event.observed_at.clone();
        self.bus
            .publish(
                event,
                tracking_runtime_metadata(
                    TrackingRuntimeHop::LocationObserved,
                    &correlation_suffix,
                    &recorded_at,
                )?,
            )
            .await?;

        self.report()
    }

    pub async fn metrics_snapshot(&self) -> ocentra_eventing::EventMetricsSnapshot {
        self.bus.metrics_snapshot().await
    }

    fn report(&self) -> Result<TrackingRuntimeEventFlowReport, EventingError> {
        Ok(TrackingRuntimeEventFlowReport {
            tracking_subscription_report: self.tracking_subscription_report.clone(),
            child_ai_subscription_report: self.child_ai_subscription_report.clone(),
            child_policy_subscription_report: self.child_policy_subscription_report.clone(),
            child_expected_place_policy_subscription_report: self
                .child_expected_place_policy_subscription_report
                .clone(),
            child_notification_subscription_report: self
                .child_notification_subscription_report
                .clone(),
            evidence_recorded: self.state.evidence_recorded()?,
            geofence_transition_detected: self.state.geofence_transition_detected(),
            expected_place_state_evaluated: self.state.expected_place_state_evaluated(),
            child_check_in_recorded: self.state.child_check_in_recorded(),
            ai_analysis_requested: self.state.ai_analysis_requested(),
            nearby_place_classified: self.state.nearby_place_classified(),
            ai_boundary_decision: self.state.ai_boundary_decision(),
            alert_decision: self.state.alert_decision(),
            policy_violation_detected: self.state.policy_violation_detected(),
            parent_notification_requested: self.state.parent_notification_requested(),
        })
    }
}

pub async fn publish_child_tracking_location_observed_event(
    event: TrackingLocationObservedEvent,
) -> Result<TrackingRuntimeEventFlowReport, EventingError> {
    TrackingRuntimeEventFlow::new()
        .await?
        .publish_location_observed(event)
        .await
}

async fn subscribe_tracking_location_observed_events(
    bus: &EventBus,
    state: TrackingRuntimeEventState,
) -> Result<SubscriptionReport, EventingError> {
    bus.subscribe::<TrackingLocationObservedEvent, _, _>(
        EventSubscriber::new(
            SubscriberId::parse(constants::tracking_runtime::SUBSCRIBER_CHILD_TRACKING_OBSERVER)?,
            EventType::parse(constants::tracking_runtime::TRACKING_LOCATION_OBSERVED_EVENT_TYPE)?,
            TargetHandler::parse(
                constants::tracking_runtime::TARGET_HANDLER_CHILD_TRACKING_OBSERVER,
            )?,
        ),
        move |context| {
            let state = state.clone();
            async move {
                let validation =
                    ocentra_tracking_core::validate_tracking_location_observation(
                        context.payload(),
                    );
                if validation.result_state
                    == ocentra_tracking_core::TrackingLocationValidationResultState::Rejected
                {
                    return Err(EventingError::InvalidValue {
                        field: "tracking.location.validation",
                        value: validation.validation_state.to_string(),
                    });
                }

                state.record_location_observed(context.payload().clone());
                let observation_report =
                    ocentra_tracking_core::observe_tracking_location(context.payload().clone());
                let evidence = observation_report.evidence_recorded.clone();
                state.record_evidence(evidence.clone());
                context
                    .publisher()
                    .publish(
                        evidence.clone(),
                        tracking_runtime_metadata(
                            TrackingRuntimeHop::EvidenceRecorded,
                            evidence.evidence_ref.as_str(),
                            &evidence.source_observed_at,
                        )?,
                    )
                    .await?;

                let geofence =
                    ocentra_tracking_core::tracking_geofence_transition_from_evidence(&evidence);
                state.record_geofence_transition(geofence.clone());
                context
                    .publisher()
                    .publish(
                        geofence.clone(),
                        tracking_runtime_metadata(
                            TrackingRuntimeHop::GeofenceTransitionDetected,
                            geofence.transition_id.as_str(),
                            &geofence.source_observed_at,
                        )?,
                    )
                    .await?;

                let expected_place =
                    ocentra_tracking_core::tracking_expected_place_state_from_evidence(&evidence);
                state.record_expected_place_state(expected_place.clone());
                context
                    .publisher()
                    .publish(
                        expected_place.clone(),
                        tracking_runtime_metadata(
                            TrackingRuntimeHop::ExpectedPlaceStateEvaluated,
                            expected_place.evaluation_id.as_str(),
                            &expected_place.source_observed_at,
                        )?,
                    )
                    .await?;

                let check_in = ocentra_tracking_core::tracking_child_check_in_from_location(
                    &observation_report.location_observed,
                    vec![evidence.evidence_ref.clone()],
                );
                state.record_child_check_in(check_in.clone());
                context
                    .publisher()
                    .publish(
                        check_in.clone(),
                        tracking_runtime_metadata(
                            TrackingRuntimeHop::ChildCheckInRecorded,
                            check_in.check_in_id.as_str(),
                            &check_in.checked_in_at,
                        )?,
                    )
                    .await?;

                if let Some(ai_request) = observation_report.ai_analysis_requested {
                    state.record_ai_analysis_request(ai_request.clone());
                    context
                        .publisher()
                        .publish(
                            ai_request.clone(),
                            tracking_runtime_metadata(
                                TrackingRuntimeHop::AiAnalysisRequested,
                                ai_request.ai_request_id.as_str(),
                                &ai_request.source_observed_at,
                            )?,
                        )
                        .await?;
                }
                Ok(())
            }
        },
    )
    .await
}

async fn subscribe_child_ai_tracking_analysis_events(
    bus: &EventBus,
    state: TrackingRuntimeEventState,
) -> Result<SubscriptionReport, EventingError> {
    bus.subscribe::<TrackingAiAnalysisRequestedEvent, _, _>(
        EventSubscriber::new(
            SubscriberId::parse(
                constants::tracking_runtime::SUBSCRIBER_CHILD_AI_TRACKING_ANALYZER,
            )?,
            EventType::parse(
                constants::tracking_runtime::TRACKING_AI_ANALYSIS_REQUESTED_EVENT_TYPE,
            )?,
            TargetHandler::parse(
                constants::tracking_runtime::TARGET_HANDLER_CHILD_AI_TRACKING_ANALYZER,
            )?,
        ),
        move |context| {
            let state = state.clone();
            async move {
                let classified =
                    ocentra_child_ai_core::classify_tracking_nearby_place(context.payload())?;
                state.record_nearby_place_classified(classified.clone());
                context
                    .publisher()
                    .publish(
                        classified.clone(),
                        tracking_runtime_metadata(
                            TrackingRuntimeHop::NearbyPlaceClassified,
                            classified.source_ai_request_id.as_str(),
                            &classified.source_observed_at,
                        )?,
                    )
                    .await?;
                Ok(())
            }
        },
    )
    .await
}

async fn subscribe_child_policy_tracking_analysis_events(
    bus: &EventBus,
    state: TrackingRuntimeEventState,
) -> Result<SubscriptionReport, EventingError> {
    bus.subscribe::<TrackingNearbyPlaceClassifiedEvent, _, _>(
        EventSubscriber::new(
            SubscriberId::parse(
                constants::tracking_runtime::SUBSCRIBER_CHILD_POLICY_TRACKING_ANALYZER,
            )?,
            EventType::parse(
                constants::tracking_runtime::TRACKING_NEARBY_PLACE_CLASSIFIED_EVENT_TYPE,
            )?,
            TargetHandler::parse(
                constants::tracking_runtime::TARGET_HANDLER_CHILD_POLICY_TRACKING_ANALYZER,
            )?,
        ),
        move |context| {
            let state = state.clone();
            async move {
                let Some(ai_request) = state.ai_analysis_requested() else {
                    return Ok(());
                };
                let ai_boundary_decision =
                    ocentra_tracking_core::validate_tracking_ai_result_as_evidence(
                        &ai_request,
                        context.payload(),
                    );
                state.record_ai_boundary_decision(ai_boundary_decision.clone());
                if ai_boundary_decision.decision_state
                    != constants::tracking_runtime::AI_RESULT_ACCEPTED_AS_EVIDENCE
                {
                    return Ok(());
                }
                let policy_decision =
                    ocentra_child_policy_core::evaluate_tracking_nearby_place_policy(
                        context.payload(),
                    );
                if let Some(violation) = policy_decision.policy_violation_detected {
                    state.record_policy_violation_detected(violation.clone());
                    context
                        .publisher()
                        .publish(
                            violation.clone(),
                            tracking_runtime_metadata(
                                TrackingRuntimeHop::PolicyViolationDetected,
                                violation.violation_id.as_str(),
                                &violation.detected_at,
                            )?,
                        )
                        .await?;
                }
                Ok(())
            }
        },
    )
    .await
}

async fn subscribe_child_policy_tracking_expected_place_events(
    bus: &EventBus,
    state: TrackingRuntimeEventState,
) -> Result<SubscriptionReport, EventingError> {
    bus.subscribe::<TrackingExpectedPlaceStateEvaluatedEvent, _, _>(
        EventSubscriber::new(
            SubscriberId::parse(
                constants::tracking_runtime::SUBSCRIBER_CHILD_POLICY_EXPECTED_PLACE_EVALUATOR,
            )?,
            EventType::parse(
                constants::tracking_runtime::TRACKING_EXPECTED_PLACE_STATE_EVALUATED_EVENT_TYPE,
            )?,
            TargetHandler::parse(
                constants::tracking_runtime::TARGET_HANDLER_CHILD_POLICY_EXPECTED_PLACE_EVALUATOR,
            )?,
        ),
        move |context| {
            let state = state.clone();
            async move {
                let policy_decision =
                    ocentra_child_policy_core::evaluate_tracking_expected_place_policy(
                        context.payload(),
                    );
                if let Some(violation) = policy_decision.policy_violation_detected {
                    state.record_policy_violation_detected(violation.clone());
                    context
                        .publisher()
                        .publish(
                            violation.clone(),
                            tracking_runtime_metadata(
                                TrackingRuntimeHop::PolicyViolationDetected,
                                violation.violation_id.as_str(),
                                &violation.detected_at,
                            )?,
                        )
                        .await?;
                }
                Ok(())
            }
        },
    )
    .await
}

async fn subscribe_child_notification_policy_events(
    bus: &EventBus,
    state: TrackingRuntimeEventState,
) -> Result<SubscriptionReport, EventingError> {
    bus.subscribe::<TrackingPolicyViolationDetectedEvent, _, _>(
        EventSubscriber::new(
            SubscriberId::parse(
                constants::tracking_runtime::SUBSCRIBER_CHILD_NOTIFICATION_POLICY_BRIDGE,
            )?,
            EventType::parse(
                constants::tracking_runtime::TRACKING_POLICY_VIOLATION_DETECTED_EVENT_TYPE,
            )?,
            TargetHandler::parse(
                constants::tracking_runtime::TARGET_HANDLER_CHILD_NOTIFICATION_POLICY_BRIDGE,
            )?,
        ),
        move |context| {
            let state = state.clone();
            async move {
                let Some(location_observed) = state.location_observed() else {
                    return Ok(());
                };
                if location_observed.config.notification_mode == TrackingNotificationMode::Disabled {
                    return Ok(());
                }
                let recent_duplicate_count =
                    state.recent_policy_violation_duplicate_count(context.payload());
                let alert_decision = ocentra_tracking_core::evaluate_tracking_alert(
                    context.payload(),
                    recent_duplicate_count,
                );
                state.record_alert_decision(alert_decision.clone());
                state.record_policy_violation_history(context.payload().clone());
                if alert_decision.parent_notification_state
                    != TrackingParentNotificationDecisionState::Allowed
                {
                    return Ok(());
                }

                let notification =
                    ocentra_child_notification_core::request_parent_notification_from_policy_violation(
                        context.payload(),
                    );
                state.record_parent_notification_requested(notification.clone());
                context
                    .publisher()
                    .publish(
                            notification.clone(),
                            tracking_runtime_metadata(
                                TrackingRuntimeHop::ParentNotificationRequested,
                                notification.notification_id.as_str(),
                                &notification.requested_at,
                            )?,
                        )
                        .await?;
                Ok(())
            }
        },
    )
    .await
}

#[derive(Clone, Debug, Default)]
struct TrackingRuntimeEventState {
    location_observed: Arc<Mutex<Option<TrackingLocationObservedEvent>>>,
    evidence_recorded: Arc<Mutex<Option<TrackingEvidenceRecordedEvent>>>,
    geofence_transition_detected: Arc<Mutex<Option<TrackingGeofenceTransitionDetectedEvent>>>,
    expected_place_state_evaluated: Arc<Mutex<Option<TrackingExpectedPlaceStateEvaluatedEvent>>>,
    child_check_in_recorded: Arc<Mutex<Option<TrackingChildCheckInRecordedEvent>>>,
    ai_analysis_requested: Arc<Mutex<Option<TrackingAiAnalysisRequestedEvent>>>,
    nearby_place_classified: Arc<Mutex<Option<TrackingNearbyPlaceClassifiedEvent>>>,
    ai_boundary_decision: Arc<Mutex<Option<TrackingAiBoundaryDecision>>>,
    alert_decision: Arc<Mutex<Option<TrackingAlertDecision>>>,
    policy_violation_detected: Arc<Mutex<Option<TrackingPolicyViolationDetectedEvent>>>,
    parent_notification_requested: Arc<Mutex<Option<ParentNotificationRequestedEvent>>>,
    policy_violation_history: Arc<Mutex<Vec<TrackingPolicyViolationDetectedEvent>>>,
}

impl TrackingRuntimeEventState {
    fn reset_for_new_observation(&self) {
        *self
            .location_observed
            .lock()
            .expect(constants::tracking_runtime::ERROR_TRACKING_RUNTIME_FLOW_RECORDED) = None;
        *self
            .evidence_recorded
            .lock()
            .expect(constants::tracking_runtime::ERROR_TRACKING_RUNTIME_FLOW_RECORDED) = None;
        *self
            .geofence_transition_detected
            .lock()
            .expect(constants::tracking_runtime::ERROR_TRACKING_RUNTIME_FLOW_RECORDED) = None;
        *self
            .expected_place_state_evaluated
            .lock()
            .expect(constants::tracking_runtime::ERROR_TRACKING_RUNTIME_FLOW_RECORDED) = None;
        *self
            .child_check_in_recorded
            .lock()
            .expect(constants::tracking_runtime::ERROR_TRACKING_RUNTIME_FLOW_RECORDED) = None;
        *self
            .ai_analysis_requested
            .lock()
            .expect(constants::tracking_runtime::ERROR_TRACKING_RUNTIME_FLOW_RECORDED) = None;
        *self
            .nearby_place_classified
            .lock()
            .expect(constants::tracking_runtime::ERROR_TRACKING_RUNTIME_FLOW_RECORDED) = None;
        *self
            .ai_boundary_decision
            .lock()
            .expect(constants::tracking_runtime::ERROR_TRACKING_RUNTIME_FLOW_RECORDED) = None;
        *self
            .alert_decision
            .lock()
            .expect(constants::tracking_runtime::ERROR_TRACKING_RUNTIME_FLOW_RECORDED) = None;
        *self
            .policy_violation_detected
            .lock()
            .expect(constants::tracking_runtime::ERROR_TRACKING_RUNTIME_FLOW_RECORDED) = None;
        *self
            .parent_notification_requested
            .lock()
            .expect(constants::tracking_runtime::ERROR_TRACKING_RUNTIME_FLOW_RECORDED) = None;
    }

    fn record_location_observed(&self, event: TrackingLocationObservedEvent) {
        *self
            .location_observed
            .lock()
            .expect(constants::tracking_runtime::ERROR_TRACKING_RUNTIME_FLOW_RECORDED) =
            Some(event);
    }

    fn record_evidence(&self, event: TrackingEvidenceRecordedEvent) {
        *self
            .evidence_recorded
            .lock()
            .expect(constants::tracking_runtime::ERROR_TRACKING_RUNTIME_FLOW_RECORDED) =
            Some(event);
    }

    fn record_ai_analysis_request(&self, event: TrackingAiAnalysisRequestedEvent) {
        *self
            .ai_analysis_requested
            .lock()
            .expect(constants::tracking_runtime::ERROR_TRACKING_RUNTIME_FLOW_RECORDED) =
            Some(event);
    }

    fn record_geofence_transition(&self, event: TrackingGeofenceTransitionDetectedEvent) {
        *self
            .geofence_transition_detected
            .lock()
            .expect(constants::tracking_runtime::ERROR_TRACKING_RUNTIME_FLOW_RECORDED) =
            Some(event);
    }

    fn record_expected_place_state(&self, event: TrackingExpectedPlaceStateEvaluatedEvent) {
        *self
            .expected_place_state_evaluated
            .lock()
            .expect(constants::tracking_runtime::ERROR_TRACKING_RUNTIME_FLOW_RECORDED) =
            Some(event);
    }

    fn record_child_check_in(&self, event: TrackingChildCheckInRecordedEvent) {
        *self
            .child_check_in_recorded
            .lock()
            .expect(constants::tracking_runtime::ERROR_TRACKING_RUNTIME_FLOW_RECORDED) =
            Some(event);
    }

    fn record_nearby_place_classified(&self, event: TrackingNearbyPlaceClassifiedEvent) {
        *self
            .nearby_place_classified
            .lock()
            .expect(constants::tracking_runtime::ERROR_TRACKING_RUNTIME_FLOW_RECORDED) =
            Some(event);
    }

    fn record_ai_boundary_decision(&self, decision: TrackingAiBoundaryDecision) {
        *self
            .ai_boundary_decision
            .lock()
            .expect(constants::tracking_runtime::ERROR_TRACKING_RUNTIME_FLOW_RECORDED) =
            Some(decision);
    }

    fn record_alert_decision(&self, decision: TrackingAlertDecision) {
        *self
            .alert_decision
            .lock()
            .expect(constants::tracking_runtime::ERROR_TRACKING_RUNTIME_FLOW_RECORDED) =
            Some(decision);
    }

    fn record_policy_violation_detected(&self, event: TrackingPolicyViolationDetectedEvent) {
        *self
            .policy_violation_detected
            .lock()
            .expect(constants::tracking_runtime::ERROR_TRACKING_RUNTIME_FLOW_RECORDED) =
            Some(event);
    }

    fn record_parent_notification_requested(&self, event: ParentNotificationRequestedEvent) {
        *self
            .parent_notification_requested
            .lock()
            .expect(constants::tracking_runtime::ERROR_TRACKING_RUNTIME_FLOW_RECORDED) =
            Some(event);
    }

    fn record_policy_violation_history(&self, event: TrackingPolicyViolationDetectedEvent) {
        let mut history = self
            .policy_violation_history
            .lock()
            .expect(constants::tracking_runtime::ERROR_TRACKING_RUNTIME_FLOW_RECORDED);
        if history.len() >= 32 {
            history.remove(0);
        }
        history.push(event);
    }

    fn evidence_recorded(&self) -> Result<TrackingEvidenceRecordedEvent, EventingError> {
        required_runtime_flow_event(&self.evidence_recorded)
    }

    fn location_observed(&self) -> Option<TrackingLocationObservedEvent> {
        self.location_observed.lock().ok()?.clone()
    }

    fn geofence_transition_detected(&self) -> Option<TrackingGeofenceTransitionDetectedEvent> {
        self.geofence_transition_detected.lock().ok()?.clone()
    }

    fn expected_place_state_evaluated(&self) -> Option<TrackingExpectedPlaceStateEvaluatedEvent> {
        self.expected_place_state_evaluated.lock().ok()?.clone()
    }

    fn child_check_in_recorded(&self) -> Option<TrackingChildCheckInRecordedEvent> {
        self.child_check_in_recorded.lock().ok()?.clone()
    }

    fn ai_analysis_requested(&self) -> Option<TrackingAiAnalysisRequestedEvent> {
        self.ai_analysis_requested.lock().ok()?.clone()
    }

    fn nearby_place_classified(&self) -> Option<TrackingNearbyPlaceClassifiedEvent> {
        self.nearby_place_classified.lock().ok()?.clone()
    }

    fn ai_boundary_decision(&self) -> Option<TrackingAiBoundaryDecision> {
        self.ai_boundary_decision.lock().ok()?.clone()
    }

    fn alert_decision(&self) -> Option<TrackingAlertDecision> {
        self.alert_decision.lock().ok()?.clone()
    }

    fn policy_violation_detected(&self) -> Option<TrackingPolicyViolationDetectedEvent> {
        self.policy_violation_detected.lock().ok()?.clone()
    }

    fn parent_notification_requested(&self) -> Option<ParentNotificationRequestedEvent> {
        self.parent_notification_requested.lock().ok()?.clone()
    }

    fn recent_policy_violation_duplicate_count(
        &self,
        event: &TrackingPolicyViolationDetectedEvent,
    ) -> u16 {
        let Ok(history) = self.policy_violation_history.lock() else {
            return 0;
        };
        history
            .iter()
            .filter(|prior| same_policy_violation(prior, event))
            .count()
            .min(u16::MAX as usize) as u16
    }
}

fn required_runtime_flow_event<E>(value: &Arc<Mutex<Option<E>>>) -> Result<E, EventingError>
where
    E: Clone,
{
    value
        .lock()
        .expect(constants::tracking_runtime::ERROR_TRACKING_RUNTIME_FLOW_RECORDED)
        .clone()
        .ok_or_else(|| EventingError::InvalidValue {
            field: constants::tracking_runtime::ERROR_TRACKING_RUNTIME_FLOW_RECORDED,
            value: constants::tracking_runtime::TRACKING_LOCATION_OBSERVED_EVENT_TYPE.to_string(),
        })
}

fn same_policy_violation(
    left: &TrackingPolicyViolationDetectedEvent,
    right: &TrackingPolicyViolationDetectedEvent,
) -> bool {
    left.child_device_id == right.child_device_id
        && left.child_profile_id == right.child_profile_id
        && left.policy_rule_ref == right.policy_rule_ref
        && left.severity == right.severity
        && left.evidence_refs == right.evidence_refs
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum TrackingRuntimeHop {
    LocationObserved,
    EvidenceRecorded,
    GeofenceTransitionDetected,
    ExpectedPlaceStateEvaluated,
    ChildCheckInRecorded,
    AiAnalysisRequested,
    NearbyPlaceClassified,
    PolicyViolationDetected,
    ParentNotificationRequested,
}

impl TrackingRuntimeHop {
    fn source_component(self) -> &'static str {
        match self {
            Self::LocationObserved
            | Self::EvidenceRecorded
            | Self::GeofenceTransitionDetected
            | Self::ExpectedPlaceStateEvaluated
            | Self::ChildCheckInRecorded
            | Self::AiAnalysisRequested => {
                constants::tracking_runtime::SOURCE_COMPONENT_CHILD_TRACKING_RUNTIME
            }
            Self::NearbyPlaceClassified => {
                constants::tracking_runtime::SOURCE_COMPONENT_CHILD_AI_RUNTIME
            }
            Self::PolicyViolationDetected => {
                constants::tracking_runtime::SOURCE_COMPONENT_CHILD_POLICY_RUNTIME
            }
            Self::ParentNotificationRequested => {
                constants::tracking_runtime::SOURCE_COMPONENT_CHILD_NOTIFICATION_RUNTIME
            }
        }
    }

    fn runtime_role(self) -> &'static str {
        match self {
            Self::LocationObserved
            | Self::EvidenceRecorded
            | Self::GeofenceTransitionDetected
            | Self::ExpectedPlaceStateEvaluated
            | Self::ChildCheckInRecorded
            | Self::AiAnalysisRequested => constants::eventing_source::ROLE_AGENT,
            Self::NearbyPlaceClassified => constants::eventing_source::ROLE_ANALYZER,
            Self::PolicyViolationDetected => constants::eventing_source::ROLE_DECISION_ENGINE,
            Self::ParentNotificationRequested => {
                constants::eventing_source::ROLE_SIDE_EFFECT_ADAPTER
            }
        }
    }

    fn target_handler(self) -> &'static str {
        match self {
            Self::LocationObserved
            | Self::EvidenceRecorded
            | Self::GeofenceTransitionDetected
            | Self::ExpectedPlaceStateEvaluated
            | Self::ChildCheckInRecorded => {
                constants::tracking_runtime::TARGET_HANDLER_CHILD_TRACKING_OBSERVER
            }
            Self::AiAnalysisRequested => {
                constants::tracking_runtime::TARGET_HANDLER_CHILD_AI_TRACKING_ANALYZER
            }
            Self::NearbyPlaceClassified => {
                constants::tracking_runtime::TARGET_HANDLER_CHILD_POLICY_TRACKING_ANALYZER
            }
            Self::PolicyViolationDetected | Self::ParentNotificationRequested => {
                constants::tracking_runtime::TARGET_HANDLER_CHILD_NOTIFICATION_POLICY_BRIDGE
            }
        }
    }

}

fn tracking_runtime_metadata(
    hop: TrackingRuntimeHop,
    correlation_suffix: &str,
    recorded_at: &TrackingTimestamp,
) -> Result<EventMetadata, EventingError> {
    Ok(EventMetadata::from_parts(
        EventId::generated(),
        tracking_runtime_correlation_id(correlation_suffix)?,
        EventSource::new(
            EventCustody::parse(constants::child_agent::CUSTODY_CHILD_AGENT_RUNTIME)?,
            RuntimeRole::parse(hop.runtime_role())?,
            SourceService::parse(constants::peer::LOCAL_DEV_AGENT)?,
            SourceComponent::parse(hop.source_component())?,
            RuntimeInstanceId::parse(constants::child_agent::RUNTIME_INSTANCE_LOCAL_CHILD_AGENT)?,
        ),
        RecordedAt::parse(recorded_at.as_str())?,
        Some(TargetHandler::parse(hop.target_handler())?),
    ))
}

fn tracking_runtime_correlation_id(correlation_suffix: &str) -> Result<CorrelationId, EventingError> {
    let mut value = String::from(constants::tracking_runtime::CORRELATION_PREFIX);
    value.push_str(correlation_suffix);
    CorrelationId::parse(value)
}
