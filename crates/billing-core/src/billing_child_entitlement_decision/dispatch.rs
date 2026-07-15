use super::{construct, freshness};
use crate::billing_child_entitlement::{
    BillingChildEntitlementConsumptionDecision, BillingChildEntitlementRejectionReason,
    BillingChildEntitlementSnapshot, BillingChildSnapshotSignatureState,
};

pub(crate) fn decide_child_entitlement_snapshot(
    snapshot: BillingChildEntitlementSnapshot,
) -> BillingChildEntitlementConsumptionDecision {
    let BillingChildEntitlementSnapshot {
        snapshot_id,
        child_device_id,
        subscription_status,
        signature_state,
        freshness_state,
    } = snapshot;

    match signature_state {
        BillingChildSnapshotSignatureState::Missing => construct::rejected(
            snapshot_id,
            child_device_id,
            BillingChildEntitlementRejectionReason::MissingSignature,
            subscription_status,
        ),
        BillingChildSnapshotSignatureState::Invalid => construct::rejected(
            snapshot_id,
            child_device_id,
            BillingChildEntitlementRejectionReason::InvalidSignature,
            subscription_status,
        ),
        BillingChildSnapshotSignatureState::Trusted => {
            freshness::decide_trusted_child_entitlement_snapshot(
                snapshot_id,
                child_device_id,
                freshness_state,
                subscription_status,
            )
        }
    }
}
