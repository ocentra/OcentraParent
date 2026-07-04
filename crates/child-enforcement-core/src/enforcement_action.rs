#![forbid(unsafe_code)]

//! Child-side enforcement ownership boundary.
//!
//! This crate owns policy-action execution boundaries, enforcement adapter
//! orchestration, rollback/recovery state, and enforcement audit hooks. It
//! consumes protocol, eventing, evidence, and policy contracts instead of
//! defining duplicate command or event truth.

use ocentra_eventing::envelope::{DomainEvent, EventContract};
use ocentra_eventing::error::EventingError;
use ocentra_eventing::expect_value::ExpectValue;
use ocentra_eventing::ids::{AggregateKey, IdempotencyKey};
use ocentra_policy_control_core::policy_authority::ParentAuthorityState;
use serde::{Deserialize, Serialize};

use crate::enforcement_action_request_id::EnforcementActionRequestId;
use crate::enforcement_action_support::{
    child_enforcement_event_contract as support_child_enforcement_event_contract,
    child_enforcement_idempotency_key as support_child_enforcement_idempotency_key,
    enforcement_adapter_execution_state as support_enforcement_adapter_execution_state,
    enforcement_decision_ref as support_enforcement_decision_ref,
    enforcement_rollback_requirement_state as support_enforcement_rollback_requirement_state,
};
use crate::enforcement_aggregate_id::EnforcementAggregateId;
use crate::enforcement_decision_id::EnforcementDecisionId;

pub const CRATE_NAME: &str = "ocentra-child-enforcement-core";
pub(crate) const CHILD_ENFORCEMENT_SCHEMA_VERSION: u16 = 1;
pub(crate) const ENFORCEMENT_ACTION_REQUESTED_EVENT_TYPE: &str =
    "child-enforcement.action.requested";
pub(crate) const ENFORCEMENT_ACTION_DECISION_RECORDED_EVENT_TYPE: &str =
    "child-enforcement.action-decision.recorded";
pub(crate) const CHILD_ENFORCEMENT_IDEMPOTENCY_SEPARATOR: &str = ":";
pub(crate) const ENFORCEMENT_DECISION_PREFIX: &str = "child-enforcement-decision:";
const ERROR_ENFORCEMENT_DECISION_ID: &str = "child enforcement decision id";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EnforcementActionMode {
    #[serde(rename = "observe-only")]
    ObserveOnly,
    #[serde(rename = "dry-run")]
    DryRun,
    #[serde(rename = "execute")]
    Execute,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EnforcementAdapterState {
    #[serde(rename = "available")]
    Available,
    #[serde(rename = "unavailable")]
    Unavailable,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EnforcementRollbackState {
    #[serde(rename = "available")]
    Available,
    #[serde(rename = "missing")]
    Missing,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EnforcementIdempotencyState {
    #[serde(rename = "new-action")]
    NewAction,
    #[serde(rename = "already-applied")]
    AlreadyApplied,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EnforcementAdapterExecutionState {
    #[serde(rename = "execute")]
    Execute,
    #[serde(rename = "do-not-execute")]
    DoNotExecute,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EnforcementAuditRecordState {
    #[serde(rename = "record")]
    Record,
    #[serde(rename = "do-not-record")]
    DoNotRecord,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EnforcementRollbackRequirementState {
    #[serde(rename = "required-before-execute")]
    RequiredBeforeExecute,
    #[serde(rename = "not-required")]
    NotRequired,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct EnforcementActionInput {
    pub mode: EnforcementActionMode,
    pub policy_authority_state: ParentAuthorityState,
    pub adapter_state: EnforcementAdapterState,
    pub rollback_state: EnforcementRollbackState,
    pub idempotency_state: EnforcementIdempotencyState,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct EnforcementActionDecision {
    pub adapter_execution_state: EnforcementAdapterExecutionState,
    pub audit_record_state: EnforcementAuditRecordState,
    pub rollback_requirement_state: EnforcementRollbackRequirementState,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EnforcementActionRequestedEvent {
    pub aggregate_id: EnforcementAggregateId,
    pub request_id: EnforcementActionRequestId,
    pub input: EnforcementActionInput,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EnforcementActionDecisionRecordedEvent {
    pub aggregate_id: EnforcementAggregateId,
    pub decision_id: EnforcementDecisionId,
    pub source_request_id: EnforcementActionRequestId,
    pub decision: EnforcementActionDecision,
}

impl DomainEvent for EnforcementActionRequestedEvent {
    fn contract(&self) -> Result<EventContract, EventingError> {
        support_child_enforcement_event_contract(ENFORCEMENT_ACTION_REQUESTED_EVENT_TYPE)
    }

    fn aggregate_key(&self) -> Result<AggregateKey, EventingError> {
        AggregateKey::parse(self.aggregate_id.as_str())
    }

    fn idempotency_key(&self) -> Result<IdempotencyKey, EventingError> {
        support_child_enforcement_idempotency_key(
            ENFORCEMENT_ACTION_REQUESTED_EVENT_TYPE,
            &self.request_id,
        )
    }
}

impl DomainEvent for EnforcementActionDecisionRecordedEvent {
    fn contract(&self) -> Result<EventContract, EventingError> {
        support_child_enforcement_event_contract(ENFORCEMENT_ACTION_DECISION_RECORDED_EVENT_TYPE)
    }

    fn aggregate_key(&self) -> Result<AggregateKey, EventingError> {
        AggregateKey::parse(self.aggregate_id.as_str())
    }

    fn idempotency_key(&self) -> Result<IdempotencyKey, EventingError> {
        support_child_enforcement_idempotency_key(
            ENFORCEMENT_ACTION_DECISION_RECORDED_EVENT_TYPE,
            &self.decision_id,
        )
    }
}

pub fn evaluate_enforcement_action(input: EnforcementActionInput) -> EnforcementActionDecision {
    EnforcementActionDecision {
        adapter_execution_state: support_enforcement_adapter_execution_state(&input),
        audit_record_state: EnforcementAuditRecordState::Record,
        rollback_requirement_state: support_enforcement_rollback_requirement_state(&input),
    }
}

pub fn record_enforcement_action_decision(
    event: &EnforcementActionRequestedEvent,
) -> EnforcementActionDecisionRecordedEvent {
    EnforcementActionDecisionRecordedEvent {
        aggregate_id: event.aggregate_id.clone(),
        decision_id: EnforcementDecisionId::parse(support_enforcement_decision_ref(
            &event.request_id,
        ))
        .expect_value(ERROR_ENFORCEMENT_DECISION_ID),
        source_request_id: event.request_id.clone(),
        decision: evaluate_enforcement_action(event.input),
    }
}
