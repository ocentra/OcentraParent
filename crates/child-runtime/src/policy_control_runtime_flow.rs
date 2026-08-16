use ocentra_child_notification_core::policy_control_notification::{
    build_policy_control_parent_notification, PolicyControlParentNotification,
};
use ocentra_child_policy_core::policy_control_delivery_handoff::{
    apply_policy_control_delivery_handoff as apply_child_policy_control_delivery_handoff,
    queue_policy_control_delivery_for_request as queue_child_policy_control_delivery_for_request,
};
use ocentra_child_policy_core::policy_control_request_handoff::{
    confirm_policy_control_request_handoff as confirm_child_policy_control_request_handoff,
    expire_policy_control_request_handoff as expire_child_policy_control_request_handoff,
    policy_control_request_resolution_handoff as child_policy_control_request_resolution_handoff,
    register_policy_control_request_handoff as register_child_policy_control_request_handoff,
    resolve_policy_control_request_handoff as resolve_child_policy_control_request_handoff,
};
use ocentra_eventing::error::EventingError;
use ocentra_policy_control_core::policy_delivery::{
    PolicyDeliveryAttemptId, PolicyDeliveryId, PolicyDeliveryRecord, PolicyDeliveryTarget,
    PolicyDeliveryTransition,
};
use ocentra_policy_control_core::policy_request::{
    AssistantPolicyRequestConfirmation, ChildPolicyRequest, ParentPolicyApproval,
    PolicyRequestResolution, PolicyRequestTimestamp, PolicyTemporaryOverride,
};
use ocentra_policy_control_core::policy_source::{
    CompiledDomainPolicyArtifact, PolicyAuditReferenceId,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PolicyControlRequestHandoffReport {
    pub request: ChildPolicyRequest,
    pub temporary_override: Option<PolicyTemporaryOverride>,
    pub parent_notification: PolicyControlParentNotification,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PolicyControlDeliveryHandoffReport {
    pub delivery: PolicyDeliveryRecord,
    pub parent_notification: PolicyControlParentNotification,
}

pub fn register_policy_control_request_handoff(
    existing: Option<&ChildPolicyRequest>,
    candidate: ChildPolicyRequest,
) -> Result<PolicyControlRequestHandoffReport, EventingError> {
    let handoff = register_child_policy_control_request_handoff(existing, candidate)?;
    let parent_notification =
        build_policy_control_parent_notification(&handoff.request, None, None)?;

    Ok(PolicyControlRequestHandoffReport {
        request: handoff.request,
        temporary_override: None,
        parent_notification,
    })
}

pub fn confirm_policy_control_request_handoff(
    request: &ChildPolicyRequest,
    confirmation: AssistantPolicyRequestConfirmation,
) -> Result<PolicyControlRequestHandoffReport, EventingError> {
    let handoff = confirm_child_policy_control_request_handoff(request, confirmation)?;
    let parent_notification =
        build_policy_control_parent_notification(&handoff.request, None, None)?;

    Ok(PolicyControlRequestHandoffReport {
        request: handoff.request,
        temporary_override: None,
        parent_notification,
    })
}

pub fn expire_policy_control_request_handoff(
    request: &ChildPolicyRequest,
    expired_at: PolicyRequestTimestamp,
    audit_reference_id: PolicyAuditReferenceId,
) -> Result<PolicyControlRequestHandoffReport, EventingError> {
    let handoff =
        expire_child_policy_control_request_handoff(request, expired_at, audit_reference_id)?;
    let parent_notification =
        build_policy_control_parent_notification(&handoff.request, None, None)?;

    Ok(PolicyControlRequestHandoffReport {
        request: handoff.request,
        temporary_override: None,
        parent_notification,
    })
}

pub fn resolve_policy_control_request_handoff(
    request: &ChildPolicyRequest,
    approval: ParentPolicyApproval,
    existing_override: Option<&PolicyTemporaryOverride>,
) -> Result<PolicyControlRequestHandoffReport, EventingError> {
    let handoff =
        resolve_child_policy_control_request_handoff(request, approval, existing_override)?;
    let parent_notification = build_policy_control_parent_notification(
        &handoff.request,
        handoff.temporary_override.as_ref(),
        None,
    )?;

    Ok(PolicyControlRequestHandoffReport {
        request: handoff.request,
        temporary_override: handoff.temporary_override,
        parent_notification,
    })
}

pub fn queue_policy_control_delivery_handoff(
    artifact: &CompiledDomainPolicyArtifact,
    target: PolicyDeliveryTarget,
    request: &ChildPolicyRequest,
    temporary_override: Option<&PolicyTemporaryOverride>,
    delivery_id: PolicyDeliveryId,
    attempt_id: PolicyDeliveryAttemptId,
    audit_reference_ids: Vec<PolicyAuditReferenceId>,
) -> Result<PolicyControlDeliveryHandoffReport, EventingError> {
    let handoff = queue_child_policy_control_delivery_for_request(
        request,
        artifact,
        target,
        delivery_id,
        attempt_id,
        audit_reference_ids,
    )?;
    let parent_notification = build_policy_control_parent_notification(
        request,
        temporary_override,
        Some(&handoff.delivery),
    )?;

    Ok(PolicyControlDeliveryHandoffReport {
        delivery: handoff.delivery,
        parent_notification,
    })
}

pub fn apply_policy_control_delivery_handoff(
    request: &ChildPolicyRequest,
    temporary_override: Option<&PolicyTemporaryOverride>,
    current: &PolicyDeliveryRecord,
    transition: PolicyDeliveryTransition,
) -> Result<PolicyControlDeliveryHandoffReport, EventingError> {
    let handoff = apply_child_policy_control_delivery_handoff(current, transition)?;
    let parent_notification = build_policy_control_parent_notification(
        request,
        temporary_override,
        Some(&handoff.delivery),
    )?;

    Ok(PolicyControlDeliveryHandoffReport {
        delivery: handoff.delivery,
        parent_notification,
    })
}

pub fn policy_control_request_resolution_handoff(
    resolution: PolicyRequestResolution,
) -> Result<PolicyControlRequestHandoffReport, EventingError> {
    let handoff = child_policy_control_request_resolution_handoff(resolution);
    let parent_notification = build_policy_control_parent_notification(
        &handoff.request,
        handoff.temporary_override.as_ref(),
        None,
    )?;

    Ok(PolicyControlRequestHandoffReport {
        request: handoff.request,
        temporary_override: handoff.temporary_override,
        parent_notification,
    })
}
