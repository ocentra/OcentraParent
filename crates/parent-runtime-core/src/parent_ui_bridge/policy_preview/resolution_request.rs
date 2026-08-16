use super::*;

pub(super) fn build(
    validated: &super::validation::ValidatedResolution,
    handle: &str,
) -> (PolicyRequestParentResolutionRequest, StoredParentResolution) {
    let request = PolicyRequestParentResolutionRequest {
        schema_version: AGENT_PROTOCOL_SCHEMA_VERSION,
        command_id: format!("policy-parent-resolution-{handle}"),
        confirmed_audit_reference_id: validated.confirmed_audit_reference_id.clone(),
        approval_id: validated.approval_id.clone(),
        parent_actor_id: validated.parent_actor_id.clone(),
        parent_actor_role: validated.parent_actor_role,
        parent_actor_state: validated.parent_actor_state,
        decision: validated.decision,
        approved_action: validated.approved_action,
        approved_bonus_minutes: None,
        override_expires_at: validated.override_expires_at.clone(),
        decided_at: Utc::now().to_rfc3339(),
        approval_audit_reference_id: validated.approval_audit_reference_id.clone(),
        delivery_binding: validated.delivery_binding.clone(),
    };
    let stored = StoredParentResolution {
        preview_id: validated.preview_id.clone(),
        parent_actor_id: validated.parent_actor_id.clone(),
        issued_at: Instant::now(),
        in_flight: true,
    };
    (request, stored)
}

pub(super) fn approved_action_for(
    decision: PolicyRequestParentResolutionDecision,
    read_model: &ParentPolicyPreviewReadModelSnapshot,
) -> Result<Option<PolicyRequestAssistantPreviewConfirmAction>, String> {
    match decision {
        PolicyRequestParentResolutionDecision::Grant
        | PolicyRequestParentResolutionDecision::Modify => Ok(Some(approved_action(read_model)?)),
        PolicyRequestParentResolutionDecision::Deny
        | PolicyRequestParentResolutionDecision::Expire => Ok(None),
    }
}

pub(super) fn delivery_binding_for(
    decision: PolicyRequestParentResolutionDecision,
    context: &ocentra_schema::parent_ui_bridge::ParentPolicyPreviewConfirmationContext,
) -> Result<Option<PolicyRequestParentResolutionDeliveryBinding>, String> {
    match decision {
        PolicyRequestParentResolutionDecision::Grant
        | PolicyRequestParentResolutionDecision::Modify => {
            build_delivery_binding(context).map(Some)
        }
        PolicyRequestParentResolutionDecision::Deny
        | PolicyRequestParentResolutionDecision::Expire => Ok(None),
    }
}

fn build_delivery_binding(
    context: &ocentra_schema::parent_ui_bridge::ParentPolicyPreviewConfirmationContext,
) -> Result<PolicyRequestParentResolutionDeliveryBinding, String> {
    let household_id =
        super::inputs::required_context(&context.household_id, "household")?.to_string();
    let child_profile_id =
        super::inputs::required_context(&context.child_profile_id, "child profile")?.to_string();
    let source_document_id =
        super::inputs::required_context(&context.source_document_id, "source document")?
            .to_string();
    let policy_version = context.policy_version.ok_or_else(|| {
        "parent resolution policy version is unavailable; manual review required".to_string()
    })?;
    Ok(PolicyRequestParentResolutionDeliveryBinding {
        household_id,
        child_profile_id,
        device_id: context.device_id.clone(),
        source_document_id,
        policy_version,
    })
}

pub(super) fn override_expiry_for(
    decision: PolicyRequestParentResolutionDecision,
    context: &ocentra_schema::parent_ui_bridge::ParentPolicyPreviewConfirmationContext,
) -> Result<Option<String>, String> {
    match decision {
        PolicyRequestParentResolutionDecision::Grant
        | PolicyRequestParentResolutionDecision::Modify => Ok(Some(
            super::inputs::required_context(&context.expires_at, "override expiry")?.to_string(),
        )),
        PolicyRequestParentResolutionDecision::Deny
        | PolicyRequestParentResolutionDecision::Expire => Ok(None),
    }
}

fn approved_action(
    read_model: &ParentPolicyPreviewReadModelSnapshot,
) -> Result<PolicyRequestAssistantPreviewConfirmAction, String> {
    let value = read_model
        .network_mapped_policy_action
        .as_deref()
        .or(read_model.network_requested_policy_action.as_deref())
        .ok_or_else(|| {
            "parent resolution approved action is unavailable; manual review required".to_string()
        })?;
    serde_json::from_value(Value::String(value.to_string())).map_err(|_| {
        "parent resolution approved action is unsupported; manual review required".to_string()
    })
}
