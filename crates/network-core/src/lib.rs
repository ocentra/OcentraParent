#![forbid(unsafe_code)]

use ocentra_parent_agent_protocol::{
    child_domain_ai_analysis_requested_event_if_required,
    child_domain_direct_policy_evaluation_requested_event_if_required,
    child_domain_evidence_recorded_event, child_domain_observed_event,
    ChildDomainAiAnalysisRequirement,
    ChildDomainAiAnalysisRequestedEvent, ChildDomainEvidenceRecordedEvent,
    ChildDomainObservedEvent, ChildDomainObservedEventProfile, ChildDomainObservedSignal,
    ChildDomainPolicyEvaluationRequestedEvent, ChildDomainPolicyEvaluationRequirement,
    ChildDomainRefSuffix, ChildRuntimeDomain,
};

pub const CRATE_NAME: &str = "ocentra-network-core";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NetworkObservationIntent {
    FlowRequiresPolicy,
    UnknownRouteRequiresAi,
    TelemetryObservationOnly,
}

pub fn default_network_observed_event() -> ChildDomainObservedEvent {
    network_observed_event(NetworkObservationIntent::FlowRequiresPolicy)
}

pub fn network_observed_event(intent: NetworkObservationIntent) -> ChildDomainObservedEvent {
    child_domain_observed_event(network_observed_profile(intent))
}

pub fn network_observed_profile(
    intent: NetworkObservationIntent,
) -> ChildDomainObservedEventProfile {
    let (
        observed_state,
        ai_analysis_requirement,
        policy_evaluation_requirement,
    ) = match intent {
        NetworkObservationIntent::FlowRequiresPolicy => (
            ChildDomainObservedSignal::RequiresPolicy,
            ChildDomainAiAnalysisRequirement::NotRequired,
            ChildDomainPolicyEvaluationRequirement::Required,
        ),
        NetworkObservationIntent::UnknownRouteRequiresAi => (
            ChildDomainObservedSignal::RequiresAi,
            ChildDomainAiAnalysisRequirement::Required,
            ChildDomainPolicyEvaluationRequirement::Required,
        ),
        NetworkObservationIntent::TelemetryObservationOnly => (
            ChildDomainObservedSignal::ObserveOnly,
            ChildDomainAiAnalysisRequirement::NotRequired,
            ChildDomainPolicyEvaluationRequirement::NotRequired,
        ),
    };

    ChildDomainObservedEventProfile {
        domain: ChildRuntimeDomain::Network,
        subject_ref_suffix: ChildDomainRefSuffix::NetworkSubject,
        observed_state,
        ai_analysis_requirement,
        policy_evaluation_requirement,
    }
}

pub fn network_evidence_recorded_event(
    event: &ChildDomainObservedEvent,
) -> ChildDomainEvidenceRecordedEvent {
    child_domain_evidence_recorded_event(event)
}

pub fn network_ai_analysis_requested_event(
    event: &ChildDomainEvidenceRecordedEvent,
) -> Option<ChildDomainAiAnalysisRequestedEvent> {
    child_domain_ai_analysis_requested_event_if_required(event)
}

pub fn network_policy_evaluation_requested_event(
    event: &ChildDomainEvidenceRecordedEvent,
) -> Option<ChildDomainPolicyEvaluationRequestedEvent> {
    child_domain_direct_policy_evaluation_requested_event_if_required(event)
}
