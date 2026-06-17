use ocentra_billing_core::billing_subscription::{
    decide_billing_provider_webhook, project_billing_entitlement_transition,
    project_billing_entitlement_transition_event, record_billing_provider_webhook_decision_event,
    BillingAccountMatchState, BillingAggregateId, BillingEntitlementScope,
    BillingEntitlementTransitionState, BillingEntitlementUpdateRequirement,
    BillingEntitlementWriteState, BillingManualReviewRequirement, BillingProviderDuplicateState,
    BillingProviderEventDecisionState, BillingProviderEventId, BillingProviderEventKind,
    BillingProviderSignatureState, BillingProviderWebhookEvent,
    BillingProviderWebhookReceivedEvent, BillingSubscriptionLifecycleState,
};
use ocentra_eventing::envelope::DomainEvent;

fn provider_event() -> BillingProviderWebhookEvent {
    provider_event_with(
        "billing-provider-event-1",
        BillingSubscriptionLifecycleState::Active,
        BillingProviderDuplicateState::Fresh,
    )
}

fn provider_event_with(
    event_id: &str,
    lifecycle_state: BillingSubscriptionLifecycleState,
    duplicate_state: BillingProviderDuplicateState,
) -> BillingProviderWebhookEvent {
    BillingProviderWebhookEvent {
        event_id: BillingProviderEventId::parse(event_id)
            .expect("provider event ids are non-empty at the boundary"),
        event_kind: BillingProviderEventKind::SubscriptionUpdated,
        signature_state: BillingProviderSignatureState::Verified,
        duplicate_state,
        account_match_state: BillingAccountMatchState::Matched,
        lifecycle_state,
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
fn rejects_missing_signature_provider_event_without_entitlement_update() {
    let decision = decide_billing_provider_webhook(BillingProviderWebhookEvent {
        signature_state: BillingProviderSignatureState::Missing,
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
fn rejects_account_mismatched_provider_event_pending_manual_review() {
    let decision = decide_billing_provider_webhook(BillingProviderWebhookEvent {
        account_match_state: BillingAccountMatchState::Mismatched,
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
fn grace_lifecycle_stays_accepted_and_projects_an_entitlement_write() {
    let decision = decide_billing_provider_webhook(BillingProviderWebhookEvent {
        event_kind: BillingProviderEventKind::SubscriptionUpdated,
        lifecycle_state: BillingSubscriptionLifecycleState::Grace,
        ..provider_event()
    });

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
fn support_required_lifecycle_blocks_automatic_updates_pending_manual_review() {
    let decision = decide_billing_provider_webhook(BillingProviderWebhookEvent {
        event_kind: BillingProviderEventKind::CustomerPortalUpdated,
        lifecycle_state: BillingSubscriptionLifecycleState::SupportRequired,
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

#[test]
fn replayed_provider_event_reuses_idempotency_chain_and_blocks_double_grant() {
    let aggregate_id =
        BillingAggregateId::parse("billing-household-default").expect("billing aggregate");
    let fresh_received = BillingProviderWebhookReceivedEvent {
        aggregate_id: aggregate_id.clone(),
        provider_event: provider_event_with(
            "billing-provider-event-replay",
            BillingSubscriptionLifecycleState::Active,
            BillingProviderDuplicateState::Fresh,
        ),
    };
    let replayed_received = BillingProviderWebhookReceivedEvent {
        aggregate_id,
        provider_event: provider_event_with(
            "billing-provider-event-replay",
            BillingSubscriptionLifecycleState::Active,
            BillingProviderDuplicateState::Duplicate,
        ),
    };

    let fresh_decision = record_billing_provider_webhook_decision_event(fresh_received.clone());
    let replayed_decision =
        record_billing_provider_webhook_decision_event(replayed_received.clone());
    let fresh_transition = project_billing_entitlement_transition_event(
        fresh_decision.clone(),
        BillingEntitlementScope::Household,
    );
    let replayed_transition = project_billing_entitlement_transition_event(
        replayed_decision.clone(),
        BillingEntitlementScope::Household,
    );

    assert_eq!(
        fresh_received
            .idempotency_key()
            .expect("fresh webhook idempotency key")
            .as_str(),
        replayed_received
            .idempotency_key()
            .expect("replayed webhook idempotency key")
            .as_str()
    );
    assert_eq!(fresh_decision.decision_id, replayed_decision.decision_id);
    assert_eq!(
        fresh_decision
            .idempotency_key()
            .expect("fresh decision idempotency key")
            .as_str(),
        replayed_decision
            .idempotency_key()
            .expect("replayed decision idempotency key")
            .as_str()
    );
    assert_eq!(
        fresh_transition.transition_id,
        replayed_transition.transition_id
    );
    assert_eq!(
        fresh_transition
            .idempotency_key()
            .expect("fresh transition idempotency key")
            .as_str(),
        replayed_transition
            .idempotency_key()
            .expect("replayed transition idempotency key")
            .as_str()
    );
    assert_eq!(
        fresh_transition.transition.transition_state,
        BillingEntitlementTransitionState::GrantFullAccess
    );
    assert_eq!(
        replayed_decision.decision.decision_state,
        BillingProviderEventDecisionState::Rejected
    );
    assert_eq!(
        replayed_decision.decision.manual_review_requirement,
        BillingManualReviewRequirement::Required
    );
    assert_eq!(
        replayed_transition.transition.transition_state,
        BillingEntitlementTransitionState::NoWrite
    );
    assert_eq!(
        replayed_transition.transition.write_state,
        BillingEntitlementWriteState::DoNotWrite
    );
}
