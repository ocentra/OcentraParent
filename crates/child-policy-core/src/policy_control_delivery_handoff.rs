use ocentra_eventing::error::EventingError;
use ocentra_policy_control_core::policy_delivery::{
    apply_policy_delivery_transition_without_execution_receipt, queue_policy_delivery,
    PolicyDeliveryApplyOutcome, PolicyDeliveryAttemptId, PolicyDeliveryId, PolicyDeliveryRecord,
    PolicyDeliveryTarget, PolicyDeliveryTransition,
};
use ocentra_policy_control_core::policy_source::{
    CompiledDomainPolicyArtifact, PolicyAuditReferenceId,
};

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

pub fn apply_policy_control_delivery_handoff(
    current: &PolicyDeliveryRecord,
    transition: PolicyDeliveryTransition,
) -> Result<PolicyControlDeliveryApplyReport, EventingError> {
    let outcome = apply_policy_delivery_transition_without_execution_receipt(current, transition)?;
    Ok(PolicyControlDeliveryApplyReport {
        delivery: outcome.clone().into_record(),
        outcome,
    })
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PolicyControlDeliveryApplyReport {
    pub delivery: PolicyDeliveryRecord,
    pub outcome: PolicyDeliveryApplyOutcome,
}
