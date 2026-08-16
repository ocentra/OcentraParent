use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::transport::{
    PolicyRequestAssistantPreviewConfirmActorRole, PolicyRequestAssistantPreviewConfirmActorState,
};

pub(super) fn actor_role(
    value: &str,
) -> Result<PolicyRequestAssistantPreviewConfirmActorRole, String> {
    match value {
        constants::policy_control::source::ROLE_PARENT => {
            Ok(PolicyRequestAssistantPreviewConfirmActorRole::Parent)
        }
        constants::policy_control::source::ROLE_CO_PARENT => {
            Ok(PolicyRequestAssistantPreviewConfirmActorRole::CoParent)
        }
        value => Err(format!("policy preview actor role cannot confirm: {value}")),
    }
}

pub(super) fn actor_state(
    value: &str,
) -> Result<PolicyRequestAssistantPreviewConfirmActorState, String> {
    match value {
        constants::policy_control::source::ACTOR_STATE_ACTIVE => {
            Ok(PolicyRequestAssistantPreviewConfirmActorState::Active)
        }
        constants::policy_control::source::ACTOR_STATE_REVOKED => {
            Ok(PolicyRequestAssistantPreviewConfirmActorState::Revoked)
        }
        value => Err(format!("policy preview actor state is invalid: {value}")),
    }
}

pub(super) fn required_context<'a>(
    value: &'a Option<String>,
    label: &str,
) -> Result<&'a str, String> {
    value
        .as_deref()
        .filter(|value| !value.trim().is_empty())
        .ok_or_else(|| format!("policy preview {label} is unavailable; manual review required"))
}

pub(super) fn audit_reference_ids(
    context: &ocentra_schema::parent_ui_bridge::ParentPolicyPreviewConfirmationContext,
) -> Result<Vec<String>, String> {
    let values = required_context(&context.audit_reference_ids, "audit references")?
        .split(constants::delimiter::LIST)
        .filter(|value| !value.trim().is_empty())
        .map(ToString::to_string)
        .collect::<Vec<_>>();
    if values.is_empty() {
        return Err(
            "policy preview audit references are empty; manual review required".to_string(),
        );
    }
    Ok(values)
}
