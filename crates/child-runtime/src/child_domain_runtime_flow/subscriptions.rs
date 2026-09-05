use super::routing::{
    child_domain_ai_analysis_requested_event, child_domain_evidence_recorded_event,
    child_domain_policy_evaluation_requested_event, child_domain_runtime_metadata,
    ChildDomainRuntimeHop,
};
use super::state::ChildDomainRuntimeFlowState;
use ocentra_eventing::{
    bus::publisher::RootEventPublisher, bus::subscriber::EventSubscriber,
    bus::subscriber::SubscriptionReport, error::EventingError, ids::EventType, ids::SubscriberId,
    ids::TargetHandler,
};
use ocentra_parent_agent_protocol::child_domain_runtime::{
    child_domain_notification_requested_event,
    child_domain_policy_evaluation_requested_from_ai_result_event_if_required,
    ChildDomainAiAnalysisCompletedEvent, ChildDomainAiAnalysisRequestedEvent, ChildDomainEventType,
    ChildDomainEvidenceRecordedEvent, ChildDomainObservedEvent,
    ChildDomainPolicyEvaluationRequestedEvent, ChildDomainPolicyViolationDetectedEvent,
    ChildRuntimeDomain,
};
use ocentra_parent_agent_protocol::constants;

pub(super) async fn subscribe_child_domain_observer(
    bus: &RootEventPublisher,
    domain: ChildRuntimeDomain,
) -> Result<SubscriptionReport, EventingError> {
    bus.subscribe::<ChildDomainObservedEvent, _, _>(
        EventSubscriber::new(
            SubscriberId::parse(domain.observer_subscriber_id())?,
            EventType::parse(domain.observed_event_type().as_str())?,
            TargetHandler::parse(constants::child_domain_runtime::TARGET_HANDLER_DOMAIN_OBSERVER)?,
        ),
        move |context| async move {
            let evidence = child_domain_evidence_recorded_event(context.payload())?;
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
            Ok(())
        },
    )
    .await
}

pub(super) async fn subscribe_child_domain_evidence(
    bus: &RootEventPublisher,
    domain: ChildRuntimeDomain,
    state: ChildDomainRuntimeFlowState,
) -> Result<SubscriptionReport, EventingError> {
    bus.subscribe::<ChildDomainEvidenceRecordedEvent, _, _>(
        EventSubscriber::new(
            SubscriberId::parse(domain.observer_subscriber_id())?,
            EventType::parse(domain.evidence_recorded_event_type().as_str())?,
            TargetHandler::parse(constants::child_domain_runtime::TARGET_HANDLER_DOMAIN_OBSERVER)?,
        ),
        move |context| {
            let state = state.clone();
            async move {
                let evidence = context.payload().clone();
                state.record_evidence(evidence.clone());
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

pub(super) async fn subscribe_child_domain_ai(
    bus: &RootEventPublisher,
    domain: ChildRuntimeDomain,
    state: ChildDomainRuntimeFlowState,
) -> Result<SubscriptionReport, EventingError> {
    bus.subscribe::<ChildDomainAiAnalysisRequestedEvent, _, _>(
        EventSubscriber::new(
            SubscriberId::parse(constants::child_domain_runtime::SUBSCRIBER_CHILD_AI_ANALYZER)?,
            EventType::parse(domain.ai_analysis_requested_event_type().as_str())?,
            TargetHandler::parse(
                constants::child_domain_runtime::TARGET_HANDLER_CHILD_AI_ANALYZER,
            )?,
        ),
        move |context| {
            let state = state.clone();
            async move {
                let completed =
                    ocentra_child_ai_core::child_domain_analysis::complete_child_domain_ai_analysis(
                        context.payload(),
                    );
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

pub(super) async fn subscribe_child_domain_ai_policy_bridge(
    bus: &RootEventPublisher,
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

pub(super) async fn subscribe_child_domain_policy(
    bus: &RootEventPublisher,
    domain: ChildRuntimeDomain,
    state: ChildDomainRuntimeFlowState,
) -> Result<SubscriptionReport, EventingError> {
    bus.subscribe::<ChildDomainPolicyEvaluationRequestedEvent, _, _>(
        EventSubscriber::new(
            SubscriberId::parse(
                constants::child_domain_runtime::SUBSCRIBER_CHILD_POLICY_EVALUATOR,
            )?,
            EventType::parse(domain.policy_evaluation_requested_event_type().as_str())?,
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

pub(super) async fn subscribe_child_domain_notification(
    bus: &RootEventPublisher,
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
                let notification = child_domain_notification_requested_event(context.payload());
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
