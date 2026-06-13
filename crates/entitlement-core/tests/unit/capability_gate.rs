use ocentra_entitlement_core::{
    evaluate_entitlement_capability, EntitlementCapability, EntitlementCapabilityAccessState,
    EntitlementCapabilityInput, EntitlementCapabilityScope, EntitlementManualReviewState,
    EntitlementPolicyState, FamilySetupState, OfflineGraceState, SubscriptionState,
};

#[test]
fn active_subscription_allows_capability_after_family_setup() {
    let decision = evaluate_entitlement_capability(EntitlementCapabilityInput {
        capability: EntitlementCapability::Tracking,
        subscription_state: SubscriptionState::Active,
        offline_grace_state: OfflineGraceState::Inactive,
        family_setup_state: FamilySetupState::Complete,
        policy_state: EntitlementPolicyState::Clean,
        capability_scope: EntitlementCapabilityScope::LocalChildRuntime,
    });

    assert_eq!(decision.access_state, EntitlementCapabilityAccessState::Allowed);
    assert_eq!(
        decision.manual_review_state,
        EntitlementManualReviewState::NotRequired
    );
    assert_eq!(decision.capability, EntitlementCapability::Tracking);
}

#[test]
fn offline_grace_preserves_child_runtime_capability() {
    let decision = evaluate_entitlement_capability(EntitlementCapabilityInput {
        capability: EntitlementCapability::Enforcement,
        subscription_state: SubscriptionState::Inactive,
        offline_grace_state: OfflineGraceState::Active,
        family_setup_state: FamilySetupState::Complete,
        policy_state: EntitlementPolicyState::Clean,
        capability_scope: EntitlementCapabilityScope::LocalChildRuntime,
    });

    assert_eq!(decision.access_state, EntitlementCapabilityAccessState::Allowed);
    assert_eq!(decision.capability, EntitlementCapability::Enforcement);
}

#[test]
fn incomplete_family_setup_blocks_capability_even_with_subscription() {
    let decision = evaluate_entitlement_capability(EntitlementCapabilityInput {
        capability: EntitlementCapability::RemoteAccess,
        subscription_state: SubscriptionState::Active,
        offline_grace_state: OfflineGraceState::Inactive,
        family_setup_state: FamilySetupState::Incomplete,
        policy_state: EntitlementPolicyState::Clean,
        capability_scope: EntitlementCapabilityScope::LocalChildRuntime,
    });

    assert_eq!(decision.access_state, EntitlementCapabilityAccessState::Blocked);
    assert_eq!(
        decision.manual_review_state,
        EntitlementManualReviewState::Required
    );
}

#[test]
fn inactive_subscription_without_grace_blocks_capability_after_family_setup() {
    let decision = evaluate_entitlement_capability(EntitlementCapabilityInput {
        capability: EntitlementCapability::ScreenEvidence,
        subscription_state: SubscriptionState::Inactive,
        offline_grace_state: OfflineGraceState::Inactive,
        family_setup_state: FamilySetupState::Complete,
        policy_state: EntitlementPolicyState::Clean,
        capability_scope: EntitlementCapabilityScope::LocalChildRuntime,
    });

    assert_eq!(decision.access_state, EntitlementCapabilityAccessState::Blocked);
    assert_eq!(
        decision.manual_review_state,
        EntitlementManualReviewState::Required
    );
    assert_eq!(decision.capability, EntitlementCapability::ScreenEvidence);
}

#[test]
fn payment_dispute_blocks_capability_even_during_grace() {
    let decision = evaluate_entitlement_capability(EntitlementCapabilityInput {
        capability: EntitlementCapability::RemoteAccess,
        subscription_state: SubscriptionState::Inactive,
        offline_grace_state: OfflineGraceState::Active,
        family_setup_state: FamilySetupState::Complete,
        policy_state: EntitlementPolicyState::PaymentDispute,
        capability_scope: EntitlementCapabilityScope::LocalChildRuntime,
    });

    assert_eq!(decision.access_state, EntitlementCapabilityAccessState::Blocked);
    assert_eq!(
        decision.manual_review_state,
        EntitlementManualReviewState::Required
    );
}

#[test]
fn parent_portal_only_capability_does_not_unlock_child_runtime() {
    let decision = evaluate_entitlement_capability(EntitlementCapabilityInput {
        capability: EntitlementCapability::ScreenEvidence,
        subscription_state: SubscriptionState::Active,
        offline_grace_state: OfflineGraceState::Inactive,
        family_setup_state: FamilySetupState::Complete,
        policy_state: EntitlementPolicyState::Clean,
        capability_scope: EntitlementCapabilityScope::ParentPortalOnly,
    });

    assert_eq!(decision.access_state, EntitlementCapabilityAccessState::Blocked);
    assert_eq!(
        decision.manual_review_state,
        EntitlementManualReviewState::Required
    );
}
