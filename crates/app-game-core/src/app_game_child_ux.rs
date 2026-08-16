use ocentra_eventing::error::EventingError;

use crate::app_game_child_ux_resolve::resolve_notice;
use crate::app_game_child_ux_types::{
    AppGameChildUxAction, AppGameChildUxBudgetContext, AppGameChildUxInput, AppGameChildUxNotice,
};
use crate::app_game_time_budget_types::AppGameTimeBudgetDecision;

pub fn build_app_game_child_ux_notice_from_time_budget(
    decision: &AppGameTimeBudgetDecision,
    context: AppGameChildUxBudgetContext,
) -> Result<AppGameChildUxNotice, EventingError> {
    build_app_game_child_ux_notice(AppGameChildUxInput {
        subject_kind: context.subject_kind,
        runtime_decision: decision.runtime_decision.clone(),
        request_state: context.request_state,
        capability_state: context.capability_state,
        policy_rule_ref: context.policy_rule_ref,
        evidence_refs: context.evidence_refs,
        child_reason_refs: context.child_reason_refs,
        child_status_refs: context.child_status_refs,
        adapter_action_ref: context.adapter_action_ref,
    })
}

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
