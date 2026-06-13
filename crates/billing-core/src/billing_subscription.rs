#![forbid(unsafe_code)]

//! Billing and subscription provider lifecycle boundary.
//!
//! This crate owns payment-provider webhook intake, subscription lifecycle
//! classification, dispute/manual-review state, and downstream entitlement
//! update requirements. Local capability access gates stay in entitlement-core.

use ocentra_eventing::{
    AggregateKey, DomainEvent, EventContract, EventType, EventingError, IdempotencyKey,
    SchemaVersion,
};
use serde::{Deserialize, Serialize};

pub const CRATE_NAME: &str = "ocentra-billing-core";
const BILLING_SCHEMA_VERSION: u16 = 1;
const BILLING_PROVIDER_WEBHOOK_RECEIVED_EVENT_TYPE: &str = "billing.provider-webhook.received";
const BILLING_PROVIDER_WEBHOOK_DECISION_RECORDED_EVENT_TYPE: &str =
    "billing.provider-webhook.decision-recorded";
const BILLING_ENTITLEMENT_TRANSITION_PROJECTED_EVENT_TYPE: &str =
    "billing.entitlement.transition-projected";
const BILLING_CHILD_ENTITLEMENT_SNAPSHOT_RECEIVED_EVENT_TYPE: &str =
    "billing.child-entitlement-snapshot.received";
const BILLING_CHILD_ENTITLEMENT_CONSUMPTION_RECORDED_EVENT_TYPE: &str =
    "billing.child-entitlement-consumption.recorded";
const BILLING_IDEMPOTENCY_SEPARATOR: &str = ":";
const BILLING_DECISION_PREFIX: &str = "billing-decision:";
const BILLING_TRANSITION_PREFIX: &str = "billing-transition:";
const BILLING_CHILD_CONSUMPTION_PREFIX: &str = "billing-child-consumption:";
const ERROR_BILLING_DECISION_ID: &str = "billing decision id";
const ERROR_BILLING_TRANSITION_ID: &str = "billing transition id";
const ERROR_BILLING_CHILD_CONSUMPTION_ID: &str = "billing child consumption id";

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BillingProviderEventId {
    value: String,
}

impl BillingProviderEventId {
    pub fn parse(value: impl Into<String>) -> Option<Self> {
        let value = value.into();
        if value.trim().is_empty() {
            None
        } else {
            Some(Self { value })
        }
    }

    pub fn as_str(&self) -> &str {
        &self.value
    }
}

impl std::fmt::Display for BillingProviderEventId {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(self.as_str())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BillingProviderEventKind {
    #[serde(rename = "checkout-completed")]
    CheckoutCompleted,
    #[serde(rename = "subscription-created")]
    SubscriptionCreated,
    #[serde(rename = "subscription-updated")]
    SubscriptionUpdated,
    #[serde(rename = "subscription-deleted")]
    SubscriptionDeleted,
    #[serde(rename = "invoice-paid")]
    InvoicePaid,
    #[serde(rename = "invoice-payment-failed")]
    InvoicePaymentFailed,
    #[serde(rename = "payment-intent-succeeded")]
    PaymentIntentSucceeded,
    #[serde(rename = "payment-intent-failed")]
    PaymentIntentFailed,
    #[serde(rename = "customer-portal-updated")]
    CustomerPortalUpdated,
    #[serde(rename = "refund-issued")]
    RefundIssued,
    #[serde(rename = "dispute-opened")]
    DisputeOpened,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BillingProviderSignatureState {
    #[serde(rename = "verified")]
    Verified,
    #[serde(rename = "missing")]
    Missing,
    #[serde(rename = "invalid")]
    Invalid,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BillingProviderDuplicateState {
    #[serde(rename = "fresh")]
    Fresh,
    #[serde(rename = "duplicate")]
    Duplicate,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BillingAccountMatchState {
    #[serde(rename = "matched")]
    Matched,
    #[serde(rename = "mismatched")]
    Mismatched,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BillingSubscriptionLifecycleState {
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "past-due")]
    PastDue,
    #[serde(rename = "canceled")]
    Canceled,
    #[serde(rename = "disputed")]
    Disputed,
    #[serde(rename = "unknown")]
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BillingProviderEventDecisionState {
    #[serde(rename = "accepted")]
    Accepted,
    #[serde(rename = "rejected")]
    Rejected,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BillingEntitlementUpdateRequirement {
    #[serde(rename = "required")]
    Required,
    #[serde(rename = "not-required")]
    NotRequired,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BillingManualReviewRequirement {
    #[serde(rename = "required")]
    Required,
    #[serde(rename = "not-required")]
    NotRequired,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BillingEntitlementScope {
    #[serde(rename = "household")]
    Household,
    #[serde(rename = "child-device")]
    ChildDevice,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BillingEntitlementTransitionState {
    #[serde(rename = "grant-full-access")]
    GrantFullAccess,
    #[serde(rename = "limit-access")]
    LimitAccess,
    #[serde(rename = "revoke-access")]
    RevokeAccess,
    #[serde(rename = "hold-for-review")]
    HoldForReview,
    #[serde(rename = "no-write")]
    NoWrite,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BillingEntitlementWriteState {
    #[serde(rename = "write-required")]
    WriteRequired,
    #[serde(rename = "do-not-write")]
    DoNotWrite,
}

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
    #[serde(rename = "unknown-lifecycle")]
    UnknownLifecycle,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BillingProviderWebhookEvent {
    pub event_id: BillingProviderEventId,
    pub event_kind: BillingProviderEventKind,
    pub signature_state: BillingProviderSignatureState,
    pub duplicate_state: BillingProviderDuplicateState,
    pub account_match_state: BillingAccountMatchState,
    pub lifecycle_state: BillingSubscriptionLifecycleState,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BillingProviderWebhookDecision {
    pub event_id: BillingProviderEventId,
    pub decision_state: BillingProviderEventDecisionState,
    pub lifecycle_state: BillingSubscriptionLifecycleState,
    pub entitlement_update_requirement: BillingEntitlementUpdateRequirement,
    pub manual_review_requirement: BillingManualReviewRequirement,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BillingEntitlementTransition {
    pub event_id: BillingProviderEventId,
    pub scope: BillingEntitlementScope,
    pub lifecycle_state: BillingSubscriptionLifecycleState,
    pub transition_state: BillingEntitlementTransitionState,
    pub write_state: BillingEntitlementWriteState,
    pub manual_review_requirement: BillingManualReviewRequirement,
}

macro_rules! billing_text_id {
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

billing_text_id!(BillingDecisionId, "billing.decision_id");
billing_text_id!(BillingTransitionId, "billing.transition_id");
billing_text_id!(BillingAggregateId, "billing.aggregate_id");
billing_text_id!(BillingEntitlementSnapshotId, "billing.entitlement_snapshot_id");
billing_text_id!(BillingChildDeviceId, "billing.child_device_id");
billing_text_id!(
    BillingChildEntitlementConsumptionId,
    "billing.child_entitlement_consumption_id"
);

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BillingProviderWebhookReceivedEvent {
    pub aggregate_id: BillingAggregateId,
    pub provider_event: BillingProviderWebhookEvent,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BillingProviderWebhookDecisionRecordedEvent {
    pub aggregate_id: BillingAggregateId,
    pub decision_id: BillingDecisionId,
    pub decision: BillingProviderWebhookDecision,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BillingEntitlementTransitionProjectedEvent {
    pub aggregate_id: BillingAggregateId,
    pub transition_id: BillingTransitionId,
    pub source_decision_id: BillingDecisionId,
    pub transition: BillingEntitlementTransition,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BillingChildEntitlementSnapshot {
    pub snapshot_id: BillingEntitlementSnapshotId,
    pub child_device_id: BillingChildDeviceId,
    pub lifecycle_state: BillingSubscriptionLifecycleState,
    pub signature_state: BillingChildSnapshotSignatureState,
    pub freshness_state: BillingChildSnapshotFreshnessState,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BillingChildEntitlementConsumptionDecision {
    pub snapshot_id: BillingEntitlementSnapshotId,
    pub child_device_id: BillingChildDeviceId,
    pub decision_state: BillingChildEntitlementConsumptionState,
    pub lifecycle_state: BillingSubscriptionLifecycleState,
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

impl DomainEvent for BillingProviderWebhookReceivedEvent {
    fn contract(&self) -> Result<EventContract, EventingError> {
        billing_event_contract(BILLING_PROVIDER_WEBHOOK_RECEIVED_EVENT_TYPE)
    }

    fn aggregate_key(&self) -> Result<AggregateKey, EventingError> {
        AggregateKey::parse(self.aggregate_id.as_str())
    }

    fn idempotency_key(&self) -> Result<IdempotencyKey, EventingError> {
        billing_idempotency_key(
            BILLING_PROVIDER_WEBHOOK_RECEIVED_EVENT_TYPE,
            &self.provider_event.event_id,
        )
    }
}

impl DomainEvent for BillingProviderWebhookDecisionRecordedEvent {
    fn contract(&self) -> Result<EventContract, EventingError> {
        billing_event_contract(BILLING_PROVIDER_WEBHOOK_DECISION_RECORDED_EVENT_TYPE)
    }

    fn aggregate_key(&self) -> Result<AggregateKey, EventingError> {
        AggregateKey::parse(self.aggregate_id.as_str())
    }

    fn idempotency_key(&self) -> Result<IdempotencyKey, EventingError> {
        billing_idempotency_key(
            BILLING_PROVIDER_WEBHOOK_DECISION_RECORDED_EVENT_TYPE,
            &self.decision_id,
        )
    }
}

impl DomainEvent for BillingEntitlementTransitionProjectedEvent {
    fn contract(&self) -> Result<EventContract, EventingError> {
        billing_event_contract(BILLING_ENTITLEMENT_TRANSITION_PROJECTED_EVENT_TYPE)
    }

    fn aggregate_key(&self) -> Result<AggregateKey, EventingError> {
        AggregateKey::parse(self.aggregate_id.as_str())
    }

    fn idempotency_key(&self) -> Result<IdempotencyKey, EventingError> {
        billing_idempotency_key(
            BILLING_ENTITLEMENT_TRANSITION_PROJECTED_EVENT_TYPE,
            &self.transition_id,
        )
    }
}

impl DomainEvent for BillingChildEntitlementSnapshotReceivedEvent {
    fn contract(&self) -> Result<EventContract, EventingError> {
        billing_event_contract(BILLING_CHILD_ENTITLEMENT_SNAPSHOT_RECEIVED_EVENT_TYPE)
    }

    fn aggregate_key(&self) -> Result<AggregateKey, EventingError> {
        AggregateKey::parse(self.aggregate_id.as_str())
    }

    fn idempotency_key(&self) -> Result<IdempotencyKey, EventingError> {
        billing_idempotency_key(
            BILLING_CHILD_ENTITLEMENT_SNAPSHOT_RECEIVED_EVENT_TYPE,
            &self.snapshot.snapshot_id,
        )
    }
}

impl DomainEvent for BillingChildEntitlementConsumptionRecordedEvent {
    fn contract(&self) -> Result<EventContract, EventingError> {
        billing_event_contract(BILLING_CHILD_ENTITLEMENT_CONSUMPTION_RECORDED_EVENT_TYPE)
    }

    fn aggregate_key(&self) -> Result<AggregateKey, EventingError> {
        AggregateKey::parse(self.aggregate_id.as_str())
    }

    fn idempotency_key(&self) -> Result<IdempotencyKey, EventingError> {
        billing_idempotency_key(
            BILLING_CHILD_ENTITLEMENT_CONSUMPTION_RECORDED_EVENT_TYPE,
            &self.consumption_id,
        )
    }
}

pub fn decide_billing_provider_webhook(
    event: BillingProviderWebhookEvent,
) -> BillingProviderWebhookDecision {
    let accepted = event.signature_state == BillingProviderSignatureState::Verified
        && event.duplicate_state == BillingProviderDuplicateState::Fresh
        && event.account_match_state == BillingAccountMatchState::Matched;
    let manual_review_required = !accepted
        || matches!(
            event.event_kind,
            BillingProviderEventKind::DisputeOpened | BillingProviderEventKind::RefundIssued
        )
        || event.lifecycle_state == BillingSubscriptionLifecycleState::Disputed
        || event.lifecycle_state == BillingSubscriptionLifecycleState::Unknown;
    let entitlement_update_required = accepted
        && event.event_kind != BillingProviderEventKind::RefundIssued
        && matches!(
            event.lifecycle_state,
            BillingSubscriptionLifecycleState::Active
                | BillingSubscriptionLifecycleState::PastDue
                | BillingSubscriptionLifecycleState::Canceled
                | BillingSubscriptionLifecycleState::Disputed
        );

    BillingProviderWebhookDecision {
        event_id: event.event_id,
        decision_state: if accepted {
            BillingProviderEventDecisionState::Accepted
        } else {
            BillingProviderEventDecisionState::Rejected
        },
        lifecycle_state: event.lifecycle_state,
        entitlement_update_requirement: if entitlement_update_required {
            BillingEntitlementUpdateRequirement::Required
        } else {
            BillingEntitlementUpdateRequirement::NotRequired
        },
        manual_review_requirement: if manual_review_required {
            BillingManualReviewRequirement::Required
        } else {
            BillingManualReviewRequirement::NotRequired
        },
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
            lifecycle_state: snapshot.lifecycle_state,
            access_state: BillingChildEntitlementAccessState::NoChange,
            write_state: BillingEntitlementWriteState::DoNotWrite,
            manual_review_requirement: BillingManualReviewRequirement::Required,
            rejection_reason: Some(rejection_reason),
        };
    }

    let (access_state, manual_review_requirement) = match snapshot.lifecycle_state {
        BillingSubscriptionLifecycleState::Active => (
            BillingChildEntitlementAccessState::FullAccess,
            BillingManualReviewRequirement::NotRequired,
        ),
        BillingSubscriptionLifecycleState::PastDue => (
            BillingChildEntitlementAccessState::LimitedAccess,
            BillingManualReviewRequirement::NotRequired,
        ),
        BillingSubscriptionLifecycleState::Canceled => (
            BillingChildEntitlementAccessState::Revoked,
            BillingManualReviewRequirement::NotRequired,
        ),
        BillingSubscriptionLifecycleState::Disputed => (
            BillingChildEntitlementAccessState::HoldForReview,
            BillingManualReviewRequirement::Required,
        ),
        BillingSubscriptionLifecycleState::Unknown => (
            BillingChildEntitlementAccessState::NoChange,
            BillingManualReviewRequirement::Required,
        ),
    };

    BillingChildEntitlementConsumptionDecision {
        snapshot_id: snapshot.snapshot_id,
        child_device_id: snapshot.child_device_id,
        decision_state: BillingChildEntitlementConsumptionState::Accepted,
        lifecycle_state: snapshot.lifecycle_state,
        access_state,
        write_state: BillingEntitlementWriteState::WriteRequired,
        manual_review_requirement,
        rejection_reason: None,
    }
}

pub fn record_billing_provider_webhook_decision_event(
    event: BillingProviderWebhookReceivedEvent,
) -> BillingProviderWebhookDecisionRecordedEvent {
    let provider_event_id = event.provider_event.event_id.clone();
    BillingProviderWebhookDecisionRecordedEvent {
        aggregate_id: event.aggregate_id,
        decision_id: BillingDecisionId::parse(billing_decision_ref(&provider_event_id))
            .expect(ERROR_BILLING_DECISION_ID),
        decision: decide_billing_provider_webhook(event.provider_event),
    }
}

pub fn record_child_entitlement_consumption_event(
    event: BillingChildEntitlementSnapshotReceivedEvent,
) -> BillingChildEntitlementConsumptionRecordedEvent {
    let snapshot_id = event.snapshot.snapshot_id.clone();
    let child_device_id = event.snapshot.child_device_id.clone();
    BillingChildEntitlementConsumptionRecordedEvent {
        aggregate_id: event.aggregate_id,
        consumption_id: BillingChildEntitlementConsumptionId::parse(
            billing_child_consumption_ref(&snapshot_id, &child_device_id),
        )
        .expect(ERROR_BILLING_CHILD_CONSUMPTION_ID),
        decision: decide_child_entitlement_snapshot(event.snapshot),
    }
}

pub fn project_billing_entitlement_transition(
    decision: BillingProviderWebhookDecision,
    scope: BillingEntitlementScope,
) -> BillingEntitlementTransition {
    let write_allowed = decision.decision_state == BillingProviderEventDecisionState::Accepted
        && decision.entitlement_update_requirement == BillingEntitlementUpdateRequirement::Required;
    let transition_state = if !write_allowed {
        BillingEntitlementTransitionState::NoWrite
    } else {
        match decision.lifecycle_state {
            BillingSubscriptionLifecycleState::Active => {
                BillingEntitlementTransitionState::GrantFullAccess
            }
            BillingSubscriptionLifecycleState::PastDue => {
                BillingEntitlementTransitionState::LimitAccess
            }
            BillingSubscriptionLifecycleState::Canceled => {
                BillingEntitlementTransitionState::RevokeAccess
            }
            BillingSubscriptionLifecycleState::Disputed => {
                BillingEntitlementTransitionState::HoldForReview
            }
            BillingSubscriptionLifecycleState::Unknown => {
                BillingEntitlementTransitionState::NoWrite
            }
        }
    };

    BillingEntitlementTransition {
        event_id: decision.event_id,
        scope,
        lifecycle_state: decision.lifecycle_state,
        transition_state,
        write_state: if write_allowed {
            BillingEntitlementWriteState::WriteRequired
        } else {
            BillingEntitlementWriteState::DoNotWrite
        },
        manual_review_requirement: decision.manual_review_requirement,
    }
}

pub fn project_billing_entitlement_transition_event(
    event: BillingProviderWebhookDecisionRecordedEvent,
    scope: BillingEntitlementScope,
) -> BillingEntitlementTransitionProjectedEvent {
    let decision_id = event.decision_id.clone();
    BillingEntitlementTransitionProjectedEvent {
        aggregate_id: event.aggregate_id,
        transition_id: BillingTransitionId::parse(billing_transition_ref(&decision_id))
            .expect(ERROR_BILLING_TRANSITION_ID),
        source_decision_id: decision_id,
        transition: project_billing_entitlement_transition(event.decision, scope),
    }
}

fn billing_event_contract(event_type: &str) -> Result<EventContract, EventingError> {
    Ok(EventContract::new(
        EventType::parse(event_type)?,
        SchemaVersion::new(BILLING_SCHEMA_VERSION)?,
    ))
}

fn billing_idempotency_key(
    event_type: &str,
    unique_ref: impl std::fmt::Display,
) -> Result<IdempotencyKey, EventingError> {
    IdempotencyKey::parse(format!(
        "{}{}{}",
        event_type, BILLING_IDEMPOTENCY_SEPARATOR, unique_ref
    ))
}

fn billing_decision_ref(event_id: &BillingProviderEventId) -> String {
    let mut value = String::from(BILLING_DECISION_PREFIX);
    value.push_str(event_id.as_str());
    value
}

fn billing_transition_ref(decision_id: &BillingDecisionId) -> String {
    let mut value = String::from(BILLING_TRANSITION_PREFIX);
    value.push_str(decision_id.as_str());
    value
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
            if snapshot.lifecycle_state == BillingSubscriptionLifecycleState::Unknown =>
        {
            Some(BillingChildEntitlementRejectionReason::UnknownLifecycle)
        }
        BillingChildSnapshotFreshnessState::Fresh => None,
    }
}
