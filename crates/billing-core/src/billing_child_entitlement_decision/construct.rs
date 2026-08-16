use crate::billing_child_entitlement::{
    BillingChildEntitlementAccessState, BillingChildEntitlementConsumptionDecision,
    BillingChildEntitlementConsumptionState, BillingChildEntitlementRejectionReason,
};
use crate::billing_subscription::{
    BillingEntitlementWriteState, BillingManualReviewRequirement, BillingSubscriptionStatus,
};

pub(crate) fn accepted(
    snapshot_id: crate::billing_child_entitlement::BillingEntitlementSnapshotId,
    child_device_id: crate::billing_child_entitlement::BillingChildDeviceId,
    subscription_status: BillingSubscriptionStatus,
    access_state: BillingChildEntitlementAccessState,
) -> BillingChildEntitlementConsumptionDecision {
    BillingChildEntitlementConsumptionDecision {
        snapshot_id,
        child_device_id,
        decision_state: BillingChildEntitlementConsumptionState::Accepted,
        subscription_status,
        access_state,
        write_state: BillingEntitlementWriteState::WriteRequired,
        manual_review_requirement: BillingManualReviewRequirement::NotRequired,
        rejection_reason: None,
    }
}

pub(crate) fn rejected(
    snapshot_id: crate::billing_child_entitlement::BillingEntitlementSnapshotId,
    child_device_id: crate::billing_child_entitlement::BillingChildDeviceId,
    rejection_reason: BillingChildEntitlementRejectionReason,
    subscription_status: BillingSubscriptionStatus,
) -> BillingChildEntitlementConsumptionDecision {
    BillingChildEntitlementConsumptionDecision {
        snapshot_id,
        child_device_id,
        decision_state: BillingChildEntitlementConsumptionState::Rejected,
        subscription_status,
        access_state: BillingChildEntitlementAccessState::NoChange,
        write_state: BillingEntitlementWriteState::DoNotWrite,
        manual_review_requirement: BillingManualReviewRequirement::Required,
        rejection_reason: Some(rejection_reason),
    }
}
