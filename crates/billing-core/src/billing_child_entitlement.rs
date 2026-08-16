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

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct BillingChildTextId(String);

impl BillingChildTextId {
    pub fn parse(value: impl Into<String>) -> Result<Self, EventingError> {
        let value = value.into();
        if value.trim().is_empty() {
            Err(EventingError::EmptyValue {
                field: "billing child text id",
            })
        } else {
            Ok(Self(value))
        }
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl TryFrom<String> for BillingChildTextId {
    type Error = EventingError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::parse(value)
    }
}

impl From<BillingChildTextId> for String {
    fn from(value: BillingChildTextId) -> Self {
        value.0
    }
}

impl std::fmt::Display for BillingChildTextId {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(self.as_str())
    }
}

pub type BillingEntitlementSnapshotId = BillingChildTextId;
pub type BillingChildDeviceId = BillingChildTextId;
pub type BillingChildEntitlementConsumptionId = BillingChildTextId;

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
    crate::billing_child_entitlement_decision::decide_child_entitlement_snapshot(snapshot)
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
