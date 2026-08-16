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
    let authoring = policy_preview_authoring_snapshot(read_model, parent_access_state);

    ParentPolicyPreviewPanelSnapshot {
        title: "Policy preview parent authoring".to_string(),
        body: "Preview stays advisory until a parent confirms the request and a child-device contract applies it."
            .to_string(),
        summary,
        summary_details,
        cards,
        empty_message: "No policy preview has been reported yet.".to_string(),
        product_claim: policy_preview_product_claim(),
        authoring,
    }
}

fn policy_preview_authoring_snapshot(
    read_model: Option<&ParentPolicyPreviewReadModelSnapshot>,
    parent_access_state: &ParentPortalParentAccessState,
) -> Option<ParentPolicyPreviewAuthoringSnapshot> {
    let read_model = read_model?;
    let target_value = read_model.target_value.clone()?;
    let requested_action = read_model
        .network_requested_policy_action
        .clone()
        .or_else(|| read_model.decision_action.as_ref().map(ToString::to_string))
        .unwrap_or_else(|| "ask-parent".to_string());
    let confirm_action = matches!(
        parent_access_state,
        ParentPortalParentAccessState::ActiveController
    )
    .then(|| ParentPolicyPreviewActionSnapshot {
        action: ParentUiActionKind::PolicyRequestAssistantPreviewConfirmRequested,
        label: "Confirm policy preview".to_string(),
        payload: Some(serde_json::json!({
            "policyRequestAssistantPreviewConfirmRequest": serde_json::json!({
                "targetValue": target_value,
                "requestedAction": requested_action,
            })
            .to_string(),
        })),
    });

    Some(ParentPolicyPreviewAuthoringSnapshot {
        target_value,
        requested_action,
        confirm_action,
        cancel_action: ParentPolicyPreviewActionSnapshot {
            action: ParentUiActionKind::RefreshRoute,
            label: "Cancel draft".to_string(),
            payload: None,
        },
    })
}
