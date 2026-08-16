use ocentra_parent_agent_core::authenticated_delivery_execution::AuthenticatedAdapterExecutionTrace;
use ocentra_policy_control_core::policy_delivery::{
    PolicyDeliveryExecutionReceipt, PolicyDeliveryId, PolicyDeliverySequence, PolicyDeliveryState,
    PolicyDeliveryTarget,
};
use ocentra_policy_control_core::policy_source::{
    ParentPolicyDocumentId, PolicyAuditReferenceId, PolicyHouseholdId, PolicyVersion,
};

#[derive(Clone, PartialEq, Eq)]
pub struct AuthenticatedPolicyReceiptContext {
    pub delivery_id: PolicyDeliveryId,
    pub household_id: PolicyHouseholdId,
    pub policy_version: PolicyVersion,
    pub source_document_id: ParentPolicyDocumentId,
    pub target: PolicyDeliveryTarget,
    pub attempt_id: ocentra_policy_control_core::policy_delivery::PolicyDeliveryAttemptId,
    pub sequence: PolicyDeliverySequence,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AuthenticatedPolicyReceiptError {
    TraceContextMismatch,
    TraceOutcomeRejected,
    TraceReferenceRejected,
}

/// Construct the policy receipt only from an adapter-owned trace returned by
/// `AuthenticatedDeliveryExecutionStore::read_trace`. The public policy
/// receipt type remains an evidence DTO; this bridge is the only production
/// path that can bind it to the trusted adapter execution identity.
pub fn policy_receipt_from_authenticated_trace(
    trace: &AuthenticatedAdapterExecutionTrace,
    context: AuthenticatedPolicyReceiptContext,
) -> Result<PolicyDeliveryExecutionReceipt, AuthenticatedPolicyReceiptError> {
    if trace.household_id() != context.household_id.as_str()
        || trace.policy_version() != context.policy_version.as_str()
        || trace.child_profile_id() != context.target.child_profile_id.as_str()
        || trace.target_device_id() != context.target.device_id.as_str()
    {
        return Err(AuthenticatedPolicyReceiptError::TraceContextMismatch);
    }
    let (state, rollback_reference_state) = if trace.adapter_status() == "ActuallyEnforced"
        && trace.adapter_result() == "ProcessTerminated"
        && trace.observed_process_id() == Some(trace.process_id())
        && trace.observed_process_name() == Some(trace.expected_process_name())
        && trace.observed_executable_path() == Some(trace.expected_executable_path())
        && trace.observed_process_start_time() == Some(trace.process_start_time())
    {
        (PolicyDeliveryState::Applied, None)
    } else if trace.rollback_state() == "RolledBack" {
        (
            PolicyDeliveryState::RolledBack,
            Some(PolicyDeliveryState::Applied),
        )
    } else {
        return Err(AuthenticatedPolicyReceiptError::TraceOutcomeRejected);
    };
    let audit_reference_id =
        PolicyAuditReferenceId::parse(format!("authenticated-adapter-trace:{}", trace.trace_id()))
            .map_err(|_error| AuthenticatedPolicyReceiptError::TraceReferenceRejected)?;
    Ok(PolicyDeliveryExecutionReceipt {
        delivery_id: context.delivery_id,
        household_id: context.household_id,
        policy_version: context.policy_version,
        source_document_id: context.source_document_id,
        target: context.target,
        attempt_id: context.attempt_id,
        sequence: context.sequence,
        state,
        audit_reference_ids: vec![audit_reference_id],
        reason_code: None,
        rollback_reference_state,
    })
}
