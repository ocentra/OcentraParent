#![forbid(unsafe_code)]

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

pub const CRATE_NAME: &str = "ocentra-app-game-core";

pub use runtime_decision::{
    app_game_runtime_decision_recorded_event, app_game_runtime_observed_event,
    evaluate_app_game_runtime, AppGameAggregateId, AppGameAiHandoffState,
    AppGameCapabilityState, AppGameClassificationState, AppGameForegroundState,
    AppGamePolicyHandoffState, AppGameRuntimeActionState, AppGameRuntimeDecision,
    AppGameRuntimeDecisionId, AppGameRuntimeDecisionRecordedEvent, AppGameRuntimeInput,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum AppGameObservationIntent {
    ForegroundUsageRequiresPolicy,
    AmbiguousUsageRequiresAi,
    InventoryObservationOnly,
}

pub fn default_app_game_observed_event() -> ChildDomainObservedEvent {
    app_game_observed_event(AppGameObservationIntent::ForegroundUsageRequiresPolicy)
}

pub fn app_game_observed_event(intent: AppGameObservationIntent) -> ChildDomainObservedEvent {
    child_domain_observed_event(app_game_observed_profile(intent))
}

pub fn app_game_observed_profile(
    intent: AppGameObservationIntent,
) -> ChildDomainObservedEventProfile {
    let (observed_state, ai_analysis_requirement, policy_evaluation_requirement) = match intent {
        AppGameObservationIntent::ForegroundUsageRequiresPolicy => (
            ChildDomainObservedSignal::RequiresPolicy,
            ChildDomainAiAnalysisRequirement::NotRequired,
            ChildDomainPolicyEvaluationRequirement::Required,
        ),
        AppGameObservationIntent::AmbiguousUsageRequiresAi => (
            ChildDomainObservedSignal::RequiresAi,
            ChildDomainAiAnalysisRequirement::Required,
            ChildDomainPolicyEvaluationRequirement::Required,
        ),
        AppGameObservationIntent::InventoryObservationOnly => (
            ChildDomainObservedSignal::ObserveOnly,
            ChildDomainAiAnalysisRequirement::NotRequired,
            ChildDomainPolicyEvaluationRequirement::NotRequired,
        ),
    };

    ChildRuntimeDomain::AppGame.observed_profile(
        ocentra_parent_agent_protocol::ChildDomainRefSuffix::AppGameSubject,
        observed_state,
        ai_analysis_requirement,
        policy_evaluation_requirement,
    )
}

pub fn app_game_evidence_recorded_event(
    event: &ChildDomainObservedEvent,
) -> ChildDomainEvidenceRecordedEvent {
    child_domain_evidence_recorded_event(event)
}

pub fn app_game_ai_analysis_requested_event(
    event: &ChildDomainEvidenceRecordedEvent,
) -> Option<ChildDomainAiAnalysisRequestedEvent> {
    child_domain_ai_analysis_requested_event_if_required(event)
}

pub fn app_game_policy_evaluation_requested_event(
    event: &ChildDomainEvidenceRecordedEvent,
) -> Option<ChildDomainPolicyEvaluationRequestedEvent> {
    child_domain_direct_policy_evaluation_requested_event_if_required(event)
}
