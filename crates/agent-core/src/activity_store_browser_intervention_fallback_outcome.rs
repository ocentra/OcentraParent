use ocentra_parent_agent_protocol::browser_intervention_values::BrowserUnmanagedFallbackActionState;
use ocentra_parent_agent_protocol::BrowserInterventionOutcome;

pub(super) fn for_outcome(
    intervention_outcome: &Option<BrowserInterventionOutcome>,
) -> Option<BrowserUnmanagedFallbackActionState> {
    match intervention_outcome {
        Some(BrowserInterventionOutcome::Unsupported) => {
            Some(BrowserUnmanagedFallbackActionState::Unavailable)
        }
        Some(BrowserInterventionOutcome::ManualRequired) => {
            Some(BrowserUnmanagedFallbackActionState::OsBlockManualRequired)
        }
        _ => None,
    }
}
