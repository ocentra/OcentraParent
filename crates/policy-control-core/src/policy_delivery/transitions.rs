#![forbid(unsafe_code)]

use super::{
    adapter_execution_validation, policy_control, state_values, transition_rules, validation,
    CompiledDomainPolicyArtifact, EventingError, PolicyDeliveryApplyOutcome,
    PolicyDeliveryAttemptId, PolicyDeliveryExecutionReceipt, PolicyDeliveryId,
    PolicyDeliveryRecord, PolicyDeliverySequence, PolicyDeliveryTarget, PolicyDeliveryTransition,
    POLICY_DELIVERY_INITIAL_SEQUENCE_VALUE,
};

pub(super) fn queue_policy_delivery(
    artifact: &CompiledDomainPolicyArtifact,
    target: PolicyDeliveryTarget,
    delivery_id: PolicyDeliveryId,
    attempt_id: PolicyDeliveryAttemptId,
    audit_reference_ids: Vec<super::PolicyAuditReferenceId>,
) -> Result<PolicyDeliveryRecord, EventingError> {
    let last_sequence = PolicyDeliverySequence::new(POLICY_DELIVERY_INITIAL_SEQUENCE_VALUE)?;
    let record = PolicyDeliveryRecord {
        schema_version: validation::policy_delivery_schema_version()?,
        delivery_id,
        household_id: artifact.household_id.clone(),
        policy_version: artifact.policy_version,
        source_document_id: artifact.source_document_id.clone(),
        target,
        state: super::PolicyDeliveryState::Queued,
        last_sequence,
        last_attempt_id: attempt_id,
        audit_reference_ids,
        source_audit_reference_ids: artifact.audit_reference_ids.clone(),
        source_superseded_by_policy_version: artifact.superseded_by_policy_version,
        source_rollback_ref: artifact.rollback_ref.clone(),
        reason_code: None,
        superseded_by_policy_version: None,
        rollback_reference_state: None,
        execution_receipt: None,
    };
    validation::validate_policy_delivery_record(&record)?;
    Ok(record)
}

pub(super) fn apply_policy_delivery_transition_without_execution_receipt(
    current: &PolicyDeliveryRecord,
    transition: PolicyDeliveryTransition,
) -> Result<PolicyDeliveryApplyOutcome, EventingError> {
    validate_policy_delivery_transition_application(current, &transition)?;
    adapter_execution_validation::validate_policy_delivery_execution_receipt(
        current,
        &transition,
        None,
    )?;
    apply_validated_policy_delivery_transition(current, transition, None)
}

pub(super) fn apply_policy_delivery_transition_with_execution_receipt(
    current: &PolicyDeliveryRecord,
    transition: PolicyDeliveryTransition,
    receipt: PolicyDeliveryExecutionReceipt,
) -> Result<PolicyDeliveryApplyOutcome, EventingError> {
    validate_policy_delivery_transition_application(current, &transition)?;
    adapter_execution_validation::validate_policy_delivery_execution_receipt(
        current,
        &transition,
        Some(&receipt),
    )?;
    apply_validated_policy_delivery_transition(current, transition, Some(receipt))
}

fn validate_policy_delivery_transition_application(
    current: &PolicyDeliveryRecord,
    transition: &PolicyDeliveryTransition,
) -> Result<(), EventingError> {
    validation::validate_policy_delivery_record(current)?;
    validation::validate_policy_delivery_transition(transition, current.policy_version)?;
    Ok(())
}

fn apply_validated_policy_delivery_transition(
    current: &PolicyDeliveryRecord,
    transition: PolicyDeliveryTransition,
    execution_receipt: Option<PolicyDeliveryExecutionReceipt>,
) -> Result<PolicyDeliveryApplyOutcome, EventingError> {
    match transition
        .sequence
        .value()
        .cmp(&current.last_sequence.value())
    {
        std::cmp::Ordering::Less => return Ok(PolicyDeliveryApplyOutcome::Stale(current.clone())),
        std::cmp::Ordering::Equal => {
            if transition_matches_record(current, &transition, execution_receipt.as_ref()) {
                return Ok(PolicyDeliveryApplyOutcome::Duplicate(current.clone()));
            }

            return Err(EventingError::InvalidValue {
                field: policy_control::delivery::FIELD_SEQUENCE,
                value: state_values::conflicting_replay_value(transition.sequence),
            });
        }
        std::cmp::Ordering::Greater => {}
    }

    if !transition_rules::transition_allowed(current.state, transition.state) {
        return Err(EventingError::InvalidValue {
            field: policy_control::delivery::FIELD_STATE,
            value: state_values::invalid_transition_value(current.state, transition.state),
        });
    }

    let next = PolicyDeliveryRecord {
        schema_version: validation::policy_delivery_schema_version()?,
        delivery_id: current.delivery_id.clone(),
        household_id: current.household_id.clone(),
        policy_version: current.policy_version,
        source_document_id: current.source_document_id.clone(),
        target: current.target.clone(),
        state: transition.state,
        last_sequence: transition.sequence,
        last_attempt_id: transition.attempt_id,
        audit_reference_ids: transition.audit_reference_ids,
        source_audit_reference_ids: current.source_audit_reference_ids.clone(),
        source_superseded_by_policy_version: current.source_superseded_by_policy_version,
        source_rollback_ref: current.source_rollback_ref.clone(),
        reason_code: transition.reason_code,
        superseded_by_policy_version: transition.superseded_by_policy_version,
        rollback_reference_state: transition.rollback_reference_state,
        execution_receipt,
    };
    validation::validate_policy_delivery_record(&next)?;
    Ok(PolicyDeliveryApplyOutcome::Advanced(next))
}

fn transition_matches_record(
    current: &PolicyDeliveryRecord,
    transition: &PolicyDeliveryTransition,
    execution_receipt: Option<&PolicyDeliveryExecutionReceipt>,
) -> bool {
    current.state == transition.state
        && current.last_attempt_id == transition.attempt_id
        && current.audit_reference_ids == transition.audit_reference_ids
        && current.reason_code == transition.reason_code
        && current.superseded_by_policy_version == transition.superseded_by_policy_version
        && current.rollback_reference_state == transition.rollback_reference_state
        && current.execution_receipt.as_ref() == execution_receipt
}
