use ocentra_billing_core::{
    decide_child_entitlement_snapshot, record_child_entitlement_consumption_event,
    BillingAggregateId, BillingChildDeviceId, BillingChildEntitlementAccessState,
    BillingChildEntitlementConsumptionRecordedEvent, BillingChildEntitlementConsumptionState,
    BillingChildEntitlementRejectionReason, BillingChildEntitlementSnapshot,
    BillingChildEntitlementSnapshotReceivedEvent, BillingChildSnapshotFreshnessState,
    BillingChildSnapshotSignatureState,
    BillingEntitlementSnapshotId, BillingEntitlementWriteState, BillingManualReviewRequirement,
    BillingSubscriptionLifecycleState,
};
use ocentra_eventing::DomainEvent;

const BILLING_AGGREGATE_ID: &str = "billing-household-default";
const CHILD_DEVICE_ID: &str = "child-device-default";
const SNAPSHOT_ID: &str = "billing-snapshot-default";
const SNAPSHOT_RECEIVED_EVENT_TYPE: &str = "billing.child-entitlement-snapshot.received";
const CONSUMPTION_RECORDED_EVENT_TYPE: &str =
    "billing.child-entitlement-consumption.recorded";

fn snapshot(
    lifecycle_state: BillingSubscriptionLifecycleState,
    signature_state: BillingChildSnapshotSignatureState,
    freshness_state: BillingChildSnapshotFreshnessState,
) -> BillingChildEntitlementSnapshot {
    BillingChildEntitlementSnapshot {
        snapshot_id: BillingEntitlementSnapshotId::parse(SNAPSHOT_ID)
            .expect("billing entitlement snapshot id"),
        child_device_id: BillingChildDeviceId::parse(CHILD_DEVICE_ID)
            .expect("billing child device id"),
        lifecycle_state,
        signature_state,
        freshness_state,
    }
}

#[test]
fn trusted_active_child_snapshot_grants_full_access() {
    let decision = decide_child_entitlement_snapshot(snapshot(
        BillingSubscriptionLifecycleState::Active,
        BillingChildSnapshotSignatureState::Trusted,
        BillingChildSnapshotFreshnessState::Fresh,
    ));

    assert_eq!(
        decision.decision_state,
        BillingChildEntitlementConsumptionState::Accepted
    );
    assert_eq!(decision.access_state, BillingChildEntitlementAccessState::FullAccess);
    assert_eq!(decision.write_state, BillingEntitlementWriteState::WriteRequired);
    assert_eq!(
        decision.manual_review_requirement,
        BillingManualReviewRequirement::NotRequired
    );
    assert_eq!(decision.rejection_reason, None);
}

#[test]
fn stale_child_snapshot_is_rejected_without_overwriting_local_state() {
    let decision = decide_child_entitlement_snapshot(snapshot(
        BillingSubscriptionLifecycleState::Active,
        BillingChildSnapshotSignatureState::Trusted,
        BillingChildSnapshotFreshnessState::Stale,
    ));

    assert_eq!(
        decision.decision_state,
        BillingChildEntitlementConsumptionState::Rejected
    );
    assert_eq!(decision.access_state, BillingChildEntitlementAccessState::NoChange);
    assert_eq!(decision.write_state, BillingEntitlementWriteState::DoNotWrite);
    assert_eq!(
        decision.manual_review_requirement,
        BillingManualReviewRequirement::Required
    );
    assert_eq!(
        decision.rejection_reason,
        Some(BillingChildEntitlementRejectionReason::StaleSnapshot)
    );
}

#[test]
fn invalid_signature_child_snapshot_is_rejected_before_lifecycle_changes() {
    let decision = decide_child_entitlement_snapshot(snapshot(
        BillingSubscriptionLifecycleState::PastDue,
        BillingChildSnapshotSignatureState::Invalid,
        BillingChildSnapshotFreshnessState::Fresh,
    ));

    assert_eq!(
        decision.decision_state,
        BillingChildEntitlementConsumptionState::Rejected
    );
    assert_eq!(decision.access_state, BillingChildEntitlementAccessState::NoChange);
    assert_eq!(
        decision.rejection_reason,
        Some(BillingChildEntitlementRejectionReason::InvalidSignature)
    );
}

#[test]
fn consumption_record_projects_typed_domain_events() {
    let received = BillingChildEntitlementSnapshotReceivedEvent {
        aggregate_id: BillingAggregateId::parse(BILLING_AGGREGATE_ID)
            .expect("billing aggregate id"),
        snapshot: snapshot(
            BillingSubscriptionLifecycleState::Canceled,
            BillingChildSnapshotSignatureState::Trusted,
            BillingChildSnapshotFreshnessState::Fresh,
        ),
    };

    let recorded: BillingChildEntitlementConsumptionRecordedEvent =
        record_child_entitlement_consumption_event(received.clone());

    assert_eq!(
        received
            .contract()
            .expect("billing child snapshot contract")
            .event_type
            .as_str(),
        SNAPSHOT_RECEIVED_EVENT_TYPE
    );
    assert_eq!(
        recorded
            .contract()
            .expect("billing child consumption contract")
            .event_type
            .as_str(),
        CONSUMPTION_RECORDED_EVENT_TYPE
    );
    assert_eq!(recorded.aggregate_id, received.aggregate_id);
    assert_eq!(
        recorded.decision.access_state,
        BillingChildEntitlementAccessState::Revoked
    );
}
