use ocentra_billing_core::{
    decide_billing_provider_webhook, project_billing_entitlement_transition,
    project_billing_entitlement_transition_event, record_billing_provider_webhook_decision_event,
    BillingAccountMatchState, BillingAggregateId, BillingEntitlementScope,
    BillingEntitlementTransitionState, BillingEntitlementUpdateRequirement,
    BillingEntitlementWriteState, BillingManualReviewRequirement, BillingProviderDuplicateState,
    BillingProviderEventDecisionState, BillingProviderEventId, BillingProviderEventKind,
    BillingProviderSignatureState, BillingProviderWebhookEvent,
    BillingProviderWebhookReceivedEvent, BillingSubscriptionLifecycleState,
};
use ocentra_eventing::DomainEvent;

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
fn refund_event_is_safe_but_does_not_auto_update_entitlement() {
    let decision = decide_billing_provider_webhook(BillingProviderWebhookEvent {
        event_kind: BillingProviderEventKind::RefundIssued,
        lifecycle_state: BillingSubscriptionLifecycleState::Canceled,
        ..provider_event()
    });

    assert_eq!(
        decision.decision_state,
        BillingProviderEventDecisionState::Accepted
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
fn verified_unknown_lifecycle_blocks_entitlement_write_pending_manual_review() {
    let decision = decide_billing_provider_webhook(BillingProviderWebhookEvent {
        event_kind: BillingProviderEventKind::CustomerPortalUpdated,
        lifecycle_state: BillingSubscriptionLifecycleState::Unknown,
        ..provider_event()
    });

    assert_eq!(
        decision.decision_state,
        BillingProviderEventDecisionState::Accepted
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
fn provider_event_id_rejects_empty_boundary_text() {
    assert!(BillingProviderEventId::parse("").is_none());
}

#[test]
fn active_subscription_projects_household_entitlement_grant() {
    let transition = project_billing_entitlement_transition(
        decide_billing_provider_webhook(provider_event()),
        BillingEntitlementScope::Household,
    );

    assert_eq!(transition.scope, BillingEntitlementScope::Household);
    assert_eq!(
        transition.transition_state,
        BillingEntitlementTransitionState::GrantFullAccess
    );
    assert_eq!(
        transition.write_state,
        BillingEntitlementWriteState::WriteRequired
    );
    assert_eq!(
        transition.manual_review_requirement,
        BillingManualReviewRequirement::NotRequired
    );
}

#[test]
fn past_due_subscription_projects_limited_child_device_entitlement() {
    let transition = project_billing_entitlement_transition(
        decide_billing_provider_webhook(BillingProviderWebhookEvent {
            lifecycle_state: BillingSubscriptionLifecycleState::PastDue,
            ..provider_event()
        }),
        BillingEntitlementScope::ChildDevice,
    );

    assert_eq!(transition.scope, BillingEntitlementScope::ChildDevice);
    assert_eq!(
        transition.transition_state,
        BillingEntitlementTransitionState::LimitAccess
    );
    assert_eq!(
        transition.write_state,
        BillingEntitlementWriteState::WriteRequired
    );
}

#[test]
fn rejected_provider_event_projects_no_entitlement_write() {
    let transition = project_billing_entitlement_transition(
        decide_billing_provider_webhook(BillingProviderWebhookEvent {
            signature_state: BillingProviderSignatureState::Invalid,
            ..provider_event()
        }),
        BillingEntitlementScope::Household,
    );

    assert_eq!(
        transition.transition_state,
        BillingEntitlementTransitionState::NoWrite
    );
    assert_eq!(
        transition.write_state,
        BillingEntitlementWriteState::DoNotWrite
    );
    assert_eq!(
        transition.manual_review_requirement,
        BillingManualReviewRequirement::Required
    );
}

#[test]
fn provider_webhook_event_flow_records_decision_and_projects_entitlement_transition() {
    let received = BillingProviderWebhookReceivedEvent {
        aggregate_id: BillingAggregateId::parse("billing-household-default")
            .expect("billing aggregate"),
        provider_event: provider_event(),
    };

    let decision = record_billing_provider_webhook_decision_event(received.clone());
    let transition = project_billing_entitlement_transition_event(
        decision.clone(),
        BillingEntitlementScope::Household,
    );

    assert_eq!(decision.aggregate_id, received.aggregate_id);
    assert_eq!(
        decision.decision.decision_state,
        BillingProviderEventDecisionState::Accepted
    );
    assert_eq!(transition.aggregate_id, decision.aggregate_id);
    assert_eq!(transition.source_decision_id, decision.decision_id);
    assert_eq!(
        transition.transition.transition_state,
        BillingEntitlementTransitionState::GrantFullAccess
    );
    assert_eq!(
        received
            .contract()
            .expect("billing received contract")
            .event_type
            .as_str(),
        "billing.provider-webhook.received"
    );
    assert_eq!(
        transition
            .contract()
            .expect("billing transition contract")
            .event_type
            .as_str(),
        "billing.entitlement.transition-projected"
    );
}
