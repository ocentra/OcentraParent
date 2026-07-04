use ocentra_parent_agent_protocol::browser::BrowserRuntimePhase;

use super::BrowserRuntimeInput;

pub(crate) fn should_publish_phase(
    phase: BrowserRuntimePhase,
    input: &BrowserRuntimeInput,
) -> bool {
    match phase {
        BrowserRuntimePhase::AiAnalysisRequested => input.ai_request_ref.is_some(),
        BrowserRuntimePhase::AiAnalysisCompleted => input.ai_analysis_ref.is_some(),
        BrowserRuntimePhase::PolicyEvaluationRequested => input.policy_evaluation_ref.is_some(),
        BrowserRuntimePhase::PolicyDecisionCompleted => input.policy_decision_ref.is_some(),
        BrowserRuntimePhase::InterventionCommandIssued => {
            input.intervention_command_allowed && input.intervention_command_ref.is_some()
        }
        BrowserRuntimePhase::InterventionResultObserved => {
            input.intervention_command_allowed && input.intervention_result_ref.is_some()
        }
        BrowserRuntimePhase::AuditEntryCommitted => input.audit_entry_ref.is_some(),
        BrowserRuntimePhase::ReadModelProjected => input.read_model_ref.is_some(),
        BrowserRuntimePhase::EvidenceObserved | BrowserRuntimePhase::EvidenceJournaled => true,
    }
}
