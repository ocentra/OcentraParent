use std::sync::{Arc, Mutex};

use crate::event_flow_scaffold;
use ocentra_eventing::error::EventingError;
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::tracking::config_update_event::{
    ChildTrackingConfigUpdatedEvent, TrackingConfigUpdateAppliedEvent,
};

use super::TrackingConfigUpdateAppliedReport;

#[derive(Clone, Debug, Default)]
pub(super) struct TrackingConfigUpdateEventState {
    child_events: Arc<Mutex<Vec<ChildTrackingConfigUpdatedEvent>>>,
    applied_events: Arc<Mutex<Vec<TrackingConfigUpdateAppliedEvent>>>,
    applied_reports: Arc<Mutex<Vec<TrackingConfigUpdateAppliedReport>>>,
}

impl TrackingConfigUpdateEventState {
    pub(super) fn record_child_event(&self, event: ChildTrackingConfigUpdatedEvent) {
        event_flow_scaffold::record_event(
            &self.child_events,
            event,
            constants::tracking_config_update::ERROR_PARENT_CONFIG_EVENT_APPLIED,
        );
    }

    pub(super) fn record_applied_event(&self, event: TrackingConfigUpdateAppliedEvent) {
        event_flow_scaffold::record_event(
            &self.applied_events,
            event,
            constants::tracking_config_update::ERROR_CHILD_CONFIG_APPLIED_EVENT_RECORDED,
        );
    }

    pub(super) fn record_applied_report(&self, report: TrackingConfigUpdateAppliedReport) {
        event_flow_scaffold::record_event(
            &self.applied_reports,
            report,
            constants::tracking_config_update::ERROR_CHILD_CONFIG_APPLIED_EVENT_RECORDED,
        );
    }

    pub(super) fn child_event(&self) -> Result<ChildTrackingConfigUpdatedEvent, EventingError> {
        event_flow_scaffold::latest_event(
            &self.child_events,
            constants::tracking_config_update::ERROR_PARENT_CONFIG_EVENT_APPLIED,
            constants::tracking_config_update::ERROR_PARENT_CONFIG_EVENT_APPLIED,
            constants::tracking_config_update::CHILD_EVENT_TYPE,
        )
    }

    pub(super) fn applied_event(&self) -> Result<TrackingConfigUpdateAppliedEvent, EventingError> {
        event_flow_scaffold::latest_event(
            &self.applied_events,
            constants::tracking_config_update::ERROR_CHILD_CONFIG_APPLIED_EVENT_RECORDED,
            constants::tracking_config_update::ERROR_CHILD_CONFIG_APPLIED_EVENT_RECORDED,
            constants::tracking_config_update::APPLIED_EVENT_TYPE,
        )
    }

    pub(super) fn applied_report(
        &self,
    ) -> Result<TrackingConfigUpdateAppliedReport, EventingError> {
        event_flow_scaffold::latest_event(
            &self.applied_reports,
            constants::tracking_config_update::ERROR_CHILD_CONFIG_APPLIED_EVENT_RECORDED,
            constants::tracking_config_update::ERROR_CHILD_CONFIG_APPLIED_EVENT_RECORDED,
            constants::tracking_config_update::APPLIED_EVENT_TYPE,
        )
    }
}
