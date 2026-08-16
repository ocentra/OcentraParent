use ocentra_eventing::envelope::DomainEvent;
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::tracking::identifiers::{
    TrackingChildDeviceId, TrackingChildProfileId, TrackingEvaluationId, TrackingEvidenceRef,
    TrackingExpectedPlaceRef, TrackingExpectedPlaceState, TrackingObservationId,
    TrackingScheduleId, TrackingTimestamp,
};
use ocentra_parent_agent_protocol::tracking::runtime_event::{
    TrackingExpectedPlaceExceptionState, TrackingExpectedPlaceStateEvaluatedEvent,
    TrackingParentActionRequirement,
};

#[test]
fn expected_place_event_uses_tracking_contract_and_idempotency(
) -> Result<(), Box<dyn std::error::Error>> {
    let event = expected_place_fixture(None)?;

    let contract = event.contract()?;
    let idempotency = event.idempotency_key()?;

    assert_eq!(
        contract.event_type.as_str(),
        constants::tracking_runtime::TRACKING_EXPECTED_PLACE_STATE_EVALUATED_EVENT_TYPE
    );
    assert_eq!(
        idempotency.as_str(),
        format!(
            "{}:{}",
            constants::tracking_runtime::TRACKING_EXPECTED_PLACE_STATE_EVALUATED_EVENT_TYPE,
            constants::tracking_runtime::DEFAULT_EXPECTED_PLACE_EVALUATION_ID
        )
    );
    Ok(())
}

#[test]
fn expected_place_event_serializes_grace_tolerance_and_exception_citations(
) -> Result<(), Box<dyn std::error::Error>> {
    let event = expected_place_fixture(Some(TrackingExpectedPlaceExceptionState::HolidayMode))?;

    let serialized = serde_json::to_value(&event)?;

    assert_eq!(
        serialized["expectedPlaceRef"],
        constants::tracking_runtime::DEFAULT_EXPECTED_PLACE_REF
    );
    assert_eq!(
        serialized["distanceToleranceMeters"],
        constants::tracking_runtime::DEFAULT_EXPECTED_PLACE_DISTANCE_TOLERANCE_METERS
    );
    assert_eq!(
        serialized["lateGraceSeconds"],
        constants::tracking_runtime::DEFAULT_EXPECTED_PLACE_LATE_GRACE_SECONDS
    );
    assert_eq!(
        serialized["earlyExitGraceSeconds"],
        constants::tracking_runtime::DEFAULT_EXPECTED_PLACE_EARLY_EXIT_GRACE_SECONDS
    );
    assert_eq!(serialized["exceptionState"], "holiday-mode");
    Ok(())
}

fn expected_place_fixture(
    exception_state: Option<TrackingExpectedPlaceExceptionState>,
) -> Result<TrackingExpectedPlaceStateEvaluatedEvent, Box<dyn std::error::Error>> {
    Ok(TrackingExpectedPlaceStateEvaluatedEvent {
        child_device_id: TrackingChildDeviceId::parse(
            constants::tracking_runtime::DEFAULT_CHILD_DEVICE_ID,
        )?,
        child_profile_id: TrackingChildProfileId::parse(
            constants::tracking_runtime::DEFAULT_CHILD_PROFILE_ID,
        )?,
        evaluation_id: TrackingEvaluationId::parse(
            constants::tracking_runtime::DEFAULT_EXPECTED_PLACE_EVALUATION_ID,
        )?,
        schedule_id: TrackingScheduleId::parse(
            constants::tracking_runtime::DEFAULT_EXPECTED_PLACE_SCHEDULE_ID,
        )?,
        expected_place_ref: TrackingExpectedPlaceRef::parse(
            constants::tracking_runtime::DEFAULT_EXPECTED_PLACE_REF,
        )?,
        source_observation_id: TrackingObservationId::parse(
            constants::tracking_runtime::DEFAULT_OBSERVATION_ID,
        )?,
        source_observed_at: TrackingTimestamp::parse(
            constants::tracking_runtime::DEFAULT_OBSERVED_AT,
        )?,
        expected_place_state: TrackingExpectedPlaceState::parse(
            constants::tracking_runtime::EXPECTED_PLACE_STATE_WHERE_EXPECTED,
        )?,
        distance_tolerance_meters: Some(
            constants::tracking_runtime::DEFAULT_EXPECTED_PLACE_DISTANCE_TOLERANCE_METERS,
        ),
        late_grace_seconds: constants::tracking_runtime::DEFAULT_EXPECTED_PLACE_LATE_GRACE_SECONDS,
        early_exit_grace_seconds:
            constants::tracking_runtime::DEFAULT_EXPECTED_PLACE_EARLY_EXIT_GRACE_SECONDS,
        exception_state,
        reason_codes: vec![],
        evidence_refs: vec![TrackingEvidenceRef::parse(
            constants::tracking_runtime::DEFAULT_EVIDENCE_REF,
        )?],
        parent_action_requirement: TrackingParentActionRequirement::NotRequired,
    })
}
