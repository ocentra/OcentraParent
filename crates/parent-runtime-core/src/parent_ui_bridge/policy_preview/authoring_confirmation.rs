use serde_json::Value;

use super::StagedPolicyPreviewDraft;

#[path = "authoring_confirmation_action.rs"]
mod action;
#[path = "authoring_confirmation_actor.rs"]
mod actor;
#[path = "authoring_confirmation_request.rs"]
mod request;
#[path = "authoring_confirmation_target.rs"]
mod target;

pub(super) fn typed_confirm_payload(draft: &StagedPolicyPreviewDraft) -> Result<Value, String> {
    let context = draft
        .read_model
        .confirmation_context
        .as_ref()
        .ok_or_else(|| {
            "policy preview confirmation context is unavailable; manual review required".to_string()
        })?;
    let target_kind = target::target_kind(draft)?;
    let trusted_action = action::trusted_action(draft)?;
    let requested_action = action::requested_action(trusted_action)?;
    let preview_id = draft
        .read_model
        .preview_id
        .as_ref()
        .ok_or_else(|| "policy preview identifier is missing".to_string())?;
    if actor::required_context(&context.assistant_preview_id, "assistant preview")?
        != preview_id.as_str()
    {
        return Err(
            "policy preview trusted assistant preview does not match current preview".to_string(),
        );
    }
    let target_reference_id =
        actor::required_context(&context.target_reference_id, "target reference")?;
    if draft.target_value != target_reference_id {
        return Err("policy preview draft target does not match trusted request".to_string());
    }
    if draft
        .read_model
        .target_value
        .as_deref()
        .is_some_and(|value| value != target_reference_id)
    {
        return Err("policy preview trusted target does not match source request".to_string());
    }
    let actor_role =
        actor::actor_role(actor::required_context(&context.actor_role, "actor role")?)?;
    let actor_state = actor::actor_state(actor::required_context(
        &context.actor_state,
        "actor state",
    )?)?;
    let audit_reference_ids = actor::audit_reference_ids(context)?;
    let request = request::build(
        draft,
        context,
        preview_id,
        target_kind,
        target_reference_id,
        requested_action,
        actor_role,
        actor_state,
        audit_reference_ids,
    )?;
    request::serialize(request)
}
