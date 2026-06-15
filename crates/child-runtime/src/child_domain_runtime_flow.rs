use std::sync::{Arc, Mutex};

use ocentra_eventing::{
    CorrelationId, EventBus, EventCustody, EventId, EventMetadata, EventSource, EventSubscriber,
    EventType, EventingError, RecordedAt, RuntimeInstanceId, RuntimeRole, SourceComponent,
    SourceService, SubscriberId, SubscriptionReport, TargetHandler,
};
use ocentra_parent_agent_protocol::{
    constants, ChildDomainAiAnalysisRequestedEvent, ChildDomainEvidenceRecordedEvent,
    ChildDomainNotificationRequestedEvent, ChildDomainObservedEvent,
    ChildDomainPolicyEvaluationRequestedEvent, ChildDomainPolicyViolationDetectedEvent,
};

#[derive(Clone, Debug, PartialEq)]
pub struct ChildDomainRuntimeFlowReport {
    pub domain: String,
    pub observer_subscription_report: SubscriptionReport,
    pub ai_subscription_report: SubscriptionReport,
    pub policy_subscription_report: SubscriptionReport,
    pub notification_subscription_report: SubscriptionReport,
    pub evidence_recorded: ChildDomainEvidenceRecordedEvent,
    pub ai_analysis_requested: Option<ChildDomainAiAnalysisRequestedEvent>,
    pub policy_evaluation_requested: ChildDomainPolicyEvaluationRequestedEvent,
    pub policy_violation_detected: ChildDomainPolicyViolationDetectedEvent,
    pub notification_requested: ChildDomainNotificationRequestedEvent,
}

pub async fn publish_default_child_domain_runtime_flows(
) -> Result<Vec<ChildDomainRuntimeFlowReport>, EventingError> {
    let events = vec![
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
    let bus = EventBus::new();
    let state = ChildDomainRuntimeFlowState::default();
    let observer_subscription_report =
        subscribe_child_domain_observer(&bus, &event, state.clone()).await?;
    let ai_subscription_report = subscribe_child_domain_ai(&bus, &event, state.clone()).await?;
    let policy_subscription_report =
        subscribe_child_domain_policy(&bus, &event, state.clone()).await?;
    let notification_subscription_report =
        subscribe_child_domain_notification(&bus, state.clone()).await?;

    bus.publish(
        event.clone(),
        child_domain_runtime_metadata(
            constants::child_domain_runtime::SOURCE_COMPONENT_CHILD_DOMAIN_RUNTIME,
            constants::eventing_source::ROLE_AGENT,
            constants::child_domain_runtime::TARGET_HANDLER_DOMAIN_OBSERVER,
            &event.observation_id,
        )?,
    )
    .await?;

    Ok(ChildDomainRuntimeFlowReport {
        domain: event.domain,
        observer_subscription_report,
        ai_subscription_report,
        policy_subscription_report,
        notification_subscription_report,
        evidence_recorded: state.evidence_recorded()?,
        ai_analysis_requested: state.ai_analysis_requested(),
        policy_evaluation_requested: state.policy_evaluation_requested()?,
        policy_violation_detected: state.policy_violation_detected()?,
        notification_requested: state.notification_requested()?,
    })
}

async fn subscribe_child_domain_observer(
    bus: &EventBus,
    event: &ChildDomainObservedEvent,
    state: ChildDomainRuntimeFlowState,
) -> Result<SubscriptionReport, EventingError> {
    bus.subscribe::<ChildDomainObservedEvent, _, _>(
        EventSubscriber::new(
            SubscriberId::parse(child_domain_observer_subscriber_id(&event.domain)?)?,
            EventType::parse(&event.event_type)?,
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
                            constants::child_domain_runtime::SOURCE_COMPONENT_CHILD_DOMAIN_RUNTIME,
                            constants::eventing_source::ROLE_AGENT,
                            constants::child_domain_runtime::TARGET_HANDLER_DOMAIN_OBSERVER,
                            &evidence.evidence_ref,
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
                                constants::child_domain_runtime::SOURCE_COMPONENT_CHILD_DOMAIN_RUNTIME,
                                constants::eventing_source::ROLE_AGENT,
                                constants::child_domain_runtime::TARGET_HANDLER_CHILD_AI_ANALYZER,
                                &evidence.evidence_ref,
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
                                constants::child_domain_runtime::SOURCE_COMPONENT_CHILD_DOMAIN_RUNTIME,
                                constants::eventing_source::ROLE_AGENT,
                                constants::child_domain_runtime::TARGET_HANDLER_CHILD_POLICY_EVALUATOR,
                                &evidence.evidence_ref,
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
            EventType::parse(child_domain_ai_event_type(&event.domain)?)?,
            TargetHandler::parse(
                constants::child_domain_runtime::TARGET_HANDLER_CHILD_AI_ANALYZER,
            )?,
        ),
        move |context| {
            let state = state.clone();
            async move {
                let policy_request =
                    ocentra_child_ai_core::complete_child_domain_ai_analysis(context.payload());
                state.record_policy_evaluation_request(policy_request.clone());
                context
                    .publisher()
                    .publish(
                        policy_request,
                        child_domain_runtime_metadata(
                            constants::child_domain_runtime::SOURCE_COMPONENT_CHILD_AI_RUNTIME,
                            constants::eventing_source::ROLE_ANALYZER,
                            constants::child_domain_runtime::TARGET_HANDLER_CHILD_POLICY_EVALUATOR,
                            context.payload().ai_request_id.as_str(),
                        )?,
                    )
                    .await?;
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
            EventType::parse(child_domain_policy_event_type(&event.domain)?)?,
            TargetHandler::parse(
                constants::child_domain_runtime::TARGET_HANDLER_CHILD_POLICY_EVALUATOR,
            )?,
        ),
        move |context| {
            let state = state.clone();
            async move {
                let violation =
                    ocentra_child_policy_core::evaluate_child_domain_policy(context.payload());
                state.record_policy_violation(violation.clone());
                context
                .publisher()
                .publish(
                    violation,
                    child_domain_runtime_metadata(
                        constants::child_domain_runtime::SOURCE_COMPONENT_CHILD_POLICY_RUNTIME,
                        constants::eventing_source::ROLE_DECISION_ENGINE,
                        constants::child_domain_runtime::TARGET_HANDLER_CHILD_NOTIFICATION_BRIDGE,
                        context.payload().policy_request_id.as_str(),
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
            EventType::parse(constants::child_domain_runtime::POLICY_VIOLATION_DETECTED_EVENT_TYPE)?,
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
                        constants::child_domain_runtime::SOURCE_COMPONENT_CHILD_NOTIFICATION_RUNTIME,
                        constants::eventing_source::ROLE_SIDE_EFFECT_ADAPTER,
                        constants::child_domain_runtime::TARGET_HANDLER_CHILD_NOTIFICATION_BRIDGE,
                        context.payload().violation_id.as_str(),
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

    fn policy_evaluation_requested(
        &self,
    ) -> Result<ChildDomainPolicyEvaluationRequestedEvent, EventingError> {
        required_child_domain_event(&self.policy_evaluation_requested)
    }

    fn policy_violation_detected(
        &self,
    ) -> Result<ChildDomainPolicyViolationDetectedEvent, EventingError> {
        required_child_domain_event(&self.policy_violation_detected)
    }

    fn notification_requested(
        &self,
    ) -> Result<ChildDomainNotificationRequestedEvent, EventingError> {
        required_child_domain_event(&self.notification_requested)
    }
}

fn child_domain_evidence_recorded_event(
    event: &ChildDomainObservedEvent,
) -> Result<ChildDomainEvidenceRecordedEvent, EventingError> {
    match event.domain.as_str() {
        constants::child_domain_runtime::DOMAIN_APP_GAME => Ok(
            ocentra_app_game_core::app_game_evidence_recorded_event(event),
        ),
        constants::child_domain_runtime::DOMAIN_BROWSER => {
            Ok(ocentra_browser_core::browser_evidence_recorded_event(event))
        }
        constants::child_domain_runtime::DOMAIN_LAN => {
            Ok(ocentra_lan_core::lan_evidence_recorded_event(event))
        }
        constants::child_domain_runtime::DOMAIN_NETWORK => {
            Ok(ocentra_network_core::network_evidence_recorded_event(event))
        }
        constants::child_domain_runtime::DOMAIN_SCREEN => {
            Ok(ocentra_screen_core::screen_evidence_recorded_event(event))
        }
        constants::child_domain_runtime::DOMAIN_SCREEN_LIVE_VIEW => {
            Ok(ocentra_screen_live_view_core::screen_live_view_evidence_recorded_event(event))
        }
        _ => Err(invalid_child_domain(event.domain.clone())),
    }
}

fn child_domain_ai_analysis_requested_event(
    event: &ChildDomainEvidenceRecordedEvent,
) -> Result<Option<ChildDomainAiAnalysisRequestedEvent>, EventingError> {
    match event.domain.as_str() {
        constants::child_domain_runtime::DOMAIN_APP_GAME => Ok(
            ocentra_app_game_core::app_game_ai_analysis_requested_event(event),
        ),
        constants::child_domain_runtime::DOMAIN_BROWSER => Ok(
            ocentra_browser_core::browser_ai_analysis_requested_event(event),
        ),
        constants::child_domain_runtime::DOMAIN_LAN => {
            Ok(ocentra_lan_core::lan_ai_analysis_requested_event(event))
        }
        constants::child_domain_runtime::DOMAIN_NETWORK => Ok(
            ocentra_network_core::network_ai_analysis_requested_event(event),
        ),
        constants::child_domain_runtime::DOMAIN_SCREEN => Ok(
            ocentra_screen_core::screen_ai_analysis_requested_event(event),
        ),
        constants::child_domain_runtime::DOMAIN_SCREEN_LIVE_VIEW => {
            Ok(ocentra_screen_live_view_core::screen_live_view_ai_analysis_requested_event(event))
        }
        _ => Err(invalid_child_domain(event.domain.clone())),
    }
}

fn child_domain_policy_evaluation_requested_event(
    event: &ChildDomainEvidenceRecordedEvent,
) -> Result<Option<ChildDomainPolicyEvaluationRequestedEvent>, EventingError> {
    match event.domain.as_str() {
        constants::child_domain_runtime::DOMAIN_APP_GAME => {
            Ok(ocentra_app_game_core::app_game_policy_evaluation_requested_event(event))
        }
        constants::child_domain_runtime::DOMAIN_BROWSER => {
            Ok(ocentra_browser_core::browser_policy_evaluation_requested_event(event))
        }
        constants::child_domain_runtime::DOMAIN_LAN => Ok(
            ocentra_lan_core::lan_policy_evaluation_requested_event(event),
        ),
        constants::child_domain_runtime::DOMAIN_NETWORK => {
            Ok(ocentra_network_core::network_policy_evaluation_requested_event(event))
        }
        constants::child_domain_runtime::DOMAIN_SCREEN => {
            Ok(ocentra_screen_core::screen_policy_evaluation_requested_event(event))
        }
        constants::child_domain_runtime::DOMAIN_SCREEN_LIVE_VIEW => Ok(
            ocentra_screen_live_view_core::screen_live_view_policy_evaluation_requested_event(
                event,
            ),
        ),
        _ => Err(invalid_child_domain(event.domain.clone())),
    }
}

fn child_domain_observer_subscriber_id(domain: &str) -> Result<&'static str, EventingError> {
    match domain {
        constants::child_domain_runtime::DOMAIN_APP_GAME => {
            Ok(constants::child_domain_runtime::SUBSCRIBER_APP_GAME_OBSERVER)
        }
        constants::child_domain_runtime::DOMAIN_BROWSER => {
            Ok(constants::child_domain_runtime::SUBSCRIBER_BROWSER_OBSERVER)
        }
        constants::child_domain_runtime::DOMAIN_LAN => {
            Ok(constants::child_domain_runtime::SUBSCRIBER_LAN_OBSERVER)
        }
        constants::child_domain_runtime::DOMAIN_NETWORK => {
            Ok(constants::child_domain_runtime::SUBSCRIBER_NETWORK_OBSERVER)
        }
        constants::child_domain_runtime::DOMAIN_SCREEN => {
            Ok(constants::child_domain_runtime::SUBSCRIBER_SCREEN_OBSERVER)
        }
        constants::child_domain_runtime::DOMAIN_SCREEN_LIVE_VIEW => {
            Ok(constants::child_domain_runtime::SUBSCRIBER_SCREEN_LIVE_VIEW_OBSERVER)
        }
        _ => Err(invalid_child_domain(domain.to_string())),
    }
}

fn child_domain_policy_event_type(domain: &str) -> Result<&'static str, EventingError> {
    match domain {
        constants::child_domain_runtime::DOMAIN_APP_GAME => {
            Ok(constants::child_domain_runtime::APP_GAME_POLICY_EVALUATION_REQUESTED_EVENT_TYPE)
        }
        constants::child_domain_runtime::DOMAIN_BROWSER => {
            Ok(constants::child_domain_runtime::BROWSER_POLICY_EVALUATION_REQUESTED_EVENT_TYPE)
        }
        constants::child_domain_runtime::DOMAIN_LAN => {
            Ok(constants::child_domain_runtime::LAN_POLICY_EVALUATION_REQUESTED_EVENT_TYPE)
        }
        constants::child_domain_runtime::DOMAIN_NETWORK => {
            Ok(constants::child_domain_runtime::NETWORK_POLICY_EVALUATION_REQUESTED_EVENT_TYPE)
        }
        constants::child_domain_runtime::DOMAIN_SCREEN => {
            Ok(constants::child_domain_runtime::SCREEN_POLICY_EVALUATION_REQUESTED_EVENT_TYPE)
        }
        constants::child_domain_runtime::DOMAIN_SCREEN_LIVE_VIEW => {
            Ok(constants::child_domain_runtime::SCREEN_LIVE_VIEW_POLICY_EVALUATION_REQUESTED_EVENT_TYPE)
        }
        _ => Err(invalid_child_domain(domain.to_string())),
    }
}

fn child_domain_ai_event_type(domain: &str) -> Result<&'static str, EventingError> {
    match domain {
        constants::child_domain_runtime::DOMAIN_BROWSER => {
            Ok(constants::child_domain_runtime::BROWSER_AI_ANALYSIS_REQUESTED_EVENT_TYPE)
        }
        constants::child_domain_runtime::DOMAIN_SCREEN => {
            Ok(constants::child_domain_runtime::SCREEN_AI_ANALYSIS_REQUESTED_EVENT_TYPE)
        }
        constants::child_domain_runtime::DOMAIN_APP_GAME
        | constants::child_domain_runtime::DOMAIN_LAN
        | constants::child_domain_runtime::DOMAIN_NETWORK
        | constants::child_domain_runtime::DOMAIN_SCREEN_LIVE_VIEW => {
            Ok(constants::child_domain_runtime::BROWSER_AI_ANALYSIS_REQUESTED_EVENT_TYPE)
        }
        _ => Err(invalid_child_domain(domain.to_string())),
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

fn invalid_child_domain(domain: String) -> EventingError {
    EventingError::InvalidValue {
        field: constants::child_domain_runtime::ERROR_CHILD_DOMAIN_FLOW_RECORDED,
        value: domain,
    }
}

fn child_domain_runtime_metadata(
    source_component: &str,
    runtime_role: &str,
    target_handler: &str,
    correlation_suffix: &str,
) -> Result<EventMetadata, EventingError> {
    Ok(EventMetadata::from_parts(
        EventId::generated(),
        CorrelationId::parse(child_domain_runtime_correlation_id(correlation_suffix))?,
        EventSource::new(
            EventCustody::parse(constants::child_agent::CUSTODY_CHILD_AGENT_RUNTIME)?,
            RuntimeRole::parse(runtime_role)?,
            SourceService::parse(constants::peer::LOCAL_DEV_AGENT)?,
            SourceComponent::parse(source_component)?,
            RuntimeInstanceId::parse(constants::child_agent::RUNTIME_INSTANCE_LOCAL_CHILD_AGENT)?,
        ),
        RecordedAt::parse(constants::child_domain_runtime::DEFAULT_OBSERVED_AT)?,
        Some(TargetHandler::parse(target_handler)?),
    ))
}

fn child_domain_runtime_correlation_id(suffix: &str) -> String {
    let mut value = String::from(constants::child_domain_runtime::CORRELATION_PREFIX);
    value.push_str(suffix);
    value
}
