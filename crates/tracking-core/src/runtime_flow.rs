use ocentra_parent_agent_protocol::{
    constants, ParentNotificationRequestedEvent, TrackingAiAnalysisRequestedEvent,
    TrackingAiAnalysisRequirement, TrackingAiBoundaryMode, TrackingAiPurpose,
    TrackingAiRequestId, TrackingChildDeviceId, TrackingChildProfileId,
    TrackingEvidenceRecordedEvent, TrackingEvidenceRef, TrackingExpectedPlaceRef,
    TrackingLocationObservedEvent, TrackingLocationRelation, TrackingObservationId,
    TrackingParentActionRequirement, TrackingRuntimeConfig, TrackingRuntimeEnabledState,
    TrackingRuntimeMode, TrackingTimestamp,
    TrackingUncertaintyCode,
};
use ocentra_evidence::PrivatePayloadState;

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
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum TrackingLocationRelationKind {
    UncertainNearExpectedPlace,
}

impl TrackingLocationRelationKind {
    fn as_contract_text(self) -> &'static str {
        match self {
            Self::UncertainNearExpectedPlace => {
                constants::tracking_runtime::LOCATION_RELATION_UNCERTAIN_NEAR_EXPECTED_PLACE
            }
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
        allowed_ai_purpose: tracking_ai_purpose(
            TrackingAiPurposeKind::NearbyPlaceClassification,
        ),
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
    if event.channel
        == tracking_notification_channel(TrackingNotificationChannelKind::ParentPortal)
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
) -> ocentra_parent_agent_protocol::TrackingNotificationChannel {
    let value = value.as_contract_text();
    ocentra_parent_agent_protocol::TrackingNotificationChannel::parse(value).expect(value)
}
