use ocentra_parent_agent_protocol::activity::policy_preview::PolicyRequestStatus;
use ocentra_parent_agent_protocol::transport::{
    PolicyRequestAssistantPreviewConfirmClaimState, PolicyRequestParentResolutionRequest,
    PolicyRequestParentResolutionResult, PolicyRequestParentResolutionResultState,
};
use ocentra_parent_agent_protocol::AGENT_PROTOCOL_SCHEMA_VERSION;
use ocentra_policy_control_core::policy_request::PolicyRequestResolution;

use super::types::{AuditEventId, CommandId, RejectionReason, RequestIdText};

const REPLAY_RESOLUTION_REASON: &str = "replayed-resolution";

pub(crate) fn resolved(
    request: &PolicyRequestParentResolutionRequest,
    resolution: &PolicyRequestResolution,
    notification_claim_state: PolicyRequestAssistantPreviewConfirmClaimState,
    replayed: bool,
) -> PolicyRequestParentResolutionResult {
    PolicyRequestParentResolutionResult {
        schema_version: AGENT_PROTOCOL_SCHEMA_VERSION,
        command_id: request.command_id.clone(),
        confirmed_audit_reference_id: request.confirmed_audit_reference_id.clone(),
        request_id: Some(resolution.request.request_id.as_str().to_string()),
        result_state: PolicyRequestParentResolutionResultState::Resolved,
        policy_request_status: resolution.request.status,
        resolved_approval_id: resolution
            .request
            .resolved_approval_id
            .as_ref()
            .map(|value| value.as_str().to_string()),
        temporary_override_id: resolution
            .temporary_override
            .as_ref()
            .map(|value| value.override_id.as_str().to_string()),
        resolved_at: resolution
            .request
            .resolved_at
            .as_ref()
            .map(|value| value.as_str().to_string()),
        rejection_reason: replayed.then(|| REPLAY_RESOLUTION_REASON.to_string()),
        command_transport_claim_state: PolicyRequestAssistantPreviewConfirmClaimState::Claimed,
        service_validation_claim_state: PolicyRequestAssistantPreviewConfirmClaimState::Claimed,
        activity_store_lookup_claim_state: PolicyRequestAssistantPreviewConfirmClaimState::Claimed,
        policy_resolution_claim_state: PolicyRequestAssistantPreviewConfirmClaimState::Claimed,
        notification_handoff_claim_state: notification_claim_state,
        child_device_delivery_claim_state:
            PolicyRequestAssistantPreviewConfirmClaimState::Unclaimed,
        product_claim_state: PolicyRequestAssistantPreviewConfirmClaimState::Unclaimed,
    }
}

pub(crate) fn rejected(
    command_id: CommandId,
    confirmed_audit_reference_id: AuditEventId,
    request_id: Option<RequestIdText>,
    policy_request_status: PolicyRequestStatus,
    rejection_reason: RejectionReason,
    lookup_claimed: bool,
) -> PolicyRequestParentResolutionResult {
    PolicyRequestParentResolutionResult {
        schema_version: AGENT_PROTOCOL_SCHEMA_VERSION,
        command_id: command_id.0,
        confirmed_audit_reference_id: confirmed_audit_reference_id.0,
        request_id: request_id.map(|value| value.0),
        result_state: PolicyRequestParentResolutionResultState::Rejected,
        policy_request_status,
        resolved_approval_id: None,
        temporary_override_id: None,
        resolved_at: None,
        rejection_reason: Some(rejection_reason.0),
        command_transport_claim_state: PolicyRequestAssistantPreviewConfirmClaimState::Claimed,
        service_validation_claim_state: PolicyRequestAssistantPreviewConfirmClaimState::Claimed,
        activity_store_lookup_claim_state: claim_state(lookup_claimed),
        policy_resolution_claim_state: PolicyRequestAssistantPreviewConfirmClaimState::Unclaimed,
        notification_handoff_claim_state: PolicyRequestAssistantPreviewConfirmClaimState::Unclaimed,
        child_device_delivery_claim_state:
            PolicyRequestAssistantPreviewConfirmClaimState::Unclaimed,
        product_claim_state: PolicyRequestAssistantPreviewConfirmClaimState::Unclaimed,
    }
}

fn claim_state(claimed: bool) -> PolicyRequestAssistantPreviewConfirmClaimState {
    if claimed {
        PolicyRequestAssistantPreviewConfirmClaimState::Claimed
    } else {
        PolicyRequestAssistantPreviewConfirmClaimState::Unclaimed
    }
}
