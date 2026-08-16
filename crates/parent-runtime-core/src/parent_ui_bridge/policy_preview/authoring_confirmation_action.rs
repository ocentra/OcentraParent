use ocentra_parent_agent_protocol::transport::PolicyRequestAssistantPreviewConfirmAction;

use super::super::StagedPolicyPreviewDraft;

pub(super) fn trusted_action(draft: &StagedPolicyPreviewDraft) -> Result<&str, String> {
    let action = draft
        .read_model
        .network_requested_policy_action
        .as_deref()
        .or_else(|| {
            draft
                .read_model
                .decision_action
                .as_ref()
                .map(|value| value.as_str())
        })
        .ok_or_else(|| {
            "policy preview trusted requested action is unavailable; manual review required"
                .to_string()
        })?;
    if action != draft.requested_action {
        return Err("policy preview draft action does not match trusted request".to_string());
    }
    Ok(action)
}

pub(super) fn requested_action(
    value: &str,
) -> Result<PolicyRequestAssistantPreviewConfirmAction, String> {
    match value {
        "allow" => Ok(PolicyRequestAssistantPreviewConfirmAction::Allow),
        "warn" => Ok(PolicyRequestAssistantPreviewConfirmAction::Warn),
        "ask-parent" => Ok(PolicyRequestAssistantPreviewConfirmAction::AskParent),
        "time-limit" => Ok(PolicyRequestAssistantPreviewConfirmAction::TimeLimit),
        "block" => Ok(PolicyRequestAssistantPreviewConfirmAction::Block),
        value => Err(format!(
            "policy preview action cannot be confirmed: {value}"
        )),
    }
}
