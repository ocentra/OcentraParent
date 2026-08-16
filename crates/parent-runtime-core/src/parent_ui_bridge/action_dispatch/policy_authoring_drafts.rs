use super::super::super::presentation::policy_preview::authoring;
use super::super::super::*;
use super::super::state::ActionDispatchState;
use ocentra_schema::parent_ui_bridge::{
    ParentPolicyPreviewId, ParentPolicyPreviewReadModelSnapshot,
};

pub(super) fn stage(
    action: &ParentUiAction,
    read_model: &ParentPolicyPreviewReadModelSnapshot,
    parent_access_state: &ParentPortalParentAccessState,
    state: &mut ActionDispatchState,
) -> bool {
    match authoring::stage(&action.payload, read_model, parent_access_state) {
        Ok(_) => {
            state.accepted = true;
            state.message = "parent Rust facade staged a bounded policy preview draft".to_string();
        }
        Err(error) => state.reject(error),
    }
    true
}

pub(super) fn cancel(
    action: &ParentUiAction,
    preview_id: &ParentPolicyPreviewId,
    parent_access_state: &ParentPortalParentAccessState,
    state: &mut ActionDispatchState,
) -> bool {
    match authoring::cancel(&action.payload, preview_id, parent_access_state) {
        Ok(()) => {
            state.accepted = true;
            state.message = "parent Rust facade invalidated the policy preview draft".to_string();
        }
        Err(error) => state.reject(error),
    }
    true
}
