use crate::event_flow_scaffold;
use ocentra_eventing::error::EventingError;
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::tracking::runtime_event::{
    TrackingChildCheckInRecordedEvent, TrackingEvidenceRecordedEvent,
    TrackingExpectedPlaceStateEvaluatedEvent, TrackingGeofenceTransitionDetectedEvent,
    TrackingLocationObservedEvent,
};

use super::state::TrackingRuntimeEventState;

impl TrackingRuntimeEventState {
    pub(super) fn reset_for_new_observation(&self) {
        event_flow_scaffold::clear_optional_event(
            &self.location_observed,
            constants::tracking_runtime::ERROR_TRACKING_RUNTIME_FLOW_RECORDED,
        );
        event_flow_scaffold::clear_optional_event(
            &self.evidence_recorded,
            constants::tracking_runtime::ERROR_TRACKING_RUNTIME_FLOW_RECORDED,
        );
        event_flow_scaffold::clear_optional_event(
            &self.geofence_transition_detected,
            constants::tracking_runtime::ERROR_TRACKING_RUNTIME_FLOW_RECORDED,
        );
        event_flow_scaffold::clear_optional_event(
            &self.expected_place_state_evaluated,
            constants::tracking_runtime::ERROR_TRACKING_RUNTIME_FLOW_RECORDED,
        );
        event_flow_scaffold::clear_optional_event(
            &self.child_check_in_recorded,
            constants::tracking_runtime::ERROR_TRACKING_RUNTIME_FLOW_RECORDED,
        );
        event_flow_scaffold::clear_optional_event(
            &self.ai_analysis_requested,
            constants::tracking_runtime::ERROR_TRACKING_RUNTIME_FLOW_RECORDED,
        );
        event_flow_scaffold::clear_optional_event(
            &self.nearby_place_classified,
            constants::tracking_runtime::ERROR_TRACKING_RUNTIME_FLOW_RECORDED,
        );
        event_flow_scaffold::clear_optional_event(
            &self.ai_boundary_decision,
            constants::tracking_runtime::ERROR_TRACKING_RUNTIME_FLOW_RECORDED,
        );
        event_flow_scaffold::clear_optional_event(
            &self.alert_decision,
            constants::tracking_runtime::ERROR_TRACKING_RUNTIME_FLOW_RECORDED,
        );
        event_flow_scaffold::clear_optional_event(
            &self.policy_violation_detected,
            constants::tracking_runtime::ERROR_TRACKING_RUNTIME_FLOW_RECORDED,
        );
        event_flow_scaffold::clear_optional_event(
            &self.parent_notification_requested,
            constants::tracking_runtime::ERROR_TRACKING_RUNTIME_FLOW_RECORDED,
        );
    }

    pub(super) fn record_location_observed(&self, event: TrackingLocationObservedEvent) {
        event_flow_scaffold::record_optional_event(
            &self.location_observed,
            event,
            constants::tracking_runtime::ERROR_TRACKING_RUNTIME_FLOW_RECORDED,
        );
    }

    pub(super) fn record_evidence(&self, event: TrackingEvidenceRecordedEvent) {
        event_flow_scaffold::record_optional_event(
            &self.evidence_recorded,
            event,
            constants::tracking_runtime::ERROR_TRACKING_RUNTIME_FLOW_RECORDED,
        );
    }

    pub(super) fn record_geofence_transition(
        &self,
        event: TrackingGeofenceTransitionDetectedEvent,
    ) {
        event_flow_scaffold::record_optional_event(
            &self.geofence_transition_detected,
            event,
            constants::tracking_runtime::ERROR_TRACKING_RUNTIME_FLOW_RECORDED,
        );
    }

    pub(super) fn record_expected_place_state(
        &self,
        event: TrackingExpectedPlaceStateEvaluatedEvent,
    ) {
        event_flow_scaffold::record_optional_event(
            &self.expected_place_state_evaluated,
            event,
            constants::tracking_runtime::ERROR_TRACKING_RUNTIME_FLOW_RECORDED,
        );
    }

    pub(super) fn record_child_check_in(&self, event: TrackingChildCheckInRecordedEvent) {
        event_flow_scaffold::record_optional_event(
            &self.child_check_in_recorded,
            event,
            constants::tracking_runtime::ERROR_TRACKING_RUNTIME_FLOW_RECORDED,
        );
    }

    pub(super) fn evidence_recorded(&self) -> Result<TrackingEvidenceRecordedEvent, EventingError> {
        event_flow_scaffold::required_optional_event(
            &self.evidence_recorded,
            constants::tracking_runtime::ERROR_TRACKING_RUNTIME_FLOW_RECORDED,
            constants::tracking_runtime::ERROR_TRACKING_RUNTIME_FLOW_RECORDED,
            constants::tracking_runtime::TRACKING_LOCATION_OBSERVED_EVENT_TYPE,
        )
    }

    pub(super) fn location_observed(&self) -> Option<TrackingLocationObservedEvent> {
        event_flow_scaffold::optional_event(&self.location_observed)
    }

    pub(super) fn geofence_transition_detected(
        &self,
    ) -> Option<TrackingGeofenceTransitionDetectedEvent> {
        event_flow_scaffold::optional_event(&self.geofence_transition_detected)
    }

    pub(super) fn expected_place_state_evaluated(
        &self,
    ) -> Option<TrackingExpectedPlaceStateEvaluatedEvent> {
        event_flow_scaffold::optional_event(&self.expected_place_state_evaluated)
    }

    pub(super) fn child_check_in_recorded(&self) -> Option<TrackingChildCheckInRecordedEvent> {
        event_flow_scaffold::optional_event(&self.child_check_in_recorded)
    }
}
