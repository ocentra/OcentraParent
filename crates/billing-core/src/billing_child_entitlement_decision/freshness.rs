use crate::billing_child_entitlement::{
    BillingChildEntitlementConsumptionDecision, BillingChildEntitlementRejectionReason,
    BillingChildSnapshotFreshnessState,
};
use crate::billing_subscription::BillingSubscriptionStatus;

use super::construct;

pub(crate) fn decide_trusted_child_entitlement_snapshot(
    snapshot_id: crate::billing_child_entitlement::BillingEntitlementSnapshotId,
    child_device_id: crate::billing_child_entitlement::BillingChildDeviceId,
    freshness_state: BillingChildSnapshotFreshnessState,
    subscription_status: BillingSubscriptionStatus,
) -> BillingChildEntitlementConsumptionDecision {
    match freshness_state {
        BillingChildSnapshotFreshnessState::Stale => construct::rejected(
            snapshot_id,
            child_device_id,
            BillingChildEntitlementRejectionReason::StaleSnapshot,
            subscription_status,
        ),
        BillingChildSnapshotFreshnessState::Expired => construct::rejected(
            snapshot_id,
            child_device_id,
            BillingChildEntitlementRejectionReason::ExpiredSnapshot,
            subscription_status,
        ),
        BillingChildSnapshotFreshnessState::Fresh => {
            decide_fresh_trusted_child_entitlement_snapshot(
                snapshot_id,
                child_device_id,
                subscription_status,
            )
        }
    }
}

fn decide_fresh_trusted_child_entitlement_snapshot(
    snapshot_id: crate::billing_child_entitlement::BillingEntitlementSnapshotId,
    child_device_id: crate::billing_child_entitlement::BillingChildDeviceId,
    subscription_status: BillingSubscriptionStatus,
) -> BillingChildEntitlementConsumptionDecision {
    match subscription_status {
        BillingSubscriptionStatus::Unknown => construct::rejected(
            snapshot_id,
            child_device_id,
            BillingChildEntitlementRejectionReason::UnknownSubscriptionStatus,
            subscription_status,
        ),
        BillingSubscriptionStatus::Unavailable => construct::rejected(
            snapshot_id,
            child_device_id,
            BillingChildEntitlementRejectionReason::UnavailableSubscriptionStatus,
            subscription_status,
        ),
        BillingSubscriptionStatus::Trialing | BillingSubscriptionStatus::Active => {
            construct::accepted(
                snapshot_id,
                child_device_id,
                subscription_status,
                crate::billing_child_entitlement::BillingChildEntitlementAccessState::FullAccess,
            )
        }
        BillingSubscriptionStatus::Grace => construct::accepted(
            snapshot_id,
            child_device_id,
            subscription_status,
            crate::billing_child_entitlement::BillingChildEntitlementAccessState::GraceAccess,
        ),
        BillingSubscriptionStatus::PastDue => construct::accepted(
            snapshot_id,
            child_device_id,
            subscription_status,
            crate::billing_child_entitlement::BillingChildEntitlementAccessState::LimitedAccess,
        ),
        BillingSubscriptionStatus::Cancelled | BillingSubscriptionStatus::Expired => {
            construct::accepted(
                snapshot_id,
                child_device_id,
                subscription_status,
                crate::billing_child_entitlement::BillingChildEntitlementAccessState::Revoked,
            )
        }
    }
}
