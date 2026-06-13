use ocentra_parent_agent_protocol::{
    constants, TrackingExpectedPlaceStateEvaluatedEvent, TrackingNearbyPlaceClassifiedEvent,
    TrackingParentActionRequirement,
    TrackingPolicyRuleRef, TrackingPolicySeverity, TrackingPolicyViolationDetectedEvent,
    TrackingPolicyViolationId,
};

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

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TrackingExpectedPlacePolicyDecision {
    pub violation_state: TrackingPolicyViolationState,
    pub policy_violation_detected: Option<TrackingPolicyViolationDetectedEvent>,
}

pub fn evaluate_tracking_nearby_place_policy(
    event: &TrackingNearbyPlaceClassifiedEvent,
) -> TrackingNearbyPlacePolicyDecision {
    if event.parent_action_requirement != TrackingParentActionRequirement::Required {
        return tracking_nearby_place_not_detected();
    }

    if event.place_category != constants::tracking_runtime::PLACE_CATEGORY_HOSPITAL {
        return tracking_nearby_place_not_detected();
    }

    TrackingNearbyPlacePolicyDecision {
        violation_state: TrackingPolicyViolationState::Detected,
        policy_violation_detected: Some(tracking_policy_violation_detected(
            event.child_device_id.clone(),
            event.child_profile_id.clone(),
            event.evidence_refs.clone(),
        )),
    }
}

pub fn evaluate_tracking_expected_place_policy(
    event: &TrackingExpectedPlaceStateEvaluatedEvent,
) -> TrackingExpectedPlacePolicyDecision {
    if event.parent_action_requirement != TrackingParentActionRequirement::Required {
        return tracking_expected_place_not_detected();
    }

    if event.expected_place_state != constants::tracking_runtime::EXPECTED_PLACE_STATE_LEFT_EXPECTED_PLACE
        && event.expected_place_state
            != constants::tracking_runtime::EXPECTED_PLACE_STATE_LATE_ARRIVAL
    {
        return tracking_expected_place_not_detected();
    }

    TrackingExpectedPlacePolicyDecision {
        violation_state: TrackingPolicyViolationState::Detected,
        policy_violation_detected: Some(tracking_policy_violation_detected(
            event.child_device_id.clone(),
            event.child_profile_id.clone(),
            event.evidence_refs.clone(),
        )),
    }
}

fn tracking_nearby_place_not_detected() -> TrackingNearbyPlacePolicyDecision {
    TrackingNearbyPlacePolicyDecision {
        violation_state: TrackingPolicyViolationState::NotDetected,
        policy_violation_detected: None,
    }
}

fn tracking_expected_place_not_detected() -> TrackingExpectedPlacePolicyDecision {
    TrackingExpectedPlacePolicyDecision {
        violation_state: TrackingPolicyViolationState::NotDetected,
        policy_violation_detected: None,
    }
}

fn tracking_policy_violation_detected(
    child_device_id: ocentra_parent_agent_protocol::TrackingChildDeviceId,
    child_profile_id: ocentra_parent_agent_protocol::TrackingChildProfileId,
    evidence_refs: Vec<ocentra_parent_agent_protocol::TrackingEvidenceRef>,
) -> TrackingPolicyViolationDetectedEvent {
    TrackingPolicyViolationDetectedEvent {
        child_device_id,
        child_profile_id,
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
        evidence_refs,
    }
}
