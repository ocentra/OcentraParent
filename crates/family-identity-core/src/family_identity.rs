#![forbid(unsafe_code)]
#![allow(clippy::panic)]

//! Family identity and device-role ownership boundary.
//!
//! This crate owns household membership, child/profile/device role contracts,
//! local authorization decisions, invite/recovery state, and device-ownership
//! checks shared by parent and child runtimes.

use ocentra_eventing::envelope::{DomainEvent, EventContract};
use ocentra_eventing::error::EventingError;
use ocentra_eventing::ids::{AggregateKey, EventType, IdempotencyKey, SchemaVersion};
use serde::{Deserialize, Serialize};

pub const CRATE_NAME: &str = "ocentra-family-identity-core";
const FAMILY_IDENTITY_SCHEMA_VERSION: u16 = 1;
const DEVICE_SCOPE_EVALUATION_REQUESTED_EVENT_TYPE: &str =
    "family-identity.device-scope-evaluation.requested";
const DEVICE_SCOPE_DECISION_RECORDED_EVENT_TYPE: &str =
    "family-identity.device-scope-decision.recorded";
const FAMILY_IDENTITY_IDEMPOTENCY_SEPARATOR: &str = ":";
const DEVICE_SCOPE_DECISION_PREFIX: &str = "family-identity-device-scope-decision:";
const ERROR_DEVICE_SCOPE_DECISION_ID: &str = "family identity device scope decision id";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FamilyActorRole {
    #[serde(rename = "parent")]
    Parent,
    #[serde(rename = "guardian")]
    Guardian,
    #[serde(rename = "observer")]
    Observer,
    #[serde(rename = "child")]
    Child,
    #[serde(rename = "child-device-agent")]
    ChildDeviceAgent,
    #[serde(rename = "support-admin")]
    SupportAdmin,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum HouseholdMembership {
    #[serde(rename = "member")]
    Member,
    #[serde(rename = "invited")]
    Invited,
    #[serde(rename = "revoked")]
    Revoked,
    #[serde(rename = "disabled")]
    Disabled,
    #[serde(rename = "external")]
    External,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ActorAccountState {
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "suspended")]
    Suspended,
    #[serde(rename = "disabled")]
    Disabled,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ChildProfileBindingState {
    #[serde(rename = "bound")]
    Bound,
    #[serde(rename = "missing")]
    Missing,
    #[serde(rename = "unassigned")]
    Unassigned,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DeviceOwnershipScope {
    #[serde(rename = "child-profile-device")]
    ChildProfileDevice,
    #[serde(rename = "parent-controller-device")]
    ParentControllerDevice,
    #[serde(rename = "other-device")]
    OtherDevice,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ChildDisclosureState {
    #[serde(rename = "disclosed")]
    Disclosed,
    #[serde(rename = "not-disclosed")]
    NotDisclosed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DeviceScopeAuthorizationState {
    #[serde(rename = "authorized")]
    Authorized,
    #[serde(rename = "rejected")]
    Rejected,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ParentAuthorityRequirementState {
    #[serde(rename = "required")]
    Required,
    #[serde(rename = "not-required")]
    NotRequired,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DeviceTrustState {
    #[serde(rename = "pending")]
    Pending,
    #[serde(rename = "trusted")]
    Trusted,
    #[serde(rename = "revoked")]
    Revoked,
    #[serde(rename = "disabled")]
    Disabled,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SessionFreshnessState {
    #[serde(rename = "fresh")]
    Fresh,
    #[serde(rename = "stale")]
    Stale,
    #[serde(rename = "expired")]
    Expired,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct DeviceScopeInput {
    pub actor_role: FamilyActorRole,
    pub actor_account_state: ActorAccountState,
    pub household_membership: HouseholdMembership,
    pub child_profile_binding_state: ChildProfileBindingState,
    pub device_ownership_scope: DeviceOwnershipScope,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct DeviceScopeDecision {
    pub authorization_state: DeviceScopeAuthorizationState,
    pub parent_authority_requirement_state: ParentAuthorityRequirementState,
}

macro_rules! family_identity_text_id {
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

family_identity_text_id!(DeviceScopeEvaluationId, "family_identity.evaluation_id");
family_identity_text_id!(DeviceScopeDecisionId, "family_identity.decision_id");
family_identity_text_id!(FamilyIdentityAggregateId, "family_identity.aggregate_id");

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DeviceScopeEvaluationRequestedEvent {
    pub aggregate_id: FamilyIdentityAggregateId,
    pub evaluation_id: DeviceScopeEvaluationId,
    pub input: DeviceScopeInput,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DeviceScopeDecisionRecordedEvent {
    pub aggregate_id: FamilyIdentityAggregateId,
    pub decision_id: DeviceScopeDecisionId,
    pub source_evaluation_id: DeviceScopeEvaluationId,
    pub decision: DeviceScopeDecision,
}

impl DomainEvent for DeviceScopeEvaluationRequestedEvent {
    fn contract(&self) -> Result<EventContract, EventingError> {
        family_identity_event_contract(DEVICE_SCOPE_EVALUATION_REQUESTED_EVENT_TYPE)
    }

    fn aggregate_key(&self) -> Result<AggregateKey, EventingError> {
        AggregateKey::parse(self.aggregate_id.as_str())
    }

    fn idempotency_key(&self) -> Result<IdempotencyKey, EventingError> {
        family_identity_idempotency_key(
            DEVICE_SCOPE_EVALUATION_REQUESTED_EVENT_TYPE,
            &self.evaluation_id,
        )
    }
}

impl DomainEvent for DeviceScopeDecisionRecordedEvent {
    fn contract(&self) -> Result<EventContract, EventingError> {
        family_identity_event_contract(DEVICE_SCOPE_DECISION_RECORDED_EVENT_TYPE)
    }

    fn aggregate_key(&self) -> Result<AggregateKey, EventingError> {
        AggregateKey::parse(self.aggregate_id.as_str())
    }

    fn idempotency_key(&self) -> Result<IdempotencyKey, EventingError> {
        family_identity_idempotency_key(
            DEVICE_SCOPE_DECISION_RECORDED_EVENT_TYPE,
            &self.decision_id,
        )
    }
}

pub fn authorize_child_device_scope(input: DeviceScopeInput) -> DeviceScopeDecision {
    let has_parent_authority = matches!(
        input.actor_role,
        FamilyActorRole::Parent | FamilyActorRole::Guardian
    );
    let allowed = input.household_membership == HouseholdMembership::Member
        && input.actor_account_state == ActorAccountState::Active
        && input.child_profile_binding_state == ChildProfileBindingState::Bound
        && input.device_ownership_scope == DeviceOwnershipScope::ChildProfileDevice
        && has_parent_authority;

    DeviceScopeDecision {
        authorization_state: if allowed {
            DeviceScopeAuthorizationState::Authorized
        } else {
            DeviceScopeAuthorizationState::Rejected
        },
        parent_authority_requirement_state: if allowed {
            ParentAuthorityRequirementState::NotRequired
        } else {
            ParentAuthorityRequirementState::Required
        },
    }
}

pub fn record_device_scope_decision(
    event: &DeviceScopeEvaluationRequestedEvent,
) -> DeviceScopeDecisionRecordedEvent {
    DeviceScopeDecisionRecordedEvent {
        aggregate_id: event.aggregate_id.clone(),
        decision_id: DeviceScopeDecisionId::parse(device_scope_decision_ref(&event.evaluation_id))
            .unwrap_or_else(|_| panic!("{}", ERROR_DEVICE_SCOPE_DECISION_ID)),
        source_evaluation_id: event.evaluation_id.clone(),
        decision: authorize_child_device_scope(event.input),
    }
}

fn family_identity_event_contract(event_type: &str) -> Result<EventContract, EventingError> {
    Ok(EventContract::new(
        EventType::parse(event_type)?,
        SchemaVersion::new(FAMILY_IDENTITY_SCHEMA_VERSION)?,
    ))
}

fn family_identity_idempotency_key(
    event_type: &str,
    unique_ref: impl std::fmt::Display,
) -> Result<IdempotencyKey, EventingError> {
    IdempotencyKey::parse(format!(
        "{}{}{}",
        event_type, FAMILY_IDENTITY_IDEMPOTENCY_SEPARATOR, unique_ref
    ))
}

fn device_scope_decision_ref(evaluation_id: &DeviceScopeEvaluationId) -> String {
    let mut value = String::from(DEVICE_SCOPE_DECISION_PREFIX);
    value.push_str(evaluation_id.as_str());
    value
}
