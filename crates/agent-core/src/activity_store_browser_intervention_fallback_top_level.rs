use ocentra_parent_agent_protocol::browser_intervention_values::{
    BrowserBoundaryState, BrowserUnmanagedFallbackActionState,
};

use crate::activity_store_browser_intervention::BrowserInterventionReadRow;

pub(crate) fn top_level_unmanaged_fallback_action(
    row: &BrowserInterventionReadRow,
) -> BrowserUnmanagedFallbackActionState {
    if matches!(
        row.intervention.browser_boundary_state,
        BrowserBoundaryState::UnmanagedBrowserProcess | BrowserBoundaryState::BrowserLikeProcess
    ) {
        return row.unmanaged_fallback_action;
    }
    super::activity_store_browser_intervention_fallback_enforcement::fallback_action_for_unmanaged_enforcement(
        &row.unmanaged_browser_enforcement,
    )
    .unwrap_or(BrowserUnmanagedFallbackActionState::OsBlockManualRequired)
}
