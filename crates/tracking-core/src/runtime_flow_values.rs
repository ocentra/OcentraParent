use super::runtime_flow_contract_text::parse_contract_text;
use super::{
    tracking_acknowledgement_id_from_violation_id, tracking_ai_request_id_from_evidence_ref,
    tracking_check_in_id_from_observation_id, tracking_evidence_ref_from_observation_id,
    TrackingAcknowledgementId, TrackingAcknowledgementState, TrackingAcknowledgementStateValue,
    TrackingAiPurpose, TrackingAiPurposeKind, TrackingAiRequestId, TrackingCapabilityStatus,
    TrackingCheckInId, TrackingCheckInState, TrackingCheckInStateValue, TrackingChildDeviceId,
    TrackingChildProfileId, TrackingEvidenceRef, TrackingExpectedPlaceRef, TrackingGeofenceRuleRef,
    TrackingLocationRelation, TrackingLocationRelationKind, TrackingNotificationChannel,
    TrackingNotificationChannelKind, TrackingObservationId, TrackingPolicyViolationId,
    TrackingRuntimeRef, TrackingTimestamp, TrackingTimestampKind, TrackingTransitionKind,
    TrackingUncertaintyCode, TrackingUncertaintyKind,
};

pub(super) fn tracking_child_device_id(value: TrackingRuntimeRef) -> TrackingChildDeviceId {
    parse_contract_text(value.as_contract_text(), TrackingChildDeviceId::parse)
}

pub(super) fn tracking_child_profile_id(value: TrackingRuntimeRef) -> TrackingChildProfileId {
    parse_contract_text(value.as_contract_text(), TrackingChildProfileId::parse)
}

pub(super) fn tracking_observation_id(value: TrackingRuntimeRef) -> TrackingObservationId {
    parse_contract_text(value.as_contract_text(), TrackingObservationId::parse)
}

pub(super) fn tracking_timestamp(value: TrackingTimestampKind) -> TrackingTimestamp {
    parse_contract_text(value.as_contract_text(), TrackingTimestamp::parse)
}

pub(super) fn tracking_expected_place_ref(value: TrackingRuntimeRef) -> TrackingExpectedPlaceRef {
    parse_contract_text(value.as_contract_text(), TrackingExpectedPlaceRef::parse)
}

pub(super) fn tracking_evidence_ref(observation_id: &TrackingObservationId) -> TrackingEvidenceRef {
    tracking_evidence_ref_from_observation_id(observation_id)
}

pub(super) fn tracking_location_relation(
    value: TrackingLocationRelationKind,
) -> TrackingLocationRelation {
    parse_contract_text(value.as_contract_text(), TrackingLocationRelation::parse)
}

pub(super) fn tracking_ai_purpose(value: TrackingAiPurposeKind) -> TrackingAiPurpose {
    parse_contract_text(value.as_contract_text(), TrackingAiPurpose::parse)
}

pub(super) fn tracking_ai_request_id(evidence_ref: &TrackingEvidenceRef) -> TrackingAiRequestId {
    tracking_ai_request_id_from_evidence_ref(evidence_ref)
}

pub(super) fn tracking_uncertainty_code(value: TrackingUncertaintyKind) -> TrackingUncertaintyCode {
    parse_contract_text(value.as_contract_text(), TrackingUncertaintyCode::parse)
}

pub(super) fn tracking_notification_channel(
    value: TrackingNotificationChannelKind,
) -> TrackingNotificationChannel {
    parse_contract_text(value.as_contract_text(), TrackingNotificationChannel::parse)
}

pub(super) fn tracking_acknowledgement_id(
    violation_id: &TrackingPolicyViolationId,
) -> TrackingAcknowledgementId {
    tracking_acknowledgement_id_from_violation_id(violation_id)
}

pub(super) fn tracking_check_in_id(observation_id: &TrackingObservationId) -> TrackingCheckInId {
    tracking_check_in_id_from_observation_id(observation_id)
}

pub(super) fn tracking_capability_status(value: &'static str) -> TrackingCapabilityStatus {
    parse_contract_text(value, TrackingCapabilityStatus::parse)
}

pub(super) fn tracking_geofence_rule_ref(value: &'static str) -> TrackingGeofenceRuleRef {
    parse_contract_text(value, TrackingGeofenceRuleRef::parse)
}

pub(super) fn tracking_transition_kind(value: &'static str) -> TrackingTransitionKind {
    parse_contract_text(value, TrackingTransitionKind::parse)
}

pub(super) fn tracking_acknowledgement_state(
    value: TrackingAcknowledgementStateValue,
) -> TrackingAcknowledgementState {
    parse_contract_text(
        value.as_contract_text(),
        TrackingAcknowledgementState::parse,
    )
}

pub(super) fn tracking_check_in_state(value: TrackingCheckInStateValue) -> TrackingCheckInState {
    parse_contract_text(value.as_contract_text(), TrackingCheckInState::parse)
}
