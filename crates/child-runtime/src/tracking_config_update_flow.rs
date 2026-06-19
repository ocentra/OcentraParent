use std::sync::{Arc, Mutex};
use std::{path::PathBuf, time::Duration};

use crate::event_flow_scaffold;
use ocentra_eventing::{
    bus::subscriber::EventSubscriber, bus::subscriber::SubscriptionReport, bus::EventBus,
    envelope::EventMetadata, envelope::EventSource, error::EventingError, ids::CorrelationId,
    ids::EventCustody, ids::EventId, ids::EventType, ids::RecordedAt, ids::RuntimeInstanceId,
    ids::RuntimeRole, ids::SourceComponent, ids::SourceService, ids::SubscriberId,
    ids::TargetHandler, request::RequestOptions, request::RequestReport,
};
use ocentra_parent_agent_protocol::{
    child_tracking_config_updated_event_from_parent, constants,
    tracking_config_update_applied_event_from_child, ChildTrackingConfigUpdatedEvent,
    ParentTrackingConfigUpdatedEvent, TrackingConfigEffectiveState,
    TrackingConfigUpdateAppliedEvent, TrackingConfigUpdateEventName, TrackingConfigUpdateResponse,
    TrackingConfigUpdateResponseState, TrackingConfigUpdateTargetScope,
};
use ocentra_tracking_core::TrackingConfigUpdateAppliedState;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TrackingConfigUpdateAppliedReport {
    pub parent_event_type: TrackingConfigUpdateEventName,
    pub child_event_type: TrackingConfigUpdateEventName,
    pub applied_event_type: TrackingConfigUpdateEventName,
    pub target_scope: TrackingConfigUpdateTargetScope,
    pub response_state: TrackingConfigUpdateResponseState,
    pub effective_tracking_state: TrackingConfigEffectiveState,
    pub applied_state: TrackingConfigUpdateAppliedState,
}

#[derive(Clone, Debug, PartialEq)]
pub struct TrackingConfigUpdateEventFlowReport {
    pub parent_subscription_report: SubscriptionReport,
    pub child_subscription_report: SubscriptionReport,
    pub applied_subscription_report: SubscriptionReport,
    pub parent_request_report: RequestReport<TrackingConfigUpdateResponse>,
    pub child_event: ChildTrackingConfigUpdatedEvent,
    pub applied_event: TrackingConfigUpdateAppliedEvent,
    pub applied_report: TrackingConfigUpdateAppliedReport,
}

pub struct TrackingConfigUpdateEventFlow {
    bus: EventBus,
    state: TrackingConfigUpdateEventState,
    parent_subscription_report: SubscriptionReport,
    child_subscription_report: SubscriptionReport,
    applied_subscription_report: SubscriptionReport,
}

impl TrackingConfigUpdateEventFlow {
    pub async fn new() -> Result<Self, EventingError> {
        let bus = EventBus::new();
        let state = TrackingConfigUpdateEventState::default();
        let applied_subscription_report =
            subscribe_child_tracking_config_applied_events(&bus, state.clone()).await?;
        let child_subscription_report =
            subscribe_child_tracking_config_updated_events(&bus).await?;
        let parent_subscription_report =
            subscribe_parent_tracking_config_updated_events(&bus, state.clone()).await?;

        Ok(Self {
            bus,
            state,
            parent_subscription_report,
            child_subscription_report,
            applied_subscription_report,
        })
    }

    pub async fn publish_parent_config_updated(
        &self,
        parent_event: &ParentTrackingConfigUpdatedEvent,
    ) -> Result<TrackingConfigUpdateEventFlowReport, EventingError> {
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
        let child_event = self.state.child_event()?;
        let applied_event = self.state.applied_event()?;
        let applied_report = self.state.applied_report()?;

        Ok(TrackingConfigUpdateEventFlowReport {
            parent_subscription_report: self.parent_subscription_report.clone(),
            child_subscription_report: self.child_subscription_report.clone(),
            applied_subscription_report: self.applied_subscription_report.clone(),
            parent_request_report,
            child_event,
            applied_event,
            applied_report,
        })
    }

    pub async fn metrics_snapshot(&self) -> ocentra_eventing::bus::reports::EventMetricsSnapshot {
        self.bus.metrics_snapshot().await
    }

    pub async fn journal_snapshot(&self) -> Vec<ocentra_eventing::envelope::StoredEventEnvelope> {
        self.bus.journal().await
    }
}

pub async fn publish_parent_tracking_config_updated_event(
    parent_event: &ParentTrackingConfigUpdatedEvent,
) -> Result<TrackingConfigUpdateEventFlowReport, EventingError> {
    TrackingConfigUpdateEventFlow::new()
        .await?
        .publish_parent_config_updated(parent_event)
        .await
}

pub async fn subscribe_parent_tracking_config_updated_events(
    bus: &EventBus,
    state: TrackingConfigUpdateEventState,
) -> Result<SubscriptionReport, EventingError> {
    bus.subscribe::<ParentTrackingConfigUpdatedEvent, _, _>(
        EventSubscriber::new(
            SubscriberId::parse(
                constants::tracking_config_update::SUBSCRIBER_PARENT_TRACKING_CONFIG_RELAY,
            )?,
            EventType::parse(constants::tracking_config_update::PARENT_EVENT_TYPE)?,
            TargetHandler::parse(
                constants::tracking_config_update::TARGET_HANDLER_PARENT_TRACKING_CONFIG_RELAY,
            )?,
        ),
        move |context| {
            let state = state.clone();
            async move {
                let child_event =
                    child_tracking_config_updated_event_from_parent(context.payload());
                state.record_child_event(child_event.clone());
                let child_event_metadata = child_tracking_config_updated_metadata(&child_event)?;
                context
                    .publisher()
                    .publish(child_event, child_event_metadata)
                    .await?;
                context
                    .complete_request(tracking_config_update_response(
                        context.payload(),
                        state.applied_report()?,
                    ))
                    .await?;
                Ok(())
            }
        },
    )
    .await
}

pub async fn subscribe_child_tracking_config_updated_events(
    bus: &EventBus,
) -> Result<SubscriptionReport, EventingError> {
    bus.subscribe::<ChildTrackingConfigUpdatedEvent, _, _>(
        EventSubscriber::new(
            SubscriberId::parse(
                constants::tracking_config_update::SUBSCRIBER_CHILD_TRACKING_CONFIG_APPLIER,
            )?,
            EventType::parse(constants::tracking_config_update::CHILD_EVENT_TYPE)?,
            TargetHandler::parse(
                constants::tracking_config_update::TARGET_HANDLER_CHILD_TRACKING_CONFIG_APPLIER,
            )?,
        ),
        move |context| async move {
            let applied_report = apply_child_tracking_config_updated_event(context.payload());
            let applied_event = tracking_config_update_applied_event_from_report(
                context.payload(),
                &applied_report,
            );
            context
                .publisher()
                .publish(
                    applied_event,
                    child_tracking_config_applied_metadata(context.payload())?,
                )
                .await?;
            Ok(())
        },
    )
    .await
}

pub async fn subscribe_child_tracking_config_applied_events(
    bus: &EventBus,
    state: TrackingConfigUpdateEventState,
) -> Result<SubscriptionReport, EventingError> {
    bus.subscribe::<TrackingConfigUpdateAppliedEvent, _, _>(
        EventSubscriber::new(
            SubscriberId::parse(
                constants::tracking_config_update::SUBSCRIBER_CHILD_TRACKING_CONFIG_APPLIED_RECORDER,
            )?,
            EventType::parse(constants::tracking_config_update::APPLIED_EVENT_TYPE)?,
            TargetHandler::parse(
                constants::tracking_config_update::TARGET_HANDLER_CHILD_TRACKING_CONFIG_APPLIED_RECORDER,
            )?,
        ),
        move |context| {
            let state = state.clone();
            async move {
                state.record_applied_event(context.payload().clone());
                state.record_applied_report(tracking_config_update_applied_report(context.payload()));
                Ok(())
            }
        },
    )
    .await
}

fn tracking_config_update_response(
    parent_event: &ParentTrackingConfigUpdatedEvent,
    applied_report: TrackingConfigUpdateAppliedReport,
) -> TrackingConfigUpdateResponse {
    TrackingConfigUpdateResponse {
        schema_version: ocentra_parent_agent_protocol::AGENT_PROTOCOL_SCHEMA_VERSION,
        source_command_id: parent_event.source_command_id.clone(),
        response_state: applied_report.response_state,
        effective_tracking_state: applied_report
            .applied_state
            .effective_tracking_state
            .clone(),
        child_event_type: applied_report.child_event_type,
        target: parent_event.target.clone(),
        local_service_state_revision: Some(
            applied_report.applied_state.local_service_state_revision,
        ),
        durable_settings_persistence_state: applied_report
            .applied_state
            .durable_settings_persistence_state,
    }
}

#[derive(Clone, Debug, Default)]
pub struct TrackingConfigUpdateEventState {
    child_events: Arc<Mutex<Vec<ChildTrackingConfigUpdatedEvent>>>,
    applied_events: Arc<Mutex<Vec<TrackingConfigUpdateAppliedEvent>>>,
    applied_reports: Arc<Mutex<Vec<TrackingConfigUpdateAppliedReport>>>,
}

impl TrackingConfigUpdateEventState {
    fn record_child_event(&self, event: ChildTrackingConfigUpdatedEvent) {
        event_flow_scaffold::record_event(
            &self.child_events,
            event,
            constants::tracking_config_update::ERROR_PARENT_CONFIG_EVENT_APPLIED,
        );
    }

    fn record_applied_event(&self, event: TrackingConfigUpdateAppliedEvent) {
        event_flow_scaffold::record_event(
            &self.applied_events,
            event,
            constants::tracking_config_update::ERROR_CHILD_CONFIG_APPLIED_EVENT_RECORDED,
        );
    }

    fn record_applied_report(&self, report: TrackingConfigUpdateAppliedReport) {
        event_flow_scaffold::record_event(
            &self.applied_reports,
            report,
            constants::tracking_config_update::ERROR_CHILD_CONFIG_APPLIED_EVENT_RECORDED,
        );
    }

    fn child_event(&self) -> Result<ChildTrackingConfigUpdatedEvent, EventingError> {
        event_flow_scaffold::latest_event(
            &self.child_events,
            constants::tracking_config_update::ERROR_PARENT_CONFIG_EVENT_APPLIED,
            constants::tracking_config_update::ERROR_PARENT_CONFIG_EVENT_APPLIED,
            constants::tracking_config_update::CHILD_EVENT_TYPE,
        )
    }

    fn applied_event(&self) -> Result<TrackingConfigUpdateAppliedEvent, EventingError> {
        event_flow_scaffold::latest_event(
            &self.applied_events,
            constants::tracking_config_update::ERROR_CHILD_CONFIG_APPLIED_EVENT_RECORDED,
            constants::tracking_config_update::ERROR_CHILD_CONFIG_APPLIED_EVENT_RECORDED,
            constants::tracking_config_update::APPLIED_EVENT_TYPE,
        )
    }

    fn applied_report(&self) -> Result<TrackingConfigUpdateAppliedReport, EventingError> {
        event_flow_scaffold::latest_event(
            &self.applied_reports,
            constants::tracking_config_update::ERROR_CHILD_CONFIG_APPLIED_EVENT_RECORDED,
            constants::tracking_config_update::ERROR_CHILD_CONFIG_APPLIED_EVENT_RECORDED,
            constants::tracking_config_update::APPLIED_EVENT_TYPE,
        )
    }
}

pub fn tracking_config_update_event_bus() -> EventBus {
    EventBus::new()
}

pub fn tracking_config_update_parent_event_type() -> TrackingConfigUpdateEventName {
    TrackingConfigUpdateEventName::Parent
}

pub fn tracking_config_update_child_event_type() -> TrackingConfigUpdateEventName {
    TrackingConfigUpdateEventName::Child
}

pub fn tracking_config_update_applied_event_type() -> TrackingConfigUpdateEventName {
    TrackingConfigUpdateEventName::Applied
}

fn apply_child_tracking_config_updated_event(
    child_event: &ChildTrackingConfigUpdatedEvent,
) -> TrackingConfigUpdateAppliedReport {
    let applied_state = on_child_tracking_config_updated_event(child_event);
    let effective_tracking_state = applied_state.effective_tracking_state.clone();

    TrackingConfigUpdateAppliedReport {
        parent_event_type: child_event.parent_event_type.clone(),
        child_event_type: TrackingConfigUpdateEventName::Child,
        applied_event_type: TrackingConfigUpdateEventName::Applied,
        target_scope: child_event.target.scope.clone(),
        response_state: TrackingConfigUpdateResponseState::Applied,
        effective_tracking_state,
        applied_state,
    }
}

fn on_child_tracking_config_updated_event(
    child_event: &ChildTrackingConfigUpdatedEvent,
) -> TrackingConfigUpdateAppliedState {
    ocentra_tracking_core::apply_tracking_config_update(&child_event.config)
}

fn parent_tracking_config_updated_metadata(
    parent_event: &ParentTrackingConfigUpdatedEvent,
) -> Result<EventMetadata, EventingError> {
    tracking_config_update_metadata(
        parent_event.source_command_id.as_str(),
        tracking_parent_event_source()?,
        constants::tracking_config_update::TARGET_HANDLER_PARENT_TRACKING_CONFIG_RELAY,
    )
}

fn child_tracking_config_updated_metadata(
    child_event: &ChildTrackingConfigUpdatedEvent,
) -> Result<EventMetadata, EventingError> {
    tracking_config_update_metadata(
        child_event.source_command_id.as_str(),
        tracking_child_event_source()?,
        constants::tracking_config_update::TARGET_HANDLER_CHILD_TRACKING_CONFIG_APPLIER,
    )
}

fn child_tracking_config_applied_metadata(
    child_event: &ChildTrackingConfigUpdatedEvent,
) -> Result<EventMetadata, EventingError> {
    tracking_config_update_metadata(
        child_event.source_command_id.as_str(),
        tracking_child_event_source()?,
        constants::tracking_config_update::TARGET_HANDLER_CHILD_TRACKING_CONFIG_APPLIED_RECORDER,
    )
}

fn tracking_config_update_metadata(
    source_command_id: &str,
    source: EventSource,
    target_handler: &str,
) -> Result<EventMetadata, EventingError> {
    Ok(EventMetadata::from_parts(
        EventId::generated(),
        tracking_config_update_correlation_id(source_command_id)?,
        source,
        RecordedAt::parse(constants::tracking_retention_settings_write::ACCEPTED_AT)?,
        Some(TargetHandler::parse(target_handler)?),
    ))
}

fn tracking_parent_event_source() -> Result<EventSource, EventingError> {
    Ok(EventSource::new(
        EventCustody::parse(constants::eventing_source::CUSTODY_LOCAL_JOURNAL)?,
        RuntimeRole::parse(constants::eventing_source::ROLE_CONTROLLER)?,
        SourceService::parse(constants::peer::LOCAL_DEV_AGENT)?,
        SourceComponent::parse(
            constants::tracking_config_update::SOURCE_COMPONENT_PARENT_AGENT_SERVICE,
        )?,
        RuntimeInstanceId::parse(constants::peer::PORTAL_DEV)?,
    ))
}

fn tracking_child_event_source() -> Result<EventSource, EventingError> {
    Ok(EventSource::new(
        EventCustody::parse(constants::child_agent::CUSTODY_CHILD_AGENT_RUNTIME)?,
        RuntimeRole::parse(constants::eventing_source::ROLE_AGENT)?,
        SourceService::parse(constants::peer::LOCAL_DEV_AGENT)?,
        SourceComponent::parse(
            constants::tracking_config_update::SOURCE_COMPONENT_CHILD_TRACKING_RUNTIME,
        )?,
        RuntimeInstanceId::parse(constants::child_agent::RUNTIME_INSTANCE_LOCAL_CHILD_AGENT)?,
    ))
}

fn tracking_config_update_correlation_id(
    source_command_id: &str,
) -> Result<CorrelationId, EventingError> {
    let mut value = String::from(constants::tracking_config_update::CORRELATION_PREFIX);
    value.push_str(source_command_id);
    CorrelationId::parse(value)
}

fn tracking_config_update_applied_event_from_report(
    child_event: &ChildTrackingConfigUpdatedEvent,
    applied_report: &TrackingConfigUpdateAppliedReport,
) -> TrackingConfigUpdateAppliedEvent {
    tracking_config_update_applied_event_from_child(
        child_event,
        applied_report.response_state.clone(),
        applied_report.effective_tracking_state.clone(),
        applied_report.applied_state.local_service_state_revision,
        applied_report
            .applied_state
            .durable_settings_persistence_state,
    )
}

fn tracking_config_update_applied_report(
    applied_event: &TrackingConfigUpdateAppliedEvent,
) -> TrackingConfigUpdateAppliedReport {
    TrackingConfigUpdateAppliedReport {
        parent_event_type: applied_event.parent_event_type.clone(),
        child_event_type: applied_event.child_event_type.clone(),
        applied_event_type: TrackingConfigUpdateEventName::Applied,
        target_scope: applied_event.target.scope.clone(),
        response_state: applied_event.response_state.clone(),
        effective_tracking_state: applied_event.effective_tracking_state.clone(),
        applied_state: TrackingConfigUpdateAppliedState {
            local_service_state_revision: applied_event.local_service_state_revision,
            durable_settings_persistence_state: applied_event.durable_settings_persistence_state,
            effective_tracking_state: applied_event.effective_tracking_state.clone(),
        },
    }
}

pub fn tracking_retention_settings_durable_store_path() -> PathBuf {
    ocentra_tracking_core::tracking_retention_settings_durable_store_path()
}
