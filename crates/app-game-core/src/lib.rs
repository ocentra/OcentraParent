#![forbid(unsafe_code)]

pub mod app_game_category_risk_policy_routing;
pub mod app_game_notification_parent_surface_intent;
pub mod app_game_policy_evaluator_runtime;
pub mod app_game_policy_preview_handoff;
pub mod app_game_policy_preview_handoff_generated_ts;
pub mod app_game_policy_target_compiler;
pub mod app_game_policy_target_compiler_generated_ts;
pub mod app_game_source_freshness_preview_gate;
pub mod app_game_source_gated_policy_preview;
pub mod app_game_source_gated_policy_preview_timer_chain;
pub mod app_game_source_gated_policy_preview_timer_followthrough;
pub mod app_game_unknown_approval;
pub mod app_game_unknown_approval_event;
mod app_game_unknown_approval_expiry;
mod app_game_unknown_approval_projection;
mod app_game_unknown_approval_reducer;
mod app_game_unknown_approval_response;
mod app_game_unknown_approval_response_validation;
mod app_game_unknown_approval_status;
pub mod app_game_unknown_approval_types;
mod app_game_unknown_approval_validation;
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

pub const CRATE_NAME: &str = "ocentra-app-game-core";

struct AppGameObservedRequirements {
    observed_state: ChildDomainObservedSignal,
    ai_analysis_requirement: ChildDomainAiAnalysisRequirement,
    policy_evaluation_requirement: ChildDomainPolicyEvaluationRequirement,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum AppGameObservationIntent {
    ForegroundUsageRequiresPolicy,
    AmbiguousUsageRequiresAi,
    InventoryObservationOnly,
}

impl AppGameObservationIntent {
    fn observed_requirements(self) -> AppGameObservedRequirements {
        match self {
            AppGameObservationIntent::ForegroundUsageRequiresPolicy => {
                Self::foreground_usage_requires_policy_requirements()
            }
            AppGameObservationIntent::AmbiguousUsageRequiresAi => {
                Self::ambiguous_usage_requires_ai_requirements()
            }
            AppGameObservationIntent::InventoryObservationOnly => {
                Self::inventory_observation_only_requirements()
            }
        }
    }

    fn foreground_usage_requires_policy_requirements() -> AppGameObservedRequirements {
        AppGameObservedRequirements {
            observed_state: ChildDomainObservedSignal::RequiresPolicy,
            ai_analysis_requirement: ChildDomainAiAnalysisRequirement::NotRequired,
            policy_evaluation_requirement: ChildDomainPolicyEvaluationRequirement::Required,
        }
    }

    fn ambiguous_usage_requires_ai_requirements() -> AppGameObservedRequirements {
        AppGameObservedRequirements {
            observed_state: ChildDomainObservedSignal::RequiresAi,
            ai_analysis_requirement: ChildDomainAiAnalysisRequirement::Required,
            policy_evaluation_requirement: ChildDomainPolicyEvaluationRequirement::Required,
        }
    }

    fn inventory_observation_only_requirements() -> AppGameObservedRequirements {
        AppGameObservedRequirements {
            observed_state: ChildDomainObservedSignal::ObserveOnly,
            ai_analysis_requirement: ChildDomainAiAnalysisRequirement::NotRequired,
            policy_evaluation_requirement: ChildDomainPolicyEvaluationRequirement::NotRequired,
        }
    }
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
    let requirements = intent.observed_requirements();

    ChildRuntimeDomain::AppGame.observed_profile(
        ChildDomainRefSuffix::AppGameSubject,
        requirements.observed_state,
        requirements.ai_analysis_requirement,
        requirements.policy_evaluation_requirement,
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
