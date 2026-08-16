use super::*;

pub(super) struct ValidatedResolution {
    pub(super) decision: PolicyRequestParentResolutionDecision,
    pub(super) parent_actor_id: String,
    pub(super) parent_actor_role: PolicyRequestAssistantPreviewConfirmActorRole,
    pub(super) parent_actor_state: PolicyRequestAssistantPreviewConfirmActorState,
    pub(super) preview_id: String,
    pub(super) confirmed_audit_reference_id: String,
    pub(super) approval_audit_reference_id: String,
    pub(super) approval_id: String,
    pub(super) approved_action: Option<PolicyRequestAssistantPreviewConfirmAction>,
    pub(super) delivery_binding: Option<PolicyRequestParentResolutionDeliveryBinding>,
    pub(super) override_expires_at: Option<String>,
}

pub(super) fn validate(
    payload: &Value,
    read_model: &ParentPolicyPreviewReadModelSnapshot,
    parent_access_state: &ParentPortalParentAccessState,
    lan_read_model: Option<&LanBrowserAddDeviceReadModel>,
) -> Result<ValidatedResolution, String> {
    require_active_controller(parent_access_state)?;
    let decision = super::inputs::decision_from_payload(payload)?;
    let context = read_model.confirmation_context.as_ref().ok_or_else(|| {
        "parent resolution context is unavailable; manual review required".to_string()
    })?;
    let parent_actor_id = super::inputs::required_context(&context.actor_id, "actor id")?;
    let local_actor_id = super::lifecycle::local_controller_actor_id(lan_read_model)?;
    if parent_actor_id != local_actor_id {
        return Err(
            "parent resolution actor does not match local controller authority".to_string(),
        );
    }
    let parent_actor_role = super::inputs::actor_role(&context.actor_role)?;
    require_approver_role(parent_actor_role)?;
    let parent_actor_state = super::inputs::actor_state(&context.actor_state)?;
    require_active_actor(parent_actor_state)?;
    let preview_id = read_model
        .preview_id
        .as_ref()
        .ok_or_else(|| "parent resolution preview identifier is missing".to_string())?
        .as_str()
        .to_string();
    let approval_id = read_model
        .policy_approval_id
        .as_ref()
        .map(|value| value.as_str().to_string())
        .ok_or_else(|| {
            "parent resolution approval identifier is unavailable; manual review required"
                .to_string()
        })?;
    let confirmed_audit_reference_id = super::inputs::required_context(
        &context.confirmation_audit_reference_id,
        "confirmation audit reference",
    )?
    .to_string();
    let approval_audit_reference_id =
        super::inputs::single_audit_reference(context.audit_reference_ids.as_deref())?.to_string();
    let approved_action = super::request::approved_action_for(decision, read_model)?;
    let delivery_binding = super::request::delivery_binding_for(decision, context)?;
    let override_expires_at = super::request::override_expiry_for(decision, context)?;
    Ok(ValidatedResolution {
        decision,
        parent_actor_id: parent_actor_id.to_string(),
        parent_actor_role,
        parent_actor_state,
        preview_id,
        confirmed_audit_reference_id,
        approval_audit_reference_id,
        approval_id,
        approved_action,
        delivery_binding,
        override_expires_at,
    })
}

fn require_active_controller(
    parent_access_state: &ParentPortalParentAccessState,
) -> Result<(), String> {
    if !matches!(
        parent_access_state,
        ParentPortalParentAccessState::ActiveController
    ) {
        return Err("parent resolution requires active controller authority".to_string());
    }
    Ok(())
}

fn require_approver_role(
    actor_role: PolicyRequestAssistantPreviewConfirmActorRole,
) -> Result<(), String> {
    if !matches!(
        actor_role,
        PolicyRequestAssistantPreviewConfirmActorRole::Parent
            | PolicyRequestAssistantPreviewConfirmActorRole::CoParent
    ) {
        return Err("parent resolution actor role is not approver-capable".to_string());
    }
    Ok(())
}

fn require_active_actor(
    actor_state: PolicyRequestAssistantPreviewConfirmActorState,
) -> Result<(), String> {
    if actor_state != PolicyRequestAssistantPreviewConfirmActorState::Active {
        return Err("parent resolution actor is not active".to_string());
    }
    Ok(())
}
