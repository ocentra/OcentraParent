use ocentra_billing_core::billing_subscription::{
    decide_billing_provider_webhook, project_billing_entitlement_transition,
    project_billing_entitlement_transition_event, record_billing_provider_webhook_decision_event,
    BillingAccountMatchState, BillingAggregateId, BillingCollectionRecoveryState,
    BillingDisputeLifecycleState, BillingEntitlementScope, BillingEntitlementTransitionState,
    BillingEntitlementUpdateRequirement, BillingEntitlementWriteState,
    BillingManualReviewRequirement, BillingProviderChannel, BillingProviderEventDecisionState,
    BillingProviderEventId, BillingProviderEventKind, BillingProviderIdempotencyState,
    BillingProviderMode, BillingProviderOrderingState, BillingProviderPayloadParseState,
    BillingProviderReplayState, BillingProviderSignatureState, BillingProviderWebhookEvent,
    BillingProviderWebhookReceivedEvent, BillingRefundLifecycleState, BillingSubscriptionStatus,
    BillingTestLiveBoundaryState,
};
use ocentra_eventing::envelope::DomainEvent;
use ocentra_eventing::expect_value::ExpectValue;

const BILLING_AGGREGATE_ID: &str = "billing-household-default";
const BILLING_PROVIDER_EVENT_ID: &str = "billing-provider-event-default";
const BILLING_WEBHOOK_EVENT_TYPE: &str = "billing.provider-webhook.received";
const BILLING_DECISION_EVENT_TYPE: &str = "billing.provider-webhook.decision-recorded";
const BILLING_TRANSITION_EVENT_TYPE: &str = "billing.entitlement.transition-projected";

fn provider_event(
    subscription_status: BillingSubscriptionStatus,
    collection_recovery_state: BillingCollectionRecoveryState,
    refund_state: BillingRefundLifecycleState,
    dispute_state: BillingDisputeLifecycleState,
    signature_state: BillingProviderSignatureState,
) -> BillingProviderWebhookEvent {
    BillingProviderWebhookEvent {
        event_id: BillingProviderEventId::parse(BILLING_PROVIDER_EVENT_ID)
            .expect_value("billing provider event id"),
        provider: BillingProviderChannel::Stripe,
        mode: BillingProviderMode::Live,
        event_kind: BillingProviderEventKind::SubscriptionUpdated,
        signature_state,
        payload_parse_state: BillingProviderPayloadParseState::Parsed,
        idempotency_state: BillingProviderIdempotencyState::Fresh,
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
fn verified_active_subscription_requires_entitlement_grant_without_review() {
    let decision = decide_billing_provider_webhook(BillingProviderWebhookEvent {
        event_kind: BillingProviderEventKind::CheckoutCompleted,
        ..provider_event(
            BillingSubscriptionStatus::Active,
            BillingCollectionRecoveryState::Active,
            BillingRefundLifecycleState::None,
            BillingDisputeLifecycleState::None,
            BillingProviderSignatureState::Verified,
        )
    });
    let transition = project_billing_entitlement_transition(
        decision.clone(),
        BillingEntitlementScope::Household,
    );

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
    assert_eq!(
        transition.transition_state,
        BillingEntitlementTransitionState::GrantFullAccess
    );
    assert_eq!(
        transition.write_state,
        BillingEntitlementWriteState::WriteRequired
    );
}

#[test]
fn subscription_deleted_projects_household_revocation() {
    let decision = decide_billing_provider_webhook(BillingProviderWebhookEvent {
        event_kind: BillingProviderEventKind::SubscriptionDeleted,
        ..provider_event(
            BillingSubscriptionStatus::Cancelled,
            BillingCollectionRecoveryState::Cancelled,
            BillingRefundLifecycleState::None,
            BillingDisputeLifecycleState::None,
            BillingProviderSignatureState::Verified,
        )
    });
    let transition =
        project_billing_entitlement_transition(decision, BillingEntitlementScope::Household);

    assert_eq!(
        transition.transition_state,
        BillingEntitlementTransitionState::RevokeAccess
    );
    assert_eq!(
        transition.write_state,
        BillingEntitlementWriteState::WriteRequired
    );
}

#[test]
fn grace_lifecycle_projects_household_grace_access() {
    let decision = decide_billing_provider_webhook(BillingProviderWebhookEvent {
        event_kind: BillingProviderEventKind::SubscriptionUpdated,
        ..provider_event(
            BillingSubscriptionStatus::Grace,
            BillingCollectionRecoveryState::Grace,
            BillingRefundLifecycleState::None,
            BillingDisputeLifecycleState::None,
            BillingProviderSignatureState::Verified,
        )
    });
    let transition =
        project_billing_entitlement_transition(decision, BillingEntitlementScope::Household);

    assert_eq!(
        transition.transition_state,
        BillingEntitlementTransitionState::GraceAccess
    );
    assert_eq!(
        transition.write_state,
        BillingEntitlementWriteState::WriteRequired
    );
}

#[test]
fn supported_webhook_event_classes_follow_the_subscription_lifecycle_projection_matrix() {
    let scenarios = [
        (
            BillingProviderEventKind::SubscriptionCreated,
            BillingSubscriptionStatus::Trialing,
            BillingCollectionRecoveryState::Trialing,
            BillingRefundLifecycleState::None,
            BillingDisputeLifecycleState::None,
            BillingEntitlementTransitionState::GrantFullAccess,
        ),
        (
            BillingProviderEventKind::SubscriptionCreated,
            BillingSubscriptionStatus::Active,
            BillingCollectionRecoveryState::Active,
            BillingRefundLifecycleState::None,
            BillingDisputeLifecycleState::None,
            BillingEntitlementTransitionState::GrantFullAccess,
        ),
        (
            BillingProviderEventKind::InvoicePaid,
            BillingSubscriptionStatus::Active,
            BillingCollectionRecoveryState::Active,
            BillingRefundLifecycleState::None,
            BillingDisputeLifecycleState::None,
            BillingEntitlementTransitionState::GrantFullAccess,
        ),
        (
            BillingProviderEventKind::PaymentIntentSucceeded,
            BillingSubscriptionStatus::Active,
            BillingCollectionRecoveryState::Active,
            BillingRefundLifecycleState::None,
            BillingDisputeLifecycleState::None,
            BillingEntitlementTransitionState::GrantFullAccess,
        ),
        (
            BillingProviderEventKind::InvoicePaymentFailed,
            BillingSubscriptionStatus::PastDue,
            BillingCollectionRecoveryState::PastDue,
            BillingRefundLifecycleState::None,
            BillingDisputeLifecycleState::None,
            BillingEntitlementTransitionState::LimitAccess,
        ),
        (
            BillingProviderEventKind::PaymentIntentFailed,
            BillingSubscriptionStatus::PastDue,
            BillingCollectionRecoveryState::PastDue,
            BillingRefundLifecycleState::None,
            BillingDisputeLifecycleState::None,
            BillingEntitlementTransitionState::LimitAccess,
        ),
        (
            BillingProviderEventKind::SubscriptionUpdated,
            BillingSubscriptionStatus::Grace,
            BillingCollectionRecoveryState::Grace,
            BillingRefundLifecycleState::None,
            BillingDisputeLifecycleState::None,
            BillingEntitlementTransitionState::GraceAccess,
        ),
        (
            BillingProviderEventKind::CustomerPortalUpdated,
            BillingSubscriptionStatus::Cancelled,
            BillingCollectionRecoveryState::Cancelled,
            BillingRefundLifecycleState::None,
            BillingDisputeLifecycleState::None,
            BillingEntitlementTransitionState::RevokeAccess,
        ),
        (
            BillingProviderEventKind::CustomerPortalUpdated,
            BillingSubscriptionStatus::Cancelled,
            BillingCollectionRecoveryState::Cancelled,
            BillingRefundLifecycleState::None,
            BillingDisputeLifecycleState::DisputeLost,
            BillingEntitlementTransitionState::RevokeAccess,
        ),
        (
            BillingProviderEventKind::SubscriptionUpdated,
            BillingSubscriptionStatus::Active,
            BillingCollectionRecoveryState::Active,
            BillingRefundLifecycleState::None,
            BillingDisputeLifecycleState::DisputeWon,
            BillingEntitlementTransitionState::GrantFullAccess,
        ),
    ];

    for (
        event_kind,
        subscription_status,
        collection_recovery_state,
        refund_state,
        dispute_state,
        expected_transition_state,
    ) in scenarios
    {
        let decision = decide_billing_provider_webhook(BillingProviderWebhookEvent {
            event_kind,
            ..provider_event(
                subscription_status,
                collection_recovery_state,
                refund_state,
                dispute_state,
                BillingProviderSignatureState::Verified,
            )
        });
        let transition =
            project_billing_entitlement_transition(decision, BillingEntitlementScope::Household);

        assert_eq!(transition.subscription_status, subscription_status);
        assert_eq!(transition.transition_state, expected_transition_state);
        assert_eq!(
            transition.write_state,
            BillingEntitlementWriteState::WriteRequired
        );
    }
}

#[test]
fn remaining_lifecycle_edges_preserve_manual_review_and_write_rules() {
    let scenarios = [
        (
            BillingProviderEventKind::SubscriptionUpdated,
            BillingSubscriptionStatus::PastDue,
            BillingCollectionRecoveryState::Unpaid,
            BillingRefundLifecycleState::None,
            BillingDisputeLifecycleState::None,
            BillingEntitlementTransitionState::LimitAccess,
            BillingEntitlementWriteState::WriteRequired,
            BillingEntitlementUpdateRequirement::Required,
            BillingManualReviewRequirement::NotRequired,
        ),
        (
            BillingProviderEventKind::SubscriptionUpdated,
            BillingSubscriptionStatus::Cancelled,
            BillingCollectionRecoveryState::Cancelled,
            BillingRefundLifecycleState::RefundSettled,
            BillingDisputeLifecycleState::None,
            BillingEntitlementTransitionState::RevokeAccess,
            BillingEntitlementWriteState::WriteRequired,
            BillingEntitlementUpdateRequirement::Required,
            BillingManualReviewRequirement::NotRequired,
        ),
        (
            BillingProviderEventKind::DisputeOpened,
            BillingSubscriptionStatus::Active,
            BillingCollectionRecoveryState::SupportRequired,
            BillingRefundLifecycleState::None,
            BillingDisputeLifecycleState::DisputeOpened,
            BillingEntitlementTransitionState::HoldForReview,
            BillingEntitlementWriteState::WriteRequired,
            BillingEntitlementUpdateRequirement::Required,
            BillingManualReviewRequirement::Required,
        ),
        (
            BillingProviderEventKind::CustomerPortalUpdated,
            BillingSubscriptionStatus::Active,
            BillingCollectionRecoveryState::SupportRequired,
            BillingRefundLifecycleState::None,
            BillingDisputeLifecycleState::None,
            BillingEntitlementTransitionState::HoldForReview,
            BillingEntitlementWriteState::WriteRequired,
            BillingEntitlementUpdateRequirement::Required,
            BillingManualReviewRequirement::Required,
        ),
        (
            BillingProviderEventKind::CustomerPortalUpdated,
            BillingSubscriptionStatus::Unknown,
            BillingCollectionRecoveryState::SupportRequired,
            BillingRefundLifecycleState::None,
            BillingDisputeLifecycleState::None,
            BillingEntitlementTransitionState::NoWrite,
            BillingEntitlementWriteState::DoNotWrite,
            BillingEntitlementUpdateRequirement::NotRequired,
            BillingManualReviewRequirement::Required,
        ),
    ];

    for (
        event_kind,
        subscription_status,
        collection_recovery_state,
        refund_state,
        dispute_state,
        expected_transition_state,
        expected_write_state,
        expected_update_requirement,
        expected_manual_review_requirement,
    ) in scenarios
    {
        let decision = decide_billing_provider_webhook(BillingProviderWebhookEvent {
            event_kind,
            ..provider_event(
                subscription_status,
                collection_recovery_state,
                refund_state,
                dispute_state,
                BillingProviderSignatureState::Verified,
            )
        });
        let transition = project_billing_entitlement_transition(
            decision.clone(),
            BillingEntitlementScope::Household,
        );

        assert_eq!(
            decision.decision_state,
            BillingProviderEventDecisionState::Accepted
        );
        assert_eq!(
            decision.entitlement_update_requirement,
            expected_update_requirement
        );
        assert_eq!(
            decision.manual_review_requirement,
            expected_manual_review_requirement
        );
        assert_eq!(transition.subscription_status, subscription_status);
        assert_eq!(transition.transition_state, expected_transition_state);
        assert_eq!(transition.write_state, expected_write_state);
        assert_eq!(
            transition.manual_review_requirement,
            expected_manual_review_requirement
        );
    }
}

#[test]
fn invalid_signature_blocks_entitlement_write_and_requires_manual_review() {
    let decision = decide_billing_provider_webhook(provider_event(
        BillingSubscriptionStatus::Active,
        BillingCollectionRecoveryState::Active,
        BillingRefundLifecycleState::None,
        BillingDisputeLifecycleState::None,
        BillingProviderSignatureState::Invalid,
    ));
    let transition = project_billing_entitlement_transition(
        decision.clone(),
        BillingEntitlementScope::Household,
    );

    assert_eq!(
        decision.decision_state,
        BillingProviderEventDecisionState::Rejected
    );
    assert_eq!(
        decision.manual_review_requirement,
        BillingManualReviewRequirement::Required
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
fn webhook_decision_projects_typed_entitlement_transition_event() {
    let received = BillingProviderWebhookReceivedEvent {
        aggregate_id: BillingAggregateId::parse(BILLING_AGGREGATE_ID)
            .expect_value("billing aggregate id"),
        provider_event: provider_event(
            BillingSubscriptionStatus::PastDue,
            BillingCollectionRecoveryState::PastDue,
            BillingRefundLifecycleState::None,
            BillingDisputeLifecycleState::None,
            BillingProviderSignatureState::Verified,
        ),
    };

    let decision_event = record_billing_provider_webhook_decision_event(received.clone());
    let transition_event = project_billing_entitlement_transition_event(
        decision_event.clone(),
        BillingEntitlementScope::Household,
    );

    assert_eq!(
        received
            .contract()
            .expect_value("billing webhook contract")
            .event_type
            .as_str(),
        BILLING_WEBHOOK_EVENT_TYPE
    );
    assert_eq!(
        decision_event
            .contract()
            .expect_value("billing decision contract")
            .event_type
            .as_str(),
        BILLING_DECISION_EVENT_TYPE
    );
    assert_eq!(
        transition_event
            .contract()
            .expect_value("billing transition contract")
            .event_type
            .as_str(),
        BILLING_TRANSITION_EVENT_TYPE
    );
    assert_eq!(transition_event.aggregate_id, decision_event.aggregate_id);
    assert_eq!(
        transition_event.source_decision_id,
        decision_event.decision_id
    );
}
