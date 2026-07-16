#![forbid(unsafe_code)]

use ocentra_parent_agent_protocol::child_domain_runtime::{
    child_domain_ai_analysis_requested_event_if_required,
    child_domain_direct_policy_evaluation_requested_event_if_required,
    child_domain_evidence_recorded_event, child_domain_observed_event,
    ChildDomainAiAnalysisRequestedEvent, ChildDomainAiAnalysisRequirement,
    ChildDomainEvidenceRecordedEvent, ChildDomainObservedEvent, ChildDomainObservedEventProfile,
    ChildDomainObservedSignal, ChildDomainPolicyEvaluationRequestedEvent,
    ChildDomainPolicyEvaluationRequirement, ChildDomainRefSuffix, ChildRuntimeDomain,
};

use super::LanObservationIntent;

pub fn default_lan_observed_event() -> ChildDomainObservedEvent {
    lan_observed_event(LanObservationIntent::TrustedPresenceRequiresPolicy)
}

pub fn lan_observed_event(intent: LanObservationIntent) -> ChildDomainObservedEvent {
    child_domain_observed_event(lan_observed_profile(intent))
}

pub fn lan_observed_profile(intent: LanObservationIntent) -> ChildDomainObservedEventProfile {
    let (observed_state, ai_analysis_requirement, policy_evaluation_requirement) = match intent {
        LanObservationIntent::TrustedPresenceRequiresPolicy => (
            ChildDomainObservedSignal::RequiresPolicy,
            ChildDomainAiAnalysisRequirement::NotRequired,
            ChildDomainPolicyEvaluationRequirement::Required,
        ),
        LanObservationIntent::UnknownPeerRequiresAi => (
            ChildDomainObservedSignal::RequiresAi,
            ChildDomainAiAnalysisRequirement::Required,
            ChildDomainPolicyEvaluationRequirement::Required,
        ),
        LanObservationIntent::DiscoveryObservationOnly => (
            ChildDomainObservedSignal::ObserveOnly,
            ChildDomainAiAnalysisRequirement::NotRequired,
            ChildDomainPolicyEvaluationRequirement::NotRequired,
        ),
    };

    ChildDomainObservedEventProfile {
        domain: ChildRuntimeDomain::Lan,
        subject_ref_suffix: ChildDomainRefSuffix::LanSubject,
        observed_state,
        ai_analysis_requirement,
        policy_evaluation_requirement,
    }
}

pub fn lan_evidence_recorded_event(
    event: &ChildDomainObservedEvent,
) -> ChildDomainEvidenceRecordedEvent {
    child_domain_evidence_recorded_event(event)
}

pub fn lan_ai_analysis_requested_event(
    event: &ChildDomainEvidenceRecordedEvent,
) -> Option<ChildDomainAiAnalysisRequestedEvent> {
    child_domain_ai_analysis_requested_event_if_required(event)
}

pub fn lan_policy_evaluation_requested_event(
    event: &ChildDomainEvidenceRecordedEvent,
) -> Option<ChildDomainPolicyEvaluationRequestedEvent> {
    child_domain_direct_policy_evaluation_requested_event_if_required(event)
}
