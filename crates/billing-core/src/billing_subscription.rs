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
const BILLING_IDEMPOTENCY_SEPARATOR: &str = ":";
const BILLING_DECISION_PREFIX: &str = "billing-decision:";
const BILLING_TRANSITION_PREFIX: &str = "billing-transition:";
const ERROR_BILLING_DECISION_ID: &str = "billing decision id";
const ERROR_BILLING_TRANSITION_ID: &str = "billing transition id";

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
    #[serde(rename = "subscription-created")]
    SubscriptionCreated,
    #[serde(rename = "subscription-updated")]
    SubscriptionUpdated,
    #[serde(rename = "payment-succeeded")]
    PaymentSucceeded,
    #[serde(rename = "payment-failed")]
    PaymentFailed,
    #[serde(rename = "dispute-opened")]
    DisputeOpened,
    #[serde(rename = "subscription-canceled")]
    SubscriptionCanceled,
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
    let accepted = event.signature_state == BillingProviderSignatureState::Verified
        && event.duplicate_state == BillingProviderDuplicateState::Fresh
        && event.account_match_state == BillingAccountMatchState::Matched;
    let manual_review_required = !accepted
        || event.event_kind == BillingProviderEventKind::DisputeOpened
        || event.lifecycle_state == BillingSubscriptionLifecycleState::Disputed
        || event.lifecycle_state == BillingSubscriptionLifecycleState::Unknown;
    let entitlement_update_required = accepted
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
