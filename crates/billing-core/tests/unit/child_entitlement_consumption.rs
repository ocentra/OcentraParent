use ocentra_billing_core::billing_child_entitlement::{
    decide_child_entitlement_snapshot, record_child_entitlement_consumption_event,
    BillingChildDeviceId, BillingChildEntitlementAccessState,
    BillingChildEntitlementConsumptionRecordedEvent, BillingChildEntitlementConsumptionState,
    BillingChildEntitlementRejectionReason, BillingChildEntitlementSnapshot,
    BillingChildEntitlementSnapshotReceivedEvent, BillingChildSnapshotFreshnessState,
    BillingChildSnapshotSignatureState, BillingEntitlementSnapshotId,
};
use ocentra_billing_core::billing_subscription::{
    BillingAggregateId, BillingEntitlementWriteState, BillingManualReviewRequirement,
    BillingSubscriptionStatus,
};
use ocentra_eventing::envelope::DomainEvent;
use ocentra_eventing::expect_value::ExpectValue;

const BILLING_AGGREGATE_ID: &str = "billing-household-default";
const CHILD_DEVICE_ID: &str = "child-device-default";
const SNAPSHOT_ID: &str = "billing-snapshot-default";
const SNAPSHOT_RECEIVED_EVENT_TYPE: &str = "billing.child-entitlement-snapshot.received";
const CONSUMPTION_RECORDED_EVENT_TYPE: &str = "billing.child-entitlement-consumption.recorded";

fn snapshot(
    subscription_status: BillingSubscriptionStatus,
    signature_state: BillingChildSnapshotSignatureState,
    freshness_state: BillingChildSnapshotFreshnessState,
) -> BillingChildEntitlementSnapshot {
    BillingChildEntitlementSnapshot {
        snapshot_id: BillingEntitlementSnapshotId::parse(SNAPSHOT_ID)
            .expect_value("billing entitlement snapshot id"),
        child_device_id: BillingChildDeviceId::parse(CHILD_DEVICE_ID)
            .expect_value("billing child device id"),
        subscription_status,
        signature_state,
        freshness_state,
    }
}

#[test]
fn trusted_active_child_snapshot_grants_full_access() {
    let decision = decide_child_entitlement_snapshot(snapshot(
        BillingSubscriptionStatus::Active,
        BillingChildSnapshotSignatureState::Trusted,
        BillingChildSnapshotFreshnessState::Fresh,
    ));

    assert_eq!(
        decision.decision_state,
        BillingChildEntitlementConsumptionState::Accepted
    );
    assert_eq!(
        decision.access_state,
        BillingChildEntitlementAccessState::FullAccess
    );
    assert_eq!(
        decision.write_state,
        BillingEntitlementWriteState::WriteRequired
    );
    assert_eq!(
        decision.manual_review_requirement,
        BillingManualReviewRequirement::NotRequired
    );
    assert_eq!(decision.rejection_reason, None);
}

#[test]
fn trusted_grace_child_snapshot_projects_grace_access() {
    let decision = decide_child_entitlement_snapshot(snapshot(
        BillingSubscriptionStatus::Grace,
        BillingChildSnapshotSignatureState::Trusted,
        BillingChildSnapshotFreshnessState::Fresh,
    ));

    assert_eq!(
        decision.decision_state,
        BillingChildEntitlementConsumptionState::Accepted
    );
    assert_eq!(
        decision.access_state,
        BillingChildEntitlementAccessState::GraceAccess
    );
    assert_eq!(
        decision.write_state,
        BillingEntitlementWriteState::WriteRequired
    );
}

#[test]
fn stale_child_snapshot_is_rejected_without_overwriting_local_state() {
    let decision = decide_child_entitlement_snapshot(snapshot(
        BillingSubscriptionStatus::Active,
        BillingChildSnapshotSignatureState::Trusted,
        BillingChildSnapshotFreshnessState::Stale,
    ));

    assert_eq!(
        decision.decision_state,
        BillingChildEntitlementConsumptionState::Rejected
    );
    assert_eq!(
        decision.access_state,
        BillingChildEntitlementAccessState::NoChange
    );
    assert_eq!(
        decision.write_state,
        BillingEntitlementWriteState::DoNotWrite
    );
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
fn missing_signature_child_snapshot_is_rejected_before_local_access_changes() {
    let decision = decide_child_entitlement_snapshot(snapshot(
        BillingSubscriptionStatus::Active,
        BillingChildSnapshotSignatureState::Missing,
        BillingChildSnapshotFreshnessState::Fresh,
    ));

    assert_eq!(
        decision.decision_state,
        BillingChildEntitlementConsumptionState::Rejected
    );
    assert_eq!(
        decision.access_state,
        BillingChildEntitlementAccessState::NoChange
    );
    assert_eq!(
        decision.write_state,
        BillingEntitlementWriteState::DoNotWrite
    );
    assert_eq!(
        decision.manual_review_requirement,
        BillingManualReviewRequirement::Required
    );
    assert_eq!(
        decision.rejection_reason,
        Some(BillingChildEntitlementRejectionReason::MissingSignature)
    );
}

#[test]
fn invalid_signature_child_snapshot_is_rejected_before_lifecycle_changes() {
    let decision = decide_child_entitlement_snapshot(snapshot(
        BillingSubscriptionStatus::PastDue,
        BillingChildSnapshotSignatureState::Invalid,
        BillingChildSnapshotFreshnessState::Fresh,
    ));

    assert_eq!(
        decision.decision_state,
        BillingChildEntitlementConsumptionState::Rejected
    );
    assert_eq!(
        decision.access_state,
        BillingChildEntitlementAccessState::NoChange
    );
    assert_eq!(
        decision.rejection_reason,
        Some(BillingChildEntitlementRejectionReason::InvalidSignature)
    );
}

#[test]
fn expired_child_snapshot_is_rejected_without_overwriting_local_state() {
    let decision = decide_child_entitlement_snapshot(snapshot(
        BillingSubscriptionStatus::Active,
        BillingChildSnapshotSignatureState::Trusted,
        BillingChildSnapshotFreshnessState::Expired,
    ));

    assert_eq!(
        decision.decision_state,
        BillingChildEntitlementConsumptionState::Rejected
    );
    assert_eq!(
        decision.access_state,
        BillingChildEntitlementAccessState::NoChange
    );
    assert_eq!(
        decision.write_state,
        BillingEntitlementWriteState::DoNotWrite
    );
    assert_eq!(
        decision.manual_review_requirement,
        BillingManualReviewRequirement::Required
    );
    assert_eq!(
        decision.rejection_reason,
        Some(BillingChildEntitlementRejectionReason::ExpiredSnapshot)
    );
}

#[test]
fn unknown_subscription_status_child_snapshot_is_rejected_even_when_signature_and_freshness_are_valid(
) {
    let decision = decide_child_entitlement_snapshot(snapshot(
        BillingSubscriptionStatus::Unknown,
        BillingChildSnapshotSignatureState::Trusted,
        BillingChildSnapshotFreshnessState::Fresh,
    ));

    assert_eq!(
        decision.decision_state,
        BillingChildEntitlementConsumptionState::Rejected
    );
    assert_eq!(
        decision.access_state,
        BillingChildEntitlementAccessState::NoChange
    );
    assert_eq!(
        decision.write_state,
        BillingEntitlementWriteState::DoNotWrite
    );
    assert_eq!(
        decision.manual_review_requirement,
        BillingManualReviewRequirement::Required
    );
    assert_eq!(
        decision.rejection_reason,
        Some(BillingChildEntitlementRejectionReason::UnknownSubscriptionStatus)
    );
}

#[test]
fn unavailable_subscription_status_child_snapshot_is_rejected_without_overwriting_local_state() {
    let decision = decide_child_entitlement_snapshot(snapshot(
        BillingSubscriptionStatus::Unavailable,
        BillingChildSnapshotSignatureState::Trusted,
        BillingChildSnapshotFreshnessState::Fresh,
    ));

    assert_eq!(
        decision.decision_state,
        BillingChildEntitlementConsumptionState::Rejected
    );
    assert_eq!(
        decision.access_state,
        BillingChildEntitlementAccessState::NoChange
    );
    assert_eq!(
        decision.write_state,
        BillingEntitlementWriteState::DoNotWrite
    );
    assert_eq!(
        decision.manual_review_requirement,
        BillingManualReviewRequirement::Required
    );
    assert_eq!(
        decision.rejection_reason,
        Some(BillingChildEntitlementRejectionReason::UnavailableSubscriptionStatus)
    );
}

#[test]
fn cancelled_child_snapshot_revokes_access() {
    let decision = decide_child_entitlement_snapshot(snapshot(
        BillingSubscriptionStatus::Cancelled,
        BillingChildSnapshotSignatureState::Trusted,
        BillingChildSnapshotFreshnessState::Fresh,
    ));

    assert_eq!(
        decision.decision_state,
        BillingChildEntitlementConsumptionState::Accepted
    );
    assert_eq!(
        decision.access_state,
        BillingChildEntitlementAccessState::Revoked
    );
    assert_eq!(
        decision.write_state,
        BillingEntitlementWriteState::WriteRequired
    );
}

#[test]
fn consumption_record_projects_typed_domain_events() {
    let received = BillingChildEntitlementSnapshotReceivedEvent {
        aggregate_id: BillingAggregateId::parse(BILLING_AGGREGATE_ID)
            .expect_value("billing aggregate id"),
        snapshot: snapshot(
            BillingSubscriptionStatus::Cancelled,
            BillingChildSnapshotSignatureState::Trusted,
            BillingChildSnapshotFreshnessState::Fresh,
        ),
    };

    let recorded: BillingChildEntitlementConsumptionRecordedEvent =
        record_child_entitlement_consumption_event(received.clone());

    assert_eq!(
        received
            .contract()
            .expect_value("billing child snapshot contract")
            .event_type
            .as_str(),
        SNAPSHOT_RECEIVED_EVENT_TYPE
    );
    assert_eq!(
        recorded
            .contract()
            .expect_value("billing child consumption contract")
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
