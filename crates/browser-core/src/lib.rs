#![forbid(unsafe_code)]

pub mod browser_android_owned_shell_runtime;
pub mod browser_android_owned_shell_url_custody;
pub mod browser_game_hidden_analysis_loader;
pub mod browser_game_policy_candidate_compiler;
pub mod browser_game_url_shape_evaluator;
pub mod browser_generated_social_ts;
pub mod browser_hidden_analysis_loader;
pub mod browser_policy_questionnaire_forest;
pub mod browser_url_intelligence;
pub mod performance_budget;
pub mod runtime_decision;
pub mod social_alert_report_local_outbox_bridge;
pub mod social_alert_report_preference_preflight;
pub mod social_alert_report_preference_status_handoff;
pub mod social_alert_report_provider_dispatch_execution;
pub mod social_alert_report_scheduler_bridge;
pub mod social_applied_schedule_time_budget_proof;
pub mod social_managed_browser_policy_execution;
pub mod social_policy_candidate_compiler;
pub mod social_policy_compiler_contract;
pub mod social_schema_generated_alert_report;
pub mod social_schema_generated_values;
pub mod social_video_ai_signal_aggregate;
pub mod social_video_source_privacy;

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

pub const CRATE_NAME: &str = "ocentra-browser-core";

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
    let (observed_state, ai_analysis_requirement, policy_evaluation_requirement) =
        browser_observed_profile_components(intent);

    ChildRuntimeDomain::Browser.observed_profile(
        ChildDomainRefSuffix::BrowserSubject,
        observed_state,
        ai_analysis_requirement,
        policy_evaluation_requirement,
    )
}

fn browser_observed_profile_components(
    intent: BrowserObservationIntent,
) -> (
    ChildDomainObservedSignal,
    ChildDomainAiAnalysisRequirement,
    ChildDomainPolicyEvaluationRequirement,
) {
    match intent {
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
    }
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
