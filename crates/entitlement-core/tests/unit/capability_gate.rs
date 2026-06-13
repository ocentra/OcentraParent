use ocentra_entitlement_core::{
    evaluate_entitlement_capability, record_entitlement_capability_decision,
    EntitlementAggregateId, EntitlementCapability, EntitlementCapabilityAccessState,
    EntitlementCapabilityEvaluationRequestedEvent, EntitlementCapabilityInput,
    EntitlementCapabilityScope, EntitlementEvaluationId, EntitlementManualReviewState,
    EntitlementPolicyState, FamilySetupState, OfflineGraceState, SubscriptionState,
};
use ocentra_eventing::DomainEvent;

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

    assert_eq!(
        decision.access_state,
        EntitlementCapabilityAccessState::Allowed
    );
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

    assert_eq!(
        decision.access_state,
        EntitlementCapabilityAccessState::Allowed
    );
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

    assert_eq!(
        decision.access_state,
        EntitlementCapabilityAccessState::Blocked
    );
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

    assert_eq!(
        decision.access_state,
        EntitlementCapabilityAccessState::Blocked
    );
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

    assert_eq!(
        decision.access_state,
        EntitlementCapabilityAccessState::Blocked
    );
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

    assert_eq!(
        decision.access_state,
        EntitlementCapabilityAccessState::Blocked
    );
    assert_eq!(
        decision.manual_review_state,
        EntitlementManualReviewState::Required
    );
}

#[test]
fn entitlement_evaluation_request_records_typed_decision_event() {
    let request = EntitlementCapabilityEvaluationRequestedEvent {
        aggregate_id: EntitlementAggregateId::parse("entitlement-family-default")
            .expect("entitlement aggregate"),
        evaluation_id: EntitlementEvaluationId::parse("entitlement-evaluation-default")
            .expect("entitlement evaluation"),
        input: EntitlementCapabilityInput {
            capability: EntitlementCapability::Tracking,
            subscription_state: SubscriptionState::Active,
            offline_grace_state: OfflineGraceState::Inactive,
            family_setup_state: FamilySetupState::Complete,
            policy_state: EntitlementPolicyState::Clean,
            capability_scope: EntitlementCapabilityScope::LocalChildRuntime,
        },
    };

    let decision = record_entitlement_capability_decision(&request);

    assert_eq!(decision.aggregate_id, request.aggregate_id);
    assert_eq!(decision.source_evaluation_id, request.evaluation_id);
    assert_eq!(
        decision.decision.capability,
        EntitlementCapability::Tracking
    );
    assert_eq!(
        decision.decision.access_state,
        EntitlementCapabilityAccessState::Allowed
    );
    assert_eq!(
        request
            .contract()
            .expect("entitlement request contract")
            .event_type
            .as_str(),
        "entitlement.capability-evaluation.requested"
    );
    assert_eq!(
        decision
            .contract()
            .expect("entitlement decision contract")
            .event_type
            .as_str(),
        "entitlement.capability-decision.recorded"
    );
}
