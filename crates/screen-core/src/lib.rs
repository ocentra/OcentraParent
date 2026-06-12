#![forbid(unsafe_code)]

use ocentra_parent_agent_protocol::{
    child_domain_ref, constants, ChildDomainAiAnalysisRequestedEvent,
    ChildDomainEvidenceRecordedEvent, ChildDomainObservedEvent,
    ChildDomainPolicyEvaluationRequestedEvent,
};

pub const CRATE_NAME: &str = "ocentra-screen-core";

pub fn default_screen_observed_event() -> ChildDomainObservedEvent {
    ChildDomainObservedEvent {
        event_type: constants::child_domain_runtime::SCREEN_OBSERVED_EVENT_TYPE.to_string(),
        domain: constants::child_domain_runtime::DOMAIN_SCREEN.to_string(),
        child_device_id: constants::child_domain_runtime::DEFAULT_CHILD_DEVICE_ID.to_string(),
        child_profile_id: constants::child_domain_runtime::DEFAULT_CHILD_PROFILE_ID.to_string(),
        observation_id: child_domain_ref(
            constants::child_domain_runtime::DOMAIN_SCREEN,
            constants::child_domain_runtime::DEFAULT_OBSERVATION_ID_SUFFIX,
        ),
        subject_ref: child_domain_ref(
            constants::child_domain_runtime::DOMAIN_SCREEN,
            "screen-signal",
        ),
        observed_state: constants::child_domain_runtime::SIGNAL_REQUIRES_AI.to_string(),
        observed_at: constants::child_domain_runtime::DEFAULT_OBSERVED_AT.to_string(),
        requires_ai_analysis: true,
        requires_policy_evaluation: true,
    }
}

pub fn screen_evidence_recorded_event(
    event: &ChildDomainObservedEvent,
) -> ChildDomainEvidenceRecordedEvent {
    ChildDomainEvidenceRecordedEvent {
        event_type: constants::child_domain_runtime::SCREEN_EVIDENCE_RECORDED_EVENT_TYPE
            .to_string(),
        domain: event.domain.clone(),
        child_device_id: event.child_device_id.clone(),
        child_profile_id: event.child_profile_id.clone(),
        evidence_ref: child_domain_ref(
            &event.domain,
            constants::child_domain_runtime::DEFAULT_EVIDENCE_REF_SUFFIX,
        ),
        source_observation_id: event.observation_id.clone(),
        signal: event.observed_state.clone(),
    }
}

pub fn screen_ai_analysis_requested_event(
    event: &ChildDomainEvidenceRecordedEvent,
) -> Option<ChildDomainAiAnalysisRequestedEvent> {
    Some(ChildDomainAiAnalysisRequestedEvent {
        event_type: constants::child_domain_runtime::SCREEN_AI_ANALYSIS_REQUESTED_EVENT_TYPE
            .to_string(),
        domain: event.domain.clone(),
        child_device_id: event.child_device_id.clone(),
        child_profile_id: event.child_profile_id.clone(),
        ai_request_id: child_domain_ref(
            &event.domain,
            constants::child_domain_runtime::DEFAULT_AI_REQUEST_ID_SUFFIX,
        ),
        evidence_refs: vec![event.evidence_ref.clone()],
        allowed_analysis_purpose: constants::child_domain_runtime::AI_PURPOSE_CLASSIFICATION
            .to_string(),
        raw_private_payload_included: false,
    })
}

pub fn screen_policy_evaluation_requested_event(
    _event: &ChildDomainEvidenceRecordedEvent,
) -> Option<ChildDomainPolicyEvaluationRequestedEvent> {
    None
}
