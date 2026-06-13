use ocentra_billing_core::{
    decide_billing_provider_webhook, BillingAccountMatchState,
    BillingEntitlementUpdateRequirement, BillingManualReviewRequirement,
    BillingProviderDuplicateState, BillingProviderEventDecisionState, BillingProviderEventId,
    BillingProviderEventKind, BillingProviderSignatureState, BillingProviderWebhookEvent,
    BillingSubscriptionLifecycleState,
};

fn provider_event() -> BillingProviderWebhookEvent {
    BillingProviderWebhookEvent {
        event_id: BillingProviderEventId::parse("billing-provider-event-1")
            .expect("provider event ids are non-empty at the boundary"),
        event_kind: BillingProviderEventKind::SubscriptionUpdated,
        signature_state: BillingProviderSignatureState::Verified,
        duplicate_state: BillingProviderDuplicateState::Fresh,
        account_match_state: BillingAccountMatchState::Matched,
        lifecycle_state: BillingSubscriptionLifecycleState::Active,
    }
}

#[test]
fn accepts_verified_fresh_account_matched_provider_event() {
    let decision = decide_billing_provider_webhook(provider_event());

    assert_eq!(
        decision.decision_state,
        BillingProviderEventDecisionState::Accepted
    );
    assert_eq!(
        decision.entitlement_update_requirement,
        BillingEntitlementUpdateRequirement::Required
    );
    assert_eq!(
        decision.manual_review_requirement,
        BillingManualReviewRequirement::NotRequired
    );
}

#[test]
fn rejects_duplicate_provider_event_without_entitlement_update() {
    let decision = decide_billing_provider_webhook(BillingProviderWebhookEvent {
        duplicate_state: BillingProviderDuplicateState::Duplicate,
        ..provider_event()
    });

    assert_eq!(
        decision.decision_state,
        BillingProviderEventDecisionState::Rejected
    );
    assert_eq!(
        decision.entitlement_update_requirement,
        BillingEntitlementUpdateRequirement::NotRequired
    );
    assert_eq!(
        decision.manual_review_requirement,
        BillingManualReviewRequirement::Required
    );
}

#[test]
fn dispute_provider_event_requires_manual_review() {
    let decision = decide_billing_provider_webhook(BillingProviderWebhookEvent {
        event_kind: BillingProviderEventKind::DisputeOpened,
        lifecycle_state: BillingSubscriptionLifecycleState::Disputed,
        ..provider_event()
    });

    assert_eq!(
        decision.decision_state,
        BillingProviderEventDecisionState::Accepted
    );
    assert_eq!(
        decision.manual_review_requirement,
        BillingManualReviewRequirement::Required
    );
}

#[test]
fn provider_event_id_rejects_empty_boundary_text() {
    assert!(BillingProviderEventId::parse("").is_none());
}
