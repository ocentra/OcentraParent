use ocentra_eventing::error::EventingError;
use ocentra_policy_control_core::policy_request::{
    confirm_assistant_policy_request_preview, expire_child_policy_request,
    register_child_policy_request, resolve_parent_policy_approval,
    AssistantPolicyRequestConfirmation, ChildPolicyRequest, ParentPolicyApproval,
    PolicyRequestResolution, PolicyRequestTimestamp, PolicyTemporaryOverride,
};
use ocentra_policy_control_core::policy_source::PolicyAuditReferenceId;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PolicyControlRequestHandoffReport {
    pub request: ChildPolicyRequest,
    pub temporary_override: Option<PolicyTemporaryOverride>,
}

pub fn register_policy_control_request_handoff(
    existing: Option<&ChildPolicyRequest>,
    candidate: ChildPolicyRequest,
) -> Result<PolicyControlRequestHandoffReport, EventingError> {
    let request = register_child_policy_request(existing, candidate)?;
    Ok(PolicyControlRequestHandoffReport {
        request,
        temporary_override: None,
    })
}

pub fn confirm_policy_control_request_handoff(
    request: &ChildPolicyRequest,
    confirmation: AssistantPolicyRequestConfirmation,
) -> Result<PolicyControlRequestHandoffReport, EventingError> {
    let request = confirm_assistant_policy_request_preview(request, confirmation)?;
    Ok(PolicyControlRequestHandoffReport {
        request,
        temporary_override: None,
    })
}

pub fn expire_policy_control_request_handoff(
    request: &ChildPolicyRequest,
    expired_at: PolicyRequestTimestamp,
    audit_reference_id: PolicyAuditReferenceId,
) -> Result<PolicyControlRequestHandoffReport, EventingError> {
    let request = expire_child_policy_request(request, expired_at, audit_reference_id)?;
    Ok(PolicyControlRequestHandoffReport {
        request,
        temporary_override: None,
    })
}

pub fn resolve_policy_control_request_handoff(
    request: &ChildPolicyRequest,
    approval: ParentPolicyApproval,
    existing_override: Option<&PolicyTemporaryOverride>,
) -> Result<PolicyControlRequestHandoffReport, EventingError> {
    let resolution = resolve_parent_policy_approval(request, approval, existing_override)?;
    Ok(PolicyControlRequestHandoffReport {
        request: resolution.request,
        temporary_override: resolution.temporary_override,
    })
}

pub fn policy_control_request_resolution_handoff(
    resolution: PolicyRequestResolution,
) -> PolicyControlRequestHandoffReport {
    PolicyControlRequestHandoffReport {
        request: resolution.request,
        temporary_override: resolution.temporary_override,
    }
}
