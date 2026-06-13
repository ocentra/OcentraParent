#![forbid(unsafe_code)]

//! Child-side enforcement ownership boundary.
//!
//! This crate owns policy-action execution boundaries, enforcement adapter
//! orchestration, rollback/recovery state, and enforcement audit hooks. It
//! consumes protocol, eventing, evidence, and policy contracts instead of
//! defining duplicate command or event truth.

use ocentra_policy_control_core::ParentAuthorityState;

pub const CRATE_NAME: &str = "ocentra-child-enforcement-core";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EnforcementActionMode {
    ObserveOnly,
    DryRun,
    Execute,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EnforcementAdapterState {
    Available,
    Unavailable,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EnforcementRollbackState {
    Available,
    Missing,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EnforcementIdempotencyState {
    NewAction,
    AlreadyApplied,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EnforcementAdapterExecutionState {
    Execute,
    DoNotExecute,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EnforcementAuditRecordState {
    Record,
    DoNotRecord,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EnforcementRollbackRequirementState {
    RequiredBeforeExecute,
    NotRequired,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EnforcementActionInput {
    pub mode: EnforcementActionMode,
    pub policy_authority_state: ParentAuthorityState,
    pub adapter_state: EnforcementAdapterState,
    pub rollback_state: EnforcementRollbackState,
    pub idempotency_state: EnforcementIdempotencyState,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EnforcementActionDecision {
    pub adapter_execution_state: EnforcementAdapterExecutionState,
    pub audit_record_state: EnforcementAuditRecordState,
    pub rollback_requirement_state: EnforcementRollbackRequirementState,
}

pub fn evaluate_enforcement_action(input: EnforcementActionInput) -> EnforcementActionDecision {
    let execute_adapter = input.mode == EnforcementActionMode::Execute
        && input.policy_authority_state == ParentAuthorityState::Authorized
        && input.adapter_state == EnforcementAdapterState::Available
        && input.rollback_state == EnforcementRollbackState::Available
        && input.idempotency_state == EnforcementIdempotencyState::NewAction;

    EnforcementActionDecision {
        adapter_execution_state: if execute_adapter {
            EnforcementAdapterExecutionState::Execute
        } else {
            EnforcementAdapterExecutionState::DoNotExecute
        },
        audit_record_state: EnforcementAuditRecordState::Record,
        rollback_requirement_state: if input.mode == EnforcementActionMode::Execute
            && input.rollback_state != EnforcementRollbackState::Available
        {
            EnforcementRollbackRequirementState::RequiredBeforeExecute
        } else {
            EnforcementRollbackRequirementState::NotRequired
        },
    }
}
