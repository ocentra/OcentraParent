use ocentra_parent_agent_protocol::{
    child_domain_policy_violation_detected_event, ChildDomainPolicyEvaluationRequestedEvent,
    ChildDomainPolicyViolationDetectedEvent,
};

pub fn evaluate_child_domain_policy(
    event: &ChildDomainPolicyEvaluationRequestedEvent,
) -> ChildDomainPolicyViolationDetectedEvent {
    child_domain_policy_violation_detected_event(event)
}
