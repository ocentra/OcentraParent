#![forbid(unsafe_code)]

pub mod live_view_runtime;
pub mod live_view_worker;

use ocentra_parent_agent_protocol::child_domain_runtime::{
    child_domain_ai_analysis_requested_event_if_required,
    child_domain_direct_policy_evaluation_requested_event_if_required,
    child_domain_evidence_recorded_event, child_domain_observed_event,
    ChildDomainAiAnalysisRequestedEvent, ChildDomainAiAnalysisRequirement,
    ChildDomainEvidenceRecordedEvent, ChildDomainObservedEvent, ChildDomainObservedEventProfile,
    ChildDomainObservedSignal, ChildDomainPolicyEvaluationRequestedEvent,
    ChildDomainPolicyEvaluationRequirement, ChildDomainRefSuffix, ChildRuntimeDomain,
};

pub const CRATE_NAME: &str = "ocentra-screen-live-view-core";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ScreenLiveViewObservationIntent {
    SessionRequiresPolicy,
    UnauthorizedSessionRequiresPolicy,
    SessionHealthObservationOnly,
}

pub fn default_screen_live_view_observed_event() -> ChildDomainObservedEvent {
    screen_live_view_observed_event(ScreenLiveViewObservationIntent::SessionRequiresPolicy)
}

pub fn screen_live_view_observed_event(
    intent: ScreenLiveViewObservationIntent,
) -> ChildDomainObservedEvent {
    child_domain_observed_event(screen_live_view_observed_profile(intent))
}

pub fn screen_live_view_observed_profile(
    intent: ScreenLiveViewObservationIntent,
) -> ChildDomainObservedEventProfile {
    let (observed_state, ai_analysis_requirement, policy_evaluation_requirement) = match intent {
        ScreenLiveViewObservationIntent::SessionRequiresPolicy
        | ScreenLiveViewObservationIntent::UnauthorizedSessionRequiresPolicy => (
            ChildDomainObservedSignal::RequiresPolicy,
            ChildDomainAiAnalysisRequirement::NotRequired,
            ChildDomainPolicyEvaluationRequirement::Required,
        ),
        ScreenLiveViewObservationIntent::SessionHealthObservationOnly => (
            ChildDomainObservedSignal::ObserveOnly,
            ChildDomainAiAnalysisRequirement::NotRequired,
            ChildDomainPolicyEvaluationRequirement::NotRequired,
        ),
    };

    ChildRuntimeDomain::ScreenLiveView.observed_profile(
        ChildDomainRefSuffix::ScreenLiveViewSubject,
        observed_state,
        ai_analysis_requirement,
        policy_evaluation_requirement,
    )
}

pub fn screen_live_view_evidence_recorded_event(
    event: &ChildDomainObservedEvent,
) -> ChildDomainEvidenceRecordedEvent {
    child_domain_evidence_recorded_event(event)
}

pub fn screen_live_view_ai_analysis_requested_event(
    event: &ChildDomainEvidenceRecordedEvent,
) -> Option<ChildDomainAiAnalysisRequestedEvent> {
    child_domain_ai_analysis_requested_event_if_required(event)
}

pub fn screen_live_view_policy_evaluation_requested_event(
    event: &ChildDomainEvidenceRecordedEvent,
) -> Option<ChildDomainPolicyEvaluationRequestedEvent> {
    child_domain_direct_policy_evaluation_requested_event_if_required(event)
}
