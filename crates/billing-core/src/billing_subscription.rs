#![forbid(unsafe_code)]

//! Billing and subscription provider lifecycle boundary.
//!
//! This crate owns payment-provider webhook intake, subscription lifecycle
//! classification, dispute/manual-review state, and downstream entitlement
//! update requirements. Local capability access gates stay in entitlement-core.

use ocentra_eventing::envelope::{DomainEvent, EventContract};
use ocentra_eventing::error::EventingError;
use ocentra_eventing::expect_value::ExpectValue;
use ocentra_eventing::ids::{AggregateKey, EventType, IdempotencyKey, SchemaVersion};
use serde::{Deserialize, Serialize};

pub const CRATE_NAME: &str = "ocentra-billing-core";
const BILLING_SCHEMA_VERSION: u16 = 1;
const BILLING_PROVIDER_WEBHOOK_RECEIVED_EVENT_TYPE: &str = "billing.provider-webhook.received";
const BILLING_PROVIDER_WEBHOOK_DECISION_RECORDED_EVENT_TYPE: &str =
    "billing.provider-webhook.decision-recorded";
const BILLING_ENTITLEMENT_TRANSITION_PROJECTED_EVENT_TYPE: &str =
    "billing.entitlement.transition-projected";
const BILLING_IDEMPOTENCY_SEPARATOR: &str = ":";
const ERROR_BILLING_DECISION_ID: &str = "billing decision id";

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

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct BillingTextId(String);

impl BillingTextId {
    pub fn parse(value: impl Into<String>) -> Result<Self, EventingError> {
        let value = value.into();
        if value.trim().is_empty() {
            Err(EventingError::EmptyValue {
                field: "billing text id",
            })
        } else {
            Ok(Self(value))
        }
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl TryFrom<String> for BillingTextId {
    type Error = EventingError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::parse(value)
    }
}

impl From<BillingTextId> for String {
    fn from(value: BillingTextId) -> Self {
        value.0
    }
}

impl std::fmt::Display for BillingTextId {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(self.as_str())
    }
}

pub type BillingDecisionId = BillingTextId;
pub type BillingTransitionId = BillingTextId;
pub type BillingAggregateId = BillingTextId;

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
pub enum BillingProviderChannel {
    #[serde(rename = "stripe")]
    Stripe,
    #[serde(rename = "razorpay")]
    Razorpay,
    #[serde(rename = "paypal")]
    PayPal,
    #[serde(rename = "apple")]
    Apple,
    #[serde(rename = "google")]
    Google,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BillingProviderMode {
    #[serde(rename = "test")]
    Test,
    #[serde(rename = "live")]
    Live,
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
pub enum BillingProviderPayloadParseState {
    #[serde(rename = "parsed")]
    Parsed,
    #[serde(rename = "malformed")]
    Malformed,
    #[serde(rename = "unsupported")]
    Unsupported,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BillingProviderIdempotencyState {
    #[serde(rename = "fresh")]
    Fresh,
    #[serde(rename = "duplicate")]
    Duplicate,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BillingProviderReplayState {
    #[serde(rename = "fresh")]
    Fresh,
    #[serde(rename = "replayed")]
    Replayed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BillingProviderOrderingState {
    #[serde(rename = "in-order")]
    InOrder,
    #[serde(rename = "out-of-order")]
    OutOfOrder,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BillingAccountMatchState {
    #[serde(rename = "matched")]
    Matched,
    #[serde(rename = "mismatched")]
    Mismatched,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BillingSubscriptionStatus {
    #[serde(rename = "trialing")]
    Trialing,
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "past-due")]
    PastDue,
    #[serde(rename = "cancelled")]
    Cancelled,
    #[serde(rename = "expired")]
    Expired,
    #[serde(rename = "grace")]
    Grace,
    #[serde(rename = "unknown")]
    Unknown,
    #[serde(rename = "unavailable")]
    Unavailable,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BillingCollectionRecoveryState {
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "trialing")]
    Trialing,
    #[serde(rename = "past-due")]
    PastDue,
    #[serde(rename = "grace")]
    Grace,
    #[serde(rename = "cancelled")]
    Cancelled,
    #[serde(rename = "unpaid")]
    Unpaid,
    #[serde(rename = "support-required")]
    SupportRequired,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BillingRefundLifecycleState {
    #[serde(rename = "none")]
    None,
    #[serde(rename = "refund-requested")]
    RefundRequested,
    #[serde(rename = "refund-issued")]
    RefundIssued,
    #[serde(rename = "refund-settled")]
    RefundSettled,
    #[serde(rename = "refund-denied")]
    RefundDenied,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BillingDisputeLifecycleState {
    #[serde(rename = "none")]
    None,
    #[serde(rename = "dispute-opened")]
    DisputeOpened,
    #[serde(rename = "dispute-won")]
    DisputeWon,
    #[serde(rename = "dispute-lost")]
    DisputeLost,
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
pub enum BillingProviderRetryState {
    #[serde(rename = "not-required")]
    NotRequired,
    #[serde(rename = "queue-required")]
    QueueRequired,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BillingProviderDeadLetterState {
    #[serde(rename = "not-required")]
    NotRequired,
    #[serde(rename = "manual-required")]
    ManualRequired,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BillingProviderReconciliationState {
    #[serde(rename = "not-required")]
    NotRequired,
    #[serde(rename = "queue-required")]
    QueueRequired,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BillingTestLiveBoundaryState {
    #[serde(rename = "isolated")]
    Isolated,
    #[serde(rename = "mixed-blocked")]
    MixedBlocked,
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
    #[serde(rename = "grace-access")]
    GraceAccess,
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

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BillingProviderWebhookEvent {
    pub event_id: BillingProviderEventId,
    pub provider: BillingProviderChannel,
    pub mode: BillingProviderMode,
    pub event_kind: BillingProviderEventKind,
    pub signature_state: BillingProviderSignatureState,
    pub payload_parse_state: BillingProviderPayloadParseState,
    pub idempotency_state: BillingProviderIdempotencyState,
    pub replay_state: BillingProviderReplayState,
    pub ordering_state: BillingProviderOrderingState,
    pub account_match_state: BillingAccountMatchState,
    pub test_live_boundary_state: BillingTestLiveBoundaryState,
    pub subscription_status: BillingSubscriptionStatus,
    pub collection_recovery_state: BillingCollectionRecoveryState,
    pub refund_state: BillingRefundLifecycleState,
    pub dispute_state: BillingDisputeLifecycleState,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BillingProviderWebhookDecision {
    pub event_id: BillingProviderEventId,
    pub provider: BillingProviderChannel,
    pub mode: BillingProviderMode,
    pub decision_state: BillingProviderEventDecisionState,
    pub payload_parse_state: BillingProviderPayloadParseState,
    pub idempotency_state: BillingProviderIdempotencyState,
    pub replay_state: BillingProviderReplayState,
    pub ordering_state: BillingProviderOrderingState,
    pub retry_state: BillingProviderRetryState,
    pub dead_letter_state: BillingProviderDeadLetterState,
    pub reconciliation_state: BillingProviderReconciliationState,
    pub test_live_boundary_state: BillingTestLiveBoundaryState,
    pub subscription_status: BillingSubscriptionStatus,
    pub collection_recovery_state: BillingCollectionRecoveryState,
    pub refund_state: BillingRefundLifecycleState,
    pub dispute_state: BillingDisputeLifecycleState,
    pub entitlement_update_requirement: BillingEntitlementUpdateRequirement,
    pub manual_review_requirement: BillingManualReviewRequirement,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BillingEntitlementTransition {
    pub event_id: BillingProviderEventId,
    pub scope: BillingEntitlementScope,
    pub subscription_status: BillingSubscriptionStatus,
    pub collection_recovery_state: BillingCollectionRecoveryState,
    pub refund_state: BillingRefundLifecycleState,
    pub dispute_state: BillingDisputeLifecycleState,
    pub transition_state: BillingEntitlementTransitionState,
    pub write_state: BillingEntitlementWriteState,
    pub manual_review_requirement: BillingManualReviewRequirement,
}

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

pub fn decide_billing_provider_webhook(
    event: BillingProviderWebhookEvent,
) -> BillingProviderWebhookDecision {
    crate::billing_subscription_webhook::decide_billing_provider_webhook(event)
}

pub fn record_billing_provider_webhook_decision_event(
    event: BillingProviderWebhookReceivedEvent,
) -> BillingProviderWebhookDecisionRecordedEvent {
    let provider_event_id = event.provider_event.event_id.clone();
    BillingProviderWebhookDecisionRecordedEvent {
        aggregate_id: event.aggregate_id,
        decision_id: BillingDecisionId::parse(
            crate::billing_subscription_projection::billing_decision_ref(
                provider_event_id.as_str(),
            ),
        )
        .expect_value(ERROR_BILLING_DECISION_ID),
        decision: decide_billing_provider_webhook(event.provider_event),
    }
}

pub fn project_billing_entitlement_transition(
    decision: BillingProviderWebhookDecision,
    scope: BillingEntitlementScope,
) -> BillingEntitlementTransition {
    crate::billing_subscription_projection::project_billing_entitlement_transition(decision, scope)
}

pub fn project_billing_entitlement_transition_event(
    event: BillingProviderWebhookDecisionRecordedEvent,
    scope: BillingEntitlementScope,
) -> BillingEntitlementTransitionProjectedEvent {
    crate::billing_subscription_projection::project_billing_entitlement_transition_event(
        event, scope,
    )
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
