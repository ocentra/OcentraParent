use ocentra_evidence::PrivatePayloadState;
use ocentra_parent_agent_protocol::{
    constants, ParentNotificationRequestedEvent, TrackingAcknowledgementId,
    TrackingAcknowledgementState, TrackingAiAnalysisRequestedEvent, TrackingAiAnalysisRequirement,
    TrackingAiBoundaryMode, TrackingAiPurpose, TrackingAiRequestId, TrackingCheckInId,
    TrackingCheckInState, TrackingChildCheckInRecordedEvent, TrackingChildDeviceId,
    TrackingChildProfileId, TrackingEvaluationId, TrackingEvidenceRecordedEvent,
    TrackingEvidenceRef, TrackingExpectedPlaceRef, TrackingExpectedPlaceState,
    TrackingExpectedPlaceStateEvaluatedEvent, TrackingGeofenceRuleRef,
    TrackingGeofenceTransitionDetectedEvent, TrackingLocationObservedEvent,
    TrackingLocationRelation, TrackingNotificationChannel, TrackingObservationId,
    TrackingParentAcknowledgementRecordedEvent, TrackingParentActionRequirement,
    TrackingPolicyViolationId, TrackingRuntimeConfig, TrackingRuntimeEnabledState,
    TrackingRuntimeMode, TrackingTimestamp, TrackingTransitionId, TrackingTransitionKind,
    TrackingUncertaintyCode,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TrackingRuntimeObservationReport {
    pub location_observed: TrackingLocationObservedEvent,
    pub evidence_recorded: TrackingEvidenceRecordedEvent,
    pub ai_analysis_requested: Option<TrackingAiAnalysisRequestedEvent>,
}

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
    DefaultEvidence,
    DefaultAiRequest,
    DefaultGeofenceRule,
    DefaultGeofenceTransition,
    DefaultExpectedPlaceEvaluation,
    DefaultParentAcknowledgement,
    DefaultChildCheckIn,
}

impl TrackingRuntimeRef {
    fn as_contract_text(self) -> &'static str {
        match self {
            Self::DefaultChildDevice => constants::tracking_runtime::DEFAULT_CHILD_DEVICE_ID,
            Self::DefaultChildProfile => constants::tracking_runtime::DEFAULT_CHILD_PROFILE_ID,
            Self::DefaultObservation => constants::tracking_runtime::DEFAULT_OBSERVATION_ID,
            Self::DefaultExpectedPlace => constants::tracking_runtime::DEFAULT_EXPECTED_PLACE_REF,
            Self::DefaultEvidence => constants::tracking_runtime::DEFAULT_EVIDENCE_REF,
            Self::DefaultAiRequest => constants::tracking_runtime::DEFAULT_AI_REQUEST_ID,
            Self::DefaultGeofenceRule => constants::tracking_runtime::DEFAULT_GEOFENCE_RULE_REF,
            Self::DefaultGeofenceTransition => {
                constants::tracking_runtime::DEFAULT_GEOFENCE_TRANSITION_ID
            }
            Self::DefaultExpectedPlaceEvaluation => {
                constants::tracking_runtime::DEFAULT_EXPECTED_PLACE_EVALUATION_ID
            }
            Self::DefaultParentAcknowledgement => {
                constants::tracking_runtime::DEFAULT_PARENT_ACKNOWLEDGEMENT_ID
            }
            Self::DefaultChildCheckIn => constants::tracking_runtime::DEFAULT_CHILD_CHECK_IN_ID,
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
            Self::AtExpectedPlace => {
                constants::tracking_runtime::LOCATION_RELATION_AT_EXPECTED_PLACE
            }
            Self::AwayFromExpectedPlace => {
                constants::tracking_runtime::LOCATION_RELATION_AWAY_FROM_EXPECTED_PLACE
            }
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum TrackingTransitionKindValue {
    Enter,
    Unchanged,
}

impl TrackingTransitionKindValue {
    fn as_contract_text(self) -> &'static str {
        match self {
            Self::Enter => constants::tracking_runtime::GEOFENCE_TRANSITION_ENTER,
            Self::Unchanged => constants::tracking_runtime::GEOFENCE_TRANSITION_UNCHANGED,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum TrackingExpectedPlaceStateValue {
    AtExpectedPlace,
    AwayFromExpectedPlace,
    Unknown,
}

impl TrackingExpectedPlaceStateValue {
    fn as_contract_text(self) -> &'static str {
        match self {
            Self::AtExpectedPlace => {
                constants::tracking_runtime::EXPECTED_PLACE_STATE_AT_EXPECTED_PLACE
            }
            Self::AwayFromExpectedPlace => {
                constants::tracking_runtime::EXPECTED_PLACE_STATE_AWAY_FROM_EXPECTED_PLACE
            }
            Self::Unknown => constants::tracking_runtime::EXPECTED_PLACE_STATE_UNKNOWN,
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
    TrackingLocationObservedEvent {
        child_device_id: tracking_child_device_id(TrackingRuntimeRef::DefaultChildDevice),
        child_profile_id: tracking_child_profile_id(TrackingRuntimeRef::DefaultChildProfile),
        observation_id: tracking_observation_id(TrackingRuntimeRef::DefaultObservation),
        observed_at: tracking_timestamp(TrackingTimestampKind::DefaultObservedAt),
        latitude_e7: 436531000,
        longitude_e7: -793833000,
        horizontal_accuracy_meters: 25,
        expected_place_ref: tracking_expected_place_ref(TrackingRuntimeRef::DefaultExpectedPlace),
        config: policy_eligible_child_tracking_runtime_config(),
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
    TrackingEvidenceRecordedEvent {
        child_device_id: event.child_device_id.clone(),
        child_profile_id: event.child_profile_id.clone(),
        evidence_ref: tracking_evidence_ref(TrackingRuntimeRef::DefaultEvidence),
        source_observation_id: event.observation_id.clone(),
        location_relation: tracking_location_relation(
            TrackingLocationRelationKind::UncertainNearExpectedPlace,
        ),
        ai_analysis_requirement: tracking_ai_analysis_requirement(&event.config),
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
        ai_request_id: tracking_ai_request_id(TrackingRuntimeRef::DefaultAiRequest),
        evidence_refs: vec![event.evidence_ref.clone()],
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
    TrackingGeofenceTransitionDetectedEvent {
        child_device_id: event.child_device_id.clone(),
        child_profile_id: event.child_profile_id.clone(),
        transition_id: tracking_transition_id(TrackingRuntimeRef::DefaultGeofenceTransition),
        geofence_rule_ref: tracking_geofence_rule_ref(TrackingRuntimeRef::DefaultGeofenceRule),
        source_observation_id: event.source_observation_id.clone(),
        transition_kind: tracking_transition_kind(tracking_transition_kind_from_relation(
            &event.location_relation,
        )),
        evidence_refs: vec![event.evidence_ref.clone()],
    }
}

pub fn tracking_expected_place_state_from_evidence(
    event: &TrackingEvidenceRecordedEvent,
) -> TrackingExpectedPlaceStateEvaluatedEvent {
    TrackingExpectedPlaceStateEvaluatedEvent {
        child_device_id: event.child_device_id.clone(),
        child_profile_id: event.child_profile_id.clone(),
        evaluation_id: tracking_evaluation_id(TrackingRuntimeRef::DefaultExpectedPlaceEvaluation),
        expected_place_ref: tracking_expected_place_ref(TrackingRuntimeRef::DefaultExpectedPlace),
        source_observation_id: event.source_observation_id.clone(),
        expected_place_state: tracking_expected_place_state(
            tracking_expected_place_state_from_relation(&event.location_relation),
        ),
        evidence_refs: vec![event.evidence_ref.clone()],
        parent_action_requirement: event.parent_action_requirement.clone(),
    }
}

pub fn tracking_parent_acknowledgement_from_notification(
    event: &ParentNotificationRequestedEvent,
) -> TrackingParentAcknowledgementRecordedEvent {
    TrackingParentAcknowledgementRecordedEvent {
        child_device_id: event.child_device_id.clone(),
        child_profile_id: event.child_profile_id.clone(),
        acknowledgement_id: tracking_acknowledgement_id(
            TrackingRuntimeRef::DefaultParentAcknowledgement,
        ),
        source_policy_violation_id: event.source_policy_violation_id.clone(),
        acknowledged_at: tracking_timestamp(TrackingTimestampKind::DefaultObservedAt),
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
        check_in_id: tracking_check_in_id(TrackingRuntimeRef::DefaultChildCheckIn),
        source_observation_id: event.observation_id.clone(),
        checked_in_at: event.observed_at.clone(),
        check_in_state: tracking_check_in_state(TrackingCheckInStateValue::Received),
        evidence_refs,
    }
}

fn tracking_ai_analysis_requirement(
    config: &TrackingRuntimeConfig,
) -> TrackingAiAnalysisRequirement {
    if config.tracking_enabled_state == TrackingRuntimeEnabledState::Enabled
        && config.ai_boundary_mode == TrackingAiBoundaryMode::RequestWhenUncertain
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

fn tracking_evidence_ref(value: TrackingRuntimeRef) -> TrackingEvidenceRef {
    let value = value.as_contract_text();
    TrackingEvidenceRef::parse(value).expect(value)
}

fn tracking_location_relation(value: TrackingLocationRelationKind) -> TrackingLocationRelation {
    let value = value.as_contract_text();
    TrackingLocationRelation::parse(value).expect(value)
}

fn tracking_ai_purpose(value: TrackingAiPurposeKind) -> TrackingAiPurpose {
    let value = value.as_contract_text();
    TrackingAiPurpose::parse(value).expect(value)
}

fn tracking_ai_request_id(value: TrackingRuntimeRef) -> TrackingAiRequestId {
    let value = value.as_contract_text();
    TrackingAiRequestId::parse(value).expect(value)
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

fn tracking_geofence_rule_ref(value: TrackingRuntimeRef) -> TrackingGeofenceRuleRef {
    let value = value.as_contract_text();
    TrackingGeofenceRuleRef::parse(value).expect(value)
}

fn tracking_transition_id(value: TrackingRuntimeRef) -> TrackingTransitionId {
    let value = value.as_contract_text();
    TrackingTransitionId::parse(value).expect(value)
}

fn tracking_evaluation_id(value: TrackingRuntimeRef) -> TrackingEvaluationId {
    let value = value.as_contract_text();
    TrackingEvaluationId::parse(value).expect(value)
}

fn tracking_acknowledgement_id(value: TrackingRuntimeRef) -> TrackingAcknowledgementId {
    let value = value.as_contract_text();
    TrackingAcknowledgementId::parse(value).expect(value)
}

fn tracking_check_in_id(value: TrackingRuntimeRef) -> TrackingCheckInId {
    let value = value.as_contract_text();
    TrackingCheckInId::parse(value).expect(value)
}

fn tracking_transition_kind(value: TrackingTransitionKindValue) -> TrackingTransitionKind {
    let value = value.as_contract_text();
    TrackingTransitionKind::parse(value).expect(value)
}

fn tracking_expected_place_state(
    value: TrackingExpectedPlaceStateValue,
) -> TrackingExpectedPlaceState {
    let value = value.as_contract_text();
    TrackingExpectedPlaceState::parse(value).expect(value)
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

fn tracking_transition_kind_from_relation(
    relation: &TrackingLocationRelation,
) -> TrackingTransitionKindValue {
    if relation.as_str() == constants::tracking_runtime::LOCATION_RELATION_AWAY_FROM_EXPECTED_PLACE
    {
        TrackingTransitionKindValue::Enter
    } else {
        TrackingTransitionKindValue::Unchanged
    }
}

fn tracking_expected_place_state_from_relation(
    relation: &TrackingLocationRelation,
) -> TrackingExpectedPlaceStateValue {
    if relation.as_str() == constants::tracking_runtime::LOCATION_RELATION_AT_EXPECTED_PLACE {
        TrackingExpectedPlaceStateValue::AtExpectedPlace
    } else if relation.as_str()
        == constants::tracking_runtime::LOCATION_RELATION_AWAY_FROM_EXPECTED_PLACE
    {
        TrackingExpectedPlaceStateValue::AwayFromExpectedPlace
    } else {
        TrackingExpectedPlaceStateValue::Unknown
    }
}
