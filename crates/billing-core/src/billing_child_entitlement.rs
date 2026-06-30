#![forbid(unsafe_code)]

//! Child entitlement snapshot consumption inside the billing boundary.

use crate::billing_subscription::{
    BillingAggregateId, BillingEntitlementWriteState, BillingManualReviewRequirement,
    BillingSubscriptionStatus,
};
use ocentra_eventing::envelope::{DomainEvent, EventContract};
use ocentra_eventing::error::EventingError;
use ocentra_eventing::expect_value::ExpectValue;
use ocentra_eventing::ids::{AggregateKey, EventType, IdempotencyKey, SchemaVersion};
use serde::{Deserialize, Serialize};

const BILLING_SCHEMA_VERSION: u16 = 1;
const BILLING_CHILD_ENTITLEMENT_SNAPSHOT_RECEIVED_EVENT_TYPE: &str =
    "billing.child-entitlement-snapshot.received";
const BILLING_CHILD_ENTITLEMENT_CONSUMPTION_RECORDED_EVENT_TYPE: &str =
    "billing.child-entitlement-consumption.recorded";
const BILLING_IDEMPOTENCY_SEPARATOR: &str = ":";
const BILLING_CHILD_CONSUMPTION_PREFIX: &str = "billing-child-consumption:";
const ERROR_BILLING_CHILD_CONSUMPTION_ID: &str = "billing child consumption id";

macro_rules! billing_child_text_id {
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

billing_child_text_id!(
    BillingEntitlementSnapshotId,
    "billing.entitlement_snapshot_id"
);
billing_child_text_id!(BillingChildDeviceId, "billing.child_device_id");
billing_child_text_id!(
    BillingChildEntitlementConsumptionId,
    "billing.child_entitlement_consumption_id"
);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BillingChildSnapshotSignatureState {
    #[serde(rename = "trusted")]
    Trusted,
    #[serde(rename = "missing")]
    Missing,
    #[serde(rename = "invalid")]
    Invalid,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BillingChildSnapshotFreshnessState {
    #[serde(rename = "fresh")]
    Fresh,
    #[serde(rename = "stale")]
    Stale,
    #[serde(rename = "expired")]
    Expired,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BillingChildEntitlementConsumptionState {
    #[serde(rename = "accepted")]
    Accepted,
    #[serde(rename = "rejected")]
    Rejected,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BillingChildEntitlementAccessState {
    #[serde(rename = "full-access")]
    FullAccess,
    #[serde(rename = "grace-access")]
    GraceAccess,
    #[serde(rename = "limited-access")]
    LimitedAccess,
    #[serde(rename = "revoked")]
    Revoked,
    #[serde(rename = "hold-for-review")]
    HoldForReview,
    #[serde(rename = "no-change")]
    NoChange,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BillingChildEntitlementRejectionReason {
    #[serde(rename = "missing-signature")]
    MissingSignature,
    #[serde(rename = "invalid-signature")]
    InvalidSignature,
    #[serde(rename = "stale-snapshot")]
    StaleSnapshot,
    #[serde(rename = "expired-snapshot")]
    ExpiredSnapshot,
    #[serde(rename = "unknown-subscription-status")]
    UnknownSubscriptionStatus,
    #[serde(rename = "unavailable-subscription-status")]
    UnavailableSubscriptionStatus,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BillingChildEntitlementSnapshot {
    pub snapshot_id: BillingEntitlementSnapshotId,
    pub child_device_id: BillingChildDeviceId,
    pub subscription_status: BillingSubscriptionStatus,
    pub signature_state: BillingChildSnapshotSignatureState,
    pub freshness_state: BillingChildSnapshotFreshnessState,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BillingChildEntitlementConsumptionDecision {
    pub snapshot_id: BillingEntitlementSnapshotId,
    pub child_device_id: BillingChildDeviceId,
    pub decision_state: BillingChildEntitlementConsumptionState,
    pub subscription_status: BillingSubscriptionStatus,
    pub access_state: BillingChildEntitlementAccessState,
    pub write_state: BillingEntitlementWriteState,
    pub manual_review_requirement: BillingManualReviewRequirement,
    pub rejection_reason: Option<BillingChildEntitlementRejectionReason>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BillingChildEntitlementSnapshotReceivedEvent {
    pub aggregate_id: BillingAggregateId,
    pub snapshot: BillingChildEntitlementSnapshot,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BillingChildEntitlementConsumptionRecordedEvent {
    pub aggregate_id: BillingAggregateId,
    pub consumption_id: BillingChildEntitlementConsumptionId,
    pub decision: BillingChildEntitlementConsumptionDecision,
}

impl DomainEvent for BillingChildEntitlementSnapshotReceivedEvent {
    fn contract(&self) -> Result<EventContract, EventingError> {
        billing_child_event_contract(BILLING_CHILD_ENTITLEMENT_SNAPSHOT_RECEIVED_EVENT_TYPE)
    }

    fn aggregate_key(&self) -> Result<AggregateKey, EventingError> {
        AggregateKey::parse(self.aggregate_id.as_str())
    }

    fn idempotency_key(&self) -> Result<IdempotencyKey, EventingError> {
        billing_child_idempotency_key(
            BILLING_CHILD_ENTITLEMENT_SNAPSHOT_RECEIVED_EVENT_TYPE,
            &self.snapshot.snapshot_id,
        )
    }
}

impl DomainEvent for BillingChildEntitlementConsumptionRecordedEvent {
    fn contract(&self) -> Result<EventContract, EventingError> {
        billing_child_event_contract(BILLING_CHILD_ENTITLEMENT_CONSUMPTION_RECORDED_EVENT_TYPE)
    }

    fn aggregate_key(&self) -> Result<AggregateKey, EventingError> {
        AggregateKey::parse(self.aggregate_id.as_str())
    }

    fn idempotency_key(&self) -> Result<IdempotencyKey, EventingError> {
        billing_child_idempotency_key(
            BILLING_CHILD_ENTITLEMENT_CONSUMPTION_RECORDED_EVENT_TYPE,
            &self.consumption_id,
        )
    }
}

pub fn decide_child_entitlement_snapshot(
    snapshot: BillingChildEntitlementSnapshot,
) -> BillingChildEntitlementConsumptionDecision {
    if let Some(rejection_reason) = child_entitlement_snapshot_rejection_reason(&snapshot) {
        return BillingChildEntitlementConsumptionDecision {
            snapshot_id: snapshot.snapshot_id,
            child_device_id: snapshot.child_device_id,
            decision_state: BillingChildEntitlementConsumptionState::Rejected,
            subscription_status: snapshot.subscription_status,
            access_state: BillingChildEntitlementAccessState::NoChange,
            write_state: BillingEntitlementWriteState::DoNotWrite,
            manual_review_requirement: BillingManualReviewRequirement::Required,
            rejection_reason: Some(rejection_reason),
        };
    }

    let (access_state, manual_review_requirement) =
        accepted_child_entitlement_access(snapshot.subscription_status);

    BillingChildEntitlementConsumptionDecision {
        snapshot_id: snapshot.snapshot_id,
        child_device_id: snapshot.child_device_id,
        decision_state: BillingChildEntitlementConsumptionState::Accepted,
        subscription_status: snapshot.subscription_status,
        access_state,
        write_state: BillingEntitlementWriteState::WriteRequired,
        manual_review_requirement,
        rejection_reason: None,
    }
}

pub fn record_child_entitlement_consumption_event(
    event: BillingChildEntitlementSnapshotReceivedEvent,
) -> BillingChildEntitlementConsumptionRecordedEvent {
    let snapshot_id = event.snapshot.snapshot_id.clone();
    let child_device_id = event.snapshot.child_device_id.clone();
    BillingChildEntitlementConsumptionRecordedEvent {
        aggregate_id: event.aggregate_id,
        consumption_id: BillingChildEntitlementConsumptionId::parse(billing_child_consumption_ref(
            &snapshot_id,
            &child_device_id,
        ))
        .expect_value(ERROR_BILLING_CHILD_CONSUMPTION_ID),
        decision: decide_child_entitlement_snapshot(event.snapshot),
    }
}

fn accepted_child_entitlement_access(
    subscription_status: BillingSubscriptionStatus,
) -> (
    BillingChildEntitlementAccessState,
    BillingManualReviewRequirement,
) {
    match subscription_status {
        BillingSubscriptionStatus::Trialing | BillingSubscriptionStatus::Active => (
            BillingChildEntitlementAccessState::FullAccess,
            BillingManualReviewRequirement::NotRequired,
        ),
        BillingSubscriptionStatus::Grace => (
            BillingChildEntitlementAccessState::GraceAccess,
            BillingManualReviewRequirement::NotRequired,
        ),
        BillingSubscriptionStatus::PastDue => (
            BillingChildEntitlementAccessState::LimitedAccess,
            BillingManualReviewRequirement::NotRequired,
        ),
        BillingSubscriptionStatus::Cancelled | BillingSubscriptionStatus::Expired => (
            BillingChildEntitlementAccessState::Revoked,
            BillingManualReviewRequirement::NotRequired,
        ),
        BillingSubscriptionStatus::Unknown | BillingSubscriptionStatus::Unavailable => {
            unreachable!(
                "child entitlement acceptance requires a resolved billing subscription status"
            )
        }
    }
}

fn child_entitlement_snapshot_rejection_reason(
    snapshot: &BillingChildEntitlementSnapshot,
) -> Option<BillingChildEntitlementRejectionReason> {
    match snapshot.signature_state {
        BillingChildSnapshotSignatureState::Missing => {
            return Some(BillingChildEntitlementRejectionReason::MissingSignature);
        }
        BillingChildSnapshotSignatureState::Invalid => {
            return Some(BillingChildEntitlementRejectionReason::InvalidSignature);
        }
        BillingChildSnapshotSignatureState::Trusted => {}
    }

    match snapshot.freshness_state {
        BillingChildSnapshotFreshnessState::Stale => {
            Some(BillingChildEntitlementRejectionReason::StaleSnapshot)
        }
        BillingChildSnapshotFreshnessState::Expired => {
            Some(BillingChildEntitlementRejectionReason::ExpiredSnapshot)
        }
        BillingChildSnapshotFreshnessState::Fresh
            if snapshot.subscription_status == BillingSubscriptionStatus::Unknown =>
        {
            Some(BillingChildEntitlementRejectionReason::UnknownSubscriptionStatus)
        }
        BillingChildSnapshotFreshnessState::Fresh
            if snapshot.subscription_status == BillingSubscriptionStatus::Unavailable =>
        {
            Some(BillingChildEntitlementRejectionReason::UnavailableSubscriptionStatus)
        }
        BillingChildSnapshotFreshnessState::Fresh => None,
    }
}

fn billing_child_event_contract(event_type: &str) -> Result<EventContract, EventingError> {
    Ok(EventContract::new(
        EventType::parse(event_type)?,
        SchemaVersion::new(BILLING_SCHEMA_VERSION)?,
    ))
}

fn billing_child_idempotency_key(
    event_type: &str,
    unique_ref: impl std::fmt::Display,
) -> Result<IdempotencyKey, EventingError> {
    IdempotencyKey::parse(format!(
        "{}{}{}",
        event_type, BILLING_IDEMPOTENCY_SEPARATOR, unique_ref
    ))
}

fn billing_child_consumption_ref(
    snapshot_id: &BillingEntitlementSnapshotId,
    child_device_id: &BillingChildDeviceId,
) -> String {
    let mut value = String::from(BILLING_CHILD_CONSUMPTION_PREFIX);
    value.push_str(snapshot_id.as_str());
    value.push_str(BILLING_IDEMPOTENCY_SEPARATOR);
    value.push_str(child_device_id.as_str());
    value
}
