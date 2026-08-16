use chrono::Utc;
use ocentra_parent_agent_protocol::transport::{
    PolicyRequestAssistantPreviewConfirmAction, PolicyRequestAssistantPreviewConfirmActorRole,
    PolicyRequestAssistantPreviewConfirmActorState, PolicyRequestAssistantPreviewConfirmRequest,
    PolicyRequestAssistantPreviewConfirmRequestKind,
    PolicyRequestAssistantPreviewConfirmTargetKind,
};
use ocentra_parent_agent_protocol::AGENT_PROTOCOL_SCHEMA_VERSION;
use serde_json::Value;

use super::super::StagedPolicyPreviewDraft;

pub(super) fn build(
    draft: &StagedPolicyPreviewDraft,
    context: &ocentra_schema::parent_ui_bridge::ParentPolicyPreviewConfirmationContext,
    preview_id: &ocentra_schema::parent_ui_bridge::ParentPolicyPreviewId,
    target_kind: PolicyRequestAssistantPreviewConfirmTargetKind,
    target_reference_id: &str,
    requested_action: PolicyRequestAssistantPreviewConfirmAction,
    actor_role: PolicyRequestAssistantPreviewConfirmActorRole,
    actor_state: PolicyRequestAssistantPreviewConfirmActorState,
    audit_reference_ids: Vec<String>,
) -> Result<PolicyRequestAssistantPreviewConfirmRequest, String> {
    let now = Utc::now();
    Ok(PolicyRequestAssistantPreviewConfirmRequest {
        schema_version: AGENT_PROTOCOL_SCHEMA_VERSION,
        command_id: format!("policy-preview-confirm-{}", draft.handle),
        request_id: super::actor::required_context(&context.request_id, "request id")?.to_string(),
        submission_key: super::actor::required_context(&context.submission_key, "submission key")?.to_string(),
        household_id: super::actor::required_context(&context.household_id, "household")?.to_string(),
        child_profile_id: super::actor::required_context(&context.child_profile_id, "child profile")?.to_string(),
        device_id: Some(super::actor::required_context(&context.device_id, "device")?.to_string()),
        source_document_id: super::actor::required_context(&context.source_document_id, "source document")?.to_string(),
        policy_version: context.policy_version.ok_or_else(|| {
            "policy preview policy version is unavailable; manual review required".to_string()
        })?,
        request_kind: PolicyRequestAssistantPreviewConfirmRequestKind::AskParent,
        target_kind,
        target_reference_id: target_reference_id.to_string(),
        requested_action,
        rule_id: context.rule_id.clone(),
        requested_bonus_minutes: None,
        requested_at: super::actor::required_context(&context.requested_at, "request timestamp")?.to_string(),
        expires_at: super::actor::required_context(&context.expires_at, "request expiry")?.to_string(),
        origin: ocentra_parent_agent_protocol::activity::policy_preview::PolicyRequestOrigin::AssistantDraft,
        assistant_preview_id: preview_id.as_str().to_string(),
        assistant_confirmation_state:
            ocentra_parent_agent_protocol::activity::policy_preview::PolicyAssistantConfirmationState::ParentConfirmationRequired,
        request_status: ocentra_parent_agent_protocol::activity::policy_preview::PolicyRequestStatus::PreviewOnly,
        audit_reference_ids,
        confirmation_actor_id: super::actor::required_context(&context.actor_id, "actor id")?.to_string(),
        confirmation_actor_role: actor_role,
        confirmation_actor_state: actor_state,
        confirmation_audit_reference_id: super::actor::required_context(
            &context.confirmation_audit_reference_id,
            "confirmation audit reference",
        )?
        .to_string(),
        confirmed_at: now.to_rfc3339(),
    })
}

pub(super) fn serialize(
    request: PolicyRequestAssistantPreviewConfirmRequest,
) -> Result<Value, String> {
    let request_text = serde_json::to_string(&request)
        .map_err(|_| "policy preview confirmation request could not be serialized".to_string())?;
    Ok(serde_json::json!({
        ocentra_parent_agent_protocol::constants::field::POLICY_REQUEST_ASSISTANT_PREVIEW_CONFIRM_REQUEST: request_text,
    }))
}
