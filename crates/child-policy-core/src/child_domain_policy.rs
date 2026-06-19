use ocentra_eventing::error::EventingError;
use ocentra_parent_agent_protocol::child_domain_runtime::{
    child_domain_policy_violation_detected_event, ChildDomainPolicyEvaluationRequestedEvent,
    ChildDomainPolicyViolationDetectedEvent,
};

pub fn evaluate_child_domain_policy(
    event: &ChildDomainPolicyEvaluationRequestedEvent,
) -> Result<ChildDomainPolicyViolationDetectedEvent, EventingError> {
    if event.event_type != event.domain.policy_evaluation_requested_event_type() {
        return Err(EventingError::InvalidValue {
            field: "child_domain_policy.event_type",
            value: event.event_type.as_str().to_string(),
        });
    }

    if event.evidence_refs.is_empty() {
        return Err(EventingError::InvalidValue {
            field: "child_domain_policy.evidence_refs",
            value: String::from("empty"),
        });
    }

    Ok(child_domain_policy_violation_detected_event(event))
}
