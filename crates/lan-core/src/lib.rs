#![forbid(unsafe_code)]

use ocentra_parent_agent_protocol::{
    child_domain_ref, constants, ChildDomainAiAnalysisRequestedEvent,
    ChildDomainEvidenceRecordedEvent, ChildDomainObservedEvent,
    ChildDomainPolicyEvaluationRequestedEvent,
};

pub const CRATE_NAME: &str = "ocentra-lan-core";

pub fn default_lan_observed_event() -> ChildDomainObservedEvent {
    ChildDomainObservedEvent {
        event_type: constants::child_domain_runtime::LAN_OBSERVED_EVENT_TYPE.to_string(),
        domain: constants::child_domain_runtime::DOMAIN_LAN.to_string(),
        child_device_id: constants::child_domain_runtime::DEFAULT_CHILD_DEVICE_ID.to_string(),
        child_profile_id: constants::child_domain_runtime::DEFAULT_CHILD_PROFILE_ID.to_string(),
        observation_id: child_domain_ref(
            constants::child_domain_runtime::DOMAIN_LAN,
            constants::child_domain_runtime::DEFAULT_OBSERVATION_ID_SUFFIX,
        ),
        subject_ref: child_domain_ref(constants::child_domain_runtime::DOMAIN_LAN, "peer-presence"),
        observed_state: constants::child_domain_runtime::SIGNAL_REQUIRES_POLICY.to_string(),
        observed_at: constants::child_domain_runtime::DEFAULT_OBSERVED_AT.to_string(),
        requires_ai_analysis: false,
        requires_policy_evaluation: true,
    }
}

pub fn lan_evidence_recorded_event(
    event: &ChildDomainObservedEvent,
) -> ChildDomainEvidenceRecordedEvent {
    ChildDomainEvidenceRecordedEvent {
        event_type: constants::child_domain_runtime::LAN_EVIDENCE_RECORDED_EVENT_TYPE.to_string(),
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

pub fn lan_ai_analysis_requested_event(
    _event: &ChildDomainEvidenceRecordedEvent,
) -> Option<ChildDomainAiAnalysisRequestedEvent> {
    None
}

pub fn lan_policy_evaluation_requested_event(
    event: &ChildDomainEvidenceRecordedEvent,
) -> Option<ChildDomainPolicyEvaluationRequestedEvent> {
    Some(ChildDomainPolicyEvaluationRequestedEvent {
        event_type: constants::child_domain_runtime::LAN_POLICY_EVALUATION_REQUESTED_EVENT_TYPE
            .to_string(),
        domain: event.domain.clone(),
        child_device_id: event.child_device_id.clone(),
        child_profile_id: event.child_profile_id.clone(),
        policy_request_id: child_domain_ref(
            &event.domain,
            constants::child_domain_runtime::DEFAULT_POLICY_REQUEST_ID_SUFFIX,
        ),
        evidence_refs: vec![event.evidence_ref.clone()],
        source_fact_ref: event.source_observation_id.clone(),
    })
}
