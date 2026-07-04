use super::cards::policy_preview_cards_impl;
use super::details::policy_preview_summary_details_impl;
use super::helpers::{policy_preview_product_claim, policy_preview_unavailable_summary};
use super::summary::policy_preview_summary_impl;
use super::*;

pub(super) fn policy_preview_panel_snapshot_impl(
    event: Option<&ParentRouteEventSnapshot>,
    read_model: Option<&ParentPolicyPreviewReadModelSnapshot>,
    parent_access_state: &ParentPortalParentAccessState,
) -> ParentPolicyPreviewPanelSnapshot {
    let summary = match read_model {
        None => policy_preview_unavailable_summary(event),
        Some(read_model) if read_model.returned == 0 => {
            "No policy preview rows have been reported yet.".to_string()
        }
        Some(read_model) => policy_preview_summary_impl(read_model),
    };

    let summary_details =
        policy_preview_summary_details_impl(read_model, &summary, parent_access_state);
    let cards = policy_preview_cards_impl(read_model, parent_access_state);

    ParentPolicyPreviewPanelSnapshot {
        title: "Policy preview parent authoring".to_string(),
        body: "Preview stays advisory until a parent confirms the request and a child-device contract applies it."
            .to_string(),
        summary,
        summary_details,
        cards,
        empty_message: "No policy preview has been reported yet.".to_string(),
        product_claim: policy_preview_product_claim(),
    }
}
