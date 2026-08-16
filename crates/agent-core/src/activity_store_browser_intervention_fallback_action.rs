use ocentra_parent_agent_protocol::browser_intervention_values::BrowserUnmanagedFallbackActionState;
use ocentra_parent_agent_protocol::{BrowserInterventionAction, BrowserInterventionOutcome};

pub(crate) fn fallback_action_for_intervention(
    intervention_action: &Option<BrowserInterventionAction>,
) -> Option<BrowserUnmanagedFallbackActionState> {
    match intervention_action {
        Some(BrowserInterventionAction::Allow) => {
            Some(BrowserUnmanagedFallbackActionState::AllowedUnmanagedException)
        }
        Some(BrowserInterventionAction::Warn) => {
            Some(BrowserUnmanagedFallbackActionState::WarnChild)
        }
        Some(BrowserInterventionAction::AskParent)
        | Some(BrowserInterventionAction::ApprovalHold) => {
            Some(BrowserUnmanagedFallbackActionState::AskParent)
        }
        Some(BrowserInterventionAction::TerminateProcess) => {
            Some(BrowserUnmanagedFallbackActionState::TerminateProcess)
        }
        Some(BrowserInterventionAction::RelaunchManaged) => {
            Some(BrowserUnmanagedFallbackActionState::RelaunchManagedBrowser)
        }
        Some(BrowserInterventionAction::Block)
        | Some(BrowserInterventionAction::Redirect)
        | Some(BrowserInterventionAction::TimeLimit) => {
            Some(BrowserUnmanagedFallbackActionState::OsBlockManualRequired)
        }
        Some(BrowserInterventionAction::Monitor) => {
            Some(BrowserUnmanagedFallbackActionState::ReportOnly)
        }
        _ => None,
    }
}

pub(crate) fn fallback_action_for_outcome(
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
