use ocentra_parent_agent_protocol::{
    constants, tracking_alert_evaluation_id_from_violation_id, TrackingAlertEvaluationId,
    TrackingAlertSeverity, TrackingEvidenceRef, TrackingPolicyViolationDetectedEvent,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TrackingParentNotificationDecisionState {
    Allowed,
    SuppressedDuplicate,
    SuppressedMissingEvidence,
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
    let parent_notification_state = parent_notification_state_for(event, recent_duplicate_count);

    TrackingAlertDecision {
        alert_evaluation_id: tracking_alert_evaluation_id_from_violation_id(&event.violation_id),
        severity: alert_severity_for(event, &parent_notification_state),
        parent_notification_state,
        evidence_refs: event.evidence_refs.clone(),
    }
}

fn parent_notification_state_for(
    event: &TrackingPolicyViolationDetectedEvent,
    recent_duplicate_count: u16,
) -> TrackingParentNotificationDecisionState {
    if event.evidence_refs.is_empty() {
        TrackingParentNotificationDecisionState::SuppressedMissingEvidence
    } else if recent_duplicate_count > 0 {
        TrackingParentNotificationDecisionState::SuppressedDuplicate
    } else {
        TrackingParentNotificationDecisionState::Allowed
    }
}

fn alert_severity_for(
    event: &TrackingPolicyViolationDetectedEvent,
    parent_notification_state: &TrackingParentNotificationDecisionState,
) -> TrackingAlertSeverity {
    let severity = match parent_notification_state {
        TrackingParentNotificationDecisionState::SuppressedMissingEvidence => {
            constants::tracking_runtime::ALERT_SEVERITY_INFO
        }
        TrackingParentNotificationDecisionState::Allowed
        | TrackingParentNotificationDecisionState::SuppressedDuplicate => {
            match event.severity.as_str() {
                constants::tracking_runtime::POLICY_SEVERITY_REVIEW => {
                    constants::tracking_runtime::ALERT_SEVERITY_WATCH
                }
                constants::tracking_runtime::POLICY_SEVERITY_WARNING => {
                    constants::tracking_runtime::ALERT_SEVERITY_WARNING
                }
                constants::tracking_runtime::POLICY_SEVERITY_URGENT => {
                    constants::tracking_runtime::ALERT_SEVERITY_URGENT
                }
                constants::tracking_runtime::POLICY_SEVERITY_CRITICAL => {
                    constants::tracking_runtime::ALERT_SEVERITY_CRITICAL
                }
                _ => constants::tracking_runtime::ALERT_SEVERITY_INFO,
            }
        }
    };
    TrackingAlertSeverity::parse(severity).expect(severity)
}
