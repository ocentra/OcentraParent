use ocentra_entitlement_core::entitlement_access::{
    record_entitlement_capability_decision, EntitlementAggregateId, EntitlementCapability,
    EntitlementCapabilityAccessState, EntitlementCapabilityDecisionRecordedEvent,
    EntitlementCapabilityEvaluationRequestedEvent, EntitlementCapabilityRejectionReason,
    EntitlementEvaluationId, EntitlementManualReviewState,
};
use ocentra_eventing::envelope::DomainEvent;
use ocentra_eventing::error::EventingError;
use serde_json::json;

const REQUESTED_EVENT_TYPE: &str = "entitlement.capability-evaluation.requested";
const DECISION_EVENT_TYPE: &str = "entitlement.capability-decision.recorded";

#[test]
fn capability_request_records_fail_closed_typed_decision_event() -> Result<(), EventingError> {
    let request = EntitlementCapabilityEvaluationRequestedEvent {
        aggregate_id: EntitlementAggregateId::parse("entitlement-household-default")?,
        evaluation_id: EntitlementEvaluationId::parse("entitlement-evaluation-default")?,
        input: public_input(),
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
        EntitlementCapabilityAccessState::Blocked
    );
    assert_eq!(
        decision.decision.manual_review_state,
        EntitlementManualReviewState::Required
    );
    assert_eq!(
        decision.decision.rejection_reason,
        Some(EntitlementCapabilityRejectionReason::MissingSignature)
    );
    assert_eq!(
        request.contract()?.event_type.as_str(),
        REQUESTED_EVENT_TYPE
    );
    assert_eq!(
        decision.contract()?.event_type.as_str(),
        DECISION_EVENT_TYPE
    );
    Ok(())
}

#[test]
fn decision_event_keeps_typed_ids_and_stable_idempotency_keys() -> Result<(), EventingError> {
    let request = EntitlementCapabilityEvaluationRequestedEvent {
        aggregate_id: EntitlementAggregateId::parse("entitlement-household-default")?,
        evaluation_id: EntitlementEvaluationId::parse("entitlement-evaluation-default")?,
        input: public_input(),
    };
    let decision: EntitlementCapabilityDecisionRecordedEvent =
        record_entitlement_capability_decision(&request);

    assert_eq!(
        request.idempotency_key()?.as_str(),
        "entitlement.capability-evaluation.requested:entitlement-evaluation-default"
    );
    assert_eq!(
        decision.idempotency_key()?.as_str(),
        "entitlement.capability-decision.recorded:entitlement-decision:entitlement-evaluation-default"
    );
    assert_eq!(
        decision.aggregate_key()?.as_str(),
        "entitlement-household-default"
    );
    assert_eq!(
        decision.decision.rejection_reason,
        Some(EntitlementCapabilityRejectionReason::MissingSignature)
    );
    Ok(())
}

fn public_input() -> ocentra_entitlement_core::entitlement_access::EntitlementCapabilityInput {
    serde_json::from_value(json!({
        "capability": "tracking",
        "subscription_state": "active",
        "offline_grace_state": "inactive",
        "family_setup_state": "complete",
        "policy_state": "clean",
        "capability_scope": "local-child-runtime"
    }))
    .expect("public capability input decodes")
}
