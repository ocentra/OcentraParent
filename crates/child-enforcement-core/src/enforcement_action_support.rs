use ocentra_eventing::envelope::EventContract;
use ocentra_eventing::error::EventingError;
use ocentra_eventing::ids::{EventType, IdempotencyKey, SchemaVersion};

use crate::enforcement_action::{
    EnforcementActionInput, EnforcementActionMode, EnforcementAdapterExecutionState,
    EnforcementAdapterState, EnforcementIdempotencyState, EnforcementRollbackRequirementState,
    EnforcementRollbackState, CHILD_ENFORCEMENT_IDEMPOTENCY_SEPARATOR,
    CHILD_ENFORCEMENT_SCHEMA_VERSION, ENFORCEMENT_DECISION_PREFIX,
};
use crate::enforcement_action_request_id::EnforcementActionRequestId;
use ocentra_policy_control_core::policy_authority::ParentAuthorityState;

pub fn parse_enforcement_text_id(
    value: impl Into<String>,
    field: &'static str,
) -> Result<String, EventingError> {
    let value = value.into();
    if value.trim().is_empty() {
        Err(EventingError::EmptyValue { field })
    } else {
        Ok(value)
    }
}

pub fn child_enforcement_event_contract(event_type: &str) -> Result<EventContract, EventingError> {
    Ok(EventContract::new(
        EventType::parse(event_type)?,
        SchemaVersion::new(CHILD_ENFORCEMENT_SCHEMA_VERSION)?,
    ))
}

pub fn child_enforcement_idempotency_key(
    event_type: &str,
    unique_ref: impl std::fmt::Display,
) -> Result<IdempotencyKey, EventingError> {
    IdempotencyKey::parse(format!(
        "{}{}{}",
        event_type, CHILD_ENFORCEMENT_IDEMPOTENCY_SEPARATOR, unique_ref
    ))
}

pub fn enforcement_decision_ref(request_id: &EnforcementActionRequestId) -> String {
    let mut value = String::from(ENFORCEMENT_DECISION_PREFIX);
    value.push_str(request_id.as_str());
    value
}

pub fn enforcement_adapter_execution_state(
    input: &EnforcementActionInput,
) -> EnforcementAdapterExecutionState {
    if enforcement_action_can_execute(input) {
        EnforcementAdapterExecutionState::Execute
    } else {
        EnforcementAdapterExecutionState::DoNotExecute
    }
}

fn enforcement_action_can_execute(input: &EnforcementActionInput) -> bool {
    input.mode == EnforcementActionMode::Execute
        && input.policy_authority_state == ParentAuthorityState::Authorized
        && input.adapter_state == EnforcementAdapterState::Available
        && input.rollback_state == EnforcementRollbackState::Available
        && input.idempotency_state == EnforcementIdempotencyState::NewAction
}

pub fn enforcement_rollback_requirement_state(
    input: &EnforcementActionInput,
) -> EnforcementRollbackRequirementState {
    if input.mode == EnforcementActionMode::Execute
        && input.rollback_state != EnforcementRollbackState::Available
    {
        EnforcementRollbackRequirementState::RequiredBeforeExecute
    } else {
        EnforcementRollbackRequirementState::NotRequired
    }
}
