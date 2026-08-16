use super::super::lan_route::LanRouteQuery;
use super::super::presentation::parent_access_state_for_lan_read_model;
use super::super::*;
use super::state::ActionDispatchState;

#[path = "policy_authoring_confirm.rs"]
mod confirm;
#[path = "policy_authoring_drafts.rs"]
mod drafts;
#[path = "policy_authoring_resolution.rs"]
mod resolution_dispatch;

pub(super) fn dispatch_parent_ui_action_policy_authoring(
    action: &ParentUiAction,
    lan_route_query: &LanRouteQuery,
    state: &mut ActionDispatchState,
) -> bool {
    if !is_policy_authoring_action(action) {
        return false;
    }
    if !matches!(action.route, ParentRouteId::PolicyNetwork) {
        state.reject("policy preview authoring action is bound to the policy route");
        return true;
    }
    let Some(snapshot) = load_policy_preview_read_model_snapshot(None).ok() else {
        state.reject("policy preview authoring requires a current policy preview");
        return true;
    };
    let Some(preview_id) = snapshot.read_model.preview_id.as_ref() else {
        state.reject("policy preview authoring requires a preview identifier");
        return true;
    };
    let parent_access_state = parent_access_state_for_lan_read_model(lan_route_query.read_model());
    match action.action {
        ParentUiActionKind::PolicyPreviewAuthoringDraftStaged => {
            drafts::stage(action, &snapshot.read_model, &parent_access_state, state)
        }
        ParentUiActionKind::PolicyPreviewAuthoringDraftCancelled => {
            drafts::cancel(action, preview_id, &parent_access_state, state)
        }
        ParentUiActionKind::PolicyRequestAssistantPreviewConfirmRequested => {
            confirm::confirm(action, preview_id, &parent_access_state, state)
        }
        ParentUiActionKind::PolicyRequestParentResolutionRequested => resolution_dispatch::begin(
            action,
            lan_route_query,
            &snapshot.read_model,
            &parent_access_state,
            state,
        ),
        _ => true,
    }
}

fn is_policy_authoring_action(action: &ParentUiAction) -> bool {
    matches!(
        action.action,
        ParentUiActionKind::PolicyPreviewAuthoringDraftStaged
            | ParentUiActionKind::PolicyPreviewAuthoringDraftCancelled
            | ParentUiActionKind::PolicyRequestAssistantPreviewConfirmRequested
            | ParentUiActionKind::PolicyRequestParentResolutionRequested
    )
}
