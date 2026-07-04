use std::{path::PathBuf, time::Duration};

use ocentra_eventing::bus::reports::handler::EventMetricsSnapshot;
use ocentra_eventing::{
    bus::subscriber::SubscriptionReport, bus::EventBus, error::EventingError,
    request::RequestOptions, request::RequestReport,
};
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::tracking::config_update_event::{
    ChildTrackingConfigUpdatedEvent, ParentTrackingConfigUpdatedEvent,
    TrackingConfigEffectiveState, TrackingConfigUpdateAppliedEvent, TrackingConfigUpdateEventName,
    TrackingConfigUpdateResponse, TrackingConfigUpdateResponseState,
    TrackingConfigUpdateTargetScope,
};
use ocentra_tracking_core::retention_settings::TrackingConfigUpdateAppliedState;
use state::TrackingConfigUpdateEventState;
use subscriptions::{
    subscribe_child_tracking_config_applied_events, subscribe_child_tracking_config_updated_events,
    subscribe_parent_tracking_config_updated_events,
};
use support::parent_tracking_config_updated_metadata;

mod state;
mod subscriptions;
mod support;

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

    pub async fn metrics_snapshot(&self) -> EventMetricsSnapshot {
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

pub fn tracking_retention_settings_durable_store_path() -> PathBuf {
    ocentra_tracking_core::retention_settings::tracking_retention_settings_durable_store_path()
}
