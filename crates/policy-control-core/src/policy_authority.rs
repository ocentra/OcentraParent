#![forbid(unsafe_code)]
//! Cross-domain policy-control ownership.
//!
//! This crate owns generic policy decision gating that feature crates can
//! consume before child-side enforcement. Feature domains still own their
//! evidence interpretation; enforcement crates own adapter execution.

use ocentra_eventing::envelope::{DomainEvent, EventContract};
use ocentra_eventing::error::EventingError;
use ocentra_eventing::ids::{AggregateKey, EventType, IdempotencyKey, SchemaVersion};
use serde::{Deserialize, Serialize};

pub type EvidenceReferenceState = ocentra_evidence::EvidenceReferenceState;

pub const CRATE_NAME: &str = "ocentra-policy-control-core";
const POLICY_CONTROL_SCHEMA_VERSION: u16 = 1;
const POLICY_EVALUATION_REQUESTED_EVENT_TYPE: &str = "policy-control.evaluation.requested";
const POLICY_DECISION_RESOLVED_EVENT_TYPE: &str = "policy-control.decision.resolved";
const POLICY_CONTROL_IDEMPOTENCY_SEPARATOR: &str = ":";
const POLICY_CONTROL_DECISION_PREFIX: &str = "policy-control-decision:";
const POLICY_ACTION_AUTHORIZATION_STATES: [PolicyActionAuthorizationState; 2] = [
    PolicyActionAuthorizationState::Blocked,
    PolicyActionAuthorizationState::Authorized,
];
const POLICY_ENFORCEMENT_EXECUTION_STATES: [PolicyEnforcementExecutionState; 2] = [
    PolicyEnforcementExecutionState::MustNotExecute,
    PolicyEnforcementExecutionState::MayExecute,
];
const POLICY_MANUAL_REVIEW_STATES: [PolicyManualReviewState; 2] = [
    PolicyManualReviewState::Required,
    PolicyManualReviewState::NotRequired,
];

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PolicyDecisionMode {
    #[serde(rename = "observe-only")]
    ObserveOnly,
    #[serde(rename = "preview")]
    Preview,
    #[serde(rename = "enforce")]
    Enforce,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ParentAuthorityState {
    #[serde(rename = "authorized")]
    Authorized,
    #[serde(rename = "unauthorized")]
    Unauthorized,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AiResultAuthorityState {
    #[serde(rename = "evidence-only")]
    EvidenceOnly,
    #[serde(rename = "claims-authority")]
    ClaimsAuthority,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PolicyActionAuthorizationState {
    #[serde(rename = "authorized")]
    Authorized,
    #[serde(rename = "blocked")]
    Blocked,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PolicyEnforcementExecutionState {
    #[serde(rename = "may-execute")]
    MayExecute,
    #[serde(rename = "must-not-execute")]
    MustNotExecute,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PolicyManualReviewState {
    #[serde(rename = "required")]
    Required,
    #[serde(rename = "not-required")]
    NotRequired,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PolicyConflictState {
    #[serde(rename = "no-conflict")]
    NoConflict,
    #[serde(rename = "conflict")]
    Conflict,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PolicyDecisionSource {
    #[serde(rename = "parent-policy")]
    ParentPolicy,
    #[serde(rename = "child-default")]
    ChildDefault,
    #[serde(rename = "ai-evidence")]
    AiEvidence,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PolicyConflictResolutionState {
    #[serde(rename = "use-parent-policy")]
    UseParentPolicy,
    #[serde(rename = "observe-only")]
    ObserveOnly,
    #[serde(rename = "manual-review")]
    ManualReview,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct PolicyControlInput {
    pub mode: PolicyDecisionMode,
    pub parent_authority_state: ParentAuthorityState,
    pub evidence_reference_state: EvidenceReferenceState,
    pub ai_result_authority_state: AiResultAuthorityState,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct PolicyControlDecision {
    pub action_authorization_state: PolicyActionAuthorizationState,
    pub enforcement_execution_state: PolicyEnforcementExecutionState,
    pub manual_review_state: PolicyManualReviewState,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct PolicyConflictInput {
    pub parent_authority_state: ParentAuthorityState,
    pub conflict_state: PolicyConflictState,
    pub requested_source: PolicyDecisionSource,
    pub evidence_reference_state: EvidenceReferenceState,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct PolicyConflictDecision {
    pub resolution_state: PolicyConflictResolutionState,
    pub manual_review_state: PolicyManualReviewState,
}

macro_rules! policy_control_text_id {
    ($name:ident, $field:expr) => {
        #[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
        #[serde(try_from = "String", into = "String")]
        pub struct $name(String);

        impl $name {
            pub fn parse(value: impl Into<String>) -> Result<Self, EventingError> {
                parse_non_empty_text_id(value, $field).map(Self)
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

policy_control_text_id!(PolicyControlRequestId, "policy_control.request_id");
policy_control_text_id!(PolicyControlDecisionId, "policy_control.decision_id");
policy_control_text_id!(PolicyControlAggregateId, "policy_control.aggregate_id");

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PolicyEvaluationRequestedEvent {
    pub aggregate_id: PolicyControlAggregateId,
    pub request_id: PolicyControlRequestId,
    pub input: PolicyControlInput,
    pub conflict_input: PolicyConflictInput,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PolicyDecisionResolvedEvent {
    pub aggregate_id: PolicyControlAggregateId,
    pub decision_id: PolicyControlDecisionId,
    pub source_request_id: PolicyControlRequestId,
    pub decision: PolicyControlDecision,
    pub conflict_decision: PolicyConflictDecision,
}

impl DomainEvent for PolicyEvaluationRequestedEvent {
    fn contract(&self) -> Result<EventContract, EventingError> {
        policy_control_event_contract(POLICY_EVALUATION_REQUESTED_EVENT_TYPE)
    }

    fn aggregate_key(&self) -> Result<AggregateKey, EventingError> {
        AggregateKey::parse(self.aggregate_id.as_str())
    }

    fn idempotency_key(&self) -> Result<IdempotencyKey, EventingError> {
        policy_control_idempotency_key(POLICY_EVALUATION_REQUESTED_EVENT_TYPE, &self.request_id)
    }
}

impl DomainEvent for PolicyDecisionResolvedEvent {
    fn contract(&self) -> Result<EventContract, EventingError> {
        policy_control_event_contract(POLICY_DECISION_RESOLVED_EVENT_TYPE)
    }

    fn aggregate_key(&self) -> Result<AggregateKey, EventingError> {
        AggregateKey::parse(self.aggregate_id.as_str())
    }

    fn idempotency_key(&self) -> Result<IdempotencyKey, EventingError> {
        policy_control_idempotency_key(POLICY_DECISION_RESOLVED_EVENT_TYPE, &self.decision_id)
    }
}

pub fn evaluate_policy_control(input: PolicyControlInput) -> PolicyControlDecision {
    let policy_can_authorize_action = input.parent_authority_state
        == ParentAuthorityState::Authorized
        && input.evidence_reference_state == EvidenceReferenceState::Stable
        && input.ai_result_authority_state == AiResultAuthorityState::EvidenceOnly;
    let enforcement_may_execute =
        policy_can_authorize_action && input.mode == PolicyDecisionMode::Enforce;

    PolicyControlDecision {
        action_authorization_state: POLICY_ACTION_AUTHORIZATION_STATES
            [usize::from(policy_can_authorize_action)],
        enforcement_execution_state: POLICY_ENFORCEMENT_EXECUTION_STATES
            [usize::from(enforcement_may_execute)],
        manual_review_state: POLICY_MANUAL_REVIEW_STATES[usize::from(policy_can_authorize_action)],
    }
}

pub fn resolve_policy_evaluation_request(
    event: &PolicyEvaluationRequestedEvent,
) -> PolicyDecisionResolvedEvent {
    PolicyDecisionResolvedEvent {
        aggregate_id: event.aggregate_id.clone(),
        decision_id: PolicyControlDecisionId(policy_control_decision_ref(&event.request_id)),
        source_request_id: event.request_id.clone(),
        decision: evaluate_policy_control(event.input),
        conflict_decision: resolve_policy_conflict(event.conflict_input),
    }
}

pub fn resolve_policy_conflict(input: PolicyConflictInput) -> PolicyConflictDecision {
    let has_stable_evidence = input.evidence_reference_state == EvidenceReferenceState::Stable;
    let can_use_parent_policy = input.conflict_state == PolicyConflictState::NoConflict
        && input.parent_authority_state == ParentAuthorityState::Authorized
        && input.requested_source == PolicyDecisionSource::ParentPolicy;
    let uses_ai_evidence = input.requested_source == PolicyDecisionSource::AiEvidence;
    let resolution_state = match (has_stable_evidence, can_use_parent_policy, uses_ai_evidence) {
        (false, _, _) | (_, _, true) => PolicyConflictResolutionState::ManualReview,
        (true, true, false) => PolicyConflictResolutionState::UseParentPolicy,
        _ => PolicyConflictResolutionState::ObserveOnly,
    };

    PolicyConflictDecision {
        resolution_state,
        manual_review_state: POLICY_MANUAL_REVIEW_STATES
            [usize::from(resolution_state != PolicyConflictResolutionState::ManualReview)],
    }
}

fn policy_control_event_contract(event_type: &str) -> Result<EventContract, EventingError> {
    Ok(EventContract::new(
        EventType::parse(event_type)?,
        SchemaVersion::new(POLICY_CONTROL_SCHEMA_VERSION)?,
    ))
}

fn policy_control_idempotency_key(
    event_type: &str,
    unique_ref: impl std::fmt::Display,
) -> Result<IdempotencyKey, EventingError> {
    IdempotencyKey::parse(format!(
        "{}{}{}",
        event_type, POLICY_CONTROL_IDEMPOTENCY_SEPARATOR, unique_ref
    ))
}

fn policy_control_decision_ref(request_id: &PolicyControlRequestId) -> String {
    let mut value = String::from(POLICY_CONTROL_DECISION_PREFIX);
    value.push_str(request_id.as_str());
    value
}

fn parse_non_empty_text_id(
    value: impl Into<String>,
    field: &'static str,
) -> Result<String, EventingError> {
    let value = value.into();
    if value.trim().is_empty() {
        return Err(EventingError::EmptyValue { field });
    }
    Ok(value)
}
