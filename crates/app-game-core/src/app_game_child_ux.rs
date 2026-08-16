use ocentra_eventing::error::EventingError;

use crate::app_game_child_ux_resolve::resolve_notice;
use crate::app_game_child_ux_types::{
    AppGameChildUxAction, AppGameChildUxInput, AppGameChildUxNotice,
};

pub fn build_app_game_child_ux_notice(
    input: AppGameChildUxInput,
) -> Result<AppGameChildUxNotice, EventingError> {
    if input.adapter_action_ref.is_some() {
        return Err(EventingError::InvalidValue {
            field: "app_game.child_ux.adapter_action_ref",
            value: String::from("child UX cannot claim adapter execution"),
        });
    }
    let (state, text_token, action) = resolve_notice(&input);
    if action == AppGameChildUxAction::AskParent
        && (input.evidence_refs.is_empty()
            || input.child_reason_refs.is_empty()
            || input.child_status_refs.is_empty())
    {
        return Err(EventingError::InvalidValue {
            field: "app_game.child_ux.ask_parent_refs",
            value: String::from(
                "ask-parent requires evidence, child reason, and child status refs",
            ),
        });
    }
    Ok(AppGameChildUxNotice {
        state,
        text_token,
        action,
        policy_rule_ref: input.policy_rule_ref,
        evidence_refs: input.evidence_refs,
        child_reason_refs: input.child_reason_refs,
        child_status_refs: input.child_status_refs,
        remaining_seconds: input.runtime_decision.remaining_seconds,
        adapter_dispatch_claimed: false,
    })
}
