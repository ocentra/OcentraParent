use super::{
    default_expected_place_evaluation, evaluate_expected_place_state,
    TrackingExpectedPlaceEvaluation, TrackingGeofenceEvaluation, TrackingGeofenceInsideState,
};
use super::geofence::geofence_transition_from_parts;
use ocentra_evidence::PrivatePayloadState;
use ocentra_parent_agent_protocol::{
    constants, ParentNotificationRequestedEvent, TrackingAcknowledgementId,
    TrackingAcknowledgementState, TrackingAiAnalysisRequestedEvent, TrackingAiAnalysisRequirement,
    TrackingAiBoundaryMode, TrackingAiPurpose, TrackingAiRequestId, TrackingCapabilityStatus,
    TrackingCheckInId, TrackingCheckInState, TrackingChildCheckInRecordedEvent,
    TrackingChildDeviceId, TrackingChildProfileId, TrackingEvidenceRecordedEvent,
    TrackingEvidenceRef, TrackingExpectedPlaceRef, TrackingExpectedPlaceStateEvaluatedEvent,
    TrackingGeofenceRuleRef, TrackingGeofenceTransitionDetectedEvent,
    TrackingLocationObservedEvent,
    TrackingLocationRelation, TrackingNotificationChannel, TrackingObservationId,
    TrackingParentAcknowledgementRecordedEvent, TrackingParentActionRequirement,
    TrackingRuntimeConfig, TrackingRuntimeEnabledState, TrackingRuntimeMode, TrackingTimestamp,
    TrackingUncertaintyCode, tracking_acknowledgement_id_from_violation_id,
    tracking_ai_request_id_from_evidence_ref, tracking_check_in_id_from_observation_id,
    tracking_evidence_ref_from_observation_id,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TrackingRuntimeObservationReport {
    pub location_observed: TrackingLocationObservedEvent,
    pub evidence_recorded: TrackingEvidenceRecordedEvent,
    pub ai_analysis_requested: Option<TrackingAiAnalysisRequestedEvent>,
}

const DEFAULT_EXPECTED_PLACE_LATITUDE_E7: i32 = 436531000;
const DEFAULT_EXPECTED_PLACE_LONGITUDE_E7: i32 = -793833000;
const PRECISE_EXPECTED_PLACE_ACCURACY_MAX_METERS: u16 = 15;
const EXPECTED_PLACE_MAX_DELTA_E7: u32 = 150;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TrackingPortalNotificationCandidateState {
    Candidate,
    NotCandidate,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum TrackingRuntimeRef {
    DefaultChildDevice,
    DefaultChildProfile,
    DefaultObservation,
    DefaultExpectedPlace,
}

impl TrackingRuntimeRef {
    fn as_contract_text(self) -> &'static str {
        match self {
            Self::DefaultChildDevice => constants::tracking_runtime::DEFAULT_CHILD_DEVICE_ID,
            Self::DefaultChildProfile => constants::tracking_runtime::DEFAULT_CHILD_PROFILE_ID,
            Self::DefaultObservation => constants::tracking_runtime::DEFAULT_OBSERVATION_ID,
            Self::DefaultExpectedPlace => constants::tracking_runtime::DEFAULT_EXPECTED_PLACE_REF,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum TrackingLocationRelationKind {
    UncertainNearExpectedPlace,
    AtExpectedPlace,
    AwayFromExpectedPlace,
}

impl TrackingLocationRelationKind {
    fn as_contract_text(self) -> &'static str {
        match self {
            Self::UncertainNearExpectedPlace => {
                constants::tracking_runtime::LOCATION_RELATION_UNCERTAIN_NEAR_EXPECTED_PLACE
            }
            Self::AtExpectedPlace => constants::tracking_runtime::LOCATION_RELATION_AT_EXPECTED_PLACE,
            Self::AwayFromExpectedPlace => {
                constants::tracking_runtime::LOCATION_RELATION_AWAY_FROM_EXPECTED_PLACE
            }
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum TrackingAcknowledgementStateValue {
    Acknowledged,
}

impl TrackingAcknowledgementStateValue {
    fn as_contract_text(self) -> &'static str {
        match self {
            Self::Acknowledged => constants::tracking_runtime::ACKNOWLEDGEMENT_STATE_ACKNOWLEDGED,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum TrackingCheckInStateValue {
    Received,
}

impl TrackingCheckInStateValue {
    fn as_contract_text(self) -> &'static str {
        match self {
            Self::Received => constants::tracking_runtime::CHECK_IN_STATE_RECEIVED,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum TrackingAiPurposeKind {
    NearbyPlaceClassification,
}

impl TrackingAiPurposeKind {
    fn as_contract_text(self) -> &'static str {
        match self {
            Self::NearbyPlaceClassification => {
                constants::tracking_runtime::ALLOWED_AI_PURPOSE_NEARBY_PLACE_CLASSIFICATION
            }
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum TrackingUncertaintyKind {
    NearbyPlaceClassificationRequired,
}

impl TrackingUncertaintyKind {
    fn as_contract_text(self) -> &'static str {
        match self {
            Self::NearbyPlaceClassificationRequired => {
                constants::tracking_runtime::UNCERTAINTY_CODE_NEARBY_PLACE_CLASSIFICATION_REQUIRED
            }
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum TrackingNotificationChannelKind {
    ParentPortal,
}

impl TrackingNotificationChannelKind {
    fn as_contract_text(self) -> &'static str {
        match self {
            Self::ParentPortal => constants::tracking_runtime::NOTIFICATION_CHANNEL_PARENT_PORTAL,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum TrackingTimestampKind {
    DefaultObservedAt,
}

impl TrackingTimestampKind {
    fn as_contract_text(self) -> &'static str {
        match self {
            Self::DefaultObservedAt => constants::tracking_runtime::DEFAULT_OBSERVED_AT,
        }
    }
}

pub fn default_child_tracking_runtime_config() -> TrackingRuntimeConfig {
    ocentra_parent_agent_protocol::default_tracking_runtime_config()
}

pub fn policy_eligible_child_tracking_runtime_config() -> TrackingRuntimeConfig {
    ocentra_parent_agent_protocol::policy_eligible_tracking_runtime_config()
}

pub fn default_location_observed_event() -> TrackingLocationObservedEvent {
    default_uncertain_location_observed_event()
}

pub fn default_uncertain_location_observed_event() -> TrackingLocationObservedEvent {
    TrackingLocationObservedEvent {
        child_device_id: tracking_child_device_id(TrackingRuntimeRef::DefaultChildDevice),
        child_profile_id: tracking_child_profile_id(TrackingRuntimeRef::DefaultChildProfile),
        observation_id: tracking_observation_id(TrackingRuntimeRef::DefaultObservation),
        observed_at: tracking_timestamp(TrackingTimestampKind::DefaultObservedAt),
        latitude_e7: DEFAULT_EXPECTED_PLACE_LATITUDE_E7,
        longitude_e7: DEFAULT_EXPECTED_PLACE_LONGITUDE_E7,
        horizontal_accuracy_meters: 25,
        expected_place_ref: tracking_expected_place_ref(TrackingRuntimeRef::DefaultExpectedPlace),
        config: policy_eligible_child_tracking_runtime_config(),
    }
}

pub fn default_at_expected_place_location_observed_event() -> TrackingLocationObservedEvent {
    TrackingLocationObservedEvent {
        horizontal_accuracy_meters: PRECISE_EXPECTED_PLACE_ACCURACY_MAX_METERS,
        ..default_uncertain_location_observed_event()
    }
}

pub fn default_away_from_expected_place_location_observed_event() -> TrackingLocationObservedEvent {
    TrackingLocationObservedEvent {
        latitude_e7: DEFAULT_EXPECTED_PLACE_LATITUDE_E7 + 3_500,
        longitude_e7: DEFAULT_EXPECTED_PLACE_LONGITUDE_E7 - 3_500,
        horizontal_accuracy_meters: PRECISE_EXPECTED_PLACE_ACCURACY_MAX_METERS,
        ..default_uncertain_location_observed_event()
    }
}

pub fn observe_tracking_location(
    event: TrackingLocationObservedEvent,
) -> TrackingRuntimeObservationReport {
    let evidence_recorded = record_tracking_evidence_from_location(&event);
    let ai_analysis_requested = tracking_ai_analysis_request_from_evidence(&evidence_recorded);

    TrackingRuntimeObservationReport {
        location_observed: event,
        evidence_recorded,
        ai_analysis_requested,
    }
}

pub fn record_tracking_evidence_from_location(
    event: &TrackingLocationObservedEvent,
) -> TrackingEvidenceRecordedEvent {
    let relation_kind = infer_tracking_location_relation(event);

    TrackingEvidenceRecordedEvent {
        child_device_id: event.child_device_id.clone(),
        child_profile_id: event.child_profile_id.clone(),
        evidence_ref: tracking_evidence_ref(&event.observation_id),
        source_observation_id: event.observation_id.clone(),
        source_observed_at: event.observed_at.clone(),
        expected_place_ref: event.expected_place_ref.clone(),
        location_relation: tracking_location_relation(relation_kind),
        ai_analysis_requirement: tracking_ai_analysis_requirement(&event.config, relation_kind),
        parent_action_requirement: tracking_parent_action_requirement(&event.config),
        allowed_ai_purpose: tracking_ai_purpose(TrackingAiPurposeKind::NearbyPlaceClassification),
    }
}

pub fn tracking_ai_analysis_request_from_evidence(
    event: &TrackingEvidenceRecordedEvent,
) -> Option<TrackingAiAnalysisRequestedEvent> {
    if event.ai_analysis_requirement != TrackingAiAnalysisRequirement::Required {
        return None;
    }

    Some(TrackingAiAnalysisRequestedEvent {
        child_device_id: event.child_device_id.clone(),
        child_profile_id: event.child_profile_id.clone(),
        ai_request_id: tracking_ai_request_id(&event.evidence_ref),
        evidence_refs: vec![event.evidence_ref.clone()],
        source_observed_at: event.source_observed_at.clone(),
        uncertainty_code: tracking_uncertainty_code(
            TrackingUncertaintyKind::NearbyPlaceClassificationRequired,
        ),
        allowed_analysis_purpose: event.allowed_ai_purpose.clone(),
        parent_action_requirement: event.parent_action_requirement.clone(),
        private_payload_state: PrivatePayloadState::Excluded,
    })
}

pub fn tracking_geofence_transition_from_evidence(
    event: &TrackingEvidenceRecordedEvent,
) -> TrackingGeofenceTransitionDetectedEvent {
    let evaluation = default_tracking_geofence_evaluation_from_relation(&event.location_relation);
    geofence_transition_from_parts(
        event.child_device_id.clone(),
        event.child_profile_id.clone(),
        event.source_observation_id.clone(),
        event.source_observed_at.clone(),
        TrackingGeofenceRuleRef::parse(constants::tracking_runtime::DEFAULT_GEOFENCE_RULE_REF)
            .expect(constants::tracking_runtime::DEFAULT_GEOFENCE_RULE_REF),
        evaluation,
        vec![event.evidence_ref.clone()],
    )
}

pub fn tracking_expected_place_state_from_evidence(
    event: &TrackingEvidenceRecordedEvent,
) -> TrackingExpectedPlaceStateEvaluatedEvent {
    let evaluation = default_tracking_expected_place_evaluation_from_relation(
        &event.location_relation,
        &event.parent_action_requirement,
    );
    let mut expected_place = evaluate_expected_place_state(event, evaluation);
    if event.parent_action_requirement != TrackingParentActionRequirement::Required {
        expected_place.parent_action_requirement = TrackingParentActionRequirement::NotRequired;
    }
    expected_place
}

pub fn tracking_parent_acknowledgement_from_notification(
    event: &ParentNotificationRequestedEvent,
) -> TrackingParentAcknowledgementRecordedEvent {
    TrackingParentAcknowledgementRecordedEvent {
        child_device_id: event.child_device_id.clone(),
        child_profile_id: event.child_profile_id.clone(),
        acknowledgement_id: tracking_acknowledgement_id(&event.source_policy_violation_id),
        source_policy_violation_id: event.source_policy_violation_id.clone(),
        acknowledged_at: event.requested_at.clone(),
        acknowledgement_state: tracking_acknowledgement_state(
            TrackingAcknowledgementStateValue::Acknowledged,
        ),
        evidence_refs: event.evidence_refs.clone(),
    }
}

pub fn tracking_child_check_in_from_location(
    event: &TrackingLocationObservedEvent,
    evidence_refs: Vec<TrackingEvidenceRef>,
) -> TrackingChildCheckInRecordedEvent {
    TrackingChildCheckInRecordedEvent {
        child_device_id: event.child_device_id.clone(),
        child_profile_id: event.child_profile_id.clone(),
        check_in_id: tracking_check_in_id(&event.observation_id),
        source_observation_id: event.observation_id.clone(),
        checked_in_at: event.observed_at.clone(),
        check_in_state: tracking_check_in_state(TrackingCheckInStateValue::Received),
        evidence_refs,
    }
}

fn tracking_ai_analysis_requirement(
    config: &TrackingRuntimeConfig,
    relation: TrackingLocationRelationKind,
) -> TrackingAiAnalysisRequirement {
    if config.tracking_enabled_state == TrackingRuntimeEnabledState::Enabled
        && config.ai_boundary_mode == TrackingAiBoundaryMode::RequestWhenUncertain
        && relation == TrackingLocationRelationKind::UncertainNearExpectedPlace
    {
        TrackingAiAnalysisRequirement::Required
    } else {
        TrackingAiAnalysisRequirement::NotRequired
    }
}

fn tracking_parent_action_requirement(
    config: &TrackingRuntimeConfig,
) -> TrackingParentActionRequirement {
    if config.tracking_enabled_state == TrackingRuntimeEnabledState::Enabled
        && config.tracking_mode == TrackingRuntimeMode::PolicyEligible
    {
        TrackingParentActionRequirement::Required
    } else {
        TrackingParentActionRequirement::NotRequired
    }
}

pub fn tracking_observation_portal_notification_candidate_state(
    event: &ParentNotificationRequestedEvent,
) -> TrackingPortalNotificationCandidateState {
    if event.channel == tracking_notification_channel(TrackingNotificationChannelKind::ParentPortal)
        && !event.evidence_refs.is_empty()
    {
        TrackingPortalNotificationCandidateState::Candidate
    } else {
        TrackingPortalNotificationCandidateState::NotCandidate
    }
}

fn tracking_child_device_id(value: TrackingRuntimeRef) -> TrackingChildDeviceId {
    let value = value.as_contract_text();
    TrackingChildDeviceId::parse(value).expect(value)
}

fn tracking_child_profile_id(value: TrackingRuntimeRef) -> TrackingChildProfileId {
    let value = value.as_contract_text();
    TrackingChildProfileId::parse(value).expect(value)
}

fn tracking_observation_id(value: TrackingRuntimeRef) -> TrackingObservationId {
    let value = value.as_contract_text();
    TrackingObservationId::parse(value).expect(value)
}

fn tracking_timestamp(value: TrackingTimestampKind) -> TrackingTimestamp {
    let value = value.as_contract_text();
    TrackingTimestamp::parse(value).expect(value)
}

fn tracking_expected_place_ref(value: TrackingRuntimeRef) -> TrackingExpectedPlaceRef {
    let value = value.as_contract_text();
    TrackingExpectedPlaceRef::parse(value).expect(value)
}

fn tracking_evidence_ref(observation_id: &TrackingObservationId) -> TrackingEvidenceRef {
    tracking_evidence_ref_from_observation_id(observation_id)
}

fn tracking_location_relation(value: TrackingLocationRelationKind) -> TrackingLocationRelation {
    let value = value.as_contract_text();
    TrackingLocationRelation::parse(value).expect(value)
}

fn tracking_ai_purpose(value: TrackingAiPurposeKind) -> TrackingAiPurpose {
    let value = value.as_contract_text();
    TrackingAiPurpose::parse(value).expect(value)
}

fn tracking_ai_request_id(evidence_ref: &TrackingEvidenceRef) -> TrackingAiRequestId {
    tracking_ai_request_id_from_evidence_ref(evidence_ref)
}

fn tracking_uncertainty_code(value: TrackingUncertaintyKind) -> TrackingUncertaintyCode {
    let value = value.as_contract_text();
    TrackingUncertaintyCode::parse(value).expect(value)
}

fn tracking_notification_channel(
    value: TrackingNotificationChannelKind,
) -> TrackingNotificationChannel {
    let value = value.as_contract_text();
    TrackingNotificationChannel::parse(value).expect(value)
}

fn tracking_acknowledgement_id(
    violation_id: &ocentra_parent_agent_protocol::TrackingPolicyViolationId,
) -> TrackingAcknowledgementId {
    tracking_acknowledgement_id_from_violation_id(violation_id)
}

fn tracking_check_in_id(observation_id: &TrackingObservationId) -> TrackingCheckInId {
    tracking_check_in_id_from_observation_id(observation_id)
}

fn tracking_capability_status(value: &'static str) -> TrackingCapabilityStatus {
    TrackingCapabilityStatus::parse(value).expect(value)
}

fn tracking_acknowledgement_state(
    value: TrackingAcknowledgementStateValue,
) -> TrackingAcknowledgementState {
    let value = value.as_contract_text();
    TrackingAcknowledgementState::parse(value).expect(value)
}

fn tracking_check_in_state(value: TrackingCheckInStateValue) -> TrackingCheckInState {
    let value = value.as_contract_text();
    TrackingCheckInState::parse(value).expect(value)
}

fn infer_tracking_location_relation(
    event: &TrackingLocationObservedEvent,
) -> TrackingLocationRelationKind {
    let latitude_delta = event.latitude_e7.abs_diff(DEFAULT_EXPECTED_PLACE_LATITUDE_E7);
    let longitude_delta = event.longitude_e7.abs_diff(DEFAULT_EXPECTED_PLACE_LONGITUDE_E7);
    let expected_place_delta = latitude_delta.max(longitude_delta);

    if expected_place_delta <= EXPECTED_PLACE_MAX_DELTA_E7 {
        if event.horizontal_accuracy_meters <= PRECISE_EXPECTED_PLACE_ACCURACY_MAX_METERS {
            TrackingLocationRelationKind::AtExpectedPlace
        } else {
            TrackingLocationRelationKind::UncertainNearExpectedPlace
        }
    } else {
        TrackingLocationRelationKind::AwayFromExpectedPlace
    }
}

fn default_tracking_geofence_evaluation_from_relation(
    relation: &TrackingLocationRelation,
) -> TrackingGeofenceEvaluation {
    if relation.as_str() == constants::tracking_runtime::LOCATION_RELATION_AT_EXPECTED_PLACE {
        TrackingGeofenceEvaluation {
            previous_inside_state: Some(TrackingGeofenceInsideState::Inside),
            current_inside_state: TrackingGeofenceInsideState::Inside,
            capability_status: tracking_capability_status(
                constants::tracking_runtime::CAPABILITY_STATUS_LIVE,
            ),
            distance_meters: Some(0),
            low_accuracy_near_boundary: false,
            grace_period_active: false,
        }
    } else if relation.as_str()
        == constants::tracking_runtime::LOCATION_RELATION_AWAY_FROM_EXPECTED_PLACE
    {
        TrackingGeofenceEvaluation {
            previous_inside_state: Some(TrackingGeofenceInsideState::Inside),
            current_inside_state: TrackingGeofenceInsideState::Outside,
            capability_status: tracking_capability_status(
                constants::tracking_runtime::CAPABILITY_STATUS_LIVE,
            ),
            distance_meters: Some(250),
            low_accuracy_near_boundary: false,
            grace_period_active: false,
        }
    } else {
        TrackingGeofenceEvaluation {
            previous_inside_state: Some(TrackingGeofenceInsideState::Outside),
            current_inside_state: TrackingGeofenceInsideState::Outside,
            capability_status: tracking_capability_status(
                constants::tracking_runtime::CAPABILITY_STATUS_RECENT,
            ),
            distance_meters: None,
            low_accuracy_near_boundary: true,
            grace_period_active: false,
        }
    }
}

fn default_tracking_expected_place_evaluation_from_relation(
    relation: &TrackingLocationRelation,
    parent_action_requirement: &TrackingParentActionRequirement,
) -> TrackingExpectedPlaceEvaluation {
    if relation.as_str() == constants::tracking_runtime::LOCATION_RELATION_AT_EXPECTED_PLACE {
        TrackingExpectedPlaceEvaluation {
            transition_kind: ocentra_parent_agent_protocol::TrackingTransitionKind::parse(
                constants::tracking_runtime::GEOFENCE_TRANSITION_DWELL,
            )
            .expect(constants::tracking_runtime::GEOFENCE_TRANSITION_DWELL),
            ..default_expected_place_evaluation()
        }
    } else if relation.as_str()
        == constants::tracking_runtime::LOCATION_RELATION_AWAY_FROM_EXPECTED_PLACE
    {
        let transition_kind =
            if *parent_action_requirement == TrackingParentActionRequirement::Required {
                constants::tracking_runtime::GEOFENCE_TRANSITION_EXIT
            } else {
                constants::tracking_runtime::GEOFENCE_TRANSITION_MISSED_ARRIVAL
            };

        TrackingExpectedPlaceEvaluation {
            transition_kind: ocentra_parent_agent_protocol::TrackingTransitionKind::parse(
                transition_kind,
            )
            .expect(transition_kind),
            ..default_expected_place_evaluation()
        }
    } else {
        TrackingExpectedPlaceEvaluation {
            transition_kind: ocentra_parent_agent_protocol::TrackingTransitionKind::parse(
                constants::tracking_runtime::GEOFENCE_TRANSITION_AMBIGUOUS,
            )
            .expect(constants::tracking_runtime::GEOFENCE_TRANSITION_AMBIGUOUS),
            capability_status: tracking_capability_status(
                constants::tracking_runtime::CAPABILITY_STATUS_RECENT,
            ),
            ..default_expected_place_evaluation()
        }
    }
}
