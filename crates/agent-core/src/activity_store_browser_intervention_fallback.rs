use ocentra_parent_agent_protocol::browser_intervention_values::{
    BrowserBoundaryState, BrowserInterventionAction, BrowserUnmanagedDetectionState,
    BrowserUnmanagedFallbackActionState,
};
use ocentra_parent_agent_protocol::browser_unmanaged_enforcement::BrowserUnmanagedEnforcementState;
use ocentra_parent_agent_protocol::BrowserInterventionOutcome;

use crate::activity_store_browser_intervention::BrowserInterventionReadRow;

#[path = "activity_store_browser_intervention_fallback_action.rs"]
mod activity_store_browser_intervention_fallback_action;
#[path = "activity_store_browser_intervention_fallback_context.rs"]
mod activity_store_browser_intervention_fallback_context;
#[path = "activity_store_browser_intervention_fallback_enforcement.rs"]
mod activity_store_browser_intervention_fallback_enforcement;
#[path = "activity_store_browser_intervention_fallback_top_level.rs"]
mod activity_store_browser_intervention_fallback_top_level;

pub(crate) fn inferred_unmanaged_fallback_action(
    browser_boundary_state: &BrowserBoundaryState,
    unmanaged_browser_enforcement: &BrowserUnmanagedEnforcementState,
    unmanaged_detection_state: &BrowserUnmanagedDetectionState,
    intervention_action: &Option<BrowserInterventionAction>,
    intervention_outcome: &Option<BrowserInterventionOutcome>,
) -> BrowserUnmanagedFallbackActionState {
    activity_store_browser_intervention_fallback_context::inferred_unmanaged_fallback_action(
        browser_boundary_state,
        unmanaged_browser_enforcement,
        unmanaged_detection_state,
        intervention_action,
        intervention_outcome,
    )
}

pub(crate) fn top_level_unmanaged_fallback_action(
    row: &BrowserInterventionReadRow,
) -> BrowserUnmanagedFallbackActionState {
    activity_store_browser_intervention_fallback_top_level::top_level_unmanaged_fallback_action(row)
}
