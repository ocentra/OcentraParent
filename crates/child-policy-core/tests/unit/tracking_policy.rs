use ocentra_child_policy_core::tracking_policy::TrackingPolicyViolationState;
use ocentra_eventing::expect_value::ExpectValue;
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::tracking::identifiers::{
    tracking_ai_request_id_from_evidence_ref, tracking_evaluation_id_from_observation_id,
    tracking_evidence_ref_from_observation_id, tracking_violation_id_from_ai_request_and_rule_ref,
    tracking_violation_id_from_evaluation_and_rule_ref, TrackingChildDeviceId,
    TrackingChildProfileId, TrackingConfidenceBasis, TrackingExpectedPlaceRef,
    TrackingExpectedPlaceState, TrackingNearbyPlaceAmbiguityState, TrackingNearbyPlaceProviderKind,
    TrackingObservationId, TrackingPlaceCategory, TrackingPolicyRuleRef, TrackingProviderRef,
    TrackingReasonCode, TrackingScheduleId, TrackingTimestamp,
};
use ocentra_parent_agent_protocol::tracking::runtime_event::{
    TrackingExpectedPlaceStateEvaluatedEvent, TrackingNearbyPlaceClassifiedEvent,
    TrackingParentActionRequirement,
};

fn tracking_nearby_place_fixture(
    parent_action_requirement: TrackingParentActionRequirement,
) -> TrackingNearbyPlaceClassifiedEvent {
    let observation_id =
        TrackingObservationId::parse(constants::tracking_runtime::DEFAULT_OBSERVATION_ID)
            .expect_value(constants::tracking_runtime::DEFAULT_OBSERVATION_ID);
    let evidence_ref = tracking_evidence_ref_from_observation_id(&observation_id);

    TrackingNearbyPlaceClassifiedEvent {
        child_device_id: TrackingChildDeviceId::parse(
            constants::tracking_runtime::DEFAULT_CHILD_DEVICE_ID,
        )
        .expect_value(constants::tracking_runtime::DEFAULT_CHILD_DEVICE_ID),
        child_profile_id: TrackingChildProfileId::parse(
            constants::tracking_runtime::DEFAULT_CHILD_PROFILE_ID,
        )
        .expect_value(constants::tracking_runtime::DEFAULT_CHILD_PROFILE_ID),
        source_ai_request_id: tracking_ai_request_id_from_evidence_ref(&evidence_ref),
        source_location_evidence_ref: evidence_ref.clone(),
        source_observed_at: TrackingTimestamp::parse(
            constants::tracking_runtime::DEFAULT_OBSERVED_AT,
        )
        .expect_value(constants::tracking_runtime::DEFAULT_OBSERVED_AT),
        evidence_refs: vec![evidence_ref],
        provider_kind: TrackingNearbyPlaceProviderKind::parse(
            constants::tracking_runtime::NEARBY_PROVIDER_KIND_LOCAL_CACHE,
        )
        .expect_value(constants::tracking_runtime::NEARBY_PROVIDER_KIND_LOCAL_CACHE),
        provider_ref: Some(
            TrackingProviderRef::parse(constants::tracking_runtime::DEFAULT_TRACKING_PROVIDER_REF)
                .expect_value(constants::tracking_runtime::DEFAULT_TRACKING_PROVIDER_REF),
        ),
        query_radius_meters: constants::tracking_runtime::DEFAULT_NEARBY_QUERY_RADIUS_METERS,
        distance_meters: Some(constants::tracking_runtime::DEFAULT_NEARBY_DISTANCE_METERS),
        place_category: TrackingPlaceCategory::parse(
            constants::tracking_runtime::PLACE_CATEGORY_HOSPITAL,
        )
        .expect_value(constants::tracking_runtime::PLACE_CATEGORY_HOSPITAL),
        confidence: constants::tracking_runtime::DEFAULT_NEARBY_PLACE_CONFIDENCE,
        confidence_basis: TrackingConfidenceBasis::parse(
            constants::tracking_runtime::CONFIDENCE_BASIS_AI_BOUNDARY_CONTRACT,
        )
        .expect_value(constants::tracking_runtime::CONFIDENCE_BASIS_AI_BOUNDARY_CONTRACT),
        ambiguity_state: TrackingNearbyPlaceAmbiguityState::parse(
            constants::tracking_runtime::NEARBY_PLACE_AMBIGUITY_CLEAR,
        )
        .expect_value(constants::tracking_runtime::NEARBY_PLACE_AMBIGUITY_CLEAR),
        reason_codes: vec![TrackingReasonCode::parse(
            constants::tracking_runtime::REASON_NEARBY_PLACE_SINGLE_CANDIDATE,
        )
        .expect_value(constants::tracking_runtime::REASON_NEARBY_PLACE_SINGLE_CANDIDATE)],
        parent_action_requirement,
    }
}

fn tracking_expected_place_fixture(
    expected_place_state: TrackingExpectedPlaceState,
    parent_action_requirement: TrackingParentActionRequirement,
) -> TrackingExpectedPlaceStateEvaluatedEvent {
    let observation_id =
        TrackingObservationId::parse(constants::tracking_runtime::DEFAULT_OBSERVATION_ID)
            .expect_value(constants::tracking_runtime::DEFAULT_OBSERVATION_ID);

    TrackingExpectedPlaceStateEvaluatedEvent {
        child_device_id: TrackingChildDeviceId::parse(
            constants::tracking_runtime::DEFAULT_CHILD_DEVICE_ID,
        )
        .expect_value(constants::tracking_runtime::DEFAULT_CHILD_DEVICE_ID),
        child_profile_id: TrackingChildProfileId::parse(
            constants::tracking_runtime::DEFAULT_CHILD_PROFILE_ID,
        )
        .expect_value(constants::tracking_runtime::DEFAULT_CHILD_PROFILE_ID),
        evaluation_id: tracking_evaluation_id_from_observation_id(&observation_id),
        schedule_id: TrackingScheduleId::parse(
            constants::tracking_runtime::DEFAULT_EXPECTED_PLACE_SCHEDULE_ID,
        )
        .expect_value(constants::tracking_runtime::DEFAULT_EXPECTED_PLACE_SCHEDULE_ID),
        expected_place_ref: TrackingExpectedPlaceRef::parse(
            constants::tracking_runtime::DEFAULT_EXPECTED_PLACE_REF,
        )
        .expect_value(constants::tracking_runtime::DEFAULT_EXPECTED_PLACE_REF),
        source_observation_id: observation_id.clone(),
        source_observed_at: TrackingTimestamp::parse(
            constants::tracking_runtime::DEFAULT_OBSERVED_AT,
        )
        .expect_value(constants::tracking_runtime::DEFAULT_OBSERVED_AT),
        expected_place_state,
        distance_tolerance_meters: Some(
            constants::tracking_runtime::DEFAULT_EXPECTED_PLACE_DISTANCE_TOLERANCE_METERS,
        ),
        late_grace_seconds: constants::tracking_runtime::DEFAULT_EXPECTED_PLACE_LATE_GRACE_SECONDS,
        early_exit_grace_seconds:
            constants::tracking_runtime::DEFAULT_EXPECTED_PLACE_EARLY_EXIT_GRACE_SECONDS,
        exception_state: None,
        reason_codes: vec![TrackingReasonCode::parse(
            constants::tracking_runtime::REASON_EXPECTED_PLACE_AMBIGUOUS,
        )
        .expect_value(constants::tracking_runtime::REASON_EXPECTED_PLACE_AMBIGUOUS)],
        evidence_refs: vec![tracking_evidence_ref_from_observation_id(&observation_id)],
        parent_action_requirement,
    }
}

#[test]
fn tracking_policy_emits_review_violation_for_hospital_nearby_place() {
    let classified = tracking_nearby_place_fixture(TrackingParentActionRequirement::Required);

    let decision =
        ocentra_child_policy_core::tracking_policy::evaluate_tracking_nearby_place_policy(
            &classified,
        );
    let violation = decision
        .policy_violation_detected
        .expect_value("hospital tracking policy violation is expected");
    let policy_rule_ref =
        TrackingPolicyRuleRef::parse(constants::tracking_runtime::POLICY_RULE_EXPECTED_PLACE)
            .expect_value(constants::tracking_runtime::POLICY_RULE_EXPECTED_PLACE);

    assert_eq!(
        decision.violation_state,
        TrackingPolicyViolationState::Detected
    );
    assert_eq!(
        violation.policy_rule_ref,
        constants::tracking_runtime::POLICY_RULE_EXPECTED_PLACE
    );
    assert_eq!(
        violation.severity,
        constants::tracking_runtime::POLICY_SEVERITY_REVIEW
    );
    assert_eq!(
        violation.violation_id,
        tracking_violation_id_from_ai_request_and_rule_ref(
            &classified.source_ai_request_id,
            &policy_rule_ref,
        )
    );
    assert_eq!(violation.detected_at, classified.source_observed_at);
    assert_eq!(violation.evidence_refs, classified.evidence_refs);
}

#[test]
fn tracking_policy_does_not_emit_violation_for_observe_only_mode() {
    let classified = tracking_nearby_place_fixture(TrackingParentActionRequirement::NotRequired);

    let decision =
        ocentra_child_policy_core::tracking_policy::evaluate_tracking_nearby_place_policy(
            &classified,
        );

    assert_eq!(
        decision.violation_state,
        TrackingPolicyViolationState::NotDetected
    );
    assert!(decision.policy_violation_detected.is_none());
}

#[test]
fn tracking_policy_does_not_emit_violation_for_ambiguous_nearby_place() {
    let mut classified = tracking_nearby_place_fixture(TrackingParentActionRequirement::Required);
    classified.ambiguity_state = TrackingNearbyPlaceAmbiguityState::parse(
        constants::tracking_runtime::NEARBY_PLACE_AMBIGUITY_MULTIPLE_CANDIDATES,
    )
    .expect_value(constants::tracking_runtime::NEARBY_PLACE_AMBIGUITY_MULTIPLE_CANDIDATES);

    let decision =
        ocentra_child_policy_core::tracking_policy::evaluate_tracking_nearby_place_policy(
            &classified,
        );

    assert_eq!(
        decision.violation_state,
        TrackingPolicyViolationState::NotDetected
    );
    assert!(decision.policy_violation_detected.is_none());
}

#[test]
fn tracking_policy_emits_review_violation_for_left_expected_place() {
    let evaluated = tracking_expected_place_fixture(
        TrackingExpectedPlaceState::parse(
            constants::tracking_runtime::EXPECTED_PLACE_STATE_LEFT_EXPECTED_PLACE,
        )
        .expect_value(constants::tracking_runtime::EXPECTED_PLACE_STATE_LEFT_EXPECTED_PLACE),
        TrackingParentActionRequirement::Required,
    );

    let decision =
        ocentra_child_policy_core::tracking_policy::evaluate_tracking_expected_place_policy(
            &evaluated,
        );
    let violation = decision
        .policy_violation_detected
        .expect_value("expected-place policy violation is expected");
    let policy_rule_ref =
        TrackingPolicyRuleRef::parse(constants::tracking_runtime::POLICY_RULE_EXPECTED_PLACE)
            .expect_value(constants::tracking_runtime::POLICY_RULE_EXPECTED_PLACE);

    assert_eq!(
        decision.violation_state,
        TrackingPolicyViolationState::Detected
    );
    assert_eq!(
        violation.policy_rule_ref,
        constants::tracking_runtime::POLICY_RULE_EXPECTED_PLACE
    );
    assert_eq!(
        violation.violation_id,
        tracking_violation_id_from_evaluation_and_rule_ref(
            &evaluated.evaluation_id,
            &policy_rule_ref,
        )
    );
    assert_eq!(violation.detected_at, evaluated.source_observed_at);
    assert_eq!(violation.evidence_refs, evaluated.evidence_refs);
}

#[test]
fn tracking_policy_does_not_emit_expected_place_violation_when_parent_action_is_not_required() {
    let evaluated = tracking_expected_place_fixture(
        TrackingExpectedPlaceState::parse(
            constants::tracking_runtime::EXPECTED_PLACE_STATE_LATE_ARRIVAL,
        )
        .expect_value(constants::tracking_runtime::EXPECTED_PLACE_STATE_LATE_ARRIVAL),
        TrackingParentActionRequirement::NotRequired,
    );

    let decision =
        ocentra_child_policy_core::tracking_policy::evaluate_tracking_expected_place_policy(
            &evaluated,
        );

    assert_eq!(
        decision.violation_state,
        TrackingPolicyViolationState::NotDetected
    );
    assert!(decision.policy_violation_detected.is_none());
}

#[test]
fn tracking_policy_does_not_emit_expected_place_violation_when_child_is_where_expected() {
    let evaluated = tracking_expected_place_fixture(
        TrackingExpectedPlaceState::parse(
            constants::tracking_runtime::EXPECTED_PLACE_STATE_WHERE_EXPECTED,
        )
        .expect_value(constants::tracking_runtime::EXPECTED_PLACE_STATE_WHERE_EXPECTED),
        TrackingParentActionRequirement::Required,
    );

    let decision =
        ocentra_child_policy_core::tracking_policy::evaluate_tracking_expected_place_policy(
            &evaluated,
        );

    assert_eq!(
        decision.violation_state,
        TrackingPolicyViolationState::NotDetected
    );
    assert!(decision.policy_violation_detected.is_none());
}
