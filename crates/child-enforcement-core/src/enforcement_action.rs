#![forbid(unsafe_code)]

//! Child-side enforcement ownership boundary.
//!
//! This crate owns policy-action execution boundaries, enforcement adapter
//! orchestration, rollback/recovery state, and enforcement audit hooks. It
//! consumes protocol, eventing, evidence, and policy contracts instead of
//! defining duplicate command or event truth.

use ocentra_eventing::{
    AggregateKey, DomainEvent, EventContract, EventType, EventingError, IdempotencyKey,
    SchemaVersion,
};
use ocentra_policy_control_core::policy_authority::ParentAuthorityState;
use serde::{Deserialize, Serialize};

pub const CRATE_NAME: &str = "ocentra-child-enforcement-core";
const CHILD_ENFORCEMENT_SCHEMA_VERSION: u16 = 1;
const ENFORCEMENT_ACTION_REQUESTED_EVENT_TYPE: &str = "child-enforcement.action.requested";
const ENFORCEMENT_ACTION_DECISION_RECORDED_EVENT_TYPE: &str =
    "child-enforcement.action-decision.recorded";
const CHILD_ENFORCEMENT_IDEMPOTENCY_SEPARATOR: &str = ":";
const ENFORCEMENT_DECISION_PREFIX: &str = "child-enforcement-decision:";
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

macro_rules! child_enforcement_text_id {
    ($name:ident, $field:expr) => {
        #[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
        #[serde(try_from = "String", into = "String")]
        pub struct $name(String);

        impl $name {
            pub fn parse(value: impl Into<String>) -> Result<Self, EventingError> {
                let value = value.into();
                if value.trim().is_empty() {
                    return Err(EventingError::EmptyValue { field: $field });
                }
                Ok(Self(value))
            }

            pub fn as_str(&self) -> &str {
                &self.0
            }
        }

        impl TryFrom<String> for $name {
            type Error = EventingError;

            fn try_from(value: String) -> Result<Self, Self::Error> {
                Self::parse(value)
            }
        }

        impl From<$name> for String {
            fn from(value: $name) -> Self {
                value.0
            }
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str(self.as_str())
            }
        }
    };
}

child_enforcement_text_id!(EnforcementActionRequestId, "child_enforcement.request_id");
child_enforcement_text_id!(EnforcementDecisionId, "child_enforcement.decision_id");
child_enforcement_text_id!(EnforcementAggregateId, "child_enforcement.aggregate_id");

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
        child_enforcement_event_contract(ENFORCEMENT_ACTION_REQUESTED_EVENT_TYPE)
    }

    fn aggregate_key(&self) -> Result<AggregateKey, EventingError> {
        AggregateKey::parse(self.aggregate_id.as_str())
    }

    fn idempotency_key(&self) -> Result<IdempotencyKey, EventingError> {
        child_enforcement_idempotency_key(ENFORCEMENT_ACTION_REQUESTED_EVENT_TYPE, &self.request_id)
    }
}

impl DomainEvent for EnforcementActionDecisionRecordedEvent {
    fn contract(&self) -> Result<EventContract, EventingError> {
        child_enforcement_event_contract(ENFORCEMENT_ACTION_DECISION_RECORDED_EVENT_TYPE)
    }

    fn aggregate_key(&self) -> Result<AggregateKey, EventingError> {
        AggregateKey::parse(self.aggregate_id.as_str())
    }

    fn idempotency_key(&self) -> Result<IdempotencyKey, EventingError> {
        child_enforcement_idempotency_key(
            ENFORCEMENT_ACTION_DECISION_RECORDED_EVENT_TYPE,
            &self.decision_id,
        )
    }
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

pub fn record_enforcement_action_decision(
    event: &EnforcementActionRequestedEvent,
) -> EnforcementActionDecisionRecordedEvent {
    EnforcementActionDecisionRecordedEvent {
        aggregate_id: event.aggregate_id.clone(),
        decision_id: EnforcementDecisionId::parse(enforcement_decision_ref(&event.request_id))
            .expect(ERROR_ENFORCEMENT_DECISION_ID),
        source_request_id: event.request_id.clone(),
        decision: evaluate_enforcement_action(event.input),
    }
}

fn child_enforcement_event_contract(event_type: &str) -> Result<EventContract, EventingError> {
    Ok(EventContract::new(
        EventType::parse(event_type)?,
        SchemaVersion::new(CHILD_ENFORCEMENT_SCHEMA_VERSION)?,
    ))
}

fn child_enforcement_idempotency_key(
    event_type: &str,
    unique_ref: impl std::fmt::Display,
) -> Result<IdempotencyKey, EventingError> {
    IdempotencyKey::parse(format!(
        "{}{}{}",
        event_type, CHILD_ENFORCEMENT_IDEMPOTENCY_SEPARATOR, unique_ref
    ))
}

fn enforcement_decision_ref(request_id: &EnforcementActionRequestId) -> String {
    let mut value = String::from(ENFORCEMENT_DECISION_PREFIX);
    value.push_str(request_id.as_str());
    value
}
