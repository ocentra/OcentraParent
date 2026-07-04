use ocentra_billing_core::billing_subscription::{
    decide_billing_provider_webhook, project_billing_entitlement_transition,
    project_billing_entitlement_transition_event, record_billing_provider_webhook_decision_event,
    BillingAccountMatchState, BillingAggregateId, BillingCollectionRecoveryState,
    BillingDisputeLifecycleState, BillingEntitlementScope, BillingEntitlementTransitionState,
    BillingEntitlementUpdateRequirement, BillingEntitlementWriteState,
    BillingManualReviewRequirement, BillingProviderChannel, BillingProviderDeadLetterState,
    BillingProviderEventDecisionState, BillingProviderEventId, BillingProviderEventKind,
    BillingProviderIdempotencyState, BillingProviderMode, BillingProviderOrderingState,
    BillingProviderPayloadParseState, BillingProviderReconciliationState,
    BillingProviderReplayState, BillingProviderRetryState, BillingProviderSignatureState,
    BillingProviderWebhookEvent, BillingProviderWebhookReceivedEvent, BillingRefundLifecycleState,
    BillingSubscriptionStatus, BillingTestLiveBoundaryState,
};
use ocentra_eventing::envelope::DomainEvent;
use ocentra_eventing::expect_value::ExpectValue;

fn provider_event() -> BillingProviderWebhookEvent {
    provider_event_with(
        BillingProviderEventId::parse("billing-provider-event-1")
            .expect_value("provider event ids are non-empty at the boundary"),
        BillingSubscriptionStatus::Active,
        BillingCollectionRecoveryState::Active,
        BillingRefundLifecycleState::None,
        BillingDisputeLifecycleState::None,
        BillingProviderIdempotencyState::Fresh,
    )
}

fn provider_event_with(
    event_id: BillingProviderEventId,
    subscription_status: BillingSubscriptionStatus,
    collection_recovery_state: BillingCollectionRecoveryState,
    refund_state: BillingRefundLifecycleState,
    dispute_state: BillingDisputeLifecycleState,
    idempotency_state: BillingProviderIdempotencyState,
) -> BillingProviderWebhookEvent {
    BillingProviderWebhookEvent {
        event_id,
        provider: BillingProviderChannel::Stripe,
        mode: BillingProviderMode::Live,
        event_kind: BillingProviderEventKind::SubscriptionUpdated,
        signature_state: BillingProviderSignatureState::Verified,
        payload_parse_state: BillingProviderPayloadParseState::Parsed,
        idempotency_state,
        replay_state: BillingProviderReplayState::Fresh,
        ordering_state: BillingProviderOrderingState::InOrder,
        account_match_state: BillingAccountMatchState::Matched,
        test_live_boundary_state: BillingTestLiveBoundaryState::Isolated,
        subscription_status,
        collection_recovery_state,
        refund_state,
        dispute_state,
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
        idempotency_state: BillingProviderIdempotencyState::Duplicate,
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
    assert_eq!(
        decision.dead_letter_state,
        BillingProviderDeadLetterState::NotRequired
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
    assert_eq!(
        decision.dead_letter_state,
        BillingProviderDeadLetterState::ManualRequired
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
        subscription_status: BillingSubscriptionStatus::Active,
        collection_recovery_state: BillingCollectionRecoveryState::SupportRequired,
        dispute_state: BillingDisputeLifecycleState::DisputeOpened,
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
        subscription_status: BillingSubscriptionStatus::Grace,
        collection_recovery_state: BillingCollectionRecoveryState::Grace,
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
        refund_state: BillingRefundLifecycleState::RefundIssued,
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
    assert_eq!(
        decision.reconciliation_state,
        BillingProviderReconciliationState::QueueRequired
    );
}

#[test]
fn support_required_recovery_projects_manual_review_instead_of_silent_no_write() {
    let decision = decide_billing_provider_webhook(BillingProviderWebhookEvent {
        event_kind: BillingProviderEventKind::CustomerPortalUpdated,
        collection_recovery_state: BillingCollectionRecoveryState::SupportRequired,
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
        BillingManualReviewRequirement::Required
    );
}

#[test]
fn verified_unknown_subscription_status_blocks_entitlement_write_pending_manual_review() {
    let decision = decide_billing_provider_webhook(BillingProviderWebhookEvent {
        event_kind: BillingProviderEventKind::CustomerPortalUpdated,
        subscription_status: BillingSubscriptionStatus::Unknown,
        collection_recovery_state: BillingCollectionRecoveryState::SupportRequired,
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
fn verified_provider_channels_remain_accepted_across_stripe_razorpay_and_paypal() {
    let providers = [
        BillingProviderChannel::Stripe,
        BillingProviderChannel::Razorpay,
        BillingProviderChannel::PayPal,
    ];

    for provider in providers {
        let decision = decide_billing_provider_webhook(BillingProviderWebhookEvent {
            provider,
            ..provider_event()
        });

        assert_eq!(
            decision.decision_state,
            BillingProviderEventDecisionState::Accepted
        );
        assert_eq!(decision.provider, provider);
        assert_eq!(
            decision.payload_parse_state,
            BillingProviderPayloadParseState::Parsed
        );
    }
}

#[test]
fn malformed_provider_payload_is_dead_lettered_before_any_entitlement_write() {
    let decision = decide_billing_provider_webhook(BillingProviderWebhookEvent {
        payload_parse_state: BillingProviderPayloadParseState::Malformed,
        ..provider_event()
    });
    let transition = project_billing_entitlement_transition(
        decision.clone(),
        BillingEntitlementScope::Household,
    );

    assert_eq!(
        decision.decision_state,
        BillingProviderEventDecisionState::Rejected
    );
    assert_eq!(
        decision.dead_letter_state,
        BillingProviderDeadLetterState::ManualRequired
    );
    assert_eq!(
        decision.entitlement_update_requirement,
        BillingEntitlementUpdateRequirement::NotRequired
    );
    assert_eq!(
        transition.transition_state,
        BillingEntitlementTransitionState::NoWrite
    );
    assert_eq!(
        transition.write_state,
        BillingEntitlementWriteState::DoNotWrite
    );
}

#[test]
fn mixed_test_live_boundary_is_rejected_before_any_ledger_write() {
    let decision = decide_billing_provider_webhook(BillingProviderWebhookEvent {
        mode: BillingProviderMode::Test,
        test_live_boundary_state: BillingTestLiveBoundaryState::MixedBlocked,
        ..provider_event()
    });

    assert_eq!(
        decision.decision_state,
        BillingProviderEventDecisionState::Rejected
    );
    assert_eq!(
        decision.test_live_boundary_state,
        BillingTestLiveBoundaryState::MixedBlocked
    );
    assert_eq!(
        decision.dead_letter_state,
        BillingProviderDeadLetterState::ManualRequired
    );
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
            subscription_status: BillingSubscriptionStatus::PastDue,
            collection_recovery_state: BillingCollectionRecoveryState::PastDue,
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
            .expect_value("billing aggregate"),
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
            .expect_value("billing received contract")
            .event_type
            .as_str(),
        "billing.provider-webhook.received"
    );
    assert_eq!(
        transition
            .contract()
            .expect_value("billing transition contract")
            .event_type
            .as_str(),
        "billing.entitlement.transition-projected"
    );
}

#[test]
fn replayed_provider_event_reuses_idempotency_chain_and_blocks_double_grant() {
    let aggregate_id =
        BillingAggregateId::parse("billing-household-default").expect_value("billing aggregate");
    let fresh_received = BillingProviderWebhookReceivedEvent {
        aggregate_id: aggregate_id.clone(),
        provider_event: provider_event_with(
            BillingProviderEventId::parse("billing-provider-event-replay")
                .expect_value("provider event ids are non-empty at the boundary"),
            BillingSubscriptionStatus::Active,
            BillingCollectionRecoveryState::Active,
            BillingRefundLifecycleState::None,
            BillingDisputeLifecycleState::None,
            BillingProviderIdempotencyState::Fresh,
        ),
    };
    let replayed_received = BillingProviderWebhookReceivedEvent {
        aggregate_id,
        provider_event: provider_event_with(
            BillingProviderEventId::parse("billing-provider-event-replay")
                .expect_value("provider event ids are non-empty at the boundary"),
            BillingSubscriptionStatus::Active,
            BillingCollectionRecoveryState::Active,
            BillingRefundLifecycleState::None,
            BillingDisputeLifecycleState::None,
            BillingProviderIdempotencyState::Duplicate,
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
            .expect_value("fresh webhook idempotency key")
            .as_str(),
        replayed_received
            .idempotency_key()
            .expect_value("replayed webhook idempotency key")
            .as_str()
    );
    assert_eq!(fresh_decision.decision_id, replayed_decision.decision_id);
    assert_eq!(
        fresh_decision
            .idempotency_key()
            .expect_value("fresh decision idempotency key")
            .as_str(),
        replayed_decision
            .idempotency_key()
            .expect_value("replayed decision idempotency key")
            .as_str()
    );
    assert_eq!(
        fresh_transition.transition_id,
        replayed_transition.transition_id
    );
    assert_eq!(
        fresh_transition
            .idempotency_key()
            .expect_value("fresh transition idempotency key")
            .as_str(),
        replayed_transition
            .idempotency_key()
            .expect_value("replayed transition idempotency key")
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
        replayed_decision.decision.reconciliation_state,
        BillingProviderReconciliationState::NotRequired
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

#[test]
fn out_of_order_provider_event_requires_reconciliation_and_blocks_double_grant() {
    let decision = decide_billing_provider_webhook(BillingProviderWebhookEvent {
        ordering_state: BillingProviderOrderingState::OutOfOrder,
        ..provider_event()
    });
    let transition = project_billing_entitlement_transition(
        decision.clone(),
        BillingEntitlementScope::Household,
    );

    assert_eq!(
        decision.decision_state,
        BillingProviderEventDecisionState::Rejected
    );
    assert_eq!(
        decision.reconciliation_state,
        BillingProviderReconciliationState::QueueRequired
    );
    assert_eq!(
        transition.transition_state,
        BillingEntitlementTransitionState::NoWrite
    );
    assert_eq!(
        transition.write_state,
        BillingEntitlementWriteState::DoNotWrite
    );
}

#[test]
fn payment_failure_queues_retry_follow_up_without_skipping_projection() {
    let decision = decide_billing_provider_webhook(BillingProviderWebhookEvent {
        event_kind: BillingProviderEventKind::PaymentIntentFailed,
        subscription_status: BillingSubscriptionStatus::PastDue,
        collection_recovery_state: BillingCollectionRecoveryState::PastDue,
        ..provider_event()
    });
    let transition = project_billing_entitlement_transition(
        decision.clone(),
        BillingEntitlementScope::Household,
    );

    assert_eq!(
        decision.retry_state,
        BillingProviderRetryState::QueueRequired
    );
    assert_eq!(
        decision.decision_state,
        BillingProviderEventDecisionState::Accepted
    );
    assert_eq!(
        transition.transition_state,
        BillingEntitlementTransitionState::LimitAccess
    );
    assert_eq!(
        transition.write_state,
        BillingEntitlementWriteState::WriteRequired
    );
}
