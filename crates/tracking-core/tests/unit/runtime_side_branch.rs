use ocentra_parent_agent_protocol::{
    constants, ParentNotificationRequestedEvent, TrackingAcknowledgementState,
    TrackingAiAnalysisRequirement, TrackingCheckInState, TrackingExpectedPlaceState,
    TrackingExpectedPlaceRef, TrackingNotificationChannel, TrackingObservationId,
    TrackingParentActionRequirement, TrackingPolicyRuleRef,
    TrackingTransitionKind,
    tracking_acknowledgement_id_from_violation_id, tracking_check_in_id_from_observation_id,
    tracking_evaluation_id_from_observation_id, tracking_notification_id_from_violation_id,
    tracking_transition_id_from_observation_id, tracking_violation_id_from_evaluation_and_rule_ref,
};

#[test]
fn tracking_evidence_can_branch_to_geofence_and_expected_place_events() {
    let mut observed = ocentra_tracking_core::default_location_observed_event();
    observed.observation_id = TrackingObservationId::parse("tracking-observation-side-branch")
        .expect("tracking side-branch observation id parses");
    observed.expected_place_ref = TrackingExpectedPlaceRef::parse("expected-place-side-branch")
        .expect("tracking side-branch expected place ref parses");
    let evidence = ocentra_tracking_core::record_tracking_evidence_from_location(&observed);
    let geofence = ocentra_tracking_core::tracking_geofence_transition_from_evidence(&evidence);
    let expected_place =
        ocentra_tracking_core::tracking_expected_place_state_from_evidence(&evidence);

    assert_eq!(evidence.expected_place_ref, observed.expected_place_ref);
    assert_eq!(geofence.child_device_id, evidence.child_device_id);
    assert_eq!(geofence.child_profile_id, evidence.child_profile_id);
    assert_eq!(
        geofence.transition_id,
        tracking_transition_id_from_observation_id(&observed.observation_id)
    );
    assert_eq!(
        geofence.transition_kind,
        TrackingTransitionKind::parse(constants::tracking_runtime::GEOFENCE_TRANSITION_AMBIGUOUS)
            .expect(constants::tracking_runtime::GEOFENCE_TRANSITION_AMBIGUOUS)
    );
    assert_eq!(geofence.evidence_refs, vec![evidence.evidence_ref.clone()]);
    assert_eq!(
        expected_place.expected_place_state,
        TrackingExpectedPlaceState::parse(
            constants::tracking_runtime::EXPECTED_PLACE_STATE_UNKNOWN
        )
        .expect(constants::tracking_runtime::EXPECTED_PLACE_STATE_UNKNOWN)
    );
    assert_eq!(
        expected_place.evaluation_id,
        tracking_evaluation_id_from_observation_id(&observed.observation_id)
    );
    assert_eq!(expected_place.expected_place_ref, observed.expected_place_ref);
    assert_eq!(expected_place.evidence_refs, vec![evidence.evidence_ref]);
}

#[test]
fn tracking_evidence_can_resolve_precise_expected_place_without_ai_request() {
    let observed = ocentra_tracking_core::default_at_expected_place_location_observed_event();
    let evidence = ocentra_tracking_core::record_tracking_evidence_from_location(&observed);
    let geofence = ocentra_tracking_core::tracking_geofence_transition_from_evidence(&evidence);
    let expected_place =
        ocentra_tracking_core::tracking_expected_place_state_from_evidence(&evidence);

    assert_eq!(
        evidence.location_relation,
        constants::tracking_runtime::LOCATION_RELATION_AT_EXPECTED_PLACE
    );
    assert_eq!(
        evidence.ai_analysis_requirement,
        TrackingAiAnalysisRequirement::NotRequired
    );
    assert_eq!(
        geofence.transition_kind,
        TrackingTransitionKind::parse(constants::tracking_runtime::GEOFENCE_TRANSITION_DWELL)
            .expect(constants::tracking_runtime::GEOFENCE_TRANSITION_DWELL)
    );
    assert_eq!(
        expected_place.expected_place_state,
        TrackingExpectedPlaceState::parse(
            constants::tracking_runtime::EXPECTED_PLACE_STATE_WHERE_EXPECTED
        )
        .expect(constants::tracking_runtime::EXPECTED_PLACE_STATE_WHERE_EXPECTED)
    );
}

#[test]
fn tracking_evidence_can_resolve_away_from_expected_place_and_keep_observe_only_non_authoritative() {
    let mut observed = ocentra_tracking_core::default_away_from_expected_place_location_observed_event();
    observed.config = ocentra_tracking_core::default_child_tracking_runtime_config();

    let evidence = ocentra_tracking_core::record_tracking_evidence_from_location(&observed);
    let geofence = ocentra_tracking_core::tracking_geofence_transition_from_evidence(&evidence);
    let expected_place =
        ocentra_tracking_core::tracking_expected_place_state_from_evidence(&evidence);

    assert_eq!(
        evidence.location_relation,
        constants::tracking_runtime::LOCATION_RELATION_AWAY_FROM_EXPECTED_PLACE
    );
    assert_eq!(
        evidence.ai_analysis_requirement,
        TrackingAiAnalysisRequirement::NotRequired
    );
    assert_eq!(
        geofence.transition_kind,
        TrackingTransitionKind::parse(constants::tracking_runtime::GEOFENCE_TRANSITION_EXIT)
            .expect(constants::tracking_runtime::GEOFENCE_TRANSITION_EXIT)
    );
    assert_eq!(
        expected_place.expected_place_state,
        TrackingExpectedPlaceState::parse(
            constants::tracking_runtime::EXPECTED_PLACE_STATE_LATE_ARRIVAL
        )
        .expect(constants::tracking_runtime::EXPECTED_PLACE_STATE_LATE_ARRIVAL)
    );
    assert_eq!(
        expected_place.parent_action_requirement,
        TrackingParentActionRequirement::NotRequired
    );
}

#[test]
fn parent_notification_can_be_acknowledged_without_reopening_policy_authority() {
    let observed = ocentra_tracking_core::default_location_observed_event();
    let evidence = ocentra_tracking_core::record_tracking_evidence_from_location(&observed);
    let policy_rule_ref = TrackingPolicyRuleRef::parse(
        constants::tracking_runtime::POLICY_RULE_EXPECTED_PLACE,
    )
    .expect(constants::tracking_runtime::POLICY_RULE_EXPECTED_PLACE);
    let source_policy_violation_id = tracking_violation_id_from_evaluation_and_rule_ref(
        &tracking_evaluation_id_from_observation_id(&observed.observation_id),
        &policy_rule_ref,
    );
    let notification = ParentNotificationRequestedEvent {
        child_device_id: evidence.child_device_id,
        child_profile_id: evidence.child_profile_id,
        source_policy_violation_id: source_policy_violation_id.clone(),
        notification_id: tracking_notification_id_from_violation_id(&source_policy_violation_id),
        channel: TrackingNotificationChannel::parse(
            constants::tracking_runtime::NOTIFICATION_CHANNEL_PARENT_PORTAL,
        )
        .expect(constants::tracking_runtime::NOTIFICATION_CHANNEL_PARENT_PORTAL),
        evidence_refs: vec![evidence.evidence_ref],
    };
    let acknowledgement =
        ocentra_tracking_core::tracking_parent_acknowledgement_from_notification(&notification);

    assert_eq!(
        acknowledgement.source_policy_violation_id,
        notification.source_policy_violation_id
    );
    assert_eq!(
        acknowledgement.acknowledgement_state,
        TrackingAcknowledgementState::parse(
            constants::tracking_runtime::ACKNOWLEDGEMENT_STATE_ACKNOWLEDGED,
        )
        .expect(constants::tracking_runtime::ACKNOWLEDGEMENT_STATE_ACKNOWLEDGED)
    );
    assert_eq!(
        acknowledgement.acknowledgement_id,
        tracking_acknowledgement_id_from_violation_id(&notification.source_policy_violation_id)
    );
    assert_eq!(acknowledgement.evidence_refs, notification.evidence_refs);
}

#[test]
fn child_check_in_cites_source_observation_and_evidence() {
    let observed = ocentra_tracking_core::default_location_observed_event();
    let evidence = ocentra_tracking_core::record_tracking_evidence_from_location(&observed);
    let check_in = ocentra_tracking_core::tracking_child_check_in_from_location(
        &observed,
        vec![evidence.evidence_ref],
    );

    assert_eq!(check_in.source_observation_id, observed.observation_id);
    assert_eq!(check_in.checked_in_at, observed.observed_at);
    assert_eq!(
        check_in.check_in_id,
        tracking_check_in_id_from_observation_id(&observed.observation_id)
    );
    assert_eq!(
        check_in.check_in_state,
        TrackingCheckInState::parse(constants::tracking_runtime::CHECK_IN_STATE_RECEIVED)
            .expect(constants::tracking_runtime::CHECK_IN_STATE_RECEIVED)
    );
    assert_eq!(check_in.evidence_refs.len(), 1);
}
