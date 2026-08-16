use crate::billing_subscription::{
    BillingCollectionRecoveryState, BillingDecisionId, BillingDisputeLifecycleState,
    BillingEntitlementScope, BillingEntitlementTransition,
    BillingEntitlementTransitionProjectedEvent, BillingEntitlementTransitionState,
    BillingEntitlementUpdateRequirement, BillingEntitlementWriteState,
    BillingProviderEventDecisionState, BillingProviderWebhookDecision,
    BillingProviderWebhookDecisionRecordedEvent, BillingSubscriptionStatus, BillingTransitionId,
};
use ocentra_eventing::expect_value::ExpectValue;

const BILLING_DECISION_PREFIX: &str = "billing-decision:";
const BILLING_TRANSITION_PREFIX: &str = "billing-transition:";
const ERROR_BILLING_TRANSITION_ID: &str = "billing transition id";

pub(crate) fn project_billing_entitlement_transition(
    decision: BillingProviderWebhookDecision,
    scope: BillingEntitlementScope,
) -> BillingEntitlementTransition {
    let write_allowed = matches!(
        (
            decision.decision_state,
            decision.entitlement_update_requirement,
        ),
        (
            BillingProviderEventDecisionState::Accepted,
            BillingEntitlementUpdateRequirement::Required,
        )
    );
    let transition_state = billing_transition_state(&decision, write_allowed);

    BillingEntitlementTransition {
        event_id: decision.event_id,
        scope,
        subscription_status: decision.subscription_status,
        collection_recovery_state: decision.collection_recovery_state,
        refund_state: decision.refund_state,
        dispute_state: decision.dispute_state,
        transition_state,
        write_state: if write_allowed {
            BillingEntitlementWriteState::WriteRequired
        } else {
            BillingEntitlementWriteState::DoNotWrite
        },
        manual_review_requirement: decision.manual_review_requirement,
    }
}

pub(crate) fn project_billing_entitlement_transition_event(
    event: BillingProviderWebhookDecisionRecordedEvent,
    scope: BillingEntitlementScope,
) -> BillingEntitlementTransitionProjectedEvent {
    let decision_id = event.decision_id.clone();
    BillingEntitlementTransitionProjectedEvent {
        aggregate_id: event.aggregate_id,
        transition_id: BillingTransitionId::parse(billing_transition_ref(&decision_id))
            .expect_value(ERROR_BILLING_TRANSITION_ID),
        source_decision_id: decision_id,
        transition: project_billing_entitlement_transition(event.decision, scope),
    }
}

fn billing_transition_state(
    decision: &BillingProviderWebhookDecision,
    write_allowed: bool,
) -> BillingEntitlementTransitionState {
    if !write_allowed {
        BillingEntitlementTransitionState::NoWrite
    } else if matches!(
        decision.collection_recovery_state,
        BillingCollectionRecoveryState::SupportRequired
    ) || matches!(
        decision.dispute_state,
        BillingDisputeLifecycleState::DisputeOpened
    ) {
        BillingEntitlementTransitionState::HoldForReview
    } else {
        match decision.subscription_status {
            BillingSubscriptionStatus::Trialing | BillingSubscriptionStatus::Active => {
                BillingEntitlementTransitionState::GrantFullAccess
            }
            BillingSubscriptionStatus::Grace => BillingEntitlementTransitionState::GraceAccess,
            BillingSubscriptionStatus::PastDue => BillingEntitlementTransitionState::LimitAccess,
            BillingSubscriptionStatus::Cancelled | BillingSubscriptionStatus::Expired => {
                BillingEntitlementTransitionState::RevokeAccess
            }
            BillingSubscriptionStatus::Unknown | BillingSubscriptionStatus::Unavailable => {
                BillingEntitlementTransitionState::NoWrite
            }
        }
    }
}

fn billing_transition_ref(decision_id: &BillingDecisionId) -> String {
    let mut value = String::from(BILLING_TRANSITION_PREFIX);
    value.push_str(decision_id.as_str());
    value
}

pub(crate) fn billing_decision_ref(event_id: &str) -> String {
    let mut value = String::from(BILLING_DECISION_PREFIX);
    value.push_str(event_id);
    value
}
