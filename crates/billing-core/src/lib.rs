#![forbid(unsafe_code)]

//! Billing and subscription provider lifecycle boundary.
//!
//! This crate owns payment-provider webhook intake, subscription lifecycle
//! classification, dispute/manual-review state, and downstream entitlement
//! update requirements. Local capability access gates stay in entitlement-core.

pub const CRATE_NAME: &str = "ocentra-billing-core";

#[derive(Debug, Clone, PartialEq, Eq)]
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BillingProviderEventKind {
    SubscriptionCreated,
    SubscriptionUpdated,
    PaymentSucceeded,
    PaymentFailed,
    DisputeOpened,
    SubscriptionCanceled,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BillingProviderSignatureState {
    Verified,
    Missing,
    Invalid,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BillingProviderDuplicateState {
    Fresh,
    Duplicate,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BillingAccountMatchState {
    Matched,
    Mismatched,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BillingSubscriptionLifecycleState {
    Active,
    PastDue,
    Canceled,
    Disputed,
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BillingProviderEventDecisionState {
    Accepted,
    Rejected,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BillingEntitlementUpdateRequirement {
    Required,
    NotRequired,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BillingManualReviewRequirement {
    Required,
    NotRequired,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BillingProviderWebhookEvent {
    pub event_id: BillingProviderEventId,
    pub event_kind: BillingProviderEventKind,
    pub signature_state: BillingProviderSignatureState,
    pub duplicate_state: BillingProviderDuplicateState,
    pub account_match_state: BillingAccountMatchState,
    pub lifecycle_state: BillingSubscriptionLifecycleState,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BillingProviderWebhookDecision {
    pub event_id: BillingProviderEventId,
    pub decision_state: BillingProviderEventDecisionState,
    pub lifecycle_state: BillingSubscriptionLifecycleState,
    pub entitlement_update_requirement: BillingEntitlementUpdateRequirement,
    pub manual_review_requirement: BillingManualReviewRequirement,
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
