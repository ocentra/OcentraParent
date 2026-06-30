#![forbid(unsafe_code)]

//! Entitlement capability decision boundary.
//!
//! This crate owns the entitlement capability gate and typed decision-recording
//! for evaluation requests. Broader payment and policy ownership stay outside
//! this crate.

use crate::entitlement_snapshot::EntitlementSnapshotContext;
use crate::entitlement_snapshot_values::{
    EntitlementDeviceTrustRequirementState, EntitlementDeviceTrustState,
    EntitlementPackageBuildState, EntitlementSnapshotBindingState,
    EntitlementSnapshotFreshnessState, EntitlementSnapshotSignatureState,
};
use ocentra_eventing::envelope::{DomainEvent, EventContract};
use ocentra_eventing::error::EventingError;
use ocentra_eventing::ids::{AggregateKey, EventType, IdempotencyKey, SchemaVersion};
use serde::{Deserialize, Serialize};

pub const CRATE_NAME: &str = "ocentra-entitlement-core";
const ENTITLEMENT_SCHEMA_VERSION: u16 = 1;
const ENTITLEMENT_CAPABILITY_EVALUATION_REQUESTED_EVENT_TYPE: &str =
    "entitlement.capability-evaluation.requested";
const ENTITLEMENT_CAPABILITY_DECISION_RECORDED_EVENT_TYPE: &str =
    "entitlement.capability-decision.recorded";
const ENTITLEMENT_IDEMPOTENCY_SEPARATOR: &str = ":";
const ENTITLEMENT_DECISION_PREFIX: &str = "entitlement-decision:";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EntitlementCapability {
    #[serde(rename = "tracking")]
    Tracking,
    #[serde(rename = "screen-evidence")]
    ScreenEvidence,
    #[serde(rename = "remote-access")]
    RemoteAccess,
    #[serde(rename = "enforcement")]
    Enforcement,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SubscriptionState {
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "inactive")]
    Inactive,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OfflineGraceState {
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "inactive")]
    Inactive,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FamilySetupState {
    #[serde(rename = "complete")]
    Complete,
    #[serde(rename = "incomplete")]
    Incomplete,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EntitlementPolicyState {
    #[serde(rename = "clean")]
    Clean,
    #[serde(rename = "payment-dispute")]
    PaymentDispute,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EntitlementCapabilityScope {
    #[serde(rename = "local-child-runtime")]
    LocalChildRuntime,
    #[serde(rename = "parent-portal-only")]
    ParentPortalOnly,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EntitlementCapabilityAccessState {
    #[serde(rename = "allowed")]
    Allowed,
    #[serde(rename = "blocked")]
    Blocked,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EntitlementManualReviewState {
    #[serde(rename = "required")]
    Required,
    #[serde(rename = "not-required")]
    NotRequired,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EntitlementCapabilityRejectionReason {
    #[serde(rename = "missing-signature")]
    MissingSignature,
    #[serde(rename = "invalid-signature")]
    InvalidSignature,
    #[serde(rename = "stale-snapshot")]
    StaleSnapshot,
    #[serde(rename = "expired-snapshot")]
    ExpiredSnapshot,
    #[serde(rename = "revoked-snapshot")]
    RevokedSnapshot,
    #[serde(rename = "wrong-household")]
    WrongHousehold,
    #[serde(rename = "wrong-device")]
    WrongDevice,
    #[serde(rename = "missing-device-trust")]
    MissingDeviceTrust,
    #[serde(rename = "invalid-package-build")]
    InvalidPackageBuild,
    #[serde(rename = "incomplete-family-setup")]
    IncompleteFamilySetup,
    #[serde(rename = "payment-dispute")]
    PaymentDispute,
    #[serde(rename = "parent-portal-only-scope")]
    ParentPortalOnlyScope,
    #[serde(rename = "inactive-subscription")]
    InactiveSubscription,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct EntitlementCapabilityInput {
    pub capability: EntitlementCapability,
    pub subscription_state: SubscriptionState,
    pub offline_grace_state: OfflineGraceState,
    pub family_setup_state: FamilySetupState,
    pub policy_state: EntitlementPolicyState,
    pub capability_scope: EntitlementCapabilityScope,
    pub snapshot_context: EntitlementSnapshotContext,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct EntitlementDecision {
    pub capability: EntitlementCapability,
    pub access_state: EntitlementCapabilityAccessState,
    pub manual_review_state: EntitlementManualReviewState,
    pub rejection_reason: Option<EntitlementCapabilityRejectionReason>,
}

macro_rules! entitlement_text_id {
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

entitlement_text_id!(EntitlementEvaluationId, "entitlement.evaluation_id");
entitlement_text_id!(EntitlementDecisionId, "entitlement.decision_id");
entitlement_text_id!(EntitlementAggregateId, "entitlement.aggregate_id");

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EntitlementCapabilityEvaluationRequestedEvent {
    pub aggregate_id: EntitlementAggregateId,
    pub evaluation_id: EntitlementEvaluationId,
    pub input: EntitlementCapabilityInput,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EntitlementCapabilityDecisionRecordedEvent {
    pub aggregate_id: EntitlementAggregateId,
    pub decision_id: EntitlementDecisionId,
    pub source_evaluation_id: EntitlementEvaluationId,
    pub decision: EntitlementDecision,
}

impl DomainEvent for EntitlementCapabilityEvaluationRequestedEvent {
    fn contract(&self) -> Result<EventContract, EventingError> {
        entitlement_event_contract(ENTITLEMENT_CAPABILITY_EVALUATION_REQUESTED_EVENT_TYPE)
    }

    fn aggregate_key(&self) -> Result<AggregateKey, EventingError> {
        AggregateKey::parse(self.aggregate_id.as_str())
    }

    fn idempotency_key(&self) -> Result<IdempotencyKey, EventingError> {
        entitlement_idempotency_key(
            ENTITLEMENT_CAPABILITY_EVALUATION_REQUESTED_EVENT_TYPE,
            &self.evaluation_id,
        )
    }
}

impl DomainEvent for EntitlementCapabilityDecisionRecordedEvent {
    fn contract(&self) -> Result<EventContract, EventingError> {
        entitlement_event_contract(ENTITLEMENT_CAPABILITY_DECISION_RECORDED_EVENT_TYPE)
    }

    fn aggregate_key(&self) -> Result<AggregateKey, EventingError> {
        AggregateKey::parse(self.aggregate_id.as_str())
    }

    fn idempotency_key(&self) -> Result<IdempotencyKey, EventingError> {
        entitlement_idempotency_key(
            ENTITLEMENT_CAPABILITY_DECISION_RECORDED_EVENT_TYPE,
            &self.decision_id,
        )
    }
}

pub fn evaluate_entitlement_capability(input: EntitlementCapabilityInput) -> EntitlementDecision {
    let rejection_reason = entitlement_rejection_reason(&input);
    let allowed = rejection_reason.is_none();

    EntitlementDecision {
        capability: input.capability,
        access_state: if allowed {
            EntitlementCapabilityAccessState::Allowed
        } else {
            EntitlementCapabilityAccessState::Blocked
        },
        manual_review_state: if allowed {
            EntitlementManualReviewState::NotRequired
        } else {
            EntitlementManualReviewState::Required
        },
        rejection_reason,
    }
}

pub fn record_entitlement_capability_decision(
    event: &EntitlementCapabilityEvaluationRequestedEvent,
) -> EntitlementCapabilityDecisionRecordedEvent {
    EntitlementCapabilityDecisionRecordedEvent {
        aggregate_id: event.aggregate_id.clone(),
        decision_id: EntitlementDecisionId(entitlement_decision_ref(&event.evaluation_id)),
        source_evaluation_id: event.evaluation_id.clone(),
        decision: evaluate_entitlement_capability(event.input),
    }
}

fn entitlement_event_contract(event_type: &str) -> Result<EventContract, EventingError> {
    Ok(EventContract::new(
        EventType::parse(event_type)?,
        SchemaVersion::new(ENTITLEMENT_SCHEMA_VERSION)?,
    ))
}

fn entitlement_idempotency_key(
    event_type: &str,
    unique_ref: impl std::fmt::Display,
) -> Result<IdempotencyKey, EventingError> {
    IdempotencyKey::parse(format!(
        "{}{}{}",
        event_type, ENTITLEMENT_IDEMPOTENCY_SEPARATOR, unique_ref
    ))
}

fn entitlement_decision_ref(evaluation_id: &EntitlementEvaluationId) -> String {
    let mut value = String::from(ENTITLEMENT_DECISION_PREFIX);
    value.push_str(evaluation_id.as_str());
    value
}

fn entitlement_rejection_reason(
    input: &EntitlementCapabilityInput,
) -> Option<EntitlementCapabilityRejectionReason> {
    match input.snapshot_context.signature_state {
        EntitlementSnapshotSignatureState::Missing => {
            return Some(EntitlementCapabilityRejectionReason::MissingSignature);
        }
        EntitlementSnapshotSignatureState::Invalid => {
            return Some(EntitlementCapabilityRejectionReason::InvalidSignature);
        }
        EntitlementSnapshotSignatureState::Trusted => {}
    }

    match input.snapshot_context.freshness_state {
        EntitlementSnapshotFreshnessState::Stale => {
            return Some(EntitlementCapabilityRejectionReason::StaleSnapshot);
        }
        EntitlementSnapshotFreshnessState::Expired => {
            return Some(EntitlementCapabilityRejectionReason::ExpiredSnapshot);
        }
        EntitlementSnapshotFreshnessState::Revoked => {
            return Some(EntitlementCapabilityRejectionReason::RevokedSnapshot);
        }
        EntitlementSnapshotFreshnessState::Fresh => {}
    }

    if input.snapshot_context.household_binding_state == EntitlementSnapshotBindingState::Mismatched
    {
        return Some(EntitlementCapabilityRejectionReason::WrongHousehold);
    }

    if input.snapshot_context.device_binding_state == EntitlementSnapshotBindingState::Mismatched {
        return Some(EntitlementCapabilityRejectionReason::WrongDevice);
    }

    if input.snapshot_context.device_trust_requirement_state
        == EntitlementDeviceTrustRequirementState::Required
        && input.snapshot_context.device_trust_state == EntitlementDeviceTrustState::Missing
    {
        return Some(EntitlementCapabilityRejectionReason::MissingDeviceTrust);
    }

    if input.snapshot_context.package_build_state == EntitlementPackageBuildState::Invalid {
        return Some(EntitlementCapabilityRejectionReason::InvalidPackageBuild);
    }

    if input.family_setup_state == FamilySetupState::Incomplete {
        return Some(EntitlementCapabilityRejectionReason::IncompleteFamilySetup);
    }

    if input.policy_state == EntitlementPolicyState::PaymentDispute {
        return Some(EntitlementCapabilityRejectionReason::PaymentDispute);
    }

    if input.capability_scope == EntitlementCapabilityScope::ParentPortalOnly {
        return Some(EntitlementCapabilityRejectionReason::ParentPortalOnlyScope);
    }

    if input.subscription_state == SubscriptionState::Inactive
        && input.offline_grace_state == OfflineGraceState::Inactive
    {
        return Some(EntitlementCapabilityRejectionReason::InactiveSubscription);
    }

    None
}
