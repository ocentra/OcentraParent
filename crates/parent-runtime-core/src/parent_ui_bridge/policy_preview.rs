use super::*;

#[path = "policy_preview/access_summary.rs"]
mod access_summary;
#[path = "policy_preview/access_write.rs"]
mod access_write;
#[path = "policy_preview/authoring.rs"]
pub(super) mod authoring;
#[path = "policy_preview/cards.rs"]
mod cards;
#[path = "policy_preview/details.rs"]
mod details;
#[path = "policy_preview/helpers.rs"]
mod helpers;
#[path = "policy_preview/panel.rs"]
mod panel;
#[path = "policy_preview/resolution.rs"]
pub(super) mod resolution;
#[path = "policy_preview/summary.rs"]
mod summary;

pub(super) fn policy_preview_panel_snapshot(
    event: Option<&ParentRouteEventSnapshot>,
    read_model: Option<&ParentPolicyPreviewReadModelSnapshot>,
    parent_access_state: &ParentPortalParentAccessState,
) -> ParentPolicyPreviewPanelSnapshot {
    panel::policy_preview_panel_snapshot_impl(event, read_model, parent_access_state)
}
