use std::sync::{Arc, Mutex, MutexGuard};
use std::time::Duration;

use ocentra_child_runtime::tracking_config_update_flow::{
    TrackingConfigUpdateCausalTarget, TrackingConfigUpdateEventFlow,
    TrackingConfigUpdateEventFlowReport,
};
use ocentra_eventing::{
    bus::publisher::RootEventPublisher, bus::subscriber::EventSubscriber,
    bus::subscriber::SubscriptionReport, bus::EventBus, envelope::EventMetadata,
    envelope::EventSource, error::EventingError, ids::CorrelationId, ids::EventCustody,
    ids::EventId, ids::EventType, ids::RecordedAt, ids::RuntimeInstanceId, ids::RuntimeRole,
    ids::SourceComponent, ids::SourceService, ids::SubscriberId, ids::TargetHandler,
    request::RequestOptions, request::RequestReport,
};
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::tracking::{
    config_update_event::{
        tracking_config_audit_entry_committed_event, tracking_config_change_approved_event,
        tracking_config_change_rejected_event, tracking_config_change_requested_event,
        tracking_config_policy_decision_completed_event,
        tracking_config_policy_evaluation_requested_event,
        tracking_config_portal_read_model_updated_event, ParentTrackingConfigUpdatedEvent,
        TrackingConfigAuditEntryCommittedEvent, TrackingConfigAuditOutcome,
        TrackingConfigChangeApprovedEvent, TrackingConfigChangeRejectedEvent,
        TrackingConfigChangeRequestedEvent, TrackingConfigEffectiveState,
        TrackingConfigPolicyDecisionCompletedEvent, TrackingConfigPolicyDecisionState,
        TrackingConfigPolicyEvaluationRequestedEvent, TrackingConfigPortalReadModelUpdatedEvent,
        TrackingConfigPortalUpdateKind, TrackingConfigUpdateEventName, TrackingConfigUpdateRequest,
        TrackingConfigUpdateResponse, TrackingConfigUpdateResponseState,
    },
    identifiers::TrackingPolicyRuleRef,
    retention_settings_write_command::{
        TrackingDurableSettingsPersistenceState, TrackingRemoteAiState, TrackingRemoteSyncState,
    },
};
use ocentra_parent_agent_protocol::AGENT_PROTOCOL_SCHEMA_VERSION;

use crate::tracking_dispatch::{
    parent_runtime_tracking_dispatch_evaluated_event_from_origin,
    route_parent_tracking_config_update_event_from_origin, ChildAcknowledgementState,
    ChildRuntimePublishState, ParentRuntimeOriginState,
    ParentRuntimeTrackingDispatchEvaluatedEvent,
};

#[path = "tracking_config_update_flow/policy_rules.rs"]
mod policy_rules;
use self::policy_rules::tracking_policy_rule_refs;

#[path = "tracking_config_update_flow/event_sinks.rs"]
mod event_sinks;
use self::event_sinks::{
    subscribe_tracking_config_event_sinks, TrackingConfigEventSinkSubscriptionReports,
};

#[derive(Clone, Debug, PartialEq)]
pub struct ParentTrackingConfigUpdateEventFlowReport {
    pub parent_subscription_report: SubscriptionReport,
    pub change_requested_subscription_report: SubscriptionReport,
    pub policy_evaluation_subscription_report: SubscriptionReport,
    pub decision_subscription_report: SubscriptionReport,
    pub dispatch_subscription_report: SubscriptionReport,
    pub change_approved_subscription_report: SubscriptionReport,
    pub change_rejected_subscription_report: SubscriptionReport,
    pub audit_subscription_report: SubscriptionReport,
    pub portal_subscription_report: SubscriptionReport,
    pub parent_request_report: RequestReport<TrackingConfigUpdateResponse>,
    pub change_requested_event: TrackingConfigChangeRequestedEvent,
    pub policy_evaluation_event: TrackingConfigPolicyEvaluationRequestedEvent,
    pub policy_decision_event: TrackingConfigPolicyDecisionCompletedEvent,
    pub dispatch_event: ParentRuntimeTrackingDispatchEvaluatedEvent,
    pub change_approved_event: Option<TrackingConfigChangeApprovedEvent>,
    pub change_rejected_event: Option<TrackingConfigChangeRejectedEvent>,
    pub audit_event: TrackingConfigAuditEntryCommittedEvent,
    pub portal_event: TrackingConfigPortalReadModelUpdatedEvent,
    pub child_runtime_flow: Option<TrackingConfigUpdateEventFlowReport>,
}

pub struct ParentTrackingConfigUpdateEventFlow {
    bus: RootEventPublisher,
    state: ParentTrackingConfigUpdateEventState,
    parent_subscription_report: SubscriptionReport,
    change_requested_subscription_report: SubscriptionReport,
    policy_evaluation_subscription_report: SubscriptionReport,
    decision_subscription_report: SubscriptionReport,
    event_sink_subscription_reports: TrackingConfigEventSinkSubscriptionReports,
}

impl ParentTrackingConfigUpdateEventFlow {
    pub async fn new(
        previous_event_ref: impl Into<String>,
        child_acknowledgement_state: ChildAcknowledgementState,
        origin_state: ParentRuntimeOriginState,
    ) -> Result<Self, EventingError> {
        let bus = EventBus::root();
        let child_runtime_target = TrackingConfigUpdateEventFlow::new()
            .await?
            .into_causal_target();
        let state = ParentTrackingConfigUpdateEventState::default();
        let previous_event_ref = previous_event_ref.into();
        let event_sink_subscription_reports =
            subscribe_tracking_config_event_sinks(&bus, state.clone()).await?;
        let decision_subscription_report = subscribe_tracking_config_policy_decision_events(
            &bus,
            state.clone(),
            child_runtime_target,
        )
        .await?;
        let policy_evaluation_subscription_report =
            subscribe_tracking_config_policy_evaluation_events(
                &bus,
                state.clone(),
                child_acknowledgement_state,
                origin_state,
            )
            .await?;
        let change_requested_subscription_report =
            subscribe_tracking_config_change_requested_events(&bus, state.clone()).await?;
        let parent_subscription_report = subscribe_parent_tracking_config_updated_events(
            &bus,
            state.clone(),
            previous_event_ref,
        )
        .await?;

        Ok(Self {
            bus,
            state,
            parent_subscription_report,
            change_requested_subscription_report,
            policy_evaluation_subscription_report,
            decision_subscription_report,
            event_sink_subscription_reports,
        })
    }

    pub async fn publish_parent_tracking_config_updated(
        &self,
        parent_event: &ParentTrackingConfigUpdatedEvent,
    ) -> Result<ParentTrackingConfigUpdateEventFlowReport, EventingError> {
        let parent_request_report = self
            .bus
            .publish_request(
                parent_event.clone(),
                parent_tracking_config_updated_metadata(parent_event)?,
                RequestOptions::with_timeout(Duration::from_millis(
                    constants::tracking_config_update::REQUEST_TIMEOUT_MS,
                ))?,
            )
            .await?;

        Ok(ParentTrackingConfigUpdateEventFlowReport {
            parent_subscription_report: self.parent_subscription_report.clone(),
            change_requested_subscription_report: self.change_requested_subscription_report.clone(),
            policy_evaluation_subscription_report: self
                .policy_evaluation_subscription_report
                .clone(),
            decision_subscription_report: self.decision_subscription_report.clone(),
            dispatch_subscription_report: self.event_sink_subscription_reports.dispatch.clone(),
            change_approved_subscription_report: self
                .event_sink_subscription_reports
                .change_approved
                .clone(),
            change_rejected_subscription_report: self
                .event_sink_subscription_reports
                .change_rejected
                .clone(),
            audit_subscription_report: self.event_sink_subscription_reports.audit.clone(),
            portal_subscription_report: self.event_sink_subscription_reports.portal.clone(),
            parent_request_report,
            change_requested_event: self.state.change_requested_event()?,
            policy_evaluation_event: self.state.policy_evaluation_event()?,
            policy_decision_event: self.state.policy_decision_event()?,
            dispatch_event: self.state.dispatch_event()?,
            change_approved_event: self.state.change_approved_event(),
            change_rejected_event: self.state.change_rejected_event(),
            audit_event: self.state.audit_event()?,
            portal_event: self.state.portal_event()?,
            child_runtime_flow: self.state.child_runtime_flow(),
        })
    }
}

pub async fn publish_parent_tracking_config_updated_event_flow(
    previous_event_ref: impl Into<String>,
    parent_event: &ParentTrackingConfigUpdatedEvent,
    child_acknowledgement_state: ChildAcknowledgementState,
    origin_state: ParentRuntimeOriginState,
) -> Result<ParentTrackingConfigUpdateEventFlowReport, EventingError> {
    ParentTrackingConfigUpdateEventFlow::new(
        previous_event_ref,
        child_acknowledgement_state,
        origin_state,
    )
    .await?
    .publish_parent_tracking_config_updated(parent_event)
    .await
}

async fn subscribe_parent_tracking_config_updated_events(
    bus: &RootEventPublisher,
    state: ParentTrackingConfigUpdateEventState,
    previous_event_ref: String,
) -> Result<SubscriptionReport, EventingError> {
    bus.subscribe::<ParentTrackingConfigUpdatedEvent, _, _>(
        EventSubscriber::new(
            SubscriberId::parse(
                constants::tracking_config_update::SUBSCRIBER_PARENT_TRACKING_CONFIG_CHANGE_REQUESTER,
            )?,
            EventType::parse(constants::tracking_config_update::PARENT_EVENT_TYPE)?,
            TargetHandler::parse(
                constants::tracking_config_update::TARGET_HANDLER_PARENT_TRACKING_CONFIG_CHANGE_REQUESTER,
            )?,
        ),
        move |context| {
            let state = state.clone();
            let previous_event_ref = previous_event_ref.clone();
            async move {
                state.record_parent_event(context.payload().clone());
                let change_requested = tracking_config_change_requested_event(
                    previous_event_ref.clone(),
                    context.payload(),
                );
                context
                    .publisher()
                    .publish(
                        change_requested,
                        tracking_config_change_requested_metadata(context.payload())?,
                    )
                    .await?;
                context.complete_request(state.final_response()?).await?;
                Ok(())
            }
        },
    )
    .await
}

async fn subscribe_tracking_config_change_requested_events(
    bus: &RootEventPublisher,
    state: ParentTrackingConfigUpdateEventState,
) -> Result<SubscriptionReport, EventingError> {
    bus.subscribe::<TrackingConfigChangeRequestedEvent, _, _>(
        EventSubscriber::new(
            SubscriberId::parse(
                constants::tracking_config_update::SUBSCRIBER_PARENT_TRACKING_CONFIG_POLICY_REQUESTER,
            )?,
            EventType::parse(constants::tracking_config_update::CHANGE_REQUESTED_EVENT_TYPE)?,
            TargetHandler::parse(
                constants::tracking_config_update::TARGET_HANDLER_PARENT_TRACKING_CONFIG_POLICY_REQUESTER,
            )?,
        ),
        move |context| {
            let state = state.clone();
            async move {
                state.record_change_requested_event(context.payload().clone());
                let policy_evaluation_requested = tracking_config_policy_evaluation_requested_event(
                    context.payload(),
                    tracking_policy_rule_refs(&context.payload().config),
                    false,
                );
                context
                    .publisher()
                    .publish(
                        policy_evaluation_requested,
                        tracking_config_policy_evaluation_requested_metadata(context.payload())?,
                    )
                    .await?;
                Ok(())
            }
        },
    )
    .await
}

async fn subscribe_tracking_config_policy_evaluation_events(
    bus: &RootEventPublisher,
    state: ParentTrackingConfigUpdateEventState,
    child_acknowledgement_state: ChildAcknowledgementState,
    origin_state: ParentRuntimeOriginState,
) -> Result<SubscriptionReport, EventingError> {
    bus.subscribe::<TrackingConfigPolicyEvaluationRequestedEvent, _, _>(
        EventSubscriber::new(
            SubscriberId::parse(
                constants::tracking_config_update::SUBSCRIBER_PARENT_TRACKING_CONFIG_POLICY_DECIDER,
            )?,
            EventType::parse(constants::network_flow::EVENT_POLICY_EVALUATION_REQUESTED)?,
            TargetHandler::parse(
                constants::tracking_config_update::TARGET_HANDLER_PARENT_TRACKING_CONFIG_POLICY_DECIDER,
            )?,
        ),
        move |context| {
            let state = state.clone();
            async move {
                state.record_policy_evaluation_event(context.payload().clone());
                let parent_event = state.parent_event()?;
                let dispatch_event = parent_runtime_tracking_dispatch_evaluated_event_from_origin(
                    &parent_event,
                    child_acknowledgement_state,
                    origin_state,
                );
                context
                    .publisher()
                    .publish(
                        dispatch_event,
                        tracking_config_controller_metadata(
                            context.payload().source_command_id.as_str(),
                        )?,
                    )
                    .await?;
                let dispatch_decision = route_parent_tracking_config_update_event_from_origin(
                    &parent_event,
                    child_acknowledgement_state,
                    origin_state,
                );
                let child_runtime_publish_required =
                    dispatch_decision.child_runtime_publish_state == ChildRuntimePublishState::Publish;
                let policy_decision = tracking_config_policy_decision_completed_event(
                    context.payload(),
                    if child_runtime_publish_required {
                        TrackingConfigPolicyDecisionState::Approved
                    } else {
                        TrackingConfigPolicyDecisionState::Rejected
                    },
                    child_runtime_publish_required,
                );
                context
                    .publisher()
                    .publish(
                        policy_decision,
                        tracking_config_policy_decision_completed_metadata(
                            context.payload().source_command_id.as_str(),
                        )?,
                    )
                    .await?;
                Ok(())
            }
        },
    )
    .await
}

async fn subscribe_tracking_config_policy_decision_events(
    bus: &RootEventPublisher,
    state: ParentTrackingConfigUpdateEventState,
    child_runtime_target: TrackingConfigUpdateCausalTarget,
) -> Result<SubscriptionReport, EventingError> {
    bus.subscribe::<TrackingConfigPolicyDecisionCompletedEvent, _, _>(
        EventSubscriber::new(
            SubscriberId::parse(
                constants::tracking_config_update::SUBSCRIBER_PARENT_TRACKING_CONFIG_DECISION_APPLIER,
            )?,
            EventType::parse(constants::network_flow::EVENT_POLICY_DECISION_COMPLETED)?,
            TargetHandler::parse(
                constants::tracking_config_update::TARGET_HANDLER_PARENT_TRACKING_CONFIG_DECISION_APPLIER,
            )?,
        ),
        move |context| {
            let state = state.clone();
            let child_runtime_target = child_runtime_target.clone();
            async move {
                let decision = context.payload().clone();
                state.record_policy_decision_event(decision.clone());
                let parent_event = state.parent_event()?;

                if decision.decision_state == TrackingConfigPolicyDecisionState::Approved {
                    handle_approved_tracking_config_decision(
                        context.publisher(),
                        &child_runtime_target,
                        state.clone(),
                        &decision,
                        &parent_event,
                    )
                    .await?;
                } else {
                    handle_rejected_tracking_config_decision(
                        context.publisher(),
                        state.clone(),
                        &decision,
                        &parent_event,
                    )
                    .await?;
                }
                Ok(())
            }
        },
    )
    .await
}

async fn handle_approved_tracking_config_decision(
    publisher: &ocentra_eventing::bus::publisher::EventPublisher,
    child_runtime_target: &TrackingConfigUpdateCausalTarget,
    state: ParentTrackingConfigUpdateEventState,
    decision: &TrackingConfigPolicyDecisionCompletedEvent,
    parent_event: &ParentTrackingConfigUpdatedEvent,
) -> Result<(), EventingError> {
    let change_approved = tracking_config_change_approved_event(decision);
    publisher
        .publish(
            change_approved.clone(),
            tracking_config_controller_metadata(decision.source_command_id.as_str())?,
        )
        .await?;

    let child_runtime_flow = child_runtime_target
        .publish_parent_config_updated(publisher, parent_event)
        .await
        .ok();
    state.record_child_runtime_flow(child_runtime_flow.clone());

    let (audit_outcome, update_kind, visible_manual_required, visible_unavailable) =
        if let Some(flow_report) = child_runtime_flow.as_ref() {
            state.record_final_response(flow_report.parent_request_report.response.clone());
            (
                TrackingConfigAuditOutcome::Committed,
                TrackingConfigPortalUpdateKind::TrackingConfigState,
                false,
                false,
            )
        } else {
            state.record_final_response(rejected_tracking_config_update_response(parent_event));
            (
                TrackingConfigAuditOutcome::Failed,
                TrackingConfigPortalUpdateKind::ManualRequiredState,
                true,
                true,
            )
        };

    let audit_event = tracking_config_audit_entry_committed_event(
        decision,
        change_approved.change_approved_event_ref.clone(),
        audit_outcome,
    );
    publisher
        .publish(
            audit_event.clone(),
            tracking_config_audit_entry_committed_metadata(decision.source_command_id.as_str())?,
        )
        .await?;

    let portal_event = tracking_config_portal_read_model_updated_event(
        &audit_event,
        update_kind,
        visible_manual_required,
        visible_unavailable,
    );
    publisher
        .publish(
            portal_event,
            tracking_config_portal_read_model_updated_metadata(
                decision.source_command_id.as_str(),
            )?,
        )
        .await?;
    Ok(())
}

async fn handle_rejected_tracking_config_decision(
    publisher: &ocentra_eventing::bus::publisher::EventPublisher,
    state: ParentTrackingConfigUpdateEventState,
    decision: &TrackingConfigPolicyDecisionCompletedEvent,
    parent_event: &ParentTrackingConfigUpdatedEvent,
) -> Result<(), EventingError> {
    let change_rejected = tracking_config_change_rejected_event(
        decision,
        constants::tracking_config_update::REJECTION_REASON_CHILD_RUNTIME_DISPATCH_BLOCKED,
    );
    publisher
        .publish(
            change_rejected.clone(),
            tracking_config_controller_metadata(decision.source_command_id.as_str())?,
        )
        .await?;
    let audit_event = tracking_config_audit_entry_committed_event(
        decision,
        change_rejected.change_rejected_event_ref.clone(),
        TrackingConfigAuditOutcome::Failed,
    );
    publisher
        .publish(
            audit_event.clone(),
            tracking_config_audit_entry_committed_metadata(decision.source_command_id.as_str())?,
        )
        .await?;
    let portal_event = tracking_config_portal_read_model_updated_event(
        &audit_event,
        TrackingConfigPortalUpdateKind::ManualRequiredState,
        true,
        true,
    );
    publisher
        .publish(
            portal_event,
            tracking_config_portal_read_model_updated_metadata(
                decision.source_command_id.as_str(),
            )?,
        )
        .await?;
    state.record_final_response(rejected_tracking_config_update_response(parent_event));
    Ok(())
}

#[derive(Clone, Debug, Default)]
struct ParentTrackingConfigUpdateEventState {
    parent_events: Arc<Mutex<Vec<ParentTrackingConfigUpdatedEvent>>>,
    change_requested_events: Arc<Mutex<Vec<TrackingConfigChangeRequestedEvent>>>,
    policy_evaluation_events: Arc<Mutex<Vec<TrackingConfigPolicyEvaluationRequestedEvent>>>,
    policy_decision_events: Arc<Mutex<Vec<TrackingConfigPolicyDecisionCompletedEvent>>>,
    dispatch_events: Arc<Mutex<Vec<ParentRuntimeTrackingDispatchEvaluatedEvent>>>,
    change_approved_events: Arc<Mutex<Vec<TrackingConfigChangeApprovedEvent>>>,
    change_rejected_events: Arc<Mutex<Vec<TrackingConfigChangeRejectedEvent>>>,
    audit_events: Arc<Mutex<Vec<TrackingConfigAuditEntryCommittedEvent>>>,
    portal_events: Arc<Mutex<Vec<TrackingConfigPortalReadModelUpdatedEvent>>>,
    final_responses: Arc<Mutex<Vec<TrackingConfigUpdateResponse>>>,
    child_runtime_flows: Arc<Mutex<Vec<TrackingConfigUpdateEventFlowReport>>>,
}

impl ParentTrackingConfigUpdateEventState {
    fn record_parent_event(&self, event: ParentTrackingConfigUpdatedEvent) {
        lock_recover(&self.parent_events).push(event);
    }

    fn record_change_requested_event(&self, event: TrackingConfigChangeRequestedEvent) {
        lock_recover(&self.change_requested_events).push(event);
    }

    fn record_policy_evaluation_event(&self, event: TrackingConfigPolicyEvaluationRequestedEvent) {
        lock_recover(&self.policy_evaluation_events).push(event);
    }

    fn record_policy_decision_event(&self, event: TrackingConfigPolicyDecisionCompletedEvent) {
        lock_recover(&self.policy_decision_events).push(event);
    }

    fn record_dispatch_event(&self, event: ParentRuntimeTrackingDispatchEvaluatedEvent) {
        lock_recover(&self.dispatch_events).push(event);
    }

    fn record_change_approved_event(&self, event: TrackingConfigChangeApprovedEvent) {
        lock_recover(&self.change_approved_events).push(event);
    }

    fn record_change_rejected_event(&self, event: TrackingConfigChangeRejectedEvent) {
        lock_recover(&self.change_rejected_events).push(event);
    }

    fn record_audit_event(&self, event: TrackingConfigAuditEntryCommittedEvent) {
        lock_recover(&self.audit_events).push(event);
    }

    fn record_portal_event(&self, event: TrackingConfigPortalReadModelUpdatedEvent) {
        lock_recover(&self.portal_events).push(event);
    }

    fn record_final_response(&self, response: TrackingConfigUpdateResponse) {
        lock_recover(&self.final_responses).push(response);
    }

    fn record_child_runtime_flow(&self, flow: Option<TrackingConfigUpdateEventFlowReport>) {
        if let Some(flow) = flow {
            lock_recover(&self.child_runtime_flows).push(flow);
        }
    }

    fn parent_event(&self) -> Result<ParentTrackingConfigUpdatedEvent, EventingError> {
        lock_recover(&self.parent_events)
            .last()
            .cloned()
            .ok_or_else(|| EventingError::InvalidValue {
                field: constants::tracking_config_update::ERROR_PARENT_CONFIG_EVENT_APPLIED,
                value: constants::tracking_config_update::PARENT_EVENT_TYPE.to_string(),
            })
    }

    fn change_requested_event(&self) -> Result<TrackingConfigChangeRequestedEvent, EventingError> {
        lock_recover(&self.change_requested_events)
            .last()
            .cloned()
            .ok_or_else(|| EventingError::InvalidValue {
                field: constants::tracking_config_update::ERROR_PARENT_CONFIG_EVENT_APPLIED,
                value: constants::tracking_config_update::CHANGE_REQUESTED_EVENT_TYPE.to_string(),
            })
    }

    fn policy_evaluation_event(
        &self,
    ) -> Result<TrackingConfigPolicyEvaluationRequestedEvent, EventingError> {
        lock_recover(&self.policy_evaluation_events)
            .last()
            .cloned()
            .ok_or_else(|| EventingError::InvalidValue {
                field: constants::tracking_config_update::ERROR_PARENT_CONFIG_EVENT_APPLIED,
                value: constants::network_flow::EVENT_POLICY_EVALUATION_REQUESTED.to_string(),
            })
    }

    fn policy_decision_event(
        &self,
    ) -> Result<TrackingConfigPolicyDecisionCompletedEvent, EventingError> {
        lock_recover(&self.policy_decision_events)
            .last()
            .cloned()
            .ok_or_else(|| EventingError::InvalidValue {
                field: constants::tracking_config_update::ERROR_PARENT_CONFIG_EVENT_APPLIED,
                value: constants::network_flow::EVENT_POLICY_DECISION_COMPLETED.to_string(),
            })
    }

    fn dispatch_event(&self) -> Result<ParentRuntimeTrackingDispatchEvaluatedEvent, EventingError> {
        lock_recover(&self.dispatch_events)
            .last()
            .cloned()
            .ok_or_else(|| EventingError::InvalidValue {
                field: constants::tracking_config_update::ERROR_PARENT_CONFIG_EVENT_APPLIED,
                value:
                    crate::tracking_dispatch::PARENT_RUNTIME_TRACKING_DISPATCH_EVALUATED_EVENT_TYPE
                        .to_string(),
            })
    }

    fn change_approved_event(&self) -> Option<TrackingConfigChangeApprovedEvent> {
        lock_recover(&self.change_approved_events).last().cloned()
    }

    fn change_rejected_event(&self) -> Option<TrackingConfigChangeRejectedEvent> {
        lock_recover(&self.change_rejected_events).last().cloned()
    }

    fn audit_event(&self) -> Result<TrackingConfigAuditEntryCommittedEvent, EventingError> {
        lock_recover(&self.audit_events)
            .last()
            .cloned()
            .ok_or_else(|| EventingError::InvalidValue {
                field: constants::tracking_config_update::ERROR_PARENT_CONFIG_EVENT_APPLIED,
                value: constants::network_flow::EVENT_AUDIT_ENTRY_COMMITTED.to_string(),
            })
    }

    fn portal_event(&self) -> Result<TrackingConfigPortalReadModelUpdatedEvent, EventingError> {
        lock_recover(&self.portal_events)
            .last()
            .cloned()
            .ok_or_else(|| EventingError::InvalidValue {
                field: constants::tracking_config_update::ERROR_PARENT_CONFIG_EVENT_APPLIED,
                value: constants::network_flow::EVENT_PORTAL_READ_MODEL_UPDATED.to_string(),
            })
    }

    fn final_response(&self) -> Result<TrackingConfigUpdateResponse, EventingError> {
        lock_recover(&self.final_responses)
            .last()
            .cloned()
            .ok_or_else(|| EventingError::InvalidValue {
                field: constants::tracking_config_update::ERROR_PARENT_CONFIG_EVENT_APPLIED,
                value: constants::tracking_config_update::APPLIED_EVENT_TYPE.to_string(),
            })
    }

    fn child_runtime_flow(&self) -> Option<TrackingConfigUpdateEventFlowReport> {
        lock_recover(&self.child_runtime_flows).last().cloned()
    }
}

fn lock_recover<T>(value: &Arc<Mutex<T>>) -> MutexGuard<'_, T> {
    value
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
}

fn rejected_tracking_config_update_response(
    parent_event: &ParentTrackingConfigUpdatedEvent,
) -> TrackingConfigUpdateResponse {
    TrackingConfigUpdateResponse {
        schema_version: AGENT_PROTOCOL_SCHEMA_VERSION,
        source_command_id: parent_event.source_command_id.clone(),
        response_state: TrackingConfigUpdateResponseState::Rejected,
        effective_tracking_state: TrackingConfigEffectiveState::Degraded,
        child_event_type: TrackingConfigUpdateEventName::Child,
        target: parent_event.target.clone(),
        local_service_state_revision: None,
        durable_settings_persistence_state: TrackingDurableSettingsPersistenceState::NotPersisted,
    }
}

fn parent_tracking_config_updated_metadata(
    parent_event: &ParentTrackingConfigUpdatedEvent,
) -> Result<EventMetadata, EventingError> {
    tracking_config_runtime_metadata(
        parent_event.source_command_id.as_str(),
        RuntimeRole::parse(constants::eventing_source::ROLE_CONTROLLER)?,
        Some(
            constants::tracking_config_update::TARGET_HANDLER_PARENT_TRACKING_CONFIG_CHANGE_REQUESTER,
        ),
    )
}

fn tracking_config_change_requested_metadata(
    parent_event: &ParentTrackingConfigUpdatedEvent,
) -> Result<EventMetadata, EventingError> {
    tracking_config_runtime_metadata(
        parent_event.source_command_id.as_str(),
        RuntimeRole::parse(constants::eventing_source::ROLE_CONTROLLER)?,
        Some(
            constants::tracking_config_update::TARGET_HANDLER_PARENT_TRACKING_CONFIG_POLICY_REQUESTER,
        ),
    )
}

fn tracking_config_policy_evaluation_requested_metadata(
    event: &TrackingConfigChangeRequestedEvent,
) -> Result<EventMetadata, EventingError> {
    tracking_config_runtime_metadata(
        event.source_command_id.as_str(),
        RuntimeRole::parse(constants::eventing_source::ROLE_DECISION_ENGINE)?,
        Some(
            constants::tracking_config_update::TARGET_HANDLER_PARENT_TRACKING_CONFIG_POLICY_DECIDER,
        ),
    )
}

fn tracking_config_policy_decision_completed_metadata(
    source_command_id: &str,
) -> Result<EventMetadata, EventingError> {
    tracking_config_runtime_metadata(
        source_command_id,
        RuntimeRole::parse(constants::eventing_source::ROLE_DECISION_ENGINE)?,
        Some(
            constants::tracking_config_update::TARGET_HANDLER_PARENT_TRACKING_CONFIG_DECISION_APPLIER,
        ),
    )
}

fn tracking_config_controller_metadata(
    source_command_id: &str,
) -> Result<EventMetadata, EventingError> {
    tracking_config_runtime_metadata(
        source_command_id,
        RuntimeRole::parse(constants::eventing_source::ROLE_CONTROLLER)?,
        None,
    )
}

fn tracking_config_audit_entry_committed_metadata(
    source_command_id: &str,
) -> Result<EventMetadata, EventingError> {
    tracking_config_runtime_metadata(
        source_command_id,
        RuntimeRole::parse(constants::eventing_source::ROLE_AUDIT_WRITER)?,
        None,
    )
}

fn tracking_config_portal_read_model_updated_metadata(
    source_command_id: &str,
) -> Result<EventMetadata, EventingError> {
    tracking_config_runtime_metadata(
        source_command_id,
        RuntimeRole::parse(constants::eventing_source::ROLE_READ_MODEL)?,
        None,
    )
}

fn tracking_config_runtime_metadata(
    source_command_id: &str,
    runtime_role: RuntimeRole,
    target_handler: Option<&str>,
) -> Result<EventMetadata, EventingError> {
    Ok(EventMetadata::from_parts(
        EventId::generated(),
        tracking_config_runtime_correlation_id(source_command_id)?,
        tracking_config_runtime_source(runtime_role)?,
        RecordedAt::parse(constants::tracking_retention_settings_write::ACCEPTED_AT)?,
        target_handler.map(TargetHandler::parse).transpose()?,
    ))
}

fn tracking_config_runtime_source(runtime_role: RuntimeRole) -> Result<EventSource, EventingError> {
    Ok(EventSource::new(
        EventCustody::parse(constants::eventing_source::CUSTODY_LOCAL_JOURNAL)?,
        runtime_role,
        SourceService::parse(constants::peer::LOCAL_DEV_AGENT)?,
        SourceComponent::parse(constants::tracking_config_update::SOURCE_COMPONENT_PARENT_RUNTIME)?,
        RuntimeInstanceId::parse(constants::peer::PORTAL_DEV)?,
    ))
}

fn tracking_config_runtime_correlation_id(
    source_command_id: &str,
) -> Result<CorrelationId, EventingError> {
    let mut value = String::from(constants::tracking_config_update::CORRELATION_PREFIX);
    value.push_str(source_command_id);
    CorrelationId::parse(value)
}
