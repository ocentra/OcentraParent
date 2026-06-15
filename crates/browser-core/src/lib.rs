#![forbid(unsafe_code)]

mod performance_budget;
mod runtime_decision;

use ocentra_parent_agent_protocol::{
    child_domain_ai_analysis_requested_event_if_required,
    child_domain_direct_policy_evaluation_requested_event_if_required,
    child_domain_evidence_recorded_event, child_domain_observed_event,
    ChildDomainAiAnalysisRequestedEvent, ChildDomainAiAnalysisRequirement,
    ChildDomainEvidenceRecordedEvent, ChildDomainObservedEvent, ChildDomainObservedEventProfile,
    ChildDomainObservedSignal, ChildDomainPolicyEvaluationRequestedEvent,
    ChildDomainPolicyEvaluationRequirement, ChildRuntimeDomain,
};
use serde::{Deserialize, Serialize};

pub const CRATE_NAME: &str = "ocentra-browser-core";

pub use performance_budget::{
    browser_performance_fixture_budget_matrix, evaluate_browser_performance_budget,
    BrowserPerformanceBudgetCheck, BrowserPerformanceBudgetError, BrowserPerformanceBudgetState,
};
pub use runtime_decision::{
    browser_runtime_decision_recorded_event, browser_runtime_observed_event,
    evaluate_browser_runtime, BrowserAggregateId, BrowserAiHandoffState, BrowserCapabilityState,
    BrowserClassificationState, BrowserForegroundState, BrowserPolicyHandoffState,
    BrowserRuntimeActionState, BrowserRuntimeDecision, BrowserRuntimeDecisionId,
    BrowserRuntimeDecisionRecordedEvent, BrowserRuntimeInput,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum BrowserObservationIntent {
    AmbiguousNavigationRequiresAi,
    KnownPolicyNavigationRequiresPolicy,
    InventoryObservationOnly,
}

pub fn default_browser_observed_event() -> ChildDomainObservedEvent {
    browser_observed_event(BrowserObservationIntent::AmbiguousNavigationRequiresAi)
}

pub fn browser_observed_event(intent: BrowserObservationIntent) -> ChildDomainObservedEvent {
    child_domain_observed_event(browser_observed_profile(intent))
}

pub fn browser_observed_profile(
    intent: BrowserObservationIntent,
) -> ChildDomainObservedEventProfile {
    let (observed_state, ai_analysis_requirement, policy_evaluation_requirement) = match intent {
        BrowserObservationIntent::AmbiguousNavigationRequiresAi => (
            ChildDomainObservedSignal::RequiresAi,
            ChildDomainAiAnalysisRequirement::Required,
            ChildDomainPolicyEvaluationRequirement::Required,
        ),
        BrowserObservationIntent::KnownPolicyNavigationRequiresPolicy => (
            ChildDomainObservedSignal::RequiresPolicy,
            ChildDomainAiAnalysisRequirement::NotRequired,
            ChildDomainPolicyEvaluationRequirement::Required,
        ),
        BrowserObservationIntent::InventoryObservationOnly => (
            ChildDomainObservedSignal::ObserveOnly,
            ChildDomainAiAnalysisRequirement::NotRequired,
            ChildDomainPolicyEvaluationRequirement::NotRequired,
        ),
    };

    ChildRuntimeDomain::Browser.observed_profile(
        ocentra_parent_agent_protocol::ChildDomainRefSuffix::BrowserSubject,
        observed_state,
        ai_analysis_requirement,
        policy_evaluation_requirement,
    )
}

pub fn browser_evidence_recorded_event(
    event: &ChildDomainObservedEvent,
) -> ChildDomainEvidenceRecordedEvent {
    child_domain_evidence_recorded_event(event)
}

pub fn browser_ai_analysis_requested_event(
    event: &ChildDomainEvidenceRecordedEvent,
) -> Option<ChildDomainAiAnalysisRequestedEvent> {
    child_domain_ai_analysis_requested_event_if_required(event)
}

pub fn browser_policy_evaluation_requested_event(
    event: &ChildDomainEvidenceRecordedEvent,
) -> Option<ChildDomainPolicyEvaluationRequestedEvent> {
    child_domain_direct_policy_evaluation_requested_event_if_required(event)
}
