use ocentra_parent_agent_protocol::{
    constants, ParentNotificationRequestedEvent, TrackingAcknowledgementState,
    TrackingCheckInState, TrackingExpectedPlaceState, TrackingNotificationChannel,
    TrackingNotificationId, TrackingPolicyViolationId, TrackingTransitionKind,
};

#[test]
fn tracking_evidence_can_branch_to_geofence_and_expected_place_events() {
    let observed = ocentra_tracking_core::default_location_observed_event();
    let evidence = ocentra_tracking_core::record_tracking_evidence_from_location(&observed);
    let geofence = ocentra_tracking_core::tracking_geofence_transition_from_evidence(&evidence);
    let expected_place =
        ocentra_tracking_core::tracking_expected_place_state_from_evidence(&evidence);

    assert_eq!(geofence.child_device_id, evidence.child_device_id);
    assert_eq!(geofence.child_profile_id, evidence.child_profile_id);
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
    assert_eq!(expected_place.evidence_refs, vec![evidence.evidence_ref]);
}

#[test]
fn parent_notification_can_be_acknowledged_without_reopening_policy_authority() {
    let observed = ocentra_tracking_core::default_location_observed_event();
    let evidence = ocentra_tracking_core::record_tracking_evidence_from_location(&observed);
    let notification = ParentNotificationRequestedEvent {
        child_device_id: evidence.child_device_id,
        child_profile_id: evidence.child_profile_id,
        notification_id: TrackingNotificationId::parse(
            constants::tracking_runtime::DEFAULT_NOTIFICATION_ID,
        )
        .expect(constants::tracking_runtime::DEFAULT_NOTIFICATION_ID),
        source_policy_violation_id: TrackingPolicyViolationId::parse(
            constants::tracking_runtime::DEFAULT_POLICY_VIOLATION_ID,
        )
        .expect(constants::tracking_runtime::DEFAULT_POLICY_VIOLATION_ID),
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
        check_in.check_in_state,
        TrackingCheckInState::parse(constants::tracking_runtime::CHECK_IN_STATE_RECEIVED)
            .expect(constants::tracking_runtime::CHECK_IN_STATE_RECEIVED)
    );
    assert_eq!(check_in.evidence_refs.len(), 1);
}
