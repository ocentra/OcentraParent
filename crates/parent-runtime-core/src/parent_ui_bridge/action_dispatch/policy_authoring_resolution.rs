use super::super::super::lan_route::LanRouteQuery;
use super::super::super::presentation::policy_preview::resolution;
use super::super::super::*;
use super::super::state::ActionDispatchState;
use ocentra_schema::parent_ui_bridge::ParentPolicyPreviewReadModelSnapshot;

pub(super) fn begin(
    action: &ParentUiAction,
    lan_route_query: &LanRouteQuery,
    read_model: &ParentPolicyPreviewReadModelSnapshot,
    parent_access_state: &ParentPortalParentAccessState,
    state: &mut ActionDispatchState,
) -> bool {
    let Some(lan_read_model) = lan_route_query.read_model() else {
        state.reject("parent resolution requires local controller authority");
        return true;
    };
    let result = resolution::begin(
        &action.payload,
        read_model,
        parent_access_state,
        Some(lan_read_model),
    )
    .and_then(|staged| {
        let payload = match resolution::request_payload(&staged) {
            Ok(payload) => payload,
            Err(error) => {
                let _ = resolution::restore(&staged);
                return Err(error);
            }
        };
        let relay_action = ParentUiAction {
            payload,
            ..action.clone()
        };
        super::super::dispatch_parent_ui_action_rust_owned_command(&relay_action, true, state);
        if !state.accepted {
            if let Err(error) = resolution::restore(&staged) {
                state.reject(error);
            }
            return Ok(());
        }
        match resolution::commit(&staged) {
            Ok(()) => {
                state.message =
                    "parent Rust facade relayed the typed parent resolution".to_string();
                Ok(())
            }
            Err(error) => {
                let _ = resolution::restore(&staged);
                Err(error)
            }
        }
    });
    if let Err(error) = result {
        state.reject(error);
    }
    true
}
