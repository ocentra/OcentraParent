use ocentra_child_enforcement_core::enforcement_action::{
    evaluate_enforcement_action, EnforcementActionDecision, EnforcementActionInput,
    EnforcementAdapterExecutionState,
};
use ocentra_entitlement_core::entitlement_access::{
    evaluate_entitlement_capability, EntitlementCapabilityAccessState, EntitlementCapabilityInput,
    EntitlementDecision,
};
use ocentra_eventing::envelope::{DomainEvent, EventContract};
use ocentra_eventing::error::EventingError;
use ocentra_eventing::ids::{AggregateKey, EventType, IdempotencyKey, SchemaVersion};
use ocentra_family_identity_core::family_identity::{
    authorize_child_device_scope, DeviceScopeAuthorizationState, DeviceScopeDecision,
    DeviceScopeInput,
};
use ocentra_provisioning_core::provisioning_install::{
    evaluate_provisioning_readiness, ChildRuntimeReadinessState, ProvisioningReadinessDecision,
    ProvisioningReadinessInput,
};
use ocentra_remote_access_core::remote_access_session::{
    evaluate_remote_access_session, RemoteAccessSessionAuthorizationState,
    RemoteAccessSessionDecision, RemoteAccessSessionRequest,
};
use ocentra_storage_custody_core::storage_custody::{
    evaluate_storage_custody, StorageCustodyDecision, StorageCustodyInput,
};
use serde::{Deserialize, Serialize};

const CHILD_RUNTIME_SCHEMA_VERSION: u16 = 1;
pub const CHILD_RUNTIME_PREFLIGHT_REQUESTED_EVENT_TYPE: &str = "child-runtime.preflight.requested";
pub const CHILD_RUNTIME_PREFLIGHT_DECISION_RECORDED_EVENT_TYPE: &str =
    "child-runtime.preflight-decision.recorded";
const CHILD_RUNTIME_IDEMPOTENCY_SEPARATOR: &str = ":";
const CHILD_RUNTIME_PREFLIGHT_DECISION_PREFIX: &str = "child-runtime-preflight-decision:";

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ChildRuntimeStartState {
    #[serde(rename = "allowed")]
    Allowed,
    #[serde(rename = "blocked")]
    Blocked,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ChildRuntimeManualReviewState {
    #[serde(rename = "required")]
    Required,
    #[serde(rename = "not-required")]
    NotRequired,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChildRuntimePreflightInput {
    pub device_scope_input: DeviceScopeInput,
    pub provisioning_input: ProvisioningReadinessInput,
    pub entitlement_input: EntitlementCapabilityInput,
    pub storage_custody_input: StorageCustodyInput,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChildRuntimePreflightDecision {
    pub device_scope_decision: DeviceScopeDecision,
    pub provisioning_decision: ProvisioningReadinessDecision,
    pub entitlement_decision: EntitlementDecision,
    pub storage_custody_decision: StorageCustodyDecision,
    pub runtime_start_state: ChildRuntimeStartState,
    pub manual_review_state: ChildRuntimeManualReviewState,
}

macro_rules! child_runtime_text_id {
    ($name:ident, $field:literal) => {
        #[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
        #[serde(try_from = "String", into = "String")]
        pub struct $name(String);

        impl $name {
            pub fn parse(value: impl Into<String>) -> Result<Self, EventingError> {
                parse_child_runtime_text_id($field, value).map(Self)
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
    };
}

child_runtime_text_id!(
    ChildRuntimePreflightRequestId,
    "child_runtime.preflight_request_id"
);
child_runtime_text_id!(
    ChildRuntimePreflightDecisionId,
    "child_runtime.preflight_decision_id"
);
child_runtime_text_id!(ChildRuntimeAggregateId, "child_runtime.aggregate_id");

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChildRuntimePreflightRequestedEvent {
    pub aggregate_id: ChildRuntimeAggregateId,
    pub request_id: ChildRuntimePreflightRequestId,
    pub input: ChildRuntimePreflightInput,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChildRuntimePreflightDecisionRecordedEvent {
    pub aggregate_id: ChildRuntimeAggregateId,
    pub decision_id: ChildRuntimePreflightDecisionId,
    pub source_request_id: ChildRuntimePreflightRequestId,
    pub decision: ChildRuntimePreflightDecision,
}

impl DomainEvent for ChildRuntimePreflightRequestedEvent {
    fn contract(&self) -> Result<EventContract, EventingError> {
        child_runtime_event_contract(CHILD_RUNTIME_PREFLIGHT_REQUESTED_EVENT_TYPE)
    }

    fn aggregate_key(&self) -> Result<AggregateKey, EventingError> {
        AggregateKey::parse(self.aggregate_id.as_str())
    }

    fn idempotency_key(&self) -> Result<IdempotencyKey, EventingError> {
        child_runtime_idempotency_key(
            CHILD_RUNTIME_PREFLIGHT_REQUESTED_EVENT_TYPE,
            self.request_id.as_str(),
        )
    }
}

impl DomainEvent for ChildRuntimePreflightDecisionRecordedEvent {
    fn contract(&self) -> Result<EventContract, EventingError> {
        child_runtime_event_contract(CHILD_RUNTIME_PREFLIGHT_DECISION_RECORDED_EVENT_TYPE)
    }

    fn aggregate_key(&self) -> Result<AggregateKey, EventingError> {
        AggregateKey::parse(self.aggregate_id.as_str())
    }

    fn idempotency_key(&self) -> Result<IdempotencyKey, EventingError> {
        child_runtime_idempotency_key(
            CHILD_RUNTIME_PREFLIGHT_DECISION_RECORDED_EVENT_TYPE,
            self.decision_id.as_str(),
        )
    }
}

pub fn evaluate_child_runtime_preflight(
    input: ChildRuntimePreflightInput,
) -> ChildRuntimePreflightDecision {
    let device_scope_decision = authorize_child_device_scope(input.device_scope_input);
    let provisioning_decision = evaluate_provisioning_readiness(input.provisioning_input);
    let entitlement_decision = evaluate_entitlement_capability(input.entitlement_input);
    let storage_custody_decision = evaluate_storage_custody(input.storage_custody_input);

    let runtime_allowed = device_scope_decision.authorization_state
        == DeviceScopeAuthorizationState::Authorized
        && provisioning_decision.child_runtime_readiness_state == ChildRuntimeReadinessState::Ready
        && entitlement_decision.access_state == EntitlementCapabilityAccessState::Allowed;
    let manual_review_required =
        provisioning_decision.child_runtime_readiness_state != ChildRuntimeReadinessState::Ready;

    ChildRuntimePreflightDecision {
        device_scope_decision,
        provisioning_decision,
        entitlement_decision,
        storage_custody_decision,
        runtime_start_state: if runtime_allowed {
            ChildRuntimeStartState::Allowed
        } else {
            ChildRuntimeStartState::Blocked
        },
        manual_review_state: if manual_review_required {
            ChildRuntimeManualReviewState::Required
        } else {
            ChildRuntimeManualReviewState::NotRequired
        },
    }
}

pub fn record_child_runtime_preflight_decision(
    event: &ChildRuntimePreflightRequestedEvent,
) -> Result<ChildRuntimePreflightDecisionRecordedEvent, EventingError> {
    let decision_id = ChildRuntimePreflightDecisionId::parse(format!(
        "{CHILD_RUNTIME_PREFLIGHT_DECISION_PREFIX}{}",
        event.request_id.as_str()
    ))?;

    Ok(ChildRuntimePreflightDecisionRecordedEvent {
        aggregate_id: event.aggregate_id.clone(),
        decision_id,
        source_request_id: event.request_id.clone(),
        decision: evaluate_child_runtime_preflight(event.input),
    })
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ChildRuntimeRemoteAccessDecision {
    pub session_decision: RemoteAccessSessionDecision,
    pub runtime_start_state: ChildRuntimeStartState,
}

pub fn evaluate_child_runtime_remote_access(
    request: RemoteAccessSessionRequest,
) -> ChildRuntimeRemoteAccessDecision {
    let session_decision = evaluate_remote_access_session(request);

    ChildRuntimeRemoteAccessDecision {
        runtime_start_state: if session_decision.authorization_state
            == RemoteAccessSessionAuthorizationState::Allowed
        {
            ChildRuntimeStartState::Allowed
        } else {
            ChildRuntimeStartState::Blocked
        },
        session_decision,
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ChildRuntimeEnforcementDecision {
    pub action_decision: EnforcementActionDecision,
    pub runtime_start_state: ChildRuntimeStartState,
}

pub fn evaluate_child_runtime_enforcement(
    input: EnforcementActionInput,
) -> ChildRuntimeEnforcementDecision {
    let action_decision = evaluate_enforcement_action(input);

    ChildRuntimeEnforcementDecision {
        runtime_start_state: if action_decision.adapter_execution_state
            == EnforcementAdapterExecutionState::Execute
        {
            ChildRuntimeStartState::Allowed
        } else {
            ChildRuntimeStartState::Blocked
        },
        action_decision,
    }
}

fn child_runtime_event_contract(event_type: &str) -> Result<EventContract, EventingError> {
    Ok(EventContract::new(
        EventType::parse(event_type)?,
        SchemaVersion::new(CHILD_RUNTIME_SCHEMA_VERSION)?,
    ))
}

fn child_runtime_idempotency_key(
    event_type: &str,
    unique_ref: impl std::fmt::Display,
) -> Result<IdempotencyKey, EventingError> {
    IdempotencyKey::parse(format!(
        "{}{}{}",
        event_type, CHILD_RUNTIME_IDEMPOTENCY_SEPARATOR, unique_ref
    ))
}

fn parse_child_runtime_text_id(
    field: &'static str,
    value: impl Into<String>,
) -> Result<String, EventingError> {
    let value = value.into();
    (!value.trim().is_empty())
        .then_some(value)
        .ok_or(EventingError::EmptyValue { field })
}
