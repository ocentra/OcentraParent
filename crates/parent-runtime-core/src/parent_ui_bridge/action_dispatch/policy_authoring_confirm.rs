use super::super::super::presentation::policy_preview::authoring;
use super::super::super::*;
use super::super::state::ActionDispatchState;
use ocentra_schema::parent_ui_bridge::{ParentPolicyPreviewId, ParentPortalParentAccessState};

pub(super) fn confirm(
    action: &ParentUiAction,
    preview_id: &ParentPolicyPreviewId,
    parent_access_state: &ParentPortalParentAccessState,
    state: &mut ActionDispatchState,
) -> bool {
    let result =
        authoring::consume(&action.payload, preview_id, parent_access_state).and_then(|draft| {
            let payload = match authoring::typed_confirm_payload(&draft) {
                Ok(payload) => payload,
                Err(error) => {
                    let _ = authoring::release(&draft, preview_id, parent_access_state);
                    return Err(error);
                }
            };
            let relay_action = ParentUiAction {
                payload,
                ..action.clone()
            };
            super::super::dispatch_parent_ui_action_rust_owned_command(&relay_action, true, state);
            if !state.accepted {
                let _ = authoring::release(&draft, preview_id, parent_access_state);
                return Ok(());
            }
            match authoring::commit(&draft, preview_id, parent_access_state) {
                Ok(()) => {
                    state.message =
                        "parent Rust facade relayed the typed policy preview confirmation"
                            .to_string();
                    Ok(())
                }
                Err(error) => {
                    let _ = authoring::release(&draft, preview_id, parent_access_state);
                    Err(error)
                }
            }
        });
    if let Err(error) = result {
        state.reject(error);
    }
    true
}
