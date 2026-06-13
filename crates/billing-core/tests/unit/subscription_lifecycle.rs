use ocentra_billing_core::{
    decide_billing_provider_webhook, project_billing_entitlement_transition,
    project_billing_entitlement_transition_event, record_billing_provider_webhook_decision_event,
    BillingAccountMatchState, BillingAggregateId, BillingEntitlementScope,
    BillingEntitlementTransitionState, BillingEntitlementUpdateRequirement,
    BillingEntitlementWriteState, BillingManualReviewRequirement, BillingProviderDuplicateState,
    BillingProviderEventDecisionState, BillingProviderEventId, BillingProviderEventKind,
    BillingProviderSignatureState, BillingProviderWebhookEvent, BillingProviderWebhookReceivedEvent,
    BillingSubscriptionLifecycleState,
};
use ocentra_eventing::DomainEvent;

const BILLING_AGGREGATE_ID: &str = "billing-household-default";
const BILLING_PROVIDER_EVENT_ID: &str = "billing-provider-event-default";
const BILLING_WEBHOOK_EVENT_TYPE: &str = "billing.provider-webhook.received";
const BILLING_DECISION_EVENT_TYPE: &str = "billing.provider-webhook.decision-recorded";
const BILLING_TRANSITION_EVENT_TYPE: &str = "billing.entitlement.transition-projected";

fn provider_event(
    lifecycle_state: BillingSubscriptionLifecycleState,
    signature_state: BillingProviderSignatureState,
) -> BillingProviderWebhookEvent {
    BillingProviderWebhookEvent {
        event_id: BillingProviderEventId::parse(BILLING_PROVIDER_EVENT_ID)
            .expect("billing provider event id"),
        event_kind: BillingProviderEventKind::SubscriptionUpdated,
        signature_state,
        duplicate_state: BillingProviderDuplicateState::Fresh,
        account_match_state: BillingAccountMatchState::Matched,
        lifecycle_state,
    }
}

#[test]
fn verified_active_subscription_requires_entitlement_grant_without_review() {
    let decision = decide_billing_provider_webhook(BillingProviderWebhookEvent {
        event_kind: BillingProviderEventKind::CheckoutCompleted,
        ..provider_event(
            BillingSubscriptionLifecycleState::Active,
            BillingProviderSignatureState::Verified,
        )
    });
    let transition =
        project_billing_entitlement_transition(decision.clone(), BillingEntitlementScope::Household);

    assert_eq!(decision.decision_state, BillingProviderEventDecisionState::Accepted);
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
    assert_eq!(transition.write_state, BillingEntitlementWriteState::WriteRequired);
}

#[test]
fn subscription_deleted_projects_household_revocation() {
    let decision = decide_billing_provider_webhook(BillingProviderWebhookEvent {
        event_kind: BillingProviderEventKind::SubscriptionDeleted,
        ..provider_event(
            BillingSubscriptionLifecycleState::Canceled,
            BillingProviderSignatureState::Verified,
        )
    });
    let transition =
        project_billing_entitlement_transition(decision, BillingEntitlementScope::Household);

    assert_eq!(
        transition.transition_state,
        BillingEntitlementTransitionState::RevokeAccess
    );
    assert_eq!(transition.write_state, BillingEntitlementWriteState::WriteRequired);
}

#[test]
fn supported_webhook_event_classes_follow_the_subscription_lifecycle_projection_matrix() {
    let scenarios = [
        (
            BillingProviderEventKind::SubscriptionCreated,
            BillingSubscriptionLifecycleState::Active,
            BillingEntitlementTransitionState::GrantFullAccess,
        ),
        (
            BillingProviderEventKind::InvoicePaid,
            BillingSubscriptionLifecycleState::Active,
            BillingEntitlementTransitionState::GrantFullAccess,
        ),
        (
            BillingProviderEventKind::PaymentIntentSucceeded,
            BillingSubscriptionLifecycleState::Active,
            BillingEntitlementTransitionState::GrantFullAccess,
        ),
        (
            BillingProviderEventKind::InvoicePaymentFailed,
            BillingSubscriptionLifecycleState::PastDue,
            BillingEntitlementTransitionState::LimitAccess,
        ),
        (
            BillingProviderEventKind::PaymentIntentFailed,
            BillingSubscriptionLifecycleState::PastDue,
            BillingEntitlementTransitionState::LimitAccess,
        ),
        (
            BillingProviderEventKind::CustomerPortalUpdated,
            BillingSubscriptionLifecycleState::Canceled,
            BillingEntitlementTransitionState::RevokeAccess,
        ),
    ];

    for (event_kind, lifecycle_state, expected_transition_state) in scenarios {
        let decision = decide_billing_provider_webhook(BillingProviderWebhookEvent {
            event_kind,
            ..provider_event(lifecycle_state, BillingProviderSignatureState::Verified)
        });
        let transition =
            project_billing_entitlement_transition(decision, BillingEntitlementScope::Household);

        assert_eq!(transition.lifecycle_state, lifecycle_state);
        assert_eq!(transition.transition_state, expected_transition_state);
        assert_eq!(transition.write_state, BillingEntitlementWriteState::WriteRequired);
    }
}

#[test]
fn invalid_signature_blocks_entitlement_write_and_requires_manual_review() {
    let decision = decide_billing_provider_webhook(provider_event(
        BillingSubscriptionLifecycleState::Active,
        BillingProviderSignatureState::Invalid,
    ));
    let transition =
        project_billing_entitlement_transition(decision.clone(), BillingEntitlementScope::Household);

    assert_eq!(decision.decision_state, BillingProviderEventDecisionState::Rejected);
    assert_eq!(
        decision.manual_review_requirement,
        BillingManualReviewRequirement::Required
    );
    assert_eq!(transition.transition_state, BillingEntitlementTransitionState::NoWrite);
    assert_eq!(transition.write_state, BillingEntitlementWriteState::DoNotWrite);
}

#[test]
fn webhook_decision_projects_typed_entitlement_transition_event() {
    let received = BillingProviderWebhookReceivedEvent {
        aggregate_id: BillingAggregateId::parse(BILLING_AGGREGATE_ID)
            .expect("billing aggregate id"),
        provider_event: provider_event(
            BillingSubscriptionLifecycleState::PastDue,
            BillingProviderSignatureState::Verified,
        ),
    };

    let decision_event = record_billing_provider_webhook_decision_event(received.clone());
    let transition_event =
        project_billing_entitlement_transition_event(decision_event.clone(), BillingEntitlementScope::Household);

    assert_eq!(
        received
            .contract()
            .expect("billing webhook contract")
            .event_type
            .as_str(),
        BILLING_WEBHOOK_EVENT_TYPE
    );
    assert_eq!(
        decision_event
            .contract()
            .expect("billing decision contract")
            .event_type
            .as_str(),
        BILLING_DECISION_EVENT_TYPE
    );
    assert_eq!(
        transition_event
            .contract()
            .expect("billing transition contract")
            .event_type
            .as_str(),
        BILLING_TRANSITION_EVENT_TYPE
    );
    assert_eq!(transition_event.aggregate_id, decision_event.aggregate_id);
    assert_eq!(transition_event.source_decision_id, decision_event.decision_id);
}
