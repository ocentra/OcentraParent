use ocentra_parent_agent_protocol::{
    constants, ParentNotificationRequestedEvent, TrackingAiAnalysisRequestedEvent,
    TrackingEvidenceRecordedEvent, TrackingLocationObservedEvent, TrackingRuntimeConfig,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TrackingRuntimeObservationReport {
    pub location_observed: TrackingLocationObservedEvent,
    pub evidence_recorded: TrackingEvidenceRecordedEvent,
    pub ai_analysis_requested: Option<TrackingAiAnalysisRequestedEvent>,
}

pub fn default_child_tracking_runtime_config() -> TrackingRuntimeConfig {
    ocentra_parent_agent_protocol::default_tracking_runtime_config()
}

pub fn policy_eligible_child_tracking_runtime_config() -> TrackingRuntimeConfig {
    ocentra_parent_agent_protocol::policy_eligible_tracking_runtime_config()
}

pub fn default_location_observed_event() -> TrackingLocationObservedEvent {
    TrackingLocationObservedEvent {
        child_device_id: constants::tracking_runtime::DEFAULT_CHILD_DEVICE_ID.to_string(),
        child_profile_id: constants::tracking_runtime::DEFAULT_CHILD_PROFILE_ID.to_string(),
        observation_id: constants::tracking_runtime::DEFAULT_OBSERVATION_ID.to_string(),
        observed_at: constants::tracking_runtime::DEFAULT_OBSERVED_AT.to_string(),
        latitude_e7: 436531000,
        longitude_e7: -793833000,
        horizontal_accuracy_meters: 25,
        expected_place_ref: constants::tracking_runtime::DEFAULT_EXPECTED_PLACE_REF.to_string(),
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
        evidence_ref: constants::tracking_runtime::DEFAULT_EVIDENCE_REF.to_string(),
        source_observation_id: event.observation_id.clone(),
        location_relation:
            constants::tracking_runtime::LOCATION_RELATION_UNCERTAIN_NEAR_EXPECTED_PLACE.to_string(),
        requires_ai_analysis: event.config.tracking_enabled
            && event.config.ai_boundary_mode
                == constants::tracking_runtime::AI_BOUNDARY_MODE_REQUEST_WHEN_UNCERTAIN,
        allowed_ai_purpose:
            constants::tracking_runtime::ALLOWED_AI_PURPOSE_NEARBY_PLACE_CLASSIFICATION.to_string(),
    }
}

pub fn tracking_ai_analysis_request_from_evidence(
    event: &TrackingEvidenceRecordedEvent,
) -> Option<TrackingAiAnalysisRequestedEvent> {
    if !event.requires_ai_analysis {
        return None;
    }

    Some(TrackingAiAnalysisRequestedEvent {
        child_device_id: event.child_device_id.clone(),
        child_profile_id: event.child_profile_id.clone(),
        ai_request_id: constants::tracking_runtime::DEFAULT_AI_REQUEST_ID.to_string(),
        evidence_refs: vec![event.evidence_ref.clone()],
        uncertainty_code:
            constants::tracking_runtime::UNCERTAINTY_CODE_NEARBY_PLACE_CLASSIFICATION_REQUIRED
                .to_string(),
        allowed_analysis_purpose: event.allowed_ai_purpose.clone(),
        raw_private_payload_included: false,
    })
}

pub fn tracking_observation_is_portal_notification_candidate(
    event: &ParentNotificationRequestedEvent,
) -> bool {
    event.channel == constants::tracking_runtime::NOTIFICATION_CHANNEL_PARENT_PORTAL
        && !event.evidence_refs.is_empty()
}
