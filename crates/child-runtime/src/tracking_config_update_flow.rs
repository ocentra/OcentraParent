use std::sync::{Arc, Mutex};
use std::{path::PathBuf, time::Duration};

use ocentra_eventing::{
    CorrelationId, EventBus, EventCustody, EventId, EventMetadata, EventSource, EventSubscriber,
    EventType, EventingError, RecordedAt, RequestOptions, RequestReport, RuntimeInstanceId,
    RuntimeRole, SourceComponent, SourceService, SubscriberId, SubscriptionReport, TargetHandler,
};
use ocentra_parent_agent_protocol::{
    child_tracking_config_updated_event_from_parent, constants, ChildTrackingConfigUpdatedEvent,
    ParentTrackingConfigUpdatedEvent, TrackingConfigEffectiveState, TrackingConfigUpdateEventName,
    TrackingConfigUpdateResponse, TrackingConfigUpdateResponseState,
    TrackingConfigUpdateTargetScope,
};
use ocentra_tracking_core::TrackingRetentionSettingsWriteAppliedState;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TrackingConfigUpdateAppliedReport {
    pub parent_event_type: TrackingConfigUpdateEventName,
    pub child_event_type: TrackingConfigUpdateEventName,
    pub target_scope: TrackingConfigUpdateTargetScope,
    pub applied_state: TrackingRetentionSettingsWriteAppliedState,
}

#[derive(Clone, Debug, PartialEq)]
pub struct TrackingConfigUpdateEventFlowReport {
    pub parent_subscription_report: SubscriptionReport,
    pub child_subscription_report: SubscriptionReport,
    pub parent_request_report: RequestReport<TrackingConfigUpdateResponse>,
    pub applied_report: TrackingConfigUpdateAppliedReport,
}

pub struct TrackingConfigUpdateEventFlow {
    bus: EventBus,
    state: TrackingConfigUpdateEventState,
    parent_subscription_report: SubscriptionReport,
    child_subscription_report: SubscriptionReport,
}

impl TrackingConfigUpdateEventFlow {
    pub async fn new() -> Result<Self, EventingError> {
        let bus = EventBus::new();
        let state = TrackingConfigUpdateEventState::default();
        let child_subscription_report =
            subscribe_child_tracking_config_updated_events(&bus, state.clone()).await?;
        let parent_subscription_report =
            subscribe_parent_tracking_config_updated_events(&bus, state.clone()).await?;

        Ok(Self {
            bus,
            state,
            parent_subscription_report,
            child_subscription_report,
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
        let applied_report = self.state.applied_report()?;

        Ok(TrackingConfigUpdateEventFlowReport {
            parent_subscription_report: self.parent_subscription_report.clone(),
            child_subscription_report: self.child_subscription_report.clone(),
            parent_request_report,
            applied_report,
        })
    }

    pub async fn metrics_snapshot(&self) -> ocentra_eventing::EventMetricsSnapshot {
        self.bus.metrics_snapshot().await
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
                context
                    .publisher()
                    .publish(
                        child_event,
                        child_tracking_config_updated_metadata(context.payload())?,
                    )
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
    state: TrackingConfigUpdateEventState,
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
        move |context| {
            let state = state.clone();
            async move {
                let applied_report = apply_child_tracking_config_updated_event(context.payload());
                state.record(applied_report);
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
        response_state: TrackingConfigUpdateResponseState::Applied,
        effective_tracking_state: effective_tracking_state(parent_event),
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

fn effective_tracking_state(
    parent_event: &ParentTrackingConfigUpdatedEvent,
) -> TrackingConfigEffectiveState {
    if parent_event
        .config
        .requested_retention_window_hours
        .is_some()
    {
        TrackingConfigEffectiveState::Enabled
    } else {
        TrackingConfigEffectiveState::Disabled
    }
}

#[derive(Clone, Debug, Default)]
pub struct TrackingConfigUpdateEventState {
    applied_reports: Arc<Mutex<Vec<TrackingConfigUpdateAppliedReport>>>,
}

impl TrackingConfigUpdateEventState {
    fn record(&self, report: TrackingConfigUpdateAppliedReport) {
        self.applied_reports
            .lock()
            .expect(constants::tracking_config_update::ERROR_PARENT_CONFIG_EVENT_APPLIED)
            .push(report);
    }

    fn applied_report(&self) -> Result<TrackingConfigUpdateAppliedReport, EventingError> {
        self.applied_reports
            .lock()
            .expect(constants::tracking_config_update::ERROR_PARENT_CONFIG_EVENT_APPLIED)
            .last()
            .cloned()
            .ok_or_else(|| EventingError::InvalidValue {
                field: constants::tracking_config_update::ERROR_PARENT_CONFIG_EVENT_APPLIED,
                value: constants::tracking_config_update::CHILD_EVENT_TYPE.to_string(),
            })
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

fn apply_child_tracking_config_updated_event(
    child_event: &ChildTrackingConfigUpdatedEvent,
) -> TrackingConfigUpdateAppliedReport {
    let applied_state = on_child_tracking_config_updated_event(child_event);

    TrackingConfigUpdateAppliedReport {
        parent_event_type: child_event.parent_event_type.clone(),
        child_event_type: TrackingConfigUpdateEventName::Child,
        target_scope: child_event.target.scope.clone(),
        applied_state,
    }
}

fn on_child_tracking_config_updated_event(
    child_event: &ChildTrackingConfigUpdatedEvent,
) -> TrackingRetentionSettingsWriteAppliedState {
    ocentra_tracking_core::apply_tracking_retention_settings_write(&child_event.config)
}

fn parent_tracking_config_updated_metadata(
    parent_event: &ParentTrackingConfigUpdatedEvent,
) -> Result<EventMetadata, EventingError> {
    tracking_config_update_metadata(
        parent_event,
        tracking_parent_event_source()?,
        constants::tracking_config_update::TARGET_HANDLER_PARENT_TRACKING_CONFIG_RELAY,
    )
}

fn child_tracking_config_updated_metadata(
    parent_event: &ParentTrackingConfigUpdatedEvent,
) -> Result<EventMetadata, EventingError> {
    tracking_config_update_metadata(
        parent_event,
        tracking_child_event_source()?,
        constants::tracking_config_update::TARGET_HANDLER_CHILD_TRACKING_CONFIG_APPLIER,
    )
}

fn tracking_config_update_metadata(
    parent_event: &ParentTrackingConfigUpdatedEvent,
    source: EventSource,
    target_handler: &str,
) -> Result<EventMetadata, EventingError> {
    Ok(EventMetadata::from_parts(
        EventId::generated(),
        tracking_config_update_correlation_id(parent_event)?,
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
    parent_event: &ParentTrackingConfigUpdatedEvent,
) -> Result<CorrelationId, EventingError> {
    let mut value = String::from(constants::tracking_config_update::CORRELATION_PREFIX);
    value.push_str(parent_event.source_command_id.as_str());
    CorrelationId::parse(value)
}

pub fn tracking_retention_settings_durable_store_path() -> PathBuf {
    ocentra_tracking_core::tracking_retention_settings_durable_store_path()
}
