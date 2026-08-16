use ocentra_parent_agent_protocol::browser_intervention_values::{
    BrowserBoundaryState, BrowserUnmanagedDetectionState, BrowserUnmanagedFallbackActionState,
};
use ocentra_parent_agent_protocol::browser_unmanaged_enforcement::BrowserUnmanagedEnforcementState;
use ocentra_parent_agent_protocol::{BrowserInterventionAction, BrowserInterventionOutcome};

pub(crate) fn inferred_unmanaged_fallback_action(
    browser_boundary_state: &BrowserBoundaryState,
    unmanaged_browser_enforcement: &BrowserUnmanagedEnforcementState,
    unmanaged_detection_state: &BrowserUnmanagedDetectionState,
    intervention_action: &Option<BrowserInterventionAction>,
    intervention_outcome: &Option<BrowserInterventionOutcome>,
) -> BrowserUnmanagedFallbackActionState {
    if !matches!(
        browser_boundary_state,
        BrowserBoundaryState::UnmanagedBrowserProcess | BrowserBoundaryState::BrowserLikeProcess
    ) {
        return BrowserUnmanagedFallbackActionState::Unavailable;
    }

    if matches!(
        unmanaged_detection_state,
        BrowserUnmanagedDetectionState::Terminated
    ) {
        return BrowserUnmanagedFallbackActionState::TerminateProcess;
    }

    action_fallback(intervention_action)
        .or_else(|| {
            super::activity_store_browser_intervention_fallback_enforcement::fallback_action_for_unmanaged_enforcement(
                unmanaged_browser_enforcement,
            )
        })
        .or_else(|| outcome_fallback(intervention_outcome))
        .unwrap_or(BrowserUnmanagedFallbackActionState::Unavailable)
}

fn action_fallback(
    intervention_action: &Option<BrowserInterventionAction>,
) -> Option<BrowserUnmanagedFallbackActionState> {
    super::activity_store_browser_intervention_fallback_action::fallback_action_for_intervention(
        intervention_action,
    )
}

fn outcome_fallback(
    intervention_outcome: &Option<BrowserInterventionOutcome>,
) -> Option<BrowserUnmanagedFallbackActionState> {
    super::activity_store_browser_intervention_fallback_action::fallback_action_for_outcome(
        intervention_outcome,
    )
}
