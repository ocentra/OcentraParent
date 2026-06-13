#![forbid(unsafe_code)]

//! Setup, install, and provisioning ownership boundary.
//!
//! This crate owns install journey state, pairing readiness, permission
//! onboarding, recovery, and provisioning contracts. Binary updater mechanics
//! remain in the updater crate.

use ocentra_eventing::{
    AggregateKey, DomainEvent, EventContract, EventType, EventingError, IdempotencyKey,
    SchemaVersion,
};
use ocentra_family_identity_core::{DeviceOwnershipScope, HouseholdMembership};
use serde::{Deserialize, Serialize};

pub const CRATE_NAME: &str = "ocentra-provisioning-core";
const PROVISIONING_SCHEMA_VERSION: u16 = 1;
const PROVISIONING_READINESS_EVALUATED_EVENT_TYPE: &str = "provisioning.readiness.evaluated";
const PROVISIONING_ACTION_PLANNED_EVENT_TYPE: &str = "provisioning.action.planned";
const PROVISIONING_IDEMPOTENCY_SEPARATOR: &str = ":";
const PROVISIONING_ACTION_PREFIX: &str = "provisioning-action:";
const ERROR_PROVISIONING_ACTION_ID: &str = "provisioning action id";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ParentDeviceRegistrationState {
    #[serde(rename = "registered")]
    Registered,
    #[serde(rename = "missing")]
    Missing,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RequiredPermissionState {
    #[serde(rename = "granted")]
    Granted,
    #[serde(rename = "missing")]
    Missing,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PairingTokenState {
    #[serde(rename = "valid")]
    Valid,
    #[serde(rename = "missing-or-expired")]
    MissingOrExpired,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RecoveryState {
    #[serde(rename = "normal")]
    Normal,
    #[serde(rename = "recovery-required")]
    RecoveryRequired,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ChildRuntimeReadinessState {
    #[serde(rename = "ready")]
    Ready,
    #[serde(rename = "not-ready")]
    NotReady,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ProvisioningManualStepState {
    #[serde(rename = "required")]
    Required,
    #[serde(rename = "not-required")]
    NotRequired,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ProvisioningRecoveryAction {
    #[serde(rename = "continue")]
    Continue,
    #[serde(rename = "refresh-pairing-token")]
    RefreshPairingToken,
    #[serde(rename = "re-register-parent-device")]
    ReRegisterParentDevice,
    #[serde(rename = "request-missing-permissions")]
    RequestMissingPermissions,
    #[serde(rename = "resolve-recovery")]
    ResolveRecovery,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ProvisioningChildRuntimeStartAction {
    #[serde(rename = "start")]
    Start,
    #[serde(rename = "do-not-start")]
    DoNotStart,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ProvisioningAuditState {
    #[serde(rename = "record")]
    Record,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProvisioningReadinessInput {
    pub household_membership: HouseholdMembership,
    pub parent_device_registration_state: ParentDeviceRegistrationState,
    pub child_device_ownership_scope: DeviceOwnershipScope,
    pub required_permission_state: RequiredPermissionState,
    pub pairing_token_state: PairingTokenState,
    pub recovery_state: RecoveryState,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProvisioningReadinessDecision {
    pub child_runtime_readiness_state: ChildRuntimeReadinessState,
    pub manual_step_state: ProvisioningManualStepState,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProvisioningActionPlan {
    pub child_runtime_start_action: ProvisioningChildRuntimeStartAction,
    pub recovery_action: ProvisioningRecoveryAction,
    pub audit_state: ProvisioningAuditState,
}

macro_rules! provisioning_text_id {
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

provisioning_text_id!(
    ProvisioningReadinessEvaluationId,
    "provisioning.readiness_evaluation_id"
);
provisioning_text_id!(ProvisioningActionPlanId, "provisioning.action_plan_id");
provisioning_text_id!(ProvisioningAggregateId, "provisioning.aggregate_id");

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProvisioningReadinessEvaluatedEvent {
    pub aggregate_id: ProvisioningAggregateId,
    pub evaluation_id: ProvisioningReadinessEvaluationId,
    pub input: ProvisioningReadinessInput,
    pub decision: ProvisioningReadinessDecision,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProvisioningActionPlannedEvent {
    pub aggregate_id: ProvisioningAggregateId,
    pub action_plan_id: ProvisioningActionPlanId,
    pub source_evaluation_id: ProvisioningReadinessEvaluationId,
    pub action_plan: ProvisioningActionPlan,
}

impl DomainEvent for ProvisioningReadinessEvaluatedEvent {
    fn contract(&self) -> Result<EventContract, EventingError> {
        provisioning_event_contract(PROVISIONING_READINESS_EVALUATED_EVENT_TYPE)
    }

    fn aggregate_key(&self) -> Result<AggregateKey, EventingError> {
        AggregateKey::parse(self.aggregate_id.as_str())
    }

    fn idempotency_key(&self) -> Result<IdempotencyKey, EventingError> {
        provisioning_idempotency_key(
            PROVISIONING_READINESS_EVALUATED_EVENT_TYPE,
            &self.evaluation_id,
        )
    }
}

impl DomainEvent for ProvisioningActionPlannedEvent {
    fn contract(&self) -> Result<EventContract, EventingError> {
        provisioning_event_contract(PROVISIONING_ACTION_PLANNED_EVENT_TYPE)
    }

    fn aggregate_key(&self) -> Result<AggregateKey, EventingError> {
        AggregateKey::parse(self.aggregate_id.as_str())
    }

    fn idempotency_key(&self) -> Result<IdempotencyKey, EventingError> {
        provisioning_idempotency_key(PROVISIONING_ACTION_PLANNED_EVENT_TYPE, &self.action_plan_id)
    }
}

pub fn evaluate_provisioning_readiness(
    input: ProvisioningReadinessInput,
) -> ProvisioningReadinessDecision {
    let ready_for_child_runtime = input.household_membership == HouseholdMembership::Member
        && input.parent_device_registration_state == ParentDeviceRegistrationState::Registered
        && input.child_device_ownership_scope == DeviceOwnershipScope::ChildProfileDevice
        && input.required_permission_state == RequiredPermissionState::Granted
        && input.pairing_token_state == PairingTokenState::Valid
        && input.recovery_state == RecoveryState::Normal;

    ProvisioningReadinessDecision {
        child_runtime_readiness_state: if ready_for_child_runtime {
            ChildRuntimeReadinessState::Ready
        } else {
            ChildRuntimeReadinessState::NotReady
        },
        manual_step_state: if ready_for_child_runtime {
            ProvisioningManualStepState::NotRequired
        } else {
            ProvisioningManualStepState::Required
        },
    }
}

pub fn provisioning_readiness_evaluated_event(
    aggregate_id: ProvisioningAggregateId,
    evaluation_id: ProvisioningReadinessEvaluationId,
    input: ProvisioningReadinessInput,
) -> ProvisioningReadinessEvaluatedEvent {
    ProvisioningReadinessEvaluatedEvent {
        aggregate_id,
        evaluation_id,
        input,
        decision: evaluate_provisioning_readiness(input),
    }
}

pub fn plan_provisioning_actions(input: ProvisioningReadinessInput) -> ProvisioningActionPlan {
    let decision = evaluate_provisioning_readiness(input);
    let recovery_action = if input.recovery_state == RecoveryState::RecoveryRequired {
        ProvisioningRecoveryAction::ResolveRecovery
    } else if input.parent_device_registration_state == ParentDeviceRegistrationState::Missing {
        ProvisioningRecoveryAction::ReRegisterParentDevice
    } else if input.required_permission_state == RequiredPermissionState::Missing {
        ProvisioningRecoveryAction::RequestMissingPermissions
    } else if input.pairing_token_state == PairingTokenState::MissingOrExpired {
        ProvisioningRecoveryAction::RefreshPairingToken
    } else {
        ProvisioningRecoveryAction::Continue
    };

    ProvisioningActionPlan {
        child_runtime_start_action: if decision.child_runtime_readiness_state
            == ChildRuntimeReadinessState::Ready
        {
            ProvisioningChildRuntimeStartAction::Start
        } else {
            ProvisioningChildRuntimeStartAction::DoNotStart
        },
        recovery_action,
        audit_state: ProvisioningAuditState::Record,
    }
}

pub fn provisioning_action_planned_event(
    event: ProvisioningReadinessEvaluatedEvent,
) -> ProvisioningActionPlannedEvent {
    ProvisioningActionPlannedEvent {
        aggregate_id: event.aggregate_id,
        action_plan_id: ProvisioningActionPlanId::parse(provisioning_action_ref(
            &event.evaluation_id,
        ))
        .expect(ERROR_PROVISIONING_ACTION_ID),
        source_evaluation_id: event.evaluation_id,
        action_plan: plan_provisioning_actions(event.input),
    }
}

fn provisioning_event_contract(event_type: &str) -> Result<EventContract, EventingError> {
    Ok(EventContract::new(
        EventType::parse(event_type)?,
        SchemaVersion::new(PROVISIONING_SCHEMA_VERSION)?,
    ))
}

fn provisioning_idempotency_key(
    event_type: &str,
    unique_ref: impl std::fmt::Display,
) -> Result<IdempotencyKey, EventingError> {
    IdempotencyKey::parse(format!(
        "{}{}{}",
        event_type, PROVISIONING_IDEMPOTENCY_SEPARATOR, unique_ref
    ))
}

fn provisioning_action_ref(evaluation_id: &ProvisioningReadinessEvaluationId) -> String {
    let mut value = String::from(PROVISIONING_ACTION_PREFIX);
    value.push_str(evaluation_id.as_str());
    value
}
