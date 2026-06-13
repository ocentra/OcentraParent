#![forbid(unsafe_code)]

//! Subscription and entitlement ownership boundary.
//!
//! This crate owns local entitlement state, plan capability gates, offline
//! grace policy, and payment-result contract consumption. Payment providers
//! stay outside child runtime business logic.

pub const CRATE_NAME: &str = "ocentra-entitlement-core";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EntitlementCapability {
    Tracking,
    ScreenEvidence,
    RemoteAccess,
    Enforcement,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SubscriptionState {
    Active,
    Inactive,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OfflineGraceState {
    Active,
    Inactive,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FamilySetupState {
    Complete,
    Incomplete,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EntitlementPolicyState {
    Clean,
    PaymentDispute,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EntitlementCapabilityScope {
    LocalChildRuntime,
    ParentPortalOnly,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EntitlementCapabilityAccessState {
    Allowed,
    Blocked,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EntitlementManualReviewState {
    Required,
    NotRequired,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EntitlementCapabilityInput {
    pub capability: EntitlementCapability,
    pub subscription_state: SubscriptionState,
    pub offline_grace_state: OfflineGraceState,
    pub family_setup_state: FamilySetupState,
    pub policy_state: EntitlementPolicyState,
    pub capability_scope: EntitlementCapabilityScope,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EntitlementDecision {
    pub capability: EntitlementCapability,
    pub access_state: EntitlementCapabilityAccessState,
    pub manual_review_state: EntitlementManualReviewState,
}

pub fn evaluate_entitlement_capability(input: EntitlementCapabilityInput) -> EntitlementDecision {
    let allowed = input.family_setup_state == FamilySetupState::Complete
        && input.policy_state == EntitlementPolicyState::Clean
        && input.capability_scope == EntitlementCapabilityScope::LocalChildRuntime
        && (input.subscription_state == SubscriptionState::Active
            || input.offline_grace_state == OfflineGraceState::Active);

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
    }
}
