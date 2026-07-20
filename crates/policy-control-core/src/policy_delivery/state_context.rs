#![forbid(unsafe_code)]

use super::{
    policy_control, state_values, EventingError, PolicyDeliveryExecutionReceipt,
    PolicyDeliveryRecord, PolicyDeliveryState, PolicyDeliveryTransition, PolicyReasonCode,
    PolicyVersion,
};

mod reason;
mod rollback;
mod superseded;

const ROLLED_BACK_REFERENCE_STATES: &[PolicyDeliveryState] = &[
    PolicyDeliveryState::Delivered,
    PolicyDeliveryState::Acknowledged,
    PolicyDeliveryState::Applied,
    PolicyDeliveryState::PartialDomainApply,
    PolicyDeliveryState::Degraded,
    PolicyDeliveryState::Offline,
];

#[derive(Clone, Copy)]
enum ReasonRule {
    MustBeAbsent,
    Required,
}

#[derive(Clone, Copy)]
enum SupersededRule {
    MustBeAbsent,
    RequiredNewerThanCurrent,
}

#[derive(Clone, Copy)]
enum RollbackRule {
    MustBeAbsent,
    RequiredFrom(&'static [PolicyDeliveryState]),
}

#[derive(Clone, Copy)]
struct StateContextRule {
    reason: ReasonRule,
    superseded: SupersededRule,
    rollback: RollbackRule,
}

pub(super) fn assert_state_context(
    state: PolicyDeliveryState,
    reason_code: Option<&PolicyReasonCode>,
    superseded_by_policy_version: Option<PolicyVersion>,
    rollback_reference_state: Option<PolicyDeliveryState>,
    current_policy_version: PolicyVersion,
) -> Result<(), EventingError> {
    let rule = state_context_rule(state);
    validate_reason(reason_code, state, rule.reason)?;
    validate_superseded(
        superseded_by_policy_version,
        state,
        current_policy_version,
        rule.superseded,
    )?;
    validate_rollback(rollback_reference_state, state, rule.rollback)
}

pub(super) fn assert_execution_receipt(
    current: &PolicyDeliveryRecord,
    transition: &PolicyDeliveryTransition,
    receipt: Option<&PolicyDeliveryExecutionReceipt>,
) -> Result<(), EventingError> {
    super::adapter_execution::validate_policy_delivery_execution_receipt(
        current, transition, receipt,
    )
}

fn state_context_rule(state: PolicyDeliveryState) -> StateContextRule {
    match state {
        PolicyDeliveryState::Queued
        | PolicyDeliveryState::Delivering
        | PolicyDeliveryState::Delivered
        | PolicyDeliveryState::Acknowledged
        | PolicyDeliveryState::Applied => StateContextRule {
            reason: ReasonRule::MustBeAbsent,
            superseded: SupersededRule::MustBeAbsent,
            rollback: RollbackRule::MustBeAbsent,
        },
        PolicyDeliveryState::Rejected
        | PolicyDeliveryState::Degraded
        | PolicyDeliveryState::Offline
        | PolicyDeliveryState::ExpiredBeforeDelivery
        | PolicyDeliveryState::RetryScheduled
        | PolicyDeliveryState::PartialDomainApply
        | PolicyDeliveryState::BlockedByPermission
        | PolicyDeliveryState::BlockedByCapability
        | PolicyDeliveryState::ManualRequired => StateContextRule {
            reason: ReasonRule::Required,
            superseded: SupersededRule::MustBeAbsent,
            rollback: RollbackRule::MustBeAbsent,
        },
        PolicyDeliveryState::Superseded => StateContextRule {
            reason: ReasonRule::MustBeAbsent,
            superseded: SupersededRule::RequiredNewerThanCurrent,
            rollback: RollbackRule::MustBeAbsent,
        },
        PolicyDeliveryState::RolledBack => StateContextRule {
            reason: ReasonRule::Required,
            superseded: SupersededRule::MustBeAbsent,
            rollback: RollbackRule::RequiredFrom(ROLLED_BACK_REFERENCE_STATES),
        },
    }
}

fn validate_reason(
    reason_code: Option<&PolicyReasonCode>,
    state: PolicyDeliveryState,
    rule: ReasonRule,
) -> Result<(), EventingError> {
    reason::validate(reason_code, state, rule)
}

fn validate_superseded(
    superseded_by_policy_version: Option<PolicyVersion>,
    state: PolicyDeliveryState,
    current_policy_version: PolicyVersion,
    rule: SupersededRule,
) -> Result<(), EventingError> {
    superseded::validate(
        superseded_by_policy_version,
        state,
        current_policy_version,
        rule,
    )
}

fn validate_rollback(
    rollback_reference_state: Option<PolicyDeliveryState>,
    state: PolicyDeliveryState,
    rule: RollbackRule,
) -> Result<(), EventingError> {
    rollback::validate(rollback_reference_state, state, rule)
}
