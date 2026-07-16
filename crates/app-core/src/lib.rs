#![forbid(unsafe_code)]

pub mod runtime_decision;
pub mod runtime_ids;

use ocentra_parent_agent_protocol::child_domain_runtime::{
    child_domain_ai_analysis_requested_event_if_required,
    child_domain_direct_policy_evaluation_requested_event_if_required,
    child_domain_evidence_recorded_event, child_domain_observed_event,
    ChildDomainAiAnalysisRequestedEvent, ChildDomainAiAnalysisRequirement,
    ChildDomainEvidenceRecordedEvent, ChildDomainObservedEvent, ChildDomainObservedEventProfile,
    ChildDomainObservedSignal, ChildDomainPolicyEvaluationRequestedEvent,
    ChildDomainPolicyEvaluationRequirement, ChildDomainRefSuffix, ChildRuntimeDomain,
};

pub const CRATE_NAME: &str = "ocentra-app-core";

#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum AppObservationIntent {
    ForegroundAppRequiresPolicy,
    UnknownAppRequiresAi,
    InventoryObservationOnly,
}

impl AppObservationIntent {
    fn observed_requirements(
        self,
    ) -> (
        ChildDomainObservedSignal,
        ChildDomainAiAnalysisRequirement,
        ChildDomainPolicyEvaluationRequirement,
    ) {
        match self {
            AppObservationIntent::ForegroundAppRequiresPolicy => (
                ChildDomainObservedSignal::RequiresPolicy,
                ChildDomainAiAnalysisRequirement::NotRequired,
                ChildDomainPolicyEvaluationRequirement::Required,
            ),
            AppObservationIntent::UnknownAppRequiresAi => (
                ChildDomainObservedSignal::RequiresAi,
                ChildDomainAiAnalysisRequirement::Required,
                ChildDomainPolicyEvaluationRequirement::Required,
            ),
            AppObservationIntent::InventoryObservationOnly => (
                ChildDomainObservedSignal::ObserveOnly,
                ChildDomainAiAnalysisRequirement::NotRequired,
                ChildDomainPolicyEvaluationRequirement::NotRequired,
            ),
        }
    }
}

pub fn default_app_observed_event() -> ChildDomainObservedEvent {
    app_observed_event(AppObservationIntent::ForegroundAppRequiresPolicy)
}

pub fn app_observed_event(intent: AppObservationIntent) -> ChildDomainObservedEvent {
    child_domain_observed_event(app_observed_profile(intent))
}

pub fn app_observed_profile(intent: AppObservationIntent) -> ChildDomainObservedEventProfile {
    let (observed_state, ai_analysis_requirement, policy_evaluation_requirement) =
        intent.observed_requirements();

    ChildRuntimeDomain::App.observed_profile(
        ChildDomainRefSuffix::AppSubject,
        observed_state,
        ai_analysis_requirement,
        policy_evaluation_requirement,
    )
}

pub fn app_evidence_recorded_event(
    event: &ChildDomainObservedEvent,
) -> ChildDomainEvidenceRecordedEvent {
    child_domain_evidence_recorded_event(event)
}

pub fn app_ai_analysis_requested_event(
    event: &ChildDomainEvidenceRecordedEvent,
) -> Option<ChildDomainAiAnalysisRequestedEvent> {
    child_domain_ai_analysis_requested_event_if_required(event)
}

pub fn app_policy_evaluation_requested_event(
    event: &ChildDomainEvidenceRecordedEvent,
) -> Option<ChildDomainPolicyEvaluationRequestedEvent> {
    child_domain_direct_policy_evaluation_requested_event_if_required(event)
}
