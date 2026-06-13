use ocentra_policy_control_core::ParentAuthorityState;
use ocentra_child_enforcement_core::{
    evaluate_enforcement_action, EnforcementActionInput, EnforcementActionMode,
    EnforcementAdapterExecutionState, EnforcementAdapterState, EnforcementAuditRecordState,
    EnforcementIdempotencyState, EnforcementRollbackRequirementState, EnforcementRollbackState,
};

#[test]
fn execute_mode_requires_policy_adapter_and_rollback_boundary() {
    let decision = evaluate_enforcement_action(EnforcementActionInput {
        mode: EnforcementActionMode::Execute,
        policy_authority_state: ParentAuthorityState::Authorized,
        adapter_state: EnforcementAdapterState::Available,
        rollback_state: EnforcementRollbackState::Available,
        idempotency_state: EnforcementIdempotencyState::NewAction,
    });

    assert_eq!(
        decision.adapter_execution_state,
        EnforcementAdapterExecutionState::Execute
    );
    assert_eq!(decision.audit_record_state, EnforcementAuditRecordState::Record);
    assert_eq!(
        decision.rollback_requirement_state,
        EnforcementRollbackRequirementState::NotRequired
    );
}

#[test]
fn dry_run_never_executes_adapter() {
    let decision = evaluate_enforcement_action(EnforcementActionInput {
        mode: EnforcementActionMode::DryRun,
        policy_authority_state: ParentAuthorityState::Authorized,
        adapter_state: EnforcementAdapterState::Available,
        rollback_state: EnforcementRollbackState::Available,
        idempotency_state: EnforcementIdempotencyState::NewAction,
    });

    assert_eq!(
        decision.adapter_execution_state,
        EnforcementAdapterExecutionState::DoNotExecute
    );
    assert_eq!(decision.audit_record_state, EnforcementAuditRecordState::Record);
}

#[test]
fn execute_mode_requires_rollback_before_adapter_dispatch() {
    let decision = evaluate_enforcement_action(EnforcementActionInput {
        mode: EnforcementActionMode::Execute,
        policy_authority_state: ParentAuthorityState::Authorized,
        adapter_state: EnforcementAdapterState::Available,
        rollback_state: EnforcementRollbackState::Missing,
        idempotency_state: EnforcementIdempotencyState::NewAction,
    });

    assert_eq!(
        decision.adapter_execution_state,
        EnforcementAdapterExecutionState::DoNotExecute
    );
    assert_eq!(
        decision.rollback_requirement_state,
        EnforcementRollbackRequirementState::RequiredBeforeExecute
    );
}

#[test]
fn execute_mode_requires_parent_policy_authority() {
    let decision = evaluate_enforcement_action(EnforcementActionInput {
        mode: EnforcementActionMode::Execute,
        policy_authority_state: ParentAuthorityState::Unauthorized,
        adapter_state: EnforcementAdapterState::Available,
        rollback_state: EnforcementRollbackState::Available,
        idempotency_state: EnforcementIdempotencyState::NewAction,
    });

    assert_eq!(
        decision.adapter_execution_state,
        EnforcementAdapterExecutionState::DoNotExecute
    );
    assert_eq!(decision.audit_record_state, EnforcementAuditRecordState::Record);
    assert_eq!(
        decision.rollback_requirement_state,
        EnforcementRollbackRequirementState::NotRequired
    );
}

#[test]
fn execute_mode_does_not_dispatch_when_adapter_is_unavailable() {
    let decision = evaluate_enforcement_action(EnforcementActionInput {
        mode: EnforcementActionMode::Execute,
        policy_authority_state: ParentAuthorityState::Authorized,
        adapter_state: EnforcementAdapterState::Unavailable,
        rollback_state: EnforcementRollbackState::Available,
        idempotency_state: EnforcementIdempotencyState::NewAction,
    });

    assert_eq!(
        decision.adapter_execution_state,
        EnforcementAdapterExecutionState::DoNotExecute
    );
    assert_eq!(decision.audit_record_state, EnforcementAuditRecordState::Record);
    assert_eq!(
        decision.rollback_requirement_state,
        EnforcementRollbackRequirementState::NotRequired
    );
}

#[test]
fn already_applied_action_records_audit_without_reexecuting_adapter() {
    let decision = evaluate_enforcement_action(EnforcementActionInput {
        mode: EnforcementActionMode::Execute,
        policy_authority_state: ParentAuthorityState::Authorized,
        adapter_state: EnforcementAdapterState::Available,
        rollback_state: EnforcementRollbackState::Available,
        idempotency_state: EnforcementIdempotencyState::AlreadyApplied,
    });

    assert_eq!(
        decision.adapter_execution_state,
        EnforcementAdapterExecutionState::DoNotExecute
    );
    assert_eq!(decision.audit_record_state, EnforcementAuditRecordState::Record);
    assert_eq!(
        decision.rollback_requirement_state,
        EnforcementRollbackRequirementState::NotRequired
    );
}
