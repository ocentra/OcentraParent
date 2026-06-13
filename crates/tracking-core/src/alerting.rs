use ocentra_parent_agent_protocol::{
    constants, TrackingAlertEvaluationId, TrackingAlertSeverity, TrackingEvidenceRef,
    TrackingPolicyViolationDetectedEvent,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TrackingParentNotificationDecisionState {
    Allowed,
    Suppressed,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TrackingAlertDecision {
    pub alert_evaluation_id: TrackingAlertEvaluationId,
    pub severity: TrackingAlertSeverity,
    pub parent_notification_state: TrackingParentNotificationDecisionState,
    pub evidence_refs: Vec<TrackingEvidenceRef>,
}

pub fn evaluate_tracking_alert(
    event: &TrackingPolicyViolationDetectedEvent,
    recent_duplicate_count: u16,
) -> TrackingAlertDecision {
    let parent_notification_state =
        if recent_duplicate_count == 0 && !event.evidence_refs.is_empty() {
            TrackingParentNotificationDecisionState::Allowed
        } else {
            TrackingParentNotificationDecisionState::Suppressed
        };

    TrackingAlertDecision {
        alert_evaluation_id: TrackingAlertEvaluationId::parse(
            constants::tracking_runtime::DEFAULT_ALERT_EVALUATION_ID,
        )
        .expect(constants::tracking_runtime::DEFAULT_ALERT_EVALUATION_ID),
        severity: alert_severity_for(event, &parent_notification_state),
        parent_notification_state,
        evidence_refs: event.evidence_refs.clone(),
    }
}

fn alert_severity_for(
    event: &TrackingPolicyViolationDetectedEvent,
    parent_notification_state: &TrackingParentNotificationDecisionState,
) -> TrackingAlertSeverity {
    if parent_notification_state == &TrackingParentNotificationDecisionState::Suppressed {
        TrackingAlertSeverity::parse(constants::tracking_runtime::ALERT_SEVERITY_NONE)
            .expect(constants::tracking_runtime::ALERT_SEVERITY_NONE)
    } else if event.severity == constants::tracking_runtime::POLICY_SEVERITY_REVIEW {
        TrackingAlertSeverity::parse(constants::tracking_runtime::ALERT_SEVERITY_REVIEW)
            .expect(constants::tracking_runtime::ALERT_SEVERITY_REVIEW)
    } else {
        TrackingAlertSeverity::parse(constants::tracking_runtime::ALERT_SEVERITY_URGENT)
            .expect(constants::tracking_runtime::ALERT_SEVERITY_URGENT)
    }
}
