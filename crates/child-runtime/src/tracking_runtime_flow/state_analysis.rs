use crate::event_flow_scaffold;
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::tracking::runtime_event::{
    ParentNotificationRequestedEvent, TrackingAiAnalysisRequestedEvent,
    TrackingNearbyPlaceClassifiedEvent, TrackingPolicyViolationDetectedEvent,
};
use ocentra_tracking_core::ai_boundary::TrackingAiBoundaryDecision;
use ocentra_tracking_core::alerting::TrackingAlertDecision;

use super::state::TrackingRuntimeEventState;

impl TrackingRuntimeEventState {
    pub(super) fn record_ai_analysis_request(&self, event: TrackingAiAnalysisRequestedEvent) {
        event_flow_scaffold::record_optional_event(
            &self.ai_analysis_requested,
            event,
            constants::tracking_runtime::ERROR_TRACKING_RUNTIME_FLOW_RECORDED,
        );
    }

    pub(super) fn record_nearby_place_classified(&self, event: TrackingNearbyPlaceClassifiedEvent) {
        event_flow_scaffold::record_optional_event(
            &self.nearby_place_classified,
            event,
            constants::tracking_runtime::ERROR_TRACKING_RUNTIME_FLOW_RECORDED,
        );
    }

    pub(super) fn record_ai_boundary_decision(&self, decision: TrackingAiBoundaryDecision) {
        event_flow_scaffold::record_optional_event(
            &self.ai_boundary_decision,
            decision,
            constants::tracking_runtime::ERROR_TRACKING_RUNTIME_FLOW_RECORDED,
        );
    }

    pub(super) fn record_alert_decision(&self, decision: TrackingAlertDecision) {
        event_flow_scaffold::record_optional_event(
            &self.alert_decision,
            decision,
            constants::tracking_runtime::ERROR_TRACKING_RUNTIME_FLOW_RECORDED,
        );
    }

    pub(super) fn record_policy_violation_detected(
        &self,
        event: TrackingPolicyViolationDetectedEvent,
    ) {
        event_flow_scaffold::record_optional_event(
            &self.policy_violation_detected,
            event,
            constants::tracking_runtime::ERROR_TRACKING_RUNTIME_FLOW_RECORDED,
        );
    }

    pub(super) fn record_parent_notification_requested(
        &self,
        event: ParentNotificationRequestedEvent,
    ) {
        event_flow_scaffold::record_optional_event(
            &self.parent_notification_requested,
            event,
            constants::tracking_runtime::ERROR_TRACKING_RUNTIME_FLOW_RECORDED,
        );
    }

    pub(super) fn record_policy_violation_history(
        &self,
        event: TrackingPolicyViolationDetectedEvent,
    ) {
        let mut history = crate::event_flow_scaffold::lock_recover(&self.policy_violation_history);
        if history.len() >= 32 {
            history.remove(0);
        }
        history.push(event);
    }

    pub(super) fn ai_analysis_requested(&self) -> Option<TrackingAiAnalysisRequestedEvent> {
        event_flow_scaffold::optional_event(&self.ai_analysis_requested)
    }

    pub(super) fn nearby_place_classified(&self) -> Option<TrackingNearbyPlaceClassifiedEvent> {
        event_flow_scaffold::optional_event(&self.nearby_place_classified)
    }

    pub(super) fn ai_boundary_decision(&self) -> Option<TrackingAiBoundaryDecision> {
        event_flow_scaffold::optional_event(&self.ai_boundary_decision)
    }

    pub(super) fn alert_decision(&self) -> Option<TrackingAlertDecision> {
        event_flow_scaffold::optional_event(&self.alert_decision)
    }

    pub(super) fn policy_violation_detected(&self) -> Option<TrackingPolicyViolationDetectedEvent> {
        event_flow_scaffold::optional_event(&self.policy_violation_detected)
    }

    pub(super) fn parent_notification_requested(&self) -> Option<ParentNotificationRequestedEvent> {
        event_flow_scaffold::optional_event(&self.parent_notification_requested)
    }

    pub(super) fn recent_policy_violation_duplicate_count(
        &self,
        event: &TrackingPolicyViolationDetectedEvent,
    ) -> u16 {
        let Ok(history) = self.policy_violation_history.lock() else {
            return 0;
        };
        history
            .iter()
            .filter(|prior| same_policy_violation(prior, event))
            .count()
            .min(u16::MAX as usize) as u16
    }
}

fn same_policy_violation(
    left: &TrackingPolicyViolationDetectedEvent,
    right: &TrackingPolicyViolationDetectedEvent,
) -> bool {
    left.child_device_id == right.child_device_id
        && left.child_profile_id == right.child_profile_id
        && left.policy_rule_ref == right.policy_rule_ref
        && left.severity == right.severity
        && left.evidence_refs == right.evidence_refs
}
