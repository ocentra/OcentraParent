use ocentra_eventing::error::EventingError;
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::logging::{LogFieldValue, LogFields};
use ocentra_parent_agent_protocol::transport::PolicyRequestParentResolutionRequest;
use ocentra_policy_control_core::policy_request::{
    ChildPolicyRequest, ParentPolicyApproval, PolicyApprovalId, PolicyDurationMinutes,
    PolicyRequestTimestamp,
};
use ocentra_policy_control_core::policy_source::{PolicyActorId, PolicyAuditReferenceId};

use super::mapping;
use super::types::{ErrorMessage, FieldName, FieldText, PreviousResolution, ResolutionError};

pub(crate) fn canonical_confirmed_request(
    fields: &LogFields,
) -> Result<ChildPolicyRequest, ResolutionError> {
    canonical_request(
        fields,
        FieldName(constants::policy_control::request::FIELD_CANONICAL_CONFIRMED_REQUEST_JSON),
    )
}

pub(crate) fn canonical_previous_resolution(
    fields: &LogFields,
) -> Result<PreviousResolution, ResolutionError> {
    let request = canonical_request(
        fields,
        FieldName(constants::policy_control::request::FIELD_CANONICAL_RESOLVED_REQUEST_JSON),
    )?;
    let temporary_override = fields
        .get(constants::policy_control::request::FIELD_CANONICAL_TEMPORARY_OVERRIDE_JSON)
        .and_then(string_value)
        .map(|text| {
            serde_json::from_str(&text.0)
                .map_err(|error| ResolutionError::from_message(ErrorMessage(error.to_string())))
        })
        .transpose()?;
    Ok(PreviousResolution {
        request,
        temporary_override,
    })
}

pub(crate) fn build_parent_policy_approval(
    confirmed_request: &ChildPolicyRequest,
    request: &PolicyRequestParentResolutionRequest,
) -> Result<ParentPolicyApproval, EventingError> {
    Ok(ParentPolicyApproval {
        approval_id: PolicyApprovalId::parse(request.approval_id.clone())?,
        request_id: confirmed_request.request_id.clone(),
        household_id: confirmed_request.household_id.clone(),
        policy_version: confirmed_request.policy_version,
        actor_id: PolicyActorId::parse(request.parent_actor_id.clone())?,
        actor_role: mapping::actor_role(request.parent_actor_role),
        actor_state: mapping::actor_state(request.parent_actor_state),
        decision: mapping::decision(request.decision),
        approved_action: request.approved_action.map(mapping::action),
        approved_bonus_minutes: request
            .approved_bonus_minutes
            .map(PolicyDurationMinutes::new)
            .transpose()?,
        override_expires_at: request
            .override_expires_at
            .clone()
            .map(PolicyRequestTimestamp::parse)
            .transpose()?,
        decided_at: PolicyRequestTimestamp::parse(request.decided_at.clone())?,
        audit_reference_id: PolicyAuditReferenceId::parse(
            request.approval_audit_reference_id.clone(),
        )?,
    })
}

fn canonical_request(
    fields: &LogFields,
    field_name: FieldName,
) -> Result<ChildPolicyRequest, ResolutionError> {
    let text = fields
        .get(field_name.0)
        .and_then(string_value)
        .ok_or_else(|| {
            ResolutionError::from_message(ErrorMessage(format!("missing {}", field_name.0)))
        })?;
    serde_json::from_str(&text.0).map_err(|error| {
        ResolutionError::from_message(ErrorMessage(format!("{}: {error}", field_name.0)))
    })
}

fn string_value(value: &LogFieldValue) -> Option<FieldText> {
    match value {
        LogFieldValue::String(text) => Some(FieldText(text.clone())),
        _ => None,
    }
}
