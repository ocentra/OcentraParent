#![forbid(unsafe_code)]

pub mod runtime_decision;

use ocentra_parent_agent_protocol::child_domain_runtime::{
    child_domain_ai_analysis_requested_event_if_required,
    child_domain_direct_policy_evaluation_requested_event_if_required,
    child_domain_evidence_recorded_event, child_domain_observed_event,
    ChildDomainAiAnalysisRequestedEvent, ChildDomainAiAnalysisRequirement,
    ChildDomainEvidenceRecordedEvent, ChildDomainObservedEvent, ChildDomainObservedEventProfile,
    ChildDomainObservedSignal, ChildDomainPolicyEvaluationRequestedEvent,
    ChildDomainPolicyEvaluationRequirement, ChildDomainRefSuffix, ChildRuntimeDomain,
};
use serde::{Deserialize, Serialize};

pub const CRATE_NAME: &str = "ocentra-screen-core";

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ScreenObservationIntent {
    AmbiguousContentRequiresAi,
    CaptureCapabilityRequiresPolicy,
    KnownPolicyStateRequiresPolicy,
    IdleObservationOnly,
}

pub fn default_screen_observed_event() -> ChildDomainObservedEvent {
    screen_observed_event(ScreenObservationIntent::AmbiguousContentRequiresAi)
}

pub fn screen_observed_event(intent: ScreenObservationIntent) -> ChildDomainObservedEvent {
    child_domain_observed_event(screen_observed_profile(intent))
}

pub fn screen_observed_profile(intent: ScreenObservationIntent) -> ChildDomainObservedEventProfile {
    let (observed_state, ai_analysis_requirement, policy_evaluation_requirement) = match intent {
        ScreenObservationIntent::AmbiguousContentRequiresAi => (
            ChildDomainObservedSignal::RequiresAi,
            ChildDomainAiAnalysisRequirement::Required,
            ChildDomainPolicyEvaluationRequirement::Required,
        ),
        ScreenObservationIntent::CaptureCapabilityRequiresPolicy
        | ScreenObservationIntent::KnownPolicyStateRequiresPolicy => (
            ChildDomainObservedSignal::RequiresPolicy,
            ChildDomainAiAnalysisRequirement::NotRequired,
            ChildDomainPolicyEvaluationRequirement::Required,
        ),
        ScreenObservationIntent::IdleObservationOnly => (
            ChildDomainObservedSignal::ObserveOnly,
            ChildDomainAiAnalysisRequirement::NotRequired,
            ChildDomainPolicyEvaluationRequirement::NotRequired,
        ),
    };

    ChildRuntimeDomain::Screen.observed_profile(
        ChildDomainRefSuffix::ScreenSubject,
        observed_state,
        ai_analysis_requirement,
        policy_evaluation_requirement,
    )
}

pub fn screen_evidence_recorded_event(
    event: &ChildDomainObservedEvent,
) -> ChildDomainEvidenceRecordedEvent {
    child_domain_evidence_recorded_event(event)
}

pub fn screen_ai_analysis_requested_event(
    event: &ChildDomainEvidenceRecordedEvent,
) -> Option<ChildDomainAiAnalysisRequestedEvent> {
    child_domain_ai_analysis_requested_event_if_required(event)
}

pub fn screen_policy_evaluation_requested_event(
    event: &ChildDomainEvidenceRecordedEvent,
) -> Option<ChildDomainPolicyEvaluationRequestedEvent> {
    child_domain_direct_policy_evaluation_requested_event_if_required(event)
}
