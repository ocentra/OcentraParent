#![forbid(unsafe_code)]

use ocentra_parent_agent_protocol::{
    child_domain_policy_violation_detected_event, constants,
    ChildDomainPolicyEvaluationRequestedEvent, ChildDomainPolicyViolationDetectedEvent,
    TrackingNearbyPlaceClassifiedEvent, TrackingParentActionRequirement,
    TrackingPolicyRuleRef, TrackingPolicySeverity, TrackingPolicyViolationDetectedEvent,
    TrackingPolicyViolationId,
};

pub const CRATE_NAME: &str = "ocentra-child-policy-core";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TrackingPolicyViolationState {
    Detected,
    NotDetected,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TrackingNearbyPlacePolicyDecision {
    pub violation_state: TrackingPolicyViolationState,
    pub policy_violation_detected: Option<TrackingPolicyViolationDetectedEvent>,
}

pub fn evaluate_tracking_nearby_place_policy(
    event: &TrackingNearbyPlaceClassifiedEvent,
) -> TrackingNearbyPlacePolicyDecision {
    if event.parent_action_requirement != TrackingParentActionRequirement::Required {
        return TrackingNearbyPlacePolicyDecision {
            violation_state: TrackingPolicyViolationState::NotDetected,
            policy_violation_detected: None,
        };
    }

    if event.place_category != constants::tracking_runtime::PLACE_CATEGORY_HOSPITAL {
        return TrackingNearbyPlacePolicyDecision {
            violation_state: TrackingPolicyViolationState::NotDetected,
            policy_violation_detected: None,
        };
    }

    TrackingNearbyPlacePolicyDecision {
        violation_state: TrackingPolicyViolationState::Detected,
        policy_violation_detected: Some(TrackingPolicyViolationDetectedEvent {
            child_device_id: event.child_device_id.clone(),
            child_profile_id: event.child_profile_id.clone(),
            violation_id: TrackingPolicyViolationId::parse(
                constants::tracking_runtime::DEFAULT_POLICY_VIOLATION_ID,
            )
            .expect(constants::tracking_runtime::DEFAULT_POLICY_VIOLATION_ID),
            policy_rule_ref: TrackingPolicyRuleRef::parse(
                constants::tracking_runtime::POLICY_RULE_EXPECTED_PLACE,
            )
            .expect(constants::tracking_runtime::POLICY_RULE_EXPECTED_PLACE),
            severity: TrackingPolicySeverity::parse(
                constants::tracking_runtime::POLICY_SEVERITY_REVIEW,
            )
            .expect(constants::tracking_runtime::POLICY_SEVERITY_REVIEW),
            evidence_refs: event.evidence_refs.clone(),
        }),
    }
}

pub fn evaluate_child_domain_policy(
    event: &ChildDomainPolicyEvaluationRequestedEvent,
) -> ChildDomainPolicyViolationDetectedEvent {
    child_domain_policy_violation_detected_event(event)
}
