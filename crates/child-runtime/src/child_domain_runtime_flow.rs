use std::sync::{Arc, Mutex};

use ocentra_eventing::{
    CorrelationId, EventBus, EventCustody, EventId, EventMetadata, EventSource, EventSubscriber,
    EventType, EventingError, RecordedAt, RuntimeInstanceId, RuntimeRole, SourceComponent,
    SourceService, SubscriberId, SubscriptionReport, TargetHandler,
};
use ocentra_parent_agent_protocol::{
    child_domain_policy_evaluation_requested_from_ai_result_event_if_required, constants,
    ChildDomainAiAnalysisCompletedEvent, ChildDomainAiAnalysisRequestedEvent,
    ChildDomainAiRequestId, ChildDomainEventType, ChildDomainEvidenceRecordedEvent,
    ChildDomainEvidenceRef, ChildDomainNotificationRequestedEvent, ChildDomainObservationId,
    ChildDomainObservedAt, ChildDomainObservedEvent, ChildDomainPolicyEvaluationRequestedEvent,
    ChildDomainPolicyRequestId, ChildDomainPolicyViolationDetectedEvent,
    ChildDomainPolicyViolationId, ChildRuntimeDomain,
};

#[derive(Clone, Debug, PartialEq)]
pub struct ChildDomainRuntimeFlowReport {
    pub domain: ChildRuntimeDomain,
    pub observer_subscription_report: SubscriptionReport,
    pub ai_subscription_report: SubscriptionReport,
    pub ai_policy_subscription_report: SubscriptionReport,
    pub policy_subscription_report: SubscriptionReport,
    pub notification_subscription_report: SubscriptionReport,
    pub evidence_recorded: ChildDomainEvidenceRecordedEvent,
    pub ai_analysis_requested: Option<ChildDomainAiAnalysisRequestedEvent>,
    pub ai_analysis_completed: Option<ChildDomainAiAnalysisCompletedEvent>,
    pub policy_evaluation_requested: Option<ChildDomainPolicyEvaluationRequestedEvent>,
    pub policy_violation_detected: Option<ChildDomainPolicyViolationDetectedEvent>,
    pub notification_requested: Option<ChildDomainNotificationRequestedEvent>,
}

pub struct ChildDomainRuntimeEventFlow {
    bus: EventBus,
    domain: ChildRuntimeDomain,
    observer_subscription_report: SubscriptionReport,
    ai_subscription_report: SubscriptionReport,
    ai_policy_subscription_report: SubscriptionReport,
    policy_subscription_report: SubscriptionReport,
    notification_subscription_report: SubscriptionReport,
    state: ChildDomainRuntimeFlowState,
}

impl ChildDomainRuntimeEventFlow {
    pub async fn for_event(event: &ChildDomainObservedEvent) -> Result<Self, EventingError> {
        let bus = EventBus::new();
        let state = ChildDomainRuntimeFlowState::default();
        let observer_subscription_report =
            subscribe_child_domain_observer(&bus, event, state.clone()).await?;
        let ai_subscription_report = subscribe_child_domain_ai(&bus, event, state.clone()).await?;
        let ai_policy_subscription_report =
            subscribe_child_domain_ai_policy_bridge(&bus, state.clone()).await?;
        let policy_subscription_report =
            subscribe_child_domain_policy(&bus, event, state.clone()).await?;
        let notification_subscription_report =
            subscribe_child_domain_notification(&bus, state.clone()).await?;

        Ok(Self {
            bus,
            domain: event.domain,
            observer_subscription_report,
            ai_subscription_report,
            ai_policy_subscription_report,
            policy_subscription_report,
            notification_subscription_report,
            state,
        })
    }

    pub async fn publish_observed(
        &self,
        event: ChildDomainObservedEvent,
    ) -> Result<ChildDomainRuntimeFlowReport, EventingError> {
        let recorded_at = event.observed_at.clone();
        self.bus
            .publish(
                event.clone(),
                child_domain_runtime_metadata(
                    ChildDomainRuntimeHop::Observed(&event.observation_id),
                    &recorded_at,
                )?,
            )
            .await?;

        self.report()
    }

    pub async fn metrics_snapshot(&self) -> ocentra_eventing::EventMetricsSnapshot {
        self.bus.metrics_snapshot().await
    }

    fn report(&self) -> Result<ChildDomainRuntimeFlowReport, EventingError> {
        Ok(ChildDomainRuntimeFlowReport {
            domain: self.domain,
            observer_subscription_report: self.observer_subscription_report.clone(),
            ai_subscription_report: self.ai_subscription_report.clone(),
            ai_policy_subscription_report: self.ai_policy_subscription_report.clone(),
            policy_subscription_report: self.policy_subscription_report.clone(),
            notification_subscription_report: self.notification_subscription_report.clone(),
            evidence_recorded: self.state.evidence_recorded()?,
            ai_analysis_requested: self.state.ai_analysis_requested(),
            ai_analysis_completed: self.state.ai_analysis_completed(),
            policy_evaluation_requested: self.state.policy_evaluation_requested(),
            policy_violation_detected: self.state.policy_violation_detected(),
            notification_requested: self.state.notification_requested(),
        })
    }
}

pub async fn publish_default_child_domain_runtime_flows(
) -> Result<Vec<ChildDomainRuntimeFlowReport>, EventingError> {
    let events = vec![
        ocentra_app_core::default_app_observed_event(),
        ocentra_app_game_core::default_app_game_observed_event(),
        ocentra_browser_core::default_browser_observed_event(),
        ocentra_lan_core::default_lan_observed_event(),
        ocentra_network_core::default_network_observed_event(),
        ocentra_screen_core::default_screen_observed_event(),
        ocentra_screen_live_view_core::default_screen_live_view_observed_event(),
    ];
    let mut reports = Vec::new();
    for event in events {
        reports.push(publish_child_domain_observed_event(event).await?);
    }
    Ok(reports)
}

pub async fn publish_child_domain_observed_event(
    event: ChildDomainObservedEvent,
) -> Result<ChildDomainRuntimeFlowReport, EventingError> {
    ChildDomainRuntimeEventFlow::for_event(&event)
        .await?
        .publish_observed(event)
        .await
}

async fn subscribe_child_domain_observer(
    bus: &EventBus,
    event: &ChildDomainObservedEvent,
    state: ChildDomainRuntimeFlowState,
) -> Result<SubscriptionReport, EventingError> {
    bus.subscribe::<ChildDomainObservedEvent, _, _>(
        EventSubscriber::new(
            SubscriberId::parse(event.domain.observer_subscriber_id())?,
            EventType::parse(event.event_type.as_str())?,
            TargetHandler::parse(constants::child_domain_runtime::TARGET_HANDLER_DOMAIN_OBSERVER)?,
        ),
        move |context| {
            let state = state.clone();
            async move {
                let evidence = child_domain_evidence_recorded_event(context.payload())?;
                state.record_evidence(evidence.clone());
                context
                    .publisher()
                    .publish(
                        evidence.clone(),
                        child_domain_runtime_metadata(
                            ChildDomainRuntimeHop::EvidenceRecorded(&evidence.evidence_ref),
                            &evidence.source_observed_at,
                        )?,
                    )
                    .await?;
                if let Some(ai_request) = child_domain_ai_analysis_requested_event(&evidence)? {
                    state.record_ai_analysis_request(ai_request.clone());
                    context
                        .publisher()
                        .publish(
                            ai_request,
                            child_domain_runtime_metadata(
                                ChildDomainRuntimeHop::AiAnalysisRequested(&evidence.evidence_ref),
                                &evidence.source_observed_at,
                            )?,
                        )
                        .await?;
                }
                if let Some(policy_request) =
                    child_domain_policy_evaluation_requested_event(&evidence)?
                {
                    state.record_policy_evaluation_request(policy_request.clone());
                    context
                        .publisher()
                        .publish(
                            policy_request,
                            child_domain_runtime_metadata(
                                ChildDomainRuntimeHop::PolicyEvaluationRequested(
                                    &evidence.evidence_ref,
                                ),
                                &evidence.source_observed_at,
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

async fn subscribe_child_domain_ai(
    bus: &EventBus,
    event: &ChildDomainObservedEvent,
    state: ChildDomainRuntimeFlowState,
) -> Result<SubscriptionReport, EventingError> {
    bus.subscribe::<ChildDomainAiAnalysisRequestedEvent, _, _>(
        EventSubscriber::new(
            SubscriberId::parse(constants::child_domain_runtime::SUBSCRIBER_CHILD_AI_ANALYZER)?,
            EventType::parse(event.domain.ai_analysis_requested_event_type().as_str())?,
            TargetHandler::parse(
                constants::child_domain_runtime::TARGET_HANDLER_CHILD_AI_ANALYZER,
            )?,
        ),
        move |context| {
            let state = state.clone();
            async move {
                let completed =
                    ocentra_child_ai_core::complete_child_domain_ai_analysis(context.payload());
                state.record_ai_analysis_completed(completed.clone());
                context
                    .publisher()
                    .publish(
                        completed,
                        child_domain_runtime_metadata(
                            ChildDomainRuntimeHop::AiAnalysisCompleted(
                                &context.payload().ai_request_id,
                            ),
                            &context.payload().source_observed_at,
                        )?,
                    )
                    .await?;
                Ok(())
            }
        },
    )
    .await
}

async fn subscribe_child_domain_ai_policy_bridge(
    bus: &EventBus,
    state: ChildDomainRuntimeFlowState,
) -> Result<SubscriptionReport, EventingError> {
    bus.subscribe::<ChildDomainAiAnalysisCompletedEvent, _, _>(
        EventSubscriber::new(
            SubscriberId::parse(
                constants::child_domain_runtime::SUBSCRIBER_CHILD_AI_POLICY_BRIDGE,
            )?,
            EventType::parse(ChildDomainEventType::ai_analysis_completed().as_str())?,
            TargetHandler::parse(
                constants::child_domain_runtime::TARGET_HANDLER_CHILD_POLICY_EVALUATOR,
            )?,
        ),
        move |context| {
            let state = state.clone();
            async move {
                if let Some(policy_request) =
                    child_domain_policy_evaluation_requested_from_ai_result_event_if_required(
                        context.payload(),
                    )
                {
                    state.record_policy_evaluation_request(policy_request.clone());
                    context
                        .publisher()
                        .publish(
                            policy_request,
                            child_domain_runtime_metadata(
                                ChildDomainRuntimeHop::PolicyEvaluationRequestedFromAi(
                                    &context.payload().source_ai_request_id,
                                ),
                                &context.payload().source_observed_at,
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

async fn subscribe_child_domain_policy(
    bus: &EventBus,
    event: &ChildDomainObservedEvent,
    state: ChildDomainRuntimeFlowState,
) -> Result<SubscriptionReport, EventingError> {
    bus.subscribe::<ChildDomainPolicyEvaluationRequestedEvent, _, _>(
        EventSubscriber::new(
            SubscriberId::parse(
                constants::child_domain_runtime::SUBSCRIBER_CHILD_POLICY_EVALUATOR,
            )?,
            EventType::parse(
                event
                    .domain
                    .policy_evaluation_requested_event_type()
                    .as_str(),
            )?,
            TargetHandler::parse(
                constants::child_domain_runtime::TARGET_HANDLER_CHILD_POLICY_EVALUATOR,
            )?,
        ),
        move |context| {
            let state = state.clone();
            async move {
                let violation =
                    ocentra_child_policy_core::child_domain_policy::evaluate_child_domain_policy(
                        context.payload(),
                    )?;
                state.record_policy_violation(violation.clone());
                context
                    .publisher()
                    .publish(
                        violation,
                        child_domain_runtime_metadata(
                            ChildDomainRuntimeHop::PolicyViolationDetected(
                                &context.payload().policy_request_id,
                            ),
                            &context.payload().source_observed_at,
                        )?,
                    )
                    .await?;
                Ok(())
            }
        },
    )
    .await
}

async fn subscribe_child_domain_notification(
    bus: &EventBus,
    state: ChildDomainRuntimeFlowState,
) -> Result<SubscriptionReport, EventingError> {
    bus.subscribe::<ChildDomainPolicyViolationDetectedEvent, _, _>(
        EventSubscriber::new(
            SubscriberId::parse(
                constants::child_domain_runtime::SUBSCRIBER_CHILD_NOTIFICATION_BRIDGE,
            )?,
            EventType::parse(ChildDomainEventType::policy_violation_detected().as_str())?,
            TargetHandler::parse(
                constants::child_domain_runtime::TARGET_HANDLER_CHILD_NOTIFICATION_BRIDGE,
            )?,
        ),
        move |context| {
            let state = state.clone();
            async move {
                let notification =
                    ocentra_child_notification_core::request_child_domain_parent_notification(
                        context.payload(),
                    );
                state.record_notification(notification.clone());
                context
                    .publisher()
                    .publish(
                        notification,
                        child_domain_runtime_metadata(
                            ChildDomainRuntimeHop::NotificationRequested(
                                &context.payload().violation_id,
                            ),
                            &context.payload().detected_at,
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
struct ChildDomainRuntimeFlowState {
    evidence_recorded: Arc<Mutex<Option<ChildDomainEvidenceRecordedEvent>>>,
    ai_analysis_requested: Arc<Mutex<Option<ChildDomainAiAnalysisRequestedEvent>>>,
    ai_analysis_completed: Arc<Mutex<Option<ChildDomainAiAnalysisCompletedEvent>>>,
    policy_evaluation_requested: Arc<Mutex<Option<ChildDomainPolicyEvaluationRequestedEvent>>>,
    policy_violation_detected: Arc<Mutex<Option<ChildDomainPolicyViolationDetectedEvent>>>,
    notification_requested: Arc<Mutex<Option<ChildDomainNotificationRequestedEvent>>>,
}

impl ChildDomainRuntimeFlowState {
    fn record_evidence(&self, event: ChildDomainEvidenceRecordedEvent) {
        *self
            .evidence_recorded
            .lock()
            .expect(constants::child_domain_runtime::ERROR_CHILD_DOMAIN_FLOW_RECORDED) =
            Some(event);
    }

    fn record_ai_analysis_request(&self, event: ChildDomainAiAnalysisRequestedEvent) {
        *self
            .ai_analysis_requested
            .lock()
            .expect(constants::child_domain_runtime::ERROR_CHILD_DOMAIN_FLOW_RECORDED) =
            Some(event);
    }

    fn record_ai_analysis_completed(&self, event: ChildDomainAiAnalysisCompletedEvent) {
        *self
            .ai_analysis_completed
            .lock()
            .expect(constants::child_domain_runtime::ERROR_CHILD_DOMAIN_FLOW_RECORDED) =
            Some(event);
    }

    fn record_policy_evaluation_request(&self, event: ChildDomainPolicyEvaluationRequestedEvent) {
        *self
            .policy_evaluation_requested
            .lock()
            .expect(constants::child_domain_runtime::ERROR_CHILD_DOMAIN_FLOW_RECORDED) =
            Some(event);
    }

    fn record_policy_violation(&self, event: ChildDomainPolicyViolationDetectedEvent) {
        *self
            .policy_violation_detected
            .lock()
            .expect(constants::child_domain_runtime::ERROR_CHILD_DOMAIN_FLOW_RECORDED) =
            Some(event);
    }

    fn record_notification(&self, event: ChildDomainNotificationRequestedEvent) {
        *self
            .notification_requested
            .lock()
            .expect(constants::child_domain_runtime::ERROR_CHILD_DOMAIN_FLOW_RECORDED) =
            Some(event);
    }

    fn evidence_recorded(&self) -> Result<ChildDomainEvidenceRecordedEvent, EventingError> {
        required_child_domain_event(&self.evidence_recorded)
    }

    fn ai_analysis_requested(&self) -> Option<ChildDomainAiAnalysisRequestedEvent> {
        self.ai_analysis_requested.lock().ok()?.clone()
    }

    fn ai_analysis_completed(&self) -> Option<ChildDomainAiAnalysisCompletedEvent> {
        self.ai_analysis_completed.lock().ok()?.clone()
    }

    fn policy_evaluation_requested(&self) -> Option<ChildDomainPolicyEvaluationRequestedEvent> {
        self.policy_evaluation_requested.lock().ok()?.clone()
    }

    fn policy_violation_detected(&self) -> Option<ChildDomainPolicyViolationDetectedEvent> {
        self.policy_violation_detected.lock().ok()?.clone()
    }

    fn notification_requested(&self) -> Option<ChildDomainNotificationRequestedEvent> {
        self.notification_requested.lock().ok()?.clone()
    }
}

fn child_domain_evidence_recorded_event(
    event: &ChildDomainObservedEvent,
) -> Result<ChildDomainEvidenceRecordedEvent, EventingError> {
    match event.domain {
        ChildRuntimeDomain::App => Ok(ocentra_app_core::app_evidence_recorded_event(event)),
        ChildRuntimeDomain::AppGame => Ok(ocentra_app_game_core::app_game_evidence_recorded_event(
            event,
        )),
        ChildRuntimeDomain::Browser => {
            Ok(ocentra_browser_core::browser_evidence_recorded_event(event))
        }
        ChildRuntimeDomain::Lan => Ok(ocentra_lan_core::lan_evidence_recorded_event(event)),
        ChildRuntimeDomain::Network => {
            Ok(ocentra_network_core::network_evidence_recorded_event(event))
        }
        ChildRuntimeDomain::Screen => {
            Ok(ocentra_screen_core::screen_evidence_recorded_event(event))
        }
        ChildRuntimeDomain::ScreenLiveView => {
            Ok(ocentra_screen_live_view_core::screen_live_view_evidence_recorded_event(event))
        }
    }
}

fn child_domain_ai_analysis_requested_event(
    event: &ChildDomainEvidenceRecordedEvent,
) -> Result<Option<ChildDomainAiAnalysisRequestedEvent>, EventingError> {
    match event.domain {
        ChildRuntimeDomain::App => Ok(ocentra_app_core::app_ai_analysis_requested_event(event)),
        ChildRuntimeDomain::AppGame => Ok(
            ocentra_app_game_core::app_game_ai_analysis_requested_event(event),
        ),
        ChildRuntimeDomain::Browser => Ok(
            ocentra_browser_core::browser_ai_analysis_requested_event(event),
        ),
        ChildRuntimeDomain::Lan => Ok(ocentra_lan_core::lan_ai_analysis_requested_event(event)),
        ChildRuntimeDomain::Network => Ok(
            ocentra_network_core::network_ai_analysis_requested_event(event),
        ),
        ChildRuntimeDomain::Screen => Ok(ocentra_screen_core::screen_ai_analysis_requested_event(
            event,
        )),
        ChildRuntimeDomain::ScreenLiveView => {
            Ok(ocentra_screen_live_view_core::screen_live_view_ai_analysis_requested_event(event))
        }
    }
}

fn child_domain_policy_evaluation_requested_event(
    event: &ChildDomainEvidenceRecordedEvent,
) -> Result<Option<ChildDomainPolicyEvaluationRequestedEvent>, EventingError> {
    match event.domain {
        ChildRuntimeDomain::App => Ok(ocentra_app_core::app_policy_evaluation_requested_event(
            event,
        )),
        ChildRuntimeDomain::AppGame => {
            Ok(ocentra_app_game_core::app_game_policy_evaluation_requested_event(event))
        }
        ChildRuntimeDomain::Browser => {
            Ok(ocentra_browser_core::browser_policy_evaluation_requested_event(event))
        }
        ChildRuntimeDomain::Lan => Ok(ocentra_lan_core::lan_policy_evaluation_requested_event(
            event,
        )),
        ChildRuntimeDomain::Network => {
            Ok(ocentra_network_core::network_policy_evaluation_requested_event(event))
        }
        ChildRuntimeDomain::Screen => {
            Ok(ocentra_screen_core::screen_policy_evaluation_requested_event(event))
        }
        ChildRuntimeDomain::ScreenLiveView => Ok(
            ocentra_screen_live_view_core::screen_live_view_policy_evaluation_requested_event(
                event,
            ),
        ),
    }
}

fn required_child_domain_event<E>(value: &Arc<Mutex<Option<E>>>) -> Result<E, EventingError>
where
    E: Clone,
{
    value
        .lock()
        .expect(constants::child_domain_runtime::ERROR_CHILD_DOMAIN_FLOW_RECORDED)
        .clone()
        .ok_or_else(|| EventingError::InvalidValue {
            field: constants::child_domain_runtime::ERROR_CHILD_DOMAIN_FLOW_RECORDED,
            value: constants::child_domain_runtime::SIGNAL_OBSERVE_ONLY.to_string(),
        })
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum ChildDomainRuntimeHop<'a> {
    Observed(&'a ChildDomainObservationId),
    EvidenceRecorded(&'a ChildDomainEvidenceRef),
    AiAnalysisRequested(&'a ChildDomainEvidenceRef),
    AiAnalysisCompleted(&'a ChildDomainAiRequestId),
    PolicyEvaluationRequested(&'a ChildDomainEvidenceRef),
    PolicyEvaluationRequestedFromAi(&'a ChildDomainAiRequestId),
    PolicyViolationDetected(&'a ChildDomainPolicyRequestId),
    NotificationRequested(&'a ChildDomainPolicyViolationId),
}

impl<'a> ChildDomainRuntimeHop<'a> {
    fn source_component(self) -> &'static str {
        match self {
            Self::Observed(_)
            | Self::EvidenceRecorded(_)
            | Self::AiAnalysisRequested(_)
            | Self::PolicyEvaluationRequested(_)
            | Self::PolicyEvaluationRequestedFromAi(_) => {
                constants::child_domain_runtime::SOURCE_COMPONENT_CHILD_DOMAIN_RUNTIME
            }
            Self::AiAnalysisCompleted(_) => {
                constants::child_domain_runtime::SOURCE_COMPONENT_CHILD_AI_RUNTIME
            }
            Self::PolicyViolationDetected(_) => {
                constants::child_domain_runtime::SOURCE_COMPONENT_CHILD_POLICY_RUNTIME
            }
            Self::NotificationRequested(_) => {
                constants::child_domain_runtime::SOURCE_COMPONENT_CHILD_NOTIFICATION_RUNTIME
            }
        }
    }

    fn runtime_role(self) -> &'static str {
        match self {
            Self::Observed(_)
            | Self::EvidenceRecorded(_)
            | Self::AiAnalysisRequested(_)
            | Self::PolicyEvaluationRequested(_)
            | Self::PolicyEvaluationRequestedFromAi(_) => constants::eventing_source::ROLE_AGENT,
            Self::AiAnalysisCompleted(_) => constants::eventing_source::ROLE_ANALYZER,
            Self::PolicyViolationDetected(_) => constants::eventing_source::ROLE_DECISION_ENGINE,
            Self::NotificationRequested(_) => constants::eventing_source::ROLE_SIDE_EFFECT_ADAPTER,
        }
    }

    fn target_handler(self) -> &'static str {
        match self {
            Self::Observed(_) | Self::EvidenceRecorded(_) => {
                constants::child_domain_runtime::TARGET_HANDLER_DOMAIN_OBSERVER
            }
            Self::AiAnalysisRequested(_) => {
                constants::child_domain_runtime::TARGET_HANDLER_CHILD_AI_ANALYZER
            }
            Self::AiAnalysisCompleted(_) | Self::PolicyEvaluationRequested(_) => {
                constants::child_domain_runtime::TARGET_HANDLER_CHILD_POLICY_EVALUATOR
            }
            Self::PolicyEvaluationRequestedFromAi(_) => {
                constants::child_domain_runtime::TARGET_HANDLER_CHILD_POLICY_EVALUATOR
            }
            Self::PolicyViolationDetected(_) | Self::NotificationRequested(_) => {
                constants::child_domain_runtime::TARGET_HANDLER_CHILD_NOTIFICATION_BRIDGE
            }
        }
    }

    fn correlation_ref(self) -> &'a str {
        match self {
            Self::Observed(value) => value.as_str(),
            Self::EvidenceRecorded(value)
            | Self::AiAnalysisRequested(value)
            | Self::PolicyEvaluationRequested(value) => value.as_str(),
            Self::AiAnalysisCompleted(value) => value.as_str(),
            Self::PolicyEvaluationRequestedFromAi(value) => value.as_str(),
            Self::PolicyViolationDetected(value) => value.as_str(),
            Self::NotificationRequested(value) => value.as_str(),
        }
    }
}

fn child_domain_runtime_metadata(
    hop: ChildDomainRuntimeHop<'_>,
    recorded_at: &ChildDomainObservedAt,
) -> Result<EventMetadata, EventingError> {
    Ok(EventMetadata::from_parts(
        EventId::generated(),
        CorrelationId::parse(child_domain_runtime_correlation_id(hop))?,
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

fn child_domain_runtime_correlation_id(hop: ChildDomainRuntimeHop<'_>) -> String {
    let mut value = String::from(constants::child_domain_runtime::CORRELATION_PREFIX);
    value.push_str(hop.correlation_ref());
    value
}
