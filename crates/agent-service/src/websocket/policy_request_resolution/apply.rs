use ocentra_child_runtime::policy_control_runtime_flow::policy_control_request_resolution_handoff;
use ocentra_parent_agent_protocol::transport::{
    AgentCommandEnvelope, PolicyRequestAssistantPreviewConfirmClaimState,
    PolicyRequestParentResolutionRequest, PolicyRequestParentResolutionResult,
};
use ocentra_policy_control_core::policy_request::resolve_parent_policy_approval;

use super::types::{AuditEventId, CommandId, RequestIdText, ResolutionError, ResolutionSnapshot};
use super::{domain, result, store};

pub(crate) async fn resolve(
    command: &AgentCommandEnvelope,
    request: &PolicyRequestParentResolutionRequest,
    snapshot: ResolutionSnapshot,
) -> PolicyRequestParentResolutionResult {
    let approval = match domain::build_parent_policy_approval(&snapshot.confirmed_request, request)
    {
        Ok(approval) => approval,
        Err(error) => return reject_core_error(request, &snapshot.confirmed_request, error.into()),
    };
    let base_request = snapshot
        .previous_resolution
        .as_ref()
        .map(|previous| &previous.request)
        .unwrap_or(&snapshot.confirmed_request);
    let existing_override = snapshot
        .previous_resolution
        .as_ref()
        .and_then(|previous| previous.temporary_override.as_ref());
    let resolution = match resolve_parent_policy_approval(base_request, approval, existing_override)
    {
        Ok(resolution) => resolution,
        Err(error) => return reject_core_error(request, base_request, error.into()),
    };
    let notification_claim_state =
        match policy_control_request_resolution_handoff(resolution.clone()) {
            Ok(_) => PolicyRequestAssistantPreviewConfirmClaimState::Claimed,
            Err(error) => return reject_core_error(request, base_request, error.into()),
        };
    let result = result::resolved(
        request,
        &resolution,
        notification_claim_state,
        snapshot.previous_resolution.is_some(),
    );
    if snapshot.previous_resolution.is_none() {
        let _ = store::persist_resolution(command, request, &resolution, &result).await;
    }
    result
}

fn reject_core_error(
    request: &PolicyRequestParentResolutionRequest,
    base_request: &ocentra_policy_control_core::policy_request::ChildPolicyRequest,
    error: ResolutionError,
) -> PolicyRequestParentResolutionResult {
    result::rejected(
        CommandId(request.command_id.clone()),
        AuditEventId(request.confirmed_audit_reference_id.clone()),
        Some(RequestIdText(base_request.request_id.as_str().to_string())),
        base_request.status,
        error.into_reason(),
        true,
    )
}
