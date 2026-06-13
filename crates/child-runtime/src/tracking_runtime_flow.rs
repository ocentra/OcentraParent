use std::sync::{Arc, Mutex};

use ocentra_eventing::{
    CorrelationId, EventBus, EventCustody, EventId, EventMetadata, EventSource, EventSubscriber,
    EventType, EventingError, RecordedAt, RuntimeInstanceId, RuntimeRole, SourceComponent,
    SourceService, SubscriberId, SubscriptionReport, TargetHandler,
};
use ocentra_parent_agent_protocol::{
    constants, ParentNotificationRequestedEvent, TrackingAiAnalysisRequestedEvent,
    TrackingEvidenceRecordedEvent, TrackingLocationObservedEvent,
    TrackingNearbyPlaceClassifiedEvent, TrackingPolicyViolationDetectedEvent,
};
use ocentra_tracking_core::TrackingAiBoundaryDecision;

#[derive(Clone, Debug, PartialEq)]
pub struct TrackingRuntimeEventFlowReport {
    pub tracking_subscription_report: SubscriptionReport,
    pub child_ai_subscription_report: SubscriptionReport,
    pub child_policy_subscription_report: SubscriptionReport,
    pub child_notification_subscription_report: SubscriptionReport,
    pub evidence_recorded: TrackingEvidenceRecordedEvent,
    pub ai_analysis_requested: Option<TrackingAiAnalysisRequestedEvent>,
    pub nearby_place_classified: Option<TrackingNearbyPlaceClassifiedEvent>,
    pub ai_boundary_decision: Option<TrackingAiBoundaryDecision>,
    pub policy_violation_detected: Option<TrackingPolicyViolationDetectedEvent>,
    pub parent_notification_requested: Option<ParentNotificationRequestedEvent>,
}

pub async fn publish_child_tracking_location_observed_event(
    event: TrackingLocationObservedEvent,
) -> Result<TrackingRuntimeEventFlowReport, EventingError> {
    let bus = EventBus::new();
    let state = TrackingRuntimeEventState::default();
    let tracking_subscription_report =
        subscribe_tracking_location_observed_events(&bus, state.clone()).await?;
    let child_ai_subscription_report =
        subscribe_child_ai_tracking_analysis_events(&bus, state.clone()).await?;
    let child_policy_subscription_report =
        subscribe_child_policy_tracking_analysis_events(&bus, state.clone()).await?;
    let child_notification_subscription_report =
        subscribe_child_notification_policy_events(&bus, state.clone()).await?;

    bus.publish(
        event,
        tracking_runtime_metadata(TrackingRuntimeHop::LocationObserved)?,
    )
    .await?;

    Ok(TrackingRuntimeEventFlowReport {
        tracking_subscription_report,
        child_ai_subscription_report,
        child_policy_subscription_report,
        child_notification_subscription_report,
        evidence_recorded: state.evidence_recorded()?,
        ai_analysis_requested: state.ai_analysis_requested(),
        nearby_place_classified: state.nearby_place_classified(),
        ai_boundary_decision: state.ai_boundary_decision(),
        policy_violation_detected: state.policy_violation_detected(),
        parent_notification_requested: state.parent_notification_requested(),
    })
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
                let report = ocentra_tracking_core::observe_tracking_location(
                    context.payload().clone(),
                );
                state.record_evidence(report.evidence_recorded.clone());
                context
                    .publisher()
                    .publish(
                        report.evidence_recorded,
                        tracking_runtime_metadata(TrackingRuntimeHop::EvidenceRecorded)?,
                    )
                    .await?;
                if let Some(ai_request) = report.ai_analysis_requested {
                    state.record_ai_analysis_request(ai_request.clone());
                    context
                        .publisher()
                        .publish(
                            ai_request,
                            tracking_runtime_metadata(TrackingRuntimeHop::AiAnalysisRequested)?,
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
            SubscriberId::parse(constants::tracking_runtime::SUBSCRIBER_CHILD_AI_TRACKING_ANALYZER)?,
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
                    ocentra_child_ai_core::classify_tracking_nearby_place(context.payload());
                state.record_nearby_place_classified(classified.clone());
                context
                    .publisher()
                    .publish(
                        classified,
                        tracking_runtime_metadata(TrackingRuntimeHop::NearbyPlaceClassified)?,
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
                            violation,
                            tracking_runtime_metadata(TrackingRuntimeHop::PolicyViolationDetected)?,
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
                let notification =
                    ocentra_child_notification_core::request_parent_notification_from_policy_violation(
                        context.payload(),
                    );
                state.record_parent_notification_requested(notification.clone());
                context
                    .publisher()
                    .publish(
                        notification,
                        tracking_runtime_metadata(TrackingRuntimeHop::ParentNotificationRequested)?,
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
    evidence_recorded: Arc<Mutex<Option<TrackingEvidenceRecordedEvent>>>,
    ai_analysis_requested: Arc<Mutex<Option<TrackingAiAnalysisRequestedEvent>>>,
    nearby_place_classified: Arc<Mutex<Option<TrackingNearbyPlaceClassifiedEvent>>>,
    ai_boundary_decision: Arc<Mutex<Option<TrackingAiBoundaryDecision>>>,
    policy_violation_detected: Arc<Mutex<Option<TrackingPolicyViolationDetectedEvent>>>,
    parent_notification_requested: Arc<Mutex<Option<ParentNotificationRequestedEvent>>>,
}

impl TrackingRuntimeEventState {
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

    fn evidence_recorded(&self) -> Result<TrackingEvidenceRecordedEvent, EventingError> {
        required_runtime_flow_event(&self.evidence_recorded)
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

    fn policy_violation_detected(&self) -> Option<TrackingPolicyViolationDetectedEvent> {
        self.policy_violation_detected.lock().ok()?.clone()
    }

    fn parent_notification_requested(&self) -> Option<ParentNotificationRequestedEvent> {
        self.parent_notification_requested.lock().ok()?.clone()
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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum TrackingRuntimeHop {
    LocationObserved,
    EvidenceRecorded,
    AiAnalysisRequested,
    NearbyPlaceClassified,
    PolicyViolationDetected,
    ParentNotificationRequested,
}

impl TrackingRuntimeHop {
    fn source_component(self) -> &'static str {
        match self {
            Self::LocationObserved | Self::EvidenceRecorded | Self::AiAnalysisRequested => {
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
            Self::LocationObserved | Self::EvidenceRecorded | Self::AiAnalysisRequested => {
                constants::eventing_source::ROLE_AGENT
            }
            Self::NearbyPlaceClassified => constants::eventing_source::ROLE_ANALYZER,
            Self::PolicyViolationDetected => constants::eventing_source::ROLE_DECISION_ENGINE,
            Self::ParentNotificationRequested => {
                constants::eventing_source::ROLE_SIDE_EFFECT_ADAPTER
            }
        }
    }

    fn target_handler(self) -> &'static str {
        match self {
            Self::LocationObserved | Self::EvidenceRecorded => {
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

    fn correlation_suffix(self) -> &'static str {
        match self {
            Self::LocationObserved => constants::tracking_runtime::DEFAULT_OBSERVATION_ID,
            Self::EvidenceRecorded => constants::tracking_runtime::DEFAULT_EVIDENCE_REF,
            Self::AiAnalysisRequested | Self::NearbyPlaceClassified => {
                constants::tracking_runtime::DEFAULT_AI_REQUEST_ID
            }
            Self::PolicyViolationDetected => {
                constants::tracking_runtime::DEFAULT_POLICY_VIOLATION_ID
            }
            Self::ParentNotificationRequested => {
                constants::tracking_runtime::DEFAULT_NOTIFICATION_ID
            }
        }
    }
}

fn tracking_runtime_metadata(hop: TrackingRuntimeHop) -> Result<EventMetadata, EventingError> {
    Ok(EventMetadata::from_parts(
        EventId::generated(),
        tracking_runtime_correlation_id(hop)?,
        EventSource::new(
            EventCustody::parse(constants::child_agent::CUSTODY_CHILD_AGENT_RUNTIME)?,
            RuntimeRole::parse(hop.runtime_role())?,
            SourceService::parse(constants::peer::LOCAL_DEV_AGENT)?,
            SourceComponent::parse(hop.source_component())?,
            RuntimeInstanceId::parse(constants::child_agent::RUNTIME_INSTANCE_LOCAL_CHILD_AGENT)?,
        ),
        RecordedAt::parse(constants::tracking_runtime::DEFAULT_OBSERVED_AT)?,
        Some(TargetHandler::parse(hop.target_handler())?),
    ))
}

fn tracking_runtime_correlation_id(hop: TrackingRuntimeHop) -> Result<CorrelationId, EventingError> {
    let mut value = String::from(constants::tracking_runtime::CORRELATION_PREFIX);
    value.push_str(hop.correlation_suffix());
    CorrelationId::parse(value)
}
