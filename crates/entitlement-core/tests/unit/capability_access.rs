use ocentra_entitlement_core::{
    evaluate_entitlement_capability, record_entitlement_capability_decision, EntitlementAggregateId,
    EntitlementCapability, EntitlementCapabilityAccessState,
    EntitlementCapabilityEvaluationRequestedEvent, EntitlementCapabilityInput,
    EntitlementCapabilityScope, EntitlementDecisionId, EntitlementEvaluationId,
    EntitlementManualReviewState, EntitlementPolicyState, FamilySetupState, OfflineGraceState,
    SubscriptionState,
};
use ocentra_eventing::DomainEvent;

const ENTITLEMENT_AGGREGATE_ID: &str = "entitlement-household-default";
const ENTITLEMENT_EVALUATION_ID: &str = "entitlement-evaluation-default";
const ENTITLEMENT_REQUESTED_EVENT_TYPE: &str = "entitlement.capability-evaluation.requested";
const ENTITLEMENT_DECISION_EVENT_TYPE: &str = "entitlement.capability-decision.recorded";

fn entitlement_input(capability_scope: EntitlementCapabilityScope) -> EntitlementCapabilityInput {
    EntitlementCapabilityInput {
        capability: EntitlementCapability::Tracking,
        subscription_state: SubscriptionState::Active,
        offline_grace_state: OfflineGraceState::Inactive,
        family_setup_state: FamilySetupState::Complete,
        policy_state: EntitlementPolicyState::Clean,
        capability_scope,
    }
}

#[test]
fn local_child_runtime_capability_is_allowed_for_active_clean_family() {
    let decision =
        evaluate_entitlement_capability(entitlement_input(EntitlementCapabilityScope::LocalChildRuntime));

    assert_eq!(decision.capability, EntitlementCapability::Tracking);
    assert_eq!(decision.access_state, EntitlementCapabilityAccessState::Allowed);
    assert_eq!(decision.manual_review_state, EntitlementManualReviewState::NotRequired);
}

#[test]
fn parent_portal_only_scope_is_blocked_for_child_runtime_capability() {
    let decision =
        evaluate_entitlement_capability(entitlement_input(EntitlementCapabilityScope::ParentPortalOnly));

    assert_eq!(decision.access_state, EntitlementCapabilityAccessState::Blocked);
    assert_eq!(decision.manual_review_state, EntitlementManualReviewState::Required);
}

#[test]
fn capability_request_records_typed_entitlement_decision_event() {
    let request = EntitlementCapabilityEvaluationRequestedEvent {
        aggregate_id: EntitlementAggregateId::parse(ENTITLEMENT_AGGREGATE_ID)
            .expect("entitlement aggregate id"),
        evaluation_id: EntitlementEvaluationId::parse(ENTITLEMENT_EVALUATION_ID)
            .expect("entitlement evaluation id"),
        input: entitlement_input(EntitlementCapabilityScope::LocalChildRuntime),
    };

    let decision = record_entitlement_capability_decision(&request);

    assert_eq!(decision.aggregate_id, request.aggregate_id);
    assert_eq!(decision.source_evaluation_id, request.evaluation_id);
    assert!(
        EntitlementDecisionId::parse(decision.decision_id.as_str()).is_ok(),
        "decision id remains branded"
    );
    assert_eq!(
        request
            .contract()
            .expect("entitlement request contract")
            .event_type
            .as_str(),
        ENTITLEMENT_REQUESTED_EVENT_TYPE
    );
    assert_eq!(
        decision
            .contract()
            .expect("entitlement decision contract")
            .event_type
            .as_str(),
        ENTITLEMENT_DECISION_EVENT_TYPE
    );
}
