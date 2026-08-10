use ocentra_eventing::error::EventingError;
use ocentra_parent_agent_protocol::activity::policy_preview::PolicyRequestStatus;
use ocentra_policy_control_core::policy_delivery::{
    apply_policy_delivery_transition_without_execution_receipt, queue_policy_delivery,
    PolicyDeliveryApplyOutcome, PolicyDeliveryAttemptId, PolicyDeliveryId, PolicyDeliveryRecord,
    PolicyDeliveryState, PolicyDeliveryTarget, PolicyDeliveryTransition,
};
use ocentra_policy_control_core::policy_request::{
    validate_child_policy_request, ChildPolicyRequest,
};
use ocentra_policy_control_core::policy_source::{
    CompiledDomainPolicyArtifact, PolicyAuditReferenceId, PolicyReasonCode,
};

const TRUSTED_ADAPTER_REQUIRED_REASON: &str = "trusted-adapter-required";

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PolicyControlDeliveryHandoffReport {
    pub delivery: PolicyDeliveryRecord,
}

pub fn queue_policy_control_delivery_handoff(
    artifact: &CompiledDomainPolicyArtifact,
    target: PolicyDeliveryTarget,
    delivery_id: PolicyDeliveryId,
    attempt_id: PolicyDeliveryAttemptId,
    audit_reference_ids: Vec<PolicyAuditReferenceId>,
) -> Result<PolicyControlDeliveryHandoffReport, EventingError> {
    let delivery = queue_policy_delivery(
        artifact,
        target,
        delivery_id,
        attempt_id,
        audit_reference_ids,
    )?;
    Ok(PolicyControlDeliveryHandoffReport { delivery })
}

/// Bind an approved request to the compiled artifact and its concrete child
/// delivery target before creating a delivery record.
///
/// The request, artifact, and target are separate values at this boundary. A
/// caller must not be able to mix a request from one household/version/device
/// with an artifact or target from another. This check establishes identity
/// only; receipt-required transitions still fail closed until a trusted
/// adapter supplies execution authority.
pub fn queue_policy_control_delivery_for_request(
    request: &ChildPolicyRequest,
    artifact: &CompiledDomainPolicyArtifact,
    target: PolicyDeliveryTarget,
    delivery_id: PolicyDeliveryId,
    attempt_id: PolicyDeliveryAttemptId,
    audit_reference_ids: Vec<PolicyAuditReferenceId>,
) -> Result<PolicyControlDeliveryHandoffReport, EventingError> {
    validate_request_delivery_binding(request, artifact, &target)?;
    queue_policy_control_delivery_handoff(
        artifact,
        target,
        delivery_id,
        attempt_id,
        audit_reference_ids,
    )
}

fn validate_request_delivery_binding(
    request: &ChildPolicyRequest,
    artifact: &CompiledDomainPolicyArtifact,
    target: &PolicyDeliveryTarget,
) -> Result<(), EventingError> {
    validate_child_policy_request(request)?;
    if !matches!(
        request.status,
        PolicyRequestStatus::Approved | PolicyRequestStatus::Modified
    ) {
        return Err(EventingError::InvalidValue {
            field: "policy_delivery.request_status",
            value: format!(
                "approved-or-modified-required:{}",
                request.status.as_protocol_str()
            ),
        });
    }

    if artifact.household_id != request.household_id {
        return Err(EventingError::InvalidValue {
            field: "policy_delivery.household_id",
            value: "request-artifact-mismatch".to_string(),
        });
    }
    if artifact.policy_version != request.policy_version {
        return Err(EventingError::InvalidValue {
            field: "policy_delivery.policy_version",
            value: "request-artifact-mismatch".to_string(),
        });
    }
    if artifact.source_document_id != request.source_document_id {
        return Err(EventingError::InvalidValue {
            field: "policy_delivery.source_document_id",
            value: "request-artifact-mismatch".to_string(),
        });
    }
    if target.child_profile_id != request.child_profile_id {
        return Err(EventingError::InvalidValue {
            field: "policy_delivery.target.child_profile_id",
            value: "request-target-mismatch".to_string(),
        });
    }
    let Some(request_device_id) = request.device_id.as_ref() else {
        return Err(EventingError::InvalidValue {
            field: "policy_delivery.target.device_id",
            value: "request-device-required".to_string(),
        });
    };
    if target.device_id != *request_device_id {
        return Err(EventingError::InvalidValue {
            field: "policy_delivery.target.device_id",
            value: "request-target-mismatch".to_string(),
        });
    }
    if target.domain != artifact.domain {
        return Err(EventingError::InvalidValue {
            field: "policy_delivery.target.domain",
            value: "artifact-target-mismatch".to_string(),
        });
    }

    Ok(())
}

/// Applies child-policy delivery transitions without execution authority.
///
/// Receipt-required states are downgraded to `ManualRequired`; the concrete
/// adapter boundary is intentionally not exported from this crate, so a
/// caller cannot manufacture an execution receipt and activate delivery.
pub fn apply_policy_control_delivery_handoff(
    current: &PolicyDeliveryRecord,
    transition: PolicyDeliveryTransition,
) -> Result<PolicyControlDeliveryApplyReport, EventingError> {
    let transition = fail_closed_receipt_required_transition(transition)?;
    let outcome = apply_policy_delivery_transition_without_execution_receipt(current, transition)?;
    Ok(PolicyControlDeliveryApplyReport {
        delivery: outcome.clone().into_record(),
        outcome,
    })
}

fn fail_closed_receipt_required_transition(
    mut transition: PolicyDeliveryTransition,
) -> Result<PolicyDeliveryTransition, EventingError> {
    if matches!(
        transition.state,
        PolicyDeliveryState::Acknowledged | PolicyDeliveryState::Applied
    ) {
        transition.state = PolicyDeliveryState::ManualRequired;
        transition.reason_code = Some(PolicyReasonCode::parse(TRUSTED_ADAPTER_REQUIRED_REASON)?);
        transition.superseded_by_policy_version = None;
        transition.rollback_reference_state = None;
    }
    Ok(transition)
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PolicyControlDeliveryApplyReport {
    pub delivery: PolicyDeliveryRecord,
    pub outcome: PolicyDeliveryApplyOutcome,
}
