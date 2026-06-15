#![forbid(unsafe_code)]

use ocentra_parent_agent_protocol::{
    child_domain_ref, constants, ChildDomainPolicyEvaluationRequestedEvent,
    ChildDomainPolicyViolationDetectedEvent, TrackingNearbyPlaceClassifiedEvent,
    TrackingPolicyViolationDetectedEvent,
};

pub const CRATE_NAME: &str = "ocentra-child-policy-core";

pub fn evaluate_tracking_nearby_place_policy(
    event: &TrackingNearbyPlaceClassifiedEvent,
) -> Option<TrackingPolicyViolationDetectedEvent> {
    if event.place_category != constants::tracking_runtime::PLACE_CATEGORY_HOSPITAL {
        return None;
    }

    Some(TrackingPolicyViolationDetectedEvent {
        child_device_id: event.child_device_id.clone(),
        child_profile_id: event.child_profile_id.clone(),
        violation_id: constants::tracking_runtime::DEFAULT_POLICY_VIOLATION_ID.to_string(),
        policy_rule_ref: constants::tracking_runtime::POLICY_RULE_EXPECTED_PLACE.to_string(),
        severity: constants::tracking_runtime::POLICY_SEVERITY_REVIEW.to_string(),
        evidence_refs: event.evidence_refs.clone(),
    })
}

pub fn evaluate_child_domain_policy(
    event: &ChildDomainPolicyEvaluationRequestedEvent,
) -> ChildDomainPolicyViolationDetectedEvent {
    ChildDomainPolicyViolationDetectedEvent {
        event_type: constants::child_domain_runtime::POLICY_VIOLATION_DETECTED_EVENT_TYPE
            .to_string(),
        domain: event.domain.clone(),
        child_device_id: event.child_device_id.clone(),
        child_profile_id: event.child_profile_id.clone(),
        violation_id: child_domain_ref(
            &event.domain,
            constants::child_domain_runtime::DEFAULT_POLICY_VIOLATION_ID_SUFFIX,
        ),
        policy_rule_ref: constants::child_domain_runtime::POLICY_RULE_DEFAULT.to_string(),
        severity: constants::child_domain_runtime::POLICY_SEVERITY_REVIEW.to_string(),
        evidence_refs: event.evidence_refs.clone(),
    }
}
